use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    email::send_email,
    encryption,
    models::{
        Account, AccountCreationVerification, BeginAccountCreationRequest, ConflictString,
        FinishAccountCreationRequest, Group,
    },
    prelude::*,
    random::{self, get_random_string},
};
use chrono::Utc;
use tide::{convert::json, Response, StatusCode};
use validator::Validate;

#[tracing::instrument]
pub async fn begin_account_creation(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: BeginAccountCreationRequest = req.body_json().await?;

    if body.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(body.validate().unwrap_err());
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // CHECK IF EMAIL OR HANDLE IS ALREADY IN USE BY A VERIFIED ACCOUNT

    let query = sqlx::query!(
        r#"
        SELECT 
            EXISTS(
                SELECT 1 FROM accounts WHERE email = $1
            ) AS email_exists,
            EXISTS(
                SELECT 1 FROM accounts WHERE handle = $2
            ) AS handle_exists
        "#,
        &body.email,
        &body.handle
    );

    let result = query.fetch_one(&mut *transaction).await?;

    match (result.email_exists, result.handle_exists) {
        (Some(email_exists), Some(handle_exists)) => {
            if email_exists || handle_exists {
                transaction.rollback().await?;

                let conflict_string = ConflictString {
                    conflict: if email_exists {
                        "email".to_string()
                    } else {
                        "handle".to_string()
                    },
                };

                let response = Response::builder(StatusCode::Conflict)
                    .body(json!(conflict_string))
                    .build();

                return Ok(response);
            }
        }
        _ => {
            transaction.rollback().await?;
            let response = Response::new(StatusCode::InternalServerError);
            return Ok(response);
        }
    }

    // GENERATE VERIFICATION CODE

    let verification_code = random::get_random_numbers(CONFIG.verification_code_length);

    // INSERT ACCOUNT CREATION VERIFICATION QUERY

    let account_creation_verification = AccountCreationVerification {
        email: body.email.to_owned(),
        handle: body.handle,
        verification_code,
        verification_code_created_at: Utc::now().naive_utc(),
    };

    let query = sqlx::query!(
        r#"
        INSERT INTO account_creation_verifications (
            "email",
            "handle",
            "verification_code",
            "verification_code_created_at"
        )
        VALUES ($1, $2, $3, $4)
        "#,
        account_creation_verification.email,
        account_creation_verification.handle,
        account_creation_verification.verification_code,
        account_creation_verification.verification_code_created_at
    );

    query.execute(&mut *transaction).await?;

    // REPLACE VERIFICATION CODE PLACEHOLDER IN EMAIL BODY
    // WITH THE ACTUAL VERIFICATION CODE

    let verification_code_placeholder = format!(
        "{}{}{}",
        CONFIG.email_placeholder_marker, "verification_code", CONFIG.email_placeholder_marker
    );

    let handle_placeholder = format!(
        "{}{}{}",
        CONFIG.email_placeholder_marker, "handle", CONFIG.email_placeholder_marker
    );
    let body_with_placeholders_replaced = &CONFIG
        .account_creation_verification_email_body
        .replace(
            &handle_placeholder,
            &f!("@{}", &account_creation_verification.handle),
        )
        .replace(
            &verification_code_placeholder,
            &account_creation_verification.verification_code,
        );

    // SEND VERIFICATION CODE TO EMAIL

    send_email(
        &body.email,
        &CONFIG.account_creation_verification_email_subject,
        body_with_placeholders_replaced,
        CONFIG.account_creation_verification_email_html,
    )?;

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;
    Ok(Response::new(StatusCode::Ok))
}

#[tracing::instrument]
pub async fn finish_account_creation(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: FinishAccountCreationRequest = req.body_json().await?;

    if body.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(body.validate().unwrap_err());
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // CHECK IF THERE IS A VERIFICATION CODE FOR THE GIVEN EMAIL AND HANDLE

    let query = sqlx::query!(
        r#"
            SELECT EXISTS(
                SELECT 1
                FROM account_creation_verifications
                WHERE email = $1 AND handle = $2 AND verification_code = $3
            ) AS exists
        "#,
        &body.email,
        &body.handle,
        &body.verification_code
    );

    let result = query.fetch_one(&mut *transaction).await?;

    if let Some(false) = result.exists {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::NotFound);
        return Ok(response);
    }

    // GENERATE ACCOUNT ID

    let id = get_random_string(CONFIG.account_id_length);

    // Encrypt password

    let encrypted_password = encryption::encrypt_string(&body.password)?;

    // INSERT ACCOUNT QUERY

    let account = Account {
        id,
        picture_id: CONFIG.default_picture.to_owned(),
        handle: body.handle.to_owned(),
        name: body.name,
        email: body.email.to_owned(),
        password: encrypted_password,
        group: Group::Default,
        gender: body.gender,
        theme: body.theme,
        language: body.language,
        created_at: Utc::now().naive_utc(),
        original_email_verification_code: None,
        new_email_verification_code: None,
        email_verification_codes_created_at: None,
        new_password_verification_code: None,
        new_password_verification_code_created_at: None,
    };

    let mut transaction = DATABASE_POOL.begin().await?;

    let query = sqlx::query!(
        r#"
        INSERT INTO accounts (
            "id",
            "picture_id",
            "handle",
            "name",
            "email",
            "password",
            "group",
            "gender",
            "theme",
            "language",
            "created_at"
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        account.id.to_string(),
        account.picture_id,
        account.handle,
        account.name,
        account.email,
        account.password,
        account.group.to_string(),
        account.gender.to_string(),
        account.theme.to_string(),
        account.language,
        account.created_at,
    );

    query.execute(&mut *transaction).await?;

    // DELETE ALL ROWS FROM account_creation_verifications TABLE
    // WHERE EMAIL OR HANDLE IS EQUAL TO THE ONES
    // USED IN THIS NEWLY CREATED ACCOUNT

    let query = sqlx::query!(
        r#"
        DELETE FROM account_creation_verifications
        WHERE email = $1 OR handle = $2
        "#,
        body.email,
        body.handle,
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;
    Ok(Response::new(StatusCode::Ok))
}
