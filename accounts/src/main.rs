use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    prelude::*,
    routes::{
        change_email::{begin_email_change, finish_email_change},
        change_password::{begin_password_change, finish_password_change},
        create::{begin_account_creation, finish_account_creation},
        get::get_account,
        root,
        session::{change_session_device_type, create_session, delete_session, get_some_sessions},
    },
};
use dotenv::dotenv;
use error::{Error, TokenError};
use models::SessionToken;
use sqlx::migrate;

pub mod config;
pub mod database;
pub mod email;
pub mod encryption;
pub mod error;
pub mod models;
pub mod prelude;
pub mod random;
pub mod routes;
pub mod token;

pub fn get_token_from_request(req: &tide::Request<()>) -> Result<String> {
    let authorization_header = req.header("Authorization").ok_or_else(|| {
        Error::Token(TokenError::MissingAuthorizationHeader(
            "Authorization".to_string(),
        ))
    })?;

    let authorization_header = authorization_header.as_str();

    let token = authorization_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            Error::Token(TokenError::MissingAuthorizationHeader(
                "Bearer ".to_string(),
            ))
        })?
        .to_string();

    Ok(token)
}

pub async fn verify_and_get_session_token(token: &str) -> Result<SessionToken> {
    // DECODE TOKEN

    let session_token: SessionToken = token::decode_token(token)
        .map_err(|err| Error::Token(TokenError::DecodeToken(err.to_string())))?;

    let session = &session_token.session;

    // GET SESSION AND ACCOUNT IDs FROM TOKEN

    let session_id = &session.id;
    let account_id = &session.account_id;

    // GET SESSION FROM SESSIONS TABLE WHERE SESSION ID AND ACCOUNT ID MATCH

    let query = sqlx::query!(
        r#"
            SELECT id
            FROM sessions
            WHERE id = $1 AND account_id = $2
        "#,
        session_id,
        account_id
    );

    query
        .fetch_one(&*DATABASE_POOL)
        .await
        .map_err(|_| Error::Token(TokenError::InvalidToken))?;

    Ok(session_token)
}

pub async fn get_decode_verify_and_return_session_token(
    req: &tide::Request<()>,
) -> Result<SessionToken> {
    // GET TOKEN FROM HEADER

    let token = get_token_from_request(req)?;

    // DECODE AND VERIFY TOKEN

    let session_token = verify_and_get_session_token(&token).await?;

    // RETURN SESSION TOKEN

    Ok(session_token)
}

pub fn string_to_email_placeholder(string: &str) -> String {
    f!(
        "{}{}{}",
        CONFIG.email_placeholder_marker,
        string,
        CONFIG.email_placeholder_marker
    )
}

#[async_std::main]
async fn main() -> Result<()> {
    // Start logger
    println!("Starting logger...");
    tracing_subscriber::fmt::init();

    // Load environment from .env file
    log::info!("Loading environment variables...");
    dotenv().ok();

    // Send test email if enabled
    if CONFIG.send_test_startup_email {
        log::info!("Sending test email...");
        email::send_email(
            &CONFIG.manager_email_address,
            &CONFIG.service_startup_email_subject,
            &CONFIG.service_startup_email_body,
            CONFIG.service_startup_email_html.to_owned(),
        )?;
    } else {
        log::info!("Skipping test email...");
    }

    // Run migrations on the database
    log::info!("Running migrations...");
    migrate!("./migrations").run(&*DATABASE_POOL).await.unwrap();

    // Create the server
    log::info!("Creating server...");
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(root::root);
    app.at("/get").get(get_account);
    app.at("/change/email/begin").post(begin_email_change);
    app.at("/change/email/finish").post(finish_email_change);
    app.at("/change/password/begin").post(begin_password_change);
    app.at("/change/password/finish")
        .post(finish_password_change);
    app.at("/create/begin").post(begin_account_creation);
    app.at("/create/finish").post(finish_account_creation);
    app.at("/sessions/:ammount").get(get_some_sessions);
    app.at("/session").patch(change_session_device_type);
    app.at("/session").post(create_session);
    app.at("/session").delete(delete_session);

    // Run the server
    log::info!("Running server...");
    app.listen(&CONFIG.server_host).await.unwrap();

    Ok(())
}
