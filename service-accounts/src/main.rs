mod accounts;
mod error;
mod prelude;
mod routes;
mod utils;

use accounts::delete::{
    delete_expired_account_changes, delete_expired_unverified_accounts,
    delete_unverified_accounts_not_awaiting_confirmation,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use prelude::*;
use sqlx::{migrate, PgPool};
use std::{net::ToSocketAddrs, time::Duration};
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};
use utils::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub database_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Loading Environment Variables...");
    let app_config = AppConfig::load_from_env().unwrap();
    let database_url = &app_config.database_url;
    let account_confirmation_lifespan = app_config.account_confirmation_lifespan;
    let check_timeout = app_config.check_timeout;
    let server_url = app_config.server_url;
    println!("Environment Variables Loaded...");

    println!("Connecting to Database...");
    let database_pool = PgPool::connect(database_url).await.unwrap();
    println!("Connected to Database.");

    println!("Running Database Migrations...");
    migrate!().run(&database_pool).await.unwrap();
    println!("Ran Database Migrations...");

    let app_state = AppState {
        database_pool: database_pool.clone(),
    };

    println!("Starting Check Loop Process...");
    tokio::spawn(async move {
        let timeout: Duration = Duration::from_secs(check_timeout);
        check_loop(timeout, account_confirmation_lifespan, &database_pool)
            .await
            .unwrap();
    });
    println!("Started Check Loop Process...");

    println!("Starting Server...");
    println!("Listening on: {}", &server_url);
    run_server(&server_url, app_state).await;

    Ok(())
}

async fn check_loop(
    timeout: Duration,
    account_confirmation_lifespan: i64,
    database_pool: &PgPool,
) -> Result<()> {
    loop {
        sleep(timeout).await;

        let deleted_expired_unverified_accounts_emails =
            delete_expired_unverified_accounts(account_confirmation_lifespan, database_pool)
                .await?;

        let deleted_unverified_accounts_not_awaiting_confirmation_emails =
            delete_unverified_accounts_not_awaiting_confirmation(database_pool).await?;

        let deleted_expired_account_changes_account_ids =
            delete_expired_account_changes(account_confirmation_lifespan, database_pool).await?;

        if !deleted_expired_unverified_accounts_emails.is_empty() {
            println!(
            "Deleted Expired Unverified Accounts Emails: {deleted_expired_unverified_accounts_emails:?}"
        );
        }

        if !deleted_unverified_accounts_not_awaiting_confirmation_emails.is_empty() {
            println!(
            "Deleted Unverified Accounts Not Awaiting Confirmation Emails: {deleted_unverified_accounts_not_awaiting_confirmation_emails:?}"
        );
        }

        if !deleted_expired_account_changes_account_ids.is_empty() {
            println!(
            "Deleted Expired Account Changes Account IDs: {deleted_expired_account_changes_account_ids:?}"
        );
        }
    }
}

async fn run_server(server_url: &str, app_state: AppState) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let change_router = Router::new()
        .route(
            "/username",
            patch(routes::change::username::change_username_request),
        )
        .route(
            "/username/confirm",
            post(routes::change::username::confirm_username_change_request),
        )
        .route(
            "/password",
            patch(routes::change::password::change_password_request),
        )
        .route(
            "/password/confirm",
            post(routes::change::password::confirm_change_password_request),
        )
        .route("/email", patch(routes::change::email::change_email_request))
        .route(
            "/email/one/confirm",
            post(routes::change::email::confirm_change_email_step_one_request),
        )
        .route(
            "/email/two/confirm",
            post(routes::change::email::confirm_change_email_step_two_request),
        )
        .route(
            "/delete",
            post(routes::change::delete::delete_account_request),
        )
        .route(
            "/delete/confirm",
            post(routes::change::delete::confirm_delete_account_request),
        );

    let signup_router = Router::new()
        .route("/", post(routes::signup::create_account_request))
        .route("/confirm", post(routes::signup::confirm_account_request));

    let authentication_router = Router::new()
        .route(
            "/",
            post(routes::authentication::authenticate_account_request),
        )
        .route(
            "/email",
            post(routes::authentication::authenticate_account_without_credentials_request),
        )
        .route(
            "/email/confirm",
            post(routes::authentication::confirm_authenticate_account_without_credentials_request),
        );

    let get_account_router =
        Router::new().route("/", get(routes::authentication::get_account_request));

    let confirmations_router = Router::new().route(
        "/cancel",
        post(routes::confirmations::delete_confirmations_request),
    );

    let root_router = Router::new().route("/", get(routes::root::root));

    let main_router = Router::new()
        .nest("/change", change_router)
        .nest("/account", get_account_router)
        .nest("/authenticate", authentication_router)
        .nest("/signup", signup_router)
        .nest("/confirmations", confirmations_router)
        .nest("/", root_router)
        .with_state(app_state.clone())
        .layer(cors);

    let addr = server_url.to_socket_addrs().unwrap().next().unwrap();

    axum::Server::bind(&addr)
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}
