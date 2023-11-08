use crate::{
    accounts::{
        get::get_account_from_token,
        models::AccountChange,
        update::{confirm_account_change, create_account_change},
    },
    error::error_to_status_code,
    prelude::*,
    routes::models::ConfirmationCode,
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

pub async fn delete_account_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account deletion\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_deletion_confirmation_email_subject;
    let email_title = app_config.account_deletion_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let token = authorization.token().to_owned();
    let account = match get_account_from_token(&token, database_pool).await {
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
        match create_account_change(&account.account_id, &account_change, database_pool).await {
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
        &account.account.email,
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

pub async fn confirm_delete_account_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<ConfirmationCode>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"account deletion confirmation\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = body.confirmation_code;

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

    StatusCode::OK.into_response()
}
