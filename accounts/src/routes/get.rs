use std::str::FromStr;

use crate::{
    database::DATABASE_POOL,
    get_decode_verify_and_return_session_token,
    models::{AccountSafe, Gender, Group, Theme},
};
use tide::{convert::json, Response, StatusCode};

#[tracing::instrument]
pub async fn get_account(req: tide::Request<()>) -> tide::Result {
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

    // GET ACCOUNT FROM DATABASE

    let account = sqlx::query!(
        r#"
            SELECT *
            FROM accounts 
            WHERE id = $1;
        "#,
        account_id
    );

    let result = account.fetch_one(&mut *transaction).await?;

    let account_safe = AccountSafe {
        id: result.id,
        picture_id: result.picture_id,
        handle: result.handle,
        name: result.name,
        email: result.email,
        group: Group::from_str(&result.group)?,
        gender: Gender::from_str(&result.gender)?,
        theme: Theme::from_str(&result.theme)?,
        language: result.language,
        created_at: result.created_at,
    };

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    let response = Response::builder(StatusCode::Ok)
        .body(json!(account_safe))
        .build();

    Ok(response)
}
