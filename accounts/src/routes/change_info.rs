use crate::{
    database::DATABASE_POOL, get_decode_verify_and_return_session_token,
    models::AccountInfoChangeRequest, sanitize_handle,
};
use tide::{Response, StatusCode};
use validator::Validate;

pub async fn info_change(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let mut body: AccountInfoChangeRequest = req.body_json().await?;

    if body.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(body.validate().unwrap_err());
        return Ok(response);
    };

    body.handle = body.handle.map(|handle| {
        match sanitize_handle(&handle) {
            Ok(handle) => {
                if handle.is_empty() {
                    None
                } else {
                    Some(handle)
                }
            },
            Err(_) => {
                None
            }
        }
    }).flatten();
    body.name = body.name.map(|name| name.trim().to_string());
    body.country_code = body
        .country_code
        .map(|country_code| country_code.to_lowercase().trim().to_string());

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

    // UPDATE INFO THAT IS NOT NONE IN THE DATABASE

    let query = sqlx::query!(
        r#"
            UPDATE accounts
            SET
                handle = COALESCE($1, handle),
                name = COALESCE($2, name),
                gender = COALESCE($3, gender),
                country_code = COALESCE($4, country_code)
            WHERE id = $5
        "#,
        body.handle,
        body.name,
        body.gender.map(|gender| gender.to_string()),
        body.country_code,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    };

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    Ok(Response::new(StatusCode::Ok))
}
