use crate::{
    accounts::{
        create::create_account,
        get::{get_account_from_token, get_account_token},
        models::{Account, AccountChange},
        update::{confirm_account_change, create_account_change},
    },
    error::error_to_status_code,
    routes::models::Token,
    utils::{config::AppConfig, email::send_confirmation_email, random::get_random_process_id},
    AppState,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    Json, TypedHeader,
};

use super::models::{AccountInfo, ConfirmationCode};

pub async fn create_account_request(
    State(app_state): State<AppState>,
    Json(create_account_request): Json<AccountInfo>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account create\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_creation_confirmation_email_subject;
    let email_title = app_config.account_creation_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let account = Account {
        username: create_account_request.username,
        email: create_account_request.email,
        password: create_account_request.password,
    };

    let account_id = match create_account(&account, database_pool).await {
        Ok(value) => value,
        Err(err) => {
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
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
                let status_code_number = error_to_status_code(err.clone());
                let status_code = StatusCode::from_u16(status_code_number).unwrap();
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
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    let token = match get_account_token(&account_id, database_pool).await {
        Ok(value) => value,
        Err(err) => {
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    let response = Token { token };

    (StatusCode::OK, Json(response)).into_response()
}

pub async fn confirm_account_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(confirm_account_request): Json<ConfirmationCode>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account confirm\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = confirm_account_request.confirmation_code;

    let account = match get_account_from_token(&token, &database_pool).await {
        Ok(value) => value,
        Err(err) => {
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    match confirm_account_change(&account.account_id, &confirmation_code, &database_pool).await {
        Ok(value) => value,
        Err(err) => {
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    let token = match get_account_token(&account.account_id, &database_pool).await {
        Ok(value) => value,
        Err(err) => {
            let status_code_number = error_to_status_code(err.clone());
            let status_code = StatusCode::from_u16(status_code_number).unwrap();
            println!(
                "{process_id} - Status: \"{status_code}\" Error: \"{}\"",
                err
            );
            return status_code.into_response();
        }
    };

    let response = Token { token };

    (StatusCode::OK, Json(response)).into_response()
}
