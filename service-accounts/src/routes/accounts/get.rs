use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    Json, TypedHeader,
};

use crate::{
    accounts::get::get_account_from_token, get_process_id,
    routes::accounts::models::GetAccountResponse, AppState,
};

pub async fn get_account_request(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let process_id = get_process_id();
    println!("{process_id} - Starting \"account verification\" request");

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

    let response = GetAccountResponse {
        account: account.account,
    };

    (StatusCode::OK, Json(response)).into_response()
}
