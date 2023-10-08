use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    TypedHeader,
};

use crate::{
    accounts::{delete::delete_account_change_from_account_id, get::get_account_from_token},
    utils::random::get_random_process_id,
    AppState,
};

pub async fn delete_confirmations_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let process_id = get_random_process_id();
    println!("{process_id} - Starting \"delete confirmations\" request");

    let database_pool = app_state.database_pool;

    let token = authorization.token().to_owned();

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

    match delete_account_change_from_account_id(&account.account_id, &database_pool).await {
        Ok(_) => (),
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
