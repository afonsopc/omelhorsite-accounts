use tide::{Response, StatusCode};
use validator::Validate;

use crate::{
    database::DATABASE_POOL, get_decode_verify_and_return_session_token, is_account_admin_from_id,
    models::AdminGroupChangeRequest,
};

pub async fn admin_group_change(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: AdminGroupChangeRequest = req.body_json().await?;

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

    // CHECK IF USER IS ADMIN

    match is_account_admin_from_id(&account_id).await {
        Ok(is_admin) => {
            if !is_admin {
                let response = Response::new(StatusCode::Unauthorized);
                return Ok(response);
            }
        }
        Err(_) => {
            let response = Response::new(StatusCode::InternalServerError);
            return Ok(response);
        }
    }

    // CHANGE THE GROUP

    let query = sqlx::query!(
        r#"
            UPDATE accounts 
            SET "group" = $1
            WHERE id = $2;
        "#,
        &body.group.to_string(),
        &body.account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    Ok(Response::new(StatusCode::Ok))
}
