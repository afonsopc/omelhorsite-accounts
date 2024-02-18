use tide::{Response, StatusCode};
use validator::Validate;

use crate::{
    config::CONFIG, database::DATABASE_POOL, email::send_email, encryption,
    get_decode_verify_and_return_session_token, models::FinishPasswordChangeRequest,
    random::get_random_numbers, string_to_email_placeholder,
};

pub async fn begin_password_change(req: tide::Request<()>) -> tide::Result {
    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            return Ok(response);
        }
    };

    let session = session_token.session;

    // GET ACCOUNT ID FROM TOKEN

    let account_id = session.account_id;

    // GENERATE VERIFICATION CODE

    let verification_code = get_random_numbers(CONFIG.verification_code_length);

    // GET TIMESTAMP

    let timestamp = chrono::Utc::now().naive_utc();

    // INSERT VERIFICATION CODE INTO ACCOUNT

    let query = sqlx::query!(
        r#"
            UPDATE accounts
            SET 
                new_password_verification_code = $1, 
                new_password_verification_code_created_at = $2
            WHERE id = $3;
        "#,
        &verification_code,
        &timestamp,
        &account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::NotFound);
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

    let body_with_placeholders_replaced = CONFIG
        .account_creation_verification_email_body
        .replace(&verification_code_placeholder, &verification_code)
        .replace(&handle_placeholder, &handle);

    // SEND EMAIL

    send_email(
        &email,
        &CONFIG.account_password_change_verification_subject,
        &body_with_placeholders_replaced,
        CONFIG.account_password_change_verification_html,
    )?;

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

pub async fn finish_password_change(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: FinishPasswordChangeRequest = req.body_json().await?;

    if body.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(body.validate().unwrap_err());
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            return Ok(response);
        }
    };

    let session = session_token.session;

    // GET ACCOUNT ID FROM TOKEN

    let account_id = session.account_id;

    // GET VERIFICATION CODE FROM DATABASE

    let query = sqlx::query!(
        r#"
            SELECT new_password_verification_code, new_password_verification_code_created_at
            FROM accounts
            WHERE id = $1;
        "#,
        account_id
    );

    let result = query.fetch_one(&mut *transaction).await?;

    let verification_code = result.new_password_verification_code;
    let verification_code_created_at = result.new_password_verification_code_created_at;

    // CHECK IF VERIFICATION CODE IS VALID

    if verification_code.is_none() || verification_code_created_at.is_none() {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::NotFound);
        return Ok(response);
    }

    let verification_code = verification_code.unwrap();
    let verification_code_created_at = verification_code_created_at.unwrap();

    let timestamp = chrono::Utc::now().naive_utc();

    let timestamp_difference = timestamp - verification_code_created_at;

    if timestamp_difference < chrono::Duration::seconds(0) {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // CHECK IF VERIFICATION CODE IS CORRECT

    if verification_code != body.verification_code {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // ENCRYPT PASSWORD

    let encrypted_password = encryption::encrypt_string(&body.password)?;

    // UPDATE ACCOUNT PASSWORD

    let query = sqlx::query!(
        r#"
            UPDATE accounts
            SET 
                password = $1, 
                new_password_verification_code = NULL, 
                new_password_verification_code_created_at = NULL
            WHERE id = $2;
        "#,
        &encrypted_password,
        &account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::NotFound);
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
