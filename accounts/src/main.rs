use crate::{
    config::CONFIG,
    database::DATABASE_POOL,
    prelude::*,
    routes::create::{begin_account_creation, finish_account_creation},
};
use dotenv::dotenv;
use sqlx::migrate;

#[tracing::instrument]
async fn root(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("Deus quer, o homem sonha, a obra nasce.".to_string())
}

pub mod config;
pub mod database;
pub mod email;
pub mod error;
pub mod models;
pub mod prelude;
pub mod random;
pub mod routes;

#[async_std::main]
async fn main() -> Result<()> {
    // Start femme
    println!("Starting femme...");
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

    // Create the server
    log::info!("Creating server...");
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(root);
    app.at("/create/begin").post(begin_account_creation);
    app.at("/create/finish").post(finish_account_creation);

    // Run the server
    log::info!("Running server...");
    app.listen(&CONFIG.server_host).await.unwrap();

    Ok(())
}
