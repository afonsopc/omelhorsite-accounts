use super::models::{AuthenticationCredentials, Email, EmailAndConfirmationCode};
use crate::{
    accounts::{
        get::{
            get_account_from_credentials, get_account_from_token, get_account_id_from_email,
            get_account_token,
        },
        models::AccountChange,
        update::{confirm_account_change, create_account_change},
    },
    error::error_to_status_code,
    routes::models::{AccountInfoWithoutPassword, Token},
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

pub async fn get_account_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"get account\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();

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

    let response = AccountInfoWithoutPassword {
        username: account.account.username,
        email: account.account.email,
        creation_timestamp: account.creation_timestamp,
    };

    (StatusCode::OK, Json(response)).into_response()
}

pub async fn authenticate_account_request(
    State(app_state): State<AppState>,
    Json(body): Json<AuthenticationCredentials>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account authentication\" request");

    let database_pool = app_state.database_pool;

    let account =
        match get_account_from_credentials(&body.email, &body.password, &database_pool).await {
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

pub async fn authenticate_account_without_credentials_request(
    State(app_state): State<AppState>,
    Json(body): Json<Email>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account authentication without credentials\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_email_authentication_confirmation_email_subject;
    let email_title = app_config.account_email_authentication_confirmation_email_title_message;

    let database_pool = app_state.database_pool;

    let account_id = match get_account_id_from_email(&body.email, &database_pool).await {
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
        verified: None,
        step: None,
    };

    let confirmation_code =
        match create_account_change(&account_id, &account_change, &database_pool).await {
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
        &body.email,
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

    StatusCode::OK.into_response()
}

pub async fn confirm_authenticate_account_without_credentials_request(
    State(app_state): State<AppState>,
    Json(body): Json<EmailAndConfirmationCode>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!(
        "{process_id} - Starting \"account authentication without credentials confirmation\" request"
    );

    let database_pool = app_state.database_pool;

    let confirmation_code = body.confirmation_code;

    let account_id = match get_account_id_from_email(&body.email, &database_pool).await {
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

    match confirm_account_change(&account_id, &confirmation_code, &database_pool).await {
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

    let token = match get_account_token(&account_id, &database_pool).await {
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
