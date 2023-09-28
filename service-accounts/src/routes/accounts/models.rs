use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmationCode {
    pub confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoWithoutPassword {
    pub username: String,
    pub email: String,
    pub creation_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Username {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAndConfirmationCode {
    pub email: String,
    pub confirmation_code: String,
}
