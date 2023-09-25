use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    accounts::{
        create::create_account,
        models::{Account, AccountChange},
        update::create_account_change,
    },
    get_process_id,
    prelude::*,
    utils::{config::AppConfig, email::send_confirmation_email},
    AppState,
};

use super::models::CreateAccountRequest;

pub async fn create_account_request(
    State(app_state): State<AppState>,
    Json(create_account_request): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account create\" request");
    println!("{process_id} - Account: \"{create_account_request:?}\"");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_confirmation_email_subject;
    let email_title = app_config.account_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let account = Account {
        username: create_account_request.username,
        email: create_account_request.email,
        password: create_account_request.password,
        language: create_account_request.language,
    };

    let account_id = match create_account(&account, database_pool).await {
        Ok(value) => value,
        Err(Error::CreateAccountDuplicateKey(err)) => {
            let status_code = StatusCode::CONFLICT;
            println!("{process_id} - Status: {status_code} Error: \"{}\"", err);
            return status_code.into_response();
        }
        Err(err) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    let account_change = AccountChange {
        username: None,
        email: None,
        password: None,
        verified: Some(true),
        step: None,
    };
    let confirmation_code =
        match create_account_change(&account_id, &account_change, database_pool).await {
            Ok(value) => value,
            Err(err) => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                println!(
                    "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                    err
                );
                return status_code.into_response();
            }
        };

    match send_confirmation_email(
        &account.email,
        &email_subject,
        &email_title,
        &confirmation_code,
    )
    .await
    {
        Ok(value) => value,
        Err(err) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    StatusCode::OK.into_response()
}
