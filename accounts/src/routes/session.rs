use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    encryption, get_decode_verify_and_return_session_token,
    models::{
        ChangeSessionDeviceTypeRequest, CreateSessionRequest, DeviceType, GetSessionsResponse,
        Session, SessionToken, Token,
    },
    random::get_random_string,
    token,
};
use chrono::{Duration, Utc};
use std::str::FromStr;
use tide::{convert::json, Response, StatusCode};
use validator::Validate;

#[tracing::instrument]
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

    // INSERT NEW SESSION INTO SESSIONS TABLE

    let session_id = get_random_string(CONFIG.session_id_length);
    let device_type = DeviceType::Other;
    let expire_date = Utc::now().naive_utc() + Duration::days(30);
    let created_at = Utc::now().naive_utc();

    let query = sqlx::query!(
        r#"
            INSERT INTO sessions (id, account_id, device, device_type, expire_date, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        session_id,
        account_id,
        body.device,
        device_type.to_string(),
        expire_date,
        created_at
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // CREATE TOKEN

    let session = Session {
        id: session_id,
        account_id,
        device: body.device,
        device_type,
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

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let session_id = session.id;
    let account_id = session.account_id;

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
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // COMMIT CHANGES IN DATABASE

    transaction.commit().await?;

    // SEND OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

pub async fn get_some_sessions(req: tide::Request<()>) -> tide::Result {
    // GET AMMOUNT OF SESSIONS TO GET FROM URL

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

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let session_id = &session.id;
    let account_id = &session.account_id;

    // GET SOME SESSIONS FROM SESSIONS TABLE WHERE ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            SELECT id, device, device_type, expire_date, created_at
            FROM sessions
            WHERE account_id = $1
            LIMIT $2
        "#,
        account_id,
        ammount
    );

    let result: Vec<Session> = query
        .fetch_all(&mut *transaction)
        .await?
        .into_iter()
        .map(|session| Session {
            id: session_id.to_owned(),
            account_id: account_id.to_owned(),
            device: session.device,
            device_type: DeviceType::from_str(&session.device_type).unwrap_or(DeviceType::Other),
            expire_date: session.expire_date,
            created_at: session.created_at,
        })
        .collect();

    // CREATE RESPONSE

    let all_sessions = GetSessionsResponse { sessions: result };

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

    let session_id = session.id;
    let account_id = session.account_id;

    // UPDATE SESSION IN SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            UPDATE sessions
            SET device_type = $1
            WHERE id = $2 AND account_id = $3
        "#,
        body.device_type.to_string(),
        session_id,
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
