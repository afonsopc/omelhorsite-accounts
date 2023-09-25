use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    Json, TypedHeader,
};

use crate::{
    accounts::{
        delete::delete_account_change,
        get::{get_account_change, get_account_from_token, get_account_token},
        models::AccountChange,
        update::{confirm_account_change, create_account_change},
    },
    get_process_id,
    routes::accounts::models::{
        ConfirmUpdateEmailStepTwoResponse, ConfirmUpdatePasswordResponse,
        ConfirmUpdateUsernameResponse,
    },
    utils::{config::AppConfig, email::send_confirmation_email},
    AppState,
};

use super::models::{
    ConfirmUpdateEmailStepOneRequest, ConfirmUpdateEmailStepTwoRequest,
    ConfirmUpdatePasswordRequest, ConfirmUpdateUsernameRequest, UpdateEmailRequest,
    UpdatePasswordRequest, UpdateUsernameRequest,
};

pub async fn update_username_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateUsernameRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update username\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_info_change_confirmation_email_subject;
    let email_title = app_config.account_info_change_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let token = authorization.token().to_owned();
    let account = match get_account_from_token(&token, database_pool).await {
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

    let account_change = AccountChange {
        username: Some(body.username),
        email: None,
        password: None,
        verified: None,
        step: None,
    };

    let confirmation_code =
        match create_account_change(&account.account_id, &account_change, database_pool).await {
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
        &account.account.email,
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

pub async fn confirm_update_username_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<ConfirmUpdateUsernameRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update username confirmation\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = body.confirmation_code;

    let account = match get_account_from_token(&token, &database_pool).await {
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

    match confirm_account_change(&account.account_id, &confirmation_code, &database_pool).await {
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

    let token = match get_account_token(&account.account_id, &database_pool).await {
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

    let response = ConfirmUpdateUsernameResponse { token };

    (StatusCode::OK, Json(response)).into_response()
}

pub async fn update_password_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<UpdatePasswordRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update password\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_info_change_confirmation_email_subject;
    let email_title = app_config.account_info_change_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let token = authorization.token().to_owned();
    let account = match get_account_from_token(&token, database_pool).await {
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

    let account_change = AccountChange {
        username: None,
        email: None,
        password: Some(body.password),
        verified: None,
        step: None,
    };

    let confirmation_code =
        match create_account_change(&account.account_id, &account_change, database_pool).await {
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
        &account.account.email,
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

pub async fn confirm_update_password_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<ConfirmUpdatePasswordRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update password confirmation\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = body.confirmation_code;

    let account = match get_account_from_token(&token, &database_pool).await {
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

    match confirm_account_change(&account.account_id, &confirmation_code, &database_pool).await {
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

    let token = match get_account_token(&account.account_id, &database_pool).await {
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

    let response = ConfirmUpdatePasswordResponse { token };

    (StatusCode::OK, Json(response)).into_response()
}

pub async fn update_email_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateEmailRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update email\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_info_change_confirmation_email_subject;
    let email_title = app_config.account_info_change_confirmation_email_title_message;

    let database_pool = &app_state.database_pool;

    let token = authorization.token().to_owned();
    let account = match get_account_from_token(&token, database_pool).await {
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

    let account_change = AccountChange {
        username: None,
        email: Some(body.email),
        password: None,
        verified: None,
        step: Some(1),
    };

    let confirmation_code =
        match create_account_change(&account.account_id, &account_change, database_pool).await {
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
        &account.account.email,
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

pub async fn confirm_update_email_step_one_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<ConfirmUpdateEmailStepOneRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update email step one confirmation\" request");

    let app_config = AppConfig::load_from_env().unwrap();
    let email_subject = app_config.account_info_change_confirmation_email_subject;
    let email_title = app_config.account_info_change_confirmation_email_title_message;

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = body.confirmation_code;

    let account = match get_account_from_token(&token, &database_pool).await {
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

    let account_change =
        match get_account_change(&account.account_id, &confirmation_code, &database_pool).await {
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

    let new_email = match account_change.email {
        Some(value) => value,
        _ => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!("{process_id} - Status: \"{status_code}\" Error: \"New Email From Account Change is None\"");
            return status_code.into_response();
        }
    };

    let step = match account_change.step {
        Some(value) => value,
        _ => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!("{process_id} - Status: \"{status_code}\" Error: \"Step From Account Change is None\"");
            return status_code.into_response();
        }
    };

    if step != 1 {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        println!(
            "{process_id} - Status: \"{status_code}\" Error: \"Step From Account Change is Not One\""
        );
        return status_code.into_response();
    }

    let account_change = AccountChange {
        username: None,
        email: Some(new_email.clone()),
        password: None,
        verified: None,
        step: Some(2),
    };

    match delete_account_change(&confirmation_code, &database_pool).await {
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

    let confirmation_code =
        match create_account_change(&account.account_id, &account_change, &database_pool).await {
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

    match send_confirmation_email(&new_email, &email_subject, &email_title, &confirmation_code)
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

pub async fn confirm_update_email_step_two_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(body): Json<ConfirmUpdateEmailStepTwoRequest>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account update email step two confirmation\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();
    let confirmation_code = body.confirmation_code;

    let account = match get_account_from_token(&token, &database_pool).await {
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

    let account_change =
        match get_account_change(&account.account_id, &confirmation_code, &database_pool).await {
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

    match account_change.email {
        Some(value) => value,
        _ => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!("{process_id} - Status: \"{status_code}\" Error: \"New Email From Account Change is None\"");
            return status_code.into_response();
        }
    };

    let step = match account_change.step {
        Some(value) => value,
        _ => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            println!("{process_id} - Status: \"{status_code}\" Error: \"Step From Account Change is None\"");
            return status_code.into_response();
        }
    };

    if step != 2 {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        println!(
            "{process_id} - Status: \"{status_code}\" Error: \"Step From Account Change is Not Two\""
        );
        return status_code.into_response();
    }

    match confirm_account_change(&account.account_id, &confirmation_code, &database_pool).await {
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

    let token = match get_account_token(&account.account_id, &database_pool).await {
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

    let response = ConfirmUpdateEmailStepTwoResponse { token };

    (StatusCode::OK, Json(response)).into_response()
}
