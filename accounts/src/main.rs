use crate::{config::CONFIG, database::DATABASE_POOL};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{migrate, query};
use strum_macros::Display;
use tide::{Response, StatusCode};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Display)]
enum Group {
    #[serde(rename = "administrator")]
    #[strum(serialize = "administrator")]
    Administrator,

    #[serde(rename = "moderator")]
    #[strum(serialize = "moderator")]
    Moderator,

    #[serde(rename = "default")]
    #[strum(serialize = "default")]
    Default,
}

#[derive(Debug, Serialize, Deserialize, Display)]
enum Gender {
    #[serde(rename = "male")]
    #[strum(serialize = "male")]
    Male,
    #[serde(rename = "female")]
    #[strum(serialize = "female")]
    Female,
    #[serde(rename = "not_specified")]
    #[strum(serialize = "not_specified")]
    NotSpecified,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: Uuid,
    picture_id: String,
    handle: String,
    name: String,
    email: String,
    password: String,
    group: Group,
    gender: Gender,
    language: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountSafe {
    id: Uuid,
    picture_id: String,
    handle: String,
    name: String,
    email: String,
    group: Group,
    gender: Gender,
    language: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateAccount {
    #[validate(length(min = 1), custom = "validate_handle_lenght")]
    handle: String,
    #[validate(length(min = 1), custom = "validate_name_lenght")]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    password: String,
    gender: Gender,
    #[validate(length(min = 1))]
    language: String,
}

fn validate_handle_lenght(handle: &str) -> Result<(), ValidationError> {
    if handle.len() > CONFIG.handle_max_length {
        return Err(ValidationError::new("handle_length_exceeded"));
    }

    Ok(())
}

fn validate_name_lenght(name: &str) -> Result<(), ValidationError> {
    if name.len() > CONFIG.name_max_length {
        return Err(ValidationError::new("name_length_exceeded"));
    }

    Ok(())
}

fn extract_constraint_part(constraint: &str) -> &str {
    let first_underscore = constraint.find('_').unwrap_or(0);

    let last_underscore = constraint.rfind('_').unwrap_or(constraint.len());

    &constraint[(first_underscore + 1)..last_underscore]
}

async fn create_account(mut req: tide::Request<()>) -> tide::Result {
    let account: CreateAccount = req.body_json().await?;

    if account.validate().is_err() {
        let res = Response::new(StatusCode::UnprocessableEntity);
        return Ok(res);
    };

    let id = Uuid::new_v4();
    let account_full = Account {
        id,
        picture_id: CONFIG.default_picture.to_owned(),
        handle: account.handle,
        name: account.name,
        email: account.email,
        password: account.password,
        group: Group::Default,
        gender: account.gender,
        language: account.language,
    };

    // Inicia uma transação na pool
    let mut transaction = DATABASE_POOL.begin().await?;

    // Realiza a inserção na tabela 'accounts'
    let result = query(
        r#"
        INSERT INTO accounts (id, picture_id, handle, name, email, password, "group", gender, language)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#)
        .bind(account_full.id.to_string())
        .bind(account_full.picture_id)
        .bind(account_full.handle)
        .bind(account_full.name)
        .bind(account_full.email)
        .bind(account_full.password)
        .bind(account_full.group.to_string())
        .bind(account_full.gender.to_string())
        .bind(account_full.language)
    .execute(&mut *transaction)
    .await;

    match result {
        Ok(_) => {
            transaction.commit().await?;
            let res = Response::builder(StatusCode::Created)
                .body("Account created!")
                .build();

            Ok(res)
        }
        Err(err) => {
            transaction.rollback().await?;

            let mut res = Response::new(StatusCode::InternalServerError);

            if let Some(db_error) = err.as_database_error() {
                if db_error.is_unique_violation() {
                    if let Some(constraint) = db_error.constraint() {
                        let relevant_part = extract_constraint_part(constraint);
                        res = Response::builder(StatusCode::Conflict)
                            .body(relevant_part)
                            .build();
                    }
                }
            }

            Ok(res)
        }
    }
}

async fn root(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("Deus quer, o homem sonha, a obra nasce.".to_string())
}

mod config;
mod database;

#[async_std::main]
async fn main() {
    // Load environment from .env file
    dotenv().ok();

    // Start femme
    femme::start();

    // Run migrations on the database
    migrate!("./migrations").run(&*DATABASE_POOL).await.unwrap();

    // Create the server
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(root);
    app.at("/create").post(create_account);

    // Run the server
    app.listen(&CONFIG.server_host).await.unwrap();
}
