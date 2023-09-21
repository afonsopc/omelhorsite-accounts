use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub account_confirmation_lifespan: i64,
    pub check_timeout: u64,
    pub account_ids_length: usize,
    pub account_info_change_ids_length: usize,
    pub create_id_retry_limit: usize,
    pub jwt_secret_key: String,
    pub string_encryption_processing_cost: u32,
    pub email_username: String,
    pub email_address: String,
    pub email_password: String,
    pub smtp_relay: String,
    pub confirmation_email_body: String,
    pub confirmation_email_title_message_placeholder: String,
    pub confirmation_email_confirmation_code_placeholder: String,
    pub account_info_change_confirmation_email_subject: String,
    pub account_info_change_confirmation_email_title_message: String,
    pub account_confirmation_email_subject: String,
    pub account_confirmation_email_title_message: String,
    pub max_body_size: usize,
    pub server_url: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        Ok(AppConfig {
            database_url: environment_variable("DATABASE_URL")?,
            account_confirmation_lifespan: environment_variable("ACCOUNT_CONFIRMATION_LIFESPAN")?,
            check_timeout: environment_variable("CHECK_TIMEOUT")?,
            account_ids_length: environment_variable("ACCOUNT_IDS_LENGTH")?,
            account_info_change_ids_length: environment_variable("ACCOUNT_INFO_CHANGE_IDS_LENGTH")?,
            create_id_retry_limit: environment_variable("CREATE_ID_RETRY_LIMIT")?,
            jwt_secret_key: environment_variable("JWT_SECRET_KEY")?,
            string_encryption_processing_cost: environment_variable(
                "STRING_ENCRYPTION_PROCESSING_COST",
            )?,
            email_username: environment_variable("EMAIL_USERNAME")?,
            email_address: environment_variable("EMAIL_ADDRESS")?,
            email_password: environment_variable("EMAIL_PASSWORD")?,
            smtp_relay: environment_variable("SMTP_RELAY")?,
            confirmation_email_body: environment_variable("CONFIRMATION_EMAIL_BODY")?,
            confirmation_email_title_message_placeholder: environment_variable(
                "CONFIRMATION_EMAIL_TITLE_MESSAGE_PLACEHOLDER",
            )?,
            confirmation_email_confirmation_code_placeholder: environment_variable(
                "CONFIRMATION_EMAIL_CONFIRMATION_CODE_PLACEHOLDER",
            )?,
            account_info_change_confirmation_email_subject: environment_variable(
                "ACCOUNT_INFO_CHANGE_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_info_change_confirmation_email_title_message: environment_variable(
                "ACCOUNT_INFO_CHANGE_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_confirmation_email_subject: environment_variable(
                "ACCOUNT_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            max_body_size: environment_variable("MAX_BODY_SIZE")?,
            server_url: environment_variable("SERVER_URL")?,
        })
    }
}

trait EnvVarConvertible {
    fn from_environment_variable(variable_name: &str) -> Result<Self>
    where
        Self: Sized;
}

impl EnvVarConvertible for String {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

impl EnvVarConvertible for i64 {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

impl EnvVarConvertible for u16 {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

impl EnvVarConvertible for u32 {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

impl EnvVarConvertible for u64 {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

impl EnvVarConvertible for usize {
    fn from_environment_variable(variable_name: &str) -> Result<Self> {
        match env::var(variable_name) {
            Ok(value) => match value.parse() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::ConvertConfigVariable(err.to_string())),
            },
            Err(err) => Err(Error::ReadConfigVariable(
                variable_name.to_string(),
                err.to_string(),
            )),
        }
    }
}

fn environment_variable<T: EnvVarConvertible>(variable_name: &str) -> Result<T> {
    T::from_environment_variable(variable_name)
}
