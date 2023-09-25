mod accounts;
mod error;
mod prelude;
mod routes;
mod utils;

use accounts::delete::{delete_expired_account_info_changes, delete_expired_unverified_accounts};
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
    Router,
};
use prelude::*;
use rand::Rng;
use sqlx::{migrate, PgPool};
use std::{net::ToSocketAddrs, time::Duration};
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};
use utils::config::AppConfig;

use crate::utils::random;

#[derive(Clone)]
pub struct AppState {
    pub database_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    todo!("Fazer o login e emails mais especificos...");

    let app_config = AppConfig::load_from_env().unwrap();
    let database_url = &app_config.database_url;
    let account_confirmation_lifespan = app_config.account_confirmation_lifespan;
    let check_timeout = app_config.check_timeout;
    let max_body_size = app_config.max_body_size;
    let server_url = app_config.server_url;

    println!("Connecting to Database...");
    let database_pool = PgPool::connect(database_url).await.unwrap();
    println!("Connected to Database.");

    migrate!().run(&database_pool).await.unwrap();

    let app_state = AppState {
        database_pool: database_pool.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route(
            "/accounts/create",
            post(routes::accounts::create::create_account_request),
        )
        .route(
            "/accounts/create/confirm",
            post(routes::accounts::create::confirm_account_request),
        )
        .route(
            "/accounts/update/username",
            patch(routes::accounts::update::update_username_request),
        )
        .route(
            "/accounts/update/username/confirm",
            post(routes::accounts::update::confirm_update_username_request),
        )
        .route(
            "/accounts/update/password",
            patch(routes::accounts::update::update_password_request),
        )
        .route(
            "/accounts/update/password/confirm",
            post(routes::accounts::update::confirm_update_password_request),
        )
        .route(
            "/accounts/update/email",
            patch(routes::accounts::update::update_email_request),
        )
        .route(
            "/accounts/update/email/one/confirm",
            post(routes::accounts::update::confirm_update_email_step_one_request),
        )
        .route(
            "/accounts/update/email/two/confirm",
            post(routes::accounts::update::confirm_update_email_step_two_request),
        )
        .route(
            "/accounts/get/account",
            get(routes::accounts::get::get_account_request),
        )
        .route(
            "/accounts/delete/account",
            delete(routes::accounts::delete::delete_account_request),
        )
        .route(
            "/accounts/delete/account/confirm",
            post(routes::accounts::delete::confirm_delete_account_request),
        )
        .route("/", get(routes::root::root))
        .layer(DefaultBodyLimit::max(max_body_size))
        .with_state(app_state.clone())
        .layer(cors);

    let addr = server_url.to_socket_addrs().unwrap().next().unwrap();

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    migrate!().run(&database_pool).await.unwrap();

    // Check loop
    tokio::spawn(async move {
        let timeout: Duration = Duration::from_secs(check_timeout);
        loop {
            println!("Ran Check");
            sleep(timeout).await;

            check_loop(account_confirmation_lifespan, &database_pool)
                .await
                .unwrap()
        }
    });

    println!("Listening on: http://{}", addr);
    if let Err(err) = server.await {
        panic!("Server error: {err}");
    };

    Ok(())
}

async fn check_loop(account_confirmation_lifespan: i64, database_pool: &PgPool) -> Result<()> {
    delete_expired_unverified_accounts(account_confirmation_lifespan, database_pool).await?;
    delete_expired_account_info_changes(account_confirmation_lifespan, database_pool).await?;

    Ok(())
}

fn get_random_color() -> String {
    let colors = [
        "\x1b[1;91m",
        "\x1b[1;92m",
        "\x1b[1;93m",
        "\x1b[1;94m",
        "\x1b[1;95m",
        "\x1b[1;96m",
    ];
    let mut rng = rand::thread_rng();
    let indice = rng.gen_range(0..colors.len());
    colors[indice].to_string()
}

pub fn get_process_id() -> String {
    let app_config = AppConfig::load_from_env().unwrap();
    let process_id_length = app_config.process_id_length;
    let color = get_random_color();
    let process_id = random::get_random_string(process_id_length);

    f!("{color}{process_id}\x1b[0m")
}

// let account_basic_info = AccountBasicInfo {
//     username: "Afonso".to_string(),
//     email: "afonso@mail.pt".to_string(),
//     password: "passwordsecreta".to_string(),
//     language: "pt".to_string(),
// };

// println!("Deleting all accounts.");
// delete_all_account(&database_pool).await.unwrap();

// println!("Deleting all account info changes.");
// delete_all_account_info_changes(&database_pool)
//     .await
//     .unwrap();

// println!("Creating account.");
// let account_id = create_account(&account_basic_info, &database_pool)
//     .await
//     .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// let confirmed_account_info = Account {
//     account_id: None,
//     username: None,
//     email: None,
//     password: None,
//     language: None,
//     verified: Some(true),
//     last_change_timestamp: None,
//     creation_timestamp: None,
// };

// println!("Creating account info change to verify account.");
// let account_verification_change_id =
//     create_account_change(&account_id, &confirmed_account_info, &database_pool)
//         .await
//         .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// println!("Confirming change account inf0 to verify account.");
// confirm_account_change(&account_id, &account_verification_change_id, &database_pool)
//     .await
//     .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// let new_account_info = Account {
//     account_id: None,
//     username: None,
//     email: Some("afonso@pagman.org".to_string()),
//     password: None,
//     language: None,
//     verified: None,
//     last_change_timestamp: None,
//     creation_timestamp: None,
// };

// println!("Creating change account info.");
// let account_info_change_id =
//     create_account_change(&account_id, &new_account_info, &database_pool)
//         .await
//         .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(15);
// sleep(_timeout).await;

// println!("Confirming change account info.");
// confirm_account_change(&account_id, &account_info_change_id, &database_pool)
//     .await
//     .unwrap();

// let current_timestamp = Utc::now().timestamp().to_string();
// let account = Account {
//     account_id: "1232123422223".to_string(),
//     username: "Fernando".to_string(),
//     email: "fernando@222342mail.pt".to_string(),
//     password: "fernando123".to_string(),
//     language: "pt".to_string(),
//     verified: false,
//     creation_timestamp: current_timestamp
// };

// create_account(&account, &database_pool).await.unwrap();

// let account_record = get_account(&account.account_id, &database_pool).await.unwrap();
// println!("Before: {account_record:?}");

// let new_account = Account {
//     account_id: "".to_string(),
//     username: "Manuel O Maluco".to_string(),
//     email: "".to_string(),
//     password: "".to_string(),
//     language: "".to_string(),
//     verified: true,
//     creation_timestamp: "".to_string()
// };

// update_account(&account.account_id, &new_account, &database_pool).await.unwrap();

// let new_account_record = get_account(&account.account_id, &database_pool).await.unwrap();
// println!("After: {new_account_record:?}");

// delete_account(&account.account_id, &database_pool).await.unwrap();
