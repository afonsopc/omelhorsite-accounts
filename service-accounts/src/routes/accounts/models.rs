use serde::{Deserialize, Serialize};

use crate::accounts::models::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmAccountRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmAccountResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmDeleteAccountRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsernameRequest {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdateUsernameRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdateUsernameResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdatePasswordRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdatePasswordResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdateEmailStepOneRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdateEmailStepTwoRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmUpdateEmailStepTwoResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountResponse {
    pub account: Account,
}
