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
    pub email_address: String,
    pub email_password: String,
    pub smtp_relay: String,
    pub account_creation_confirmation_email_subject: String,
    pub account_creation_confirmation_email_title_message: String,
    pub account_change_username_confirmation_email_subject: String,
    pub account_change_username_confirmation_email_title_message: String,
    pub account_change_email_one_confirmation_email_subject: String,
    pub account_change_email_one_confirmation_email_title_message: String,
    pub account_change_email_two_confirmation_email_subject: String,
    pub account_change_email_two_confirmation_email_title_message: String,
    pub account_change_password_confirmation_email_subject: String,
    pub account_change_password_confirmation_email_title_message: String,
    pub account_email_authentication_confirmation_email_subject: String,
    pub account_email_authentication_confirmation_email_title_message: String,
    pub account_deletion_confirmation_email_subject: String,
    pub account_deletion_confirmation_email_title_message: String,
    pub email_name_placeholder: String,
    pub confirmation_email_title_message_placeholder: String,
    pub confirmation_email_confirmation_code_placeholder: String,
    pub confirmation_email_body: String,
    pub max_body_size: usize,
    pub server_url: String,
    pub process_id_length: usize,
    pub name: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        Ok(AppConfig {
            server_url: environment_variable("SERVER_URL")?,
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
            max_body_size: environment_variable("MAX_BODY_SIZE")?,
            process_id_length: environment_variable("PROCESS_ID_LENGTH")?,
            name: environment_variable("NAME")?,
            email_address: environment_variable("EMAIL_ADDRESS")?,
            email_password: environment_variable("EMAIL_PASSWORD")?,
            smtp_relay: environment_variable("SMTP_RELAY")?,
            account_creation_confirmation_email_subject: environment_variable(
                "ACCOUNT_CREATION_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_creation_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CREATION_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_change_username_confirmation_email_subject: environment_variable(
                "ACCOUNT_CHANGE_USERNAME_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_change_username_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CHANGE_USERNAME_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_change_email_one_confirmation_email_subject: environment_variable(
                "ACCOUNT_CHANGE_EMAIL_ONE_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_change_email_one_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CHANGE_EMAIL_ONE_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_change_email_two_confirmation_email_subject: environment_variable(
                "ACCOUNT_CHANGE_EMAIL_TWO_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_change_email_two_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CHANGE_EMAIL_TWO_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_change_password_confirmation_email_subject: environment_variable(
                "ACCOUNT_CHANGE_PASSWORD_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_change_password_confirmation_email_title_message: environment_variable(
                "ACCOUNT_CHANGE_PASSWORD_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            account_email_authentication_confirmation_email_subject: environment_variable(
                "ACCOUNT_EMAIL_AUTHENTICATION_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_email_authentication_confirmation_email_title_message: environment_variable(
                "ACCOUNT_EMAIL_AUTHENTICATION_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
            email_name_placeholder: environment_variable("EMAIL_NAME_PLACEHOLDER")?,
            confirmation_email_title_message_placeholder: environment_variable(
                "CONFIRMATION_EMAIL_TITLE_MESSAGE_PLACEHOLDER",
            )?,
            confirmation_email_confirmation_code_placeholder: environment_variable(
                "CONFIRMATION_EMAIL_CONFIRMATION_CODE_PLACEHOLDER",
            )?,
            confirmation_email_body: environment_variable("CONFIRMATION_EMAIL_BODY")?,
            account_deletion_confirmation_email_subject: environment_variable(
                "ACCOUNT_DELETION_CONFIRMATION_EMAIL_SUBJECT",
            )?,
            account_deletion_confirmation_email_title_message: environment_variable(
                "ACCOUNT_DELETION_CONFIRMATION_EMAIL_TITLE_MESSAGE",
            )?,
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
