use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    prelude::*,
    routes::{
        change_email::{admin_email_change, begin_email_change, finish_email_change},
        change_group::admin_group_change,
        change_info::info_change,
        change_password::{admin_password_change, begin_password_change, finish_password_change},
        create::{begin_account_creation, finish_account_creation},
        delete::{admin_account_deletion, begin_account_deletion, finish_account_deletion},
        get::{get_account, get_all_accounts, get_is_admin},
        picture::upload_picture,
        root,
        session::{
            change_session_device_description, change_session_device_name,
            change_session_device_type, create_session, delete_session, get_some_sessions,
            verify_session,
        },
    },
};
use dotenv::dotenv;
use error::{DatabaseError, Error, TokenError};
use models::{Group, SessionToken};
use sqlx::migrate;
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
};

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

pub async fn is_account_admin_from_id(id: &str) -> Result<bool> {
    let query = sqlx::query!(
        r#"
            SELECT "group"
            FROM accounts
            WHERE id = $1;
        "#,
        id
    );

    let result = match query.fetch_one(&*DATABASE_POOL).await {
        Ok(result) => result,
        Err(sqlx::Error::RowNotFound) => return Err(Error::Database(DatabaseError::RowNotFound)),
        Err(error) => return Err(Error::Database(DatabaseError::FetchOne(error.to_string()))),
    };

    let group = result.group;

    Ok(group == Group::Administrator.to_string())
}

pub async fn get_id_from_handle(handle: &str) -> Result<String> {
    let query = sqlx::query!(
        r#"
            SELECT id
            FROM accounts
            WHERE handle = $1;
        "#,
        handle
    );

    let result = match query.fetch_one(&*DATABASE_POOL).await {
        Ok(result) => result,
        Err(sqlx::Error::RowNotFound) => return Err(Error::Database(DatabaseError::RowNotFound)),
        Err(error) => return Err(Error::Database(DatabaseError::FetchOne(error.to_string()))),
    };

    Ok(result.id)
}

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
    femme::start();

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

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, OPTIONS, DELETE, PATCH"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    // Create the server
    log::info!("Creating server...");
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.with(cors);
    app.at("/").get(root::root);
    app.at("/account").get(get_account);
    app.at("/admin").get(get_is_admin);
    app.at("/delete/begin").post(begin_account_deletion);
    app.at("/delete/finish").post(finish_account_deletion);
    app.at("/change").patch(info_change);
    app.at("/admin/accounts").get(get_all_accounts);
    app.at("/admin/change/group").patch(admin_group_change);
    app.at("/admin/change/email").patch(admin_email_change);
    app.at("/admin/delete").patch(admin_account_deletion);
    app.at("/admin/accounts").patch(get_all_accounts);
    app.at("/admin/change/password")
        .patch(admin_password_change);
    app.at("/change/email/begin").post(begin_email_change);
    app.at("/change/email/finish").post(finish_email_change);
    app.at("/change/password/begin").post(begin_password_change);
    app.at("/change/password/finish")
        .post(finish_password_change);
    app.at("/create/begin").post(begin_account_creation);
    app.at("/create/finish").post(finish_account_creation);
    app.at("/sessions/:start/:ammount").get(get_some_sessions);
    app.at("/session/device/type")
        .patch(change_session_device_type);
    app.at("/session/device/description")
        .patch(change_session_device_description);
    app.at("/session/device/name")
        .patch(change_session_device_name);
    app.at("/session").post(create_session);
    app.at("/session").delete(delete_session);
    app.at("/session/:session_id").delete(delete_session);
    app.at("/session/verify").get(verify_session);
    app.at("/picture").post(upload_picture);

    // Run the server
    log::info!("Running server...");
    app.listen(&CONFIG.server_host).await.unwrap();

    Ok(())
}
