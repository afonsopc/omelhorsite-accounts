use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    email::send_email,
    get_decode_verify_and_return_session_token,
    models::{BeginEmailChangeRequest, FinishEmailChangeRequest},
    prelude::*,
    random::get_random_numbers,
    string_to_email_placeholder,
};
use tide::{Response, StatusCode};
use validator::Validate;

#[tracing::instrument]
pub async fn begin_email_change(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: BeginEmailChangeRequest = req.body_json().await?;

    if body.validate().is_err() {
        let response = Response::new(StatusCode::UnprocessableEntity);
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        _ => {
            let response = Response::new(StatusCode::Unauthorized);
            return Ok(response);
        }
    };

    let session = session_token.session;

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let account_id = session.account_id;

    // CHECK IF EMAIL IS ALREADY IN USE BY A VERIFIED ACCOUNT

    let query = sqlx::query!(
        r#"
            SELECT id 
            FROM accounts 
            WHERE email = $1;
        "#,
        &body.email
    );

    if query.fetch_optional(&mut *transaction).await?.is_some() {
        let response = Response::new(StatusCode::Conflict);
        return Ok(response);
    };

    // GENERATE BOTH CODES FOR VERIFICATION

    let original_email_verification_code = get_random_numbers(CONFIG.verification_code_length);
    let new_email_verification_code = get_random_numbers(CONFIG.verification_code_length);

    let timestamp = chrono::Utc::now().naive_utc();

    // INSERT CODES INTO DATABASE

    let query = sqlx::query!(
        r#"
            UPDATE accounts 
            SET original_email_verification_code = $1, 
                new_email_verification_code = $2, 
                email_verification_codes_created_at = $3
            WHERE id = $4;
        "#,
        original_email_verification_code,
        new_email_verification_code,
        timestamp,
        account_id,
    );

    let result = query.execute(&mut *transaction).await?;
    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // GET ACCOUNT HANDLE

    let query = sqlx::query!(
        r#"
            SELECT handle, email
            FROM accounts 
            WHERE id = $1;
        "#,
        account_id
    );

    let result = query.fetch_one(&mut *transaction).await?;

    let handle = result.handle;
    let email = result.email;

    // REPLACE VERIFICATION CODE PLACEHOLDER IN EMAIL BODY
    // WITH THE ACTUAL VERIFICATION CODE

    let verification_code_placeholder = string_to_email_placeholder("verification_code");
    let handle_placeholder = string_to_email_placeholder("handle");
    let new_email_placeholder = string_to_email_placeholder("new_email");

    let original_body_with_placeholders_replaced = &CONFIG
        .account_creation_verification_email_body
        .replace(&handle_placeholder, &f!("@{}", &handle))
        .replace(&new_email_placeholder, &body.email)
        .replace(
            &verification_code_placeholder,
            &original_email_verification_code,
        );

    let new_body_with_placeholders_replaced = &CONFIG
        .account_creation_verification_email_body
        .replace(&handle_placeholder, &f!("@{}", &handle))
        .replace(&verification_code_placeholder, &new_email_verification_code);

    // SEND EMAIL TO ORIGINAL EMAIL

    send_email(
        &email,
        &CONFIG.account_creation_verification_email_subject,
        original_body_with_placeholders_replaced,
        CONFIG.account_email_change_original_email_verification_html,
    )?;

    // SEND EMAIL TO NEW EMAIL

    send_email(
        &body.email,
        &CONFIG.account_creation_verification_email_subject,
        new_body_with_placeholders_replaced,
        CONFIG.account_email_change_new_email_verification_html,
    )?;

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

#[tracing::instrument]
pub async fn finish_email_change(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: FinishEmailChangeRequest = req.body_json().await?;

    if body.validate().is_err() {
        let response = Response::new(StatusCode::UnprocessableEntity);
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        _ => {
            let response = Response::new(StatusCode::Unauthorized);
            return Ok(response);
        }
    };

    let session = session_token.session;

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let account_id = session.account_id;

    // GET VERIFICATION CODES FROM DATABASE

    let query = sqlx::query!(
        r#"
            SELECT original_email_verification_code, 
                   new_email_verification_code, 
                   email_verification_codes_created_at
            FROM accounts 
            WHERE id = $1;
        "#,
        account_id
    );

    let result = match query.fetch_optional(&mut *transaction).await? {
        Some(result) => result,
        None => {
            let response = Response::new(StatusCode::Unauthorized);
            return Ok(response);
        }
    };

    // GET CODES AND TIMESTAMP

    let (
        original_email_verification_code,
        new_email_verification_code,
        email_verification_codes_created_at,
    ) = match (
        result.original_email_verification_code,
        result.new_email_verification_code,
        result.email_verification_codes_created_at,
    ) {
        (
            Some(original_email_verification_code),
            Some(new_email_verification_code),
            Some(email_verification_codes_created_at),
        ) => (
            original_email_verification_code,
            new_email_verification_code,
            email_verification_codes_created_at,
        ),
        _ => {
            let response = Response::new(StatusCode::Unauthorized);
            return Ok(response);
        }
    };

    // CHECK IF TIMESTAMP IS EXPIRED

    let timestamp = chrono::Utc::now().naive_utc();

    let timestamp_difference = timestamp - email_verification_codes_created_at;

    if timestamp_difference < chrono::Duration::seconds(0) {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // CHECK IF CODES MATCH

    if original_email_verification_code != body.original_email_verification_code
        || new_email_verification_code != body.new_email_verification_code
    {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // SET CODES AND TIMESTAMP TO NULL

    let query = sqlx::query!(
        r#"
            UPDATE accounts 
            SET original_email_verification_code = NULL, 
                new_email_verification_code = NULL, 
                email_verification_codes_created_at = NULL
            WHERE id = $1;
        "#,
        account_id,
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // UPDATE ACCOUNT EMAIL

    let query = sqlx::query!(
        r#"
            UPDATE accounts 
            SET email = $1
            WHERE id = $2;
        "#,
        body.email,
        account_id,
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // DELETE ALL SESSIONS FOR ACCOUNT

    let query = sqlx::query!(
        r#"
            DELETE FROM sessions
            WHERE account_id = $1;
        "#,
        account_id,
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() < 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    Ok(Response::new(StatusCode::Ok))
}
