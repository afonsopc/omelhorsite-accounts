use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct AccountsTable {
    pub account_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub last_change_timestamp: String,
    pub creation_timestamp: String,
}

#[derive(FromRow, Debug)]
pub struct AccountChangesTable {
    pub account_change_id: String,
    pub account_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub verified: Option<bool>,
    pub step: Option<i16>,
    pub creation_timestamp: String,
}

#[derive(FromRow, Debug)]
pub struct AccountId {
    pub account_id: String,
}

#[derive(FromRow, Debug)]
pub struct AccountEmail {
    pub email: String,
}

#[derive(Debug)]
pub struct AccountChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub verified: Option<bool>,
    pub step: Option<i16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountWithExtraInfo {
    pub account_id: String,
    pub account: Account,
    pub creation_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTokenClaims {
    pub account_id: String,
    pub last_change_timestamp: String,
    pub exp: usize,
}
