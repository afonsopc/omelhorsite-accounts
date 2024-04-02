use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    encryption, get_decode_verify_and_return_session_token, is_account_admin_from_id,
    models::{
        ChangeSessionDeviceDescriptionRequest, ChangeSessionDeviceNameRequest,
        ChangeSessionDeviceTypeRequest, CreateSessionRequest, DeviceType, Session, SessionList,
        SessionToken, SessionTokenInfo, Token,
    },
    random::get_random_string,
    token,
};
use chrono::{Duration, Utc};
use std::str::FromStr;
use tide::{convert::json, Response, StatusCode};
use validator::Validate;

pub async fn create_session(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: CreateSessionRequest = req.body_json().await?;

    if body.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(body.validate().unwrap_err());
        return Ok(response);
    };

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET ENCRYPTED PASSWORD AND ID FROM ACCOUNT WITH GIVEN EMAIL

    let query = sqlx::query!(
        r#"
            SELECT id, password
            FROM accounts
            WHERE email = $1
        "#,
        body.email
    );

    let result = match query.fetch_optional(&mut *transaction).await? {
        Some(result) => result,
        None => {
            let response = Response::new(StatusCode::NotFound);
            return Ok(response);
        }
    };

    let account_id = result.id;

    // CHECK IF GIVEN PASSWORD EQUAL TO ENCRYPTED PASSWORD

    let password_is_correct =
        encryption::compare_plain_to_encrypted_string(&body.password, &result.password)?;

    if !password_is_correct {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // GET USERS COUNTRY FROM THE IP ADDRESS

    let ip_address = req
        .peer_addr()
        .map(|addr| addr.split(':').collect::<Vec<&str>>()[0]);

    // INSERT NEW SESSION INTO SESSIONS TABLE

    let session_id = get_random_string(CONFIG.session_id_length);
    let device_type = DeviceType::Other;
    let expire_date = Utc::now().naive_utc() + Duration::days(30);
    let created_at = Utc::now().naive_utc();

    let query = sqlx::query!(
        r#"
            INSERT INTO sessions (id, account_id, device_name, device_description, device_type, ip_address, expire_date, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        session_id,
        account_id,
        body.device_name,
        body.device_description,
        device_type.to_string(),
        ip_address,
        expire_date,
        created_at
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // CREATE TOKEN

    let session = SessionTokenInfo {
        id: session_id,
        account_id,
        expire_date,
        created_at,
    };

    let session_token = SessionToken {
        session,
        exp: expire_date.timestamp() as usize,
    };

    let token = Token {
        token: token::create_token(&session_token)?,
    };

    transaction.commit().await?;

    let response = Response::builder(StatusCode::Ok).body(json!(token)).build();

    Ok(response)
}

pub async fn delete_session(req: tide::Request<()>) -> tide::Result {
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

    // GET ACCOUNT ID FROM OPTIONAL PARAMS IF USER IS ADMIN

    let account_id = match req.param("account_id") {
        Ok(account_id) => match is_account_admin_from_id(&session.id).await {
            Ok(is_admin) => {
                if !is_admin {
                    session.account_id
                } else {
                    account_id.to_string()
                }
            }
            _ => {
                let response = Response::new(StatusCode::InternalServerError);
                return Ok(response);
            }
        },
        _ => session.account_id,
    };

    // GET THE SESSION ID TO DELETE, IF PROVIDED IN THE URL, USE THAT
    // ELSE USE THE SESSION ID FROM THE TOKEN

    let session_id: String = match req.param("session_id") {
        Ok(start) => start.to_string(),
        _ => session.id,
    };

    // DELETE SESSION FROM SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            DELETE FROM sessions
            WHERE id = $1 AND account_id = $2
        "#,
        session_id,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::NotFound);
        return Ok(response);
    }

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

#[tracing::instrument]
pub async fn get_some_sessions(req: tide::Request<()>) -> tide::Result {
    // GET THE STARTING INDEX FOR THE SESSIONS TO GET

    let start_index: i64 = match req.param("start") {
        Ok(start) => match start.parse() {
            Ok(start) => start,
            _ => {
                let response = Response::new(StatusCode::UnprocessableEntity);
                return Ok(response);
            }
        },
        _ => {
            let response = Response::new(StatusCode::UnprocessableEntity);
            return Ok(response);
        }
    };

    // GET THE AMMOUNT OF SESSIONS TO GET

    let ammount: i64 = match req.param("ammount") {
        Ok(ammount) => match ammount.parse() {
            Ok(ammount) => ammount,
            _ => {
                let response = Response::new(StatusCode::UnprocessableEntity);
                return Ok(response);
            }
        },
        _ => {
            let response = Response::new(StatusCode::UnprocessableEntity);
            return Ok(response);
        }
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

    // GET ACCOUNT ID FROM OPTIONAL PARAMS IF USER IS ADMIN

    let account_id = match req.param("account_id") {
        Ok(account_id) => match is_account_admin_from_id(&session.id).await {
            Ok(is_admin) => {
                if !is_admin {
                    session.account_id
                } else {
                    account_id.to_string()
                }
            }
            _ => {
                let response = Response::new(StatusCode::InternalServerError);
                return Ok(response);
            }
        },
        _ => session.account_id,
    };

    // GET SOME SESSIONS FROM SESSIONS TABLE WHERE ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            SELECT *
            FROM sessions
            WHERE account_id = $1
            ORDER BY id
            OFFSET $2
            LIMIT $3
        "#,
        account_id,
        start_index,
        ammount
    );

    let result: Vec<Session> = query
        .fetch_all(&mut *transaction)
        .await?
        .into_iter()
        .map(|session| Session {
            id: session.id,
            account_id: account_id.to_owned(),
            device_name: session.device_name,
            device_description: session.device_description,
            device_type: DeviceType::from_str(&session.device_type).unwrap_or(DeviceType::Other),
            ip_address: session.ip_address,
            expire_date: session.expire_date,
            created_at: session.created_at,
        })
        .collect();

    // CREATE RESPONSE

    let all_sessions = SessionList { sessions: result };

    let response = Response::builder(StatusCode::Ok)
        .body(json!(all_sessions))
        .build();

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(response)
}

pub async fn change_session_device_type(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: ChangeSessionDeviceTypeRequest = req.body_json().await?;

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

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let account_id = session.account_id;

    // UPDATE SESSION IN SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            UPDATE sessions
            SET device_type = $1
            WHERE id = $2 AND account_id = $3
        "#,
        body.device_type.to_string(),
        body.session_id,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

pub async fn change_session_device_name(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: ChangeSessionDeviceNameRequest = req.body_json().await?;

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

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let account_id = session.account_id;

    // UPDATE SESSION IN SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            UPDATE sessions
            SET device_name = $1
            WHERE id = $2 AND account_id = $3
        "#,
        body.device_name,
        body.session_id,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

pub async fn change_session_device_description(mut req: tide::Request<()>) -> tide::Result {
    // GET REQUEST BODY AND VALIDATE IT

    let body: ChangeSessionDeviceDescriptionRequest = req.body_json().await?;

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

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let account_id = session.account_id;

    // UPDATE SESSION IN SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            UPDATE sessions
            SET device_description = $1
            WHERE id = $2 AND account_id = $3
        "#,
        body.device_description,
        body.session_id,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

pub async fn verify_session(req: tide::Request<()>) -> tide::Result {
    // GET, DECODE AND VERIFY TOKEN
    // IF IT IS VALID, RETURN OK ELSE UNAUTHORIZED
    match get_decode_verify_and_return_session_token(&req).await {
        Ok(_) => Ok(Response::new(StatusCode::Ok)),
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            Ok(response)
        }
    }
}
