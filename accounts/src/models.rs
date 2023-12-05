use crate::config::CONFIG;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Group {
    #[serde(rename = "administrator")]
    #[strum(serialize = "administrator")]
    Administrator,

    #[serde(rename = "moderator")]
    #[strum(serialize = "moderator")]
    Moderator,

    #[serde(rename = "default")]
    #[strum(serialize = "default")]
    Default,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Gender {
    #[serde(rename = "male")]
    #[strum(serialize = "male")]
    Male,
    #[serde(rename = "female")]
    #[strum(serialize = "female")]
    Female,
    #[serde(rename = "not_specified")]
    #[strum(serialize = "not_specified")]
    NotSpecified,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Theme {
    #[serde(rename = "dark")]
    #[strum(serialize = "dark")]
    Dark,
    #[serde(rename = "light")]
    #[strum(serialize = "light")]
    Light,
    #[serde(rename = "automatic")]
    #[strum(serialize = "automatic")]
    Automatic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub picture_id: String,
    pub handle: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub group: Group,
    pub gender: Gender,
    pub theme: Theme,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub original_email_verification_code: Option<String>,
    pub original_email_verification_code_created_at: Option<NaiveDateTime>,
    pub new_email_verification_code: Option<String>,
    pub new_email_verification_code_created_at: Option<NaiveDateTime>,
    pub new_password_verification_code: Option<String>,
    pub new_password_verification_code_created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSafe {
    pub id: Uuid,
    pub picture_id: String,
    pub handle: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub group: Group,
    pub gender: Gender,
    pub theme: Theme,
    pub language: String,
    pub created_at: NaiveDateTime,
}

// Region: Account Creation

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCreationVerification {
    pub email: String,
    pub handle: String,
    pub verification_code: String,
    pub verification_code_created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BeginAccountCreationRequest {
    #[validate(length(min = 1), custom = "validate_handle_lenght")]
    pub handle: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinishAccountCreationRequest {
    #[validate(custom = "validate_verification_code_lenght")]
    pub verification_code: String,
    #[validate(length(min = 1), custom = "validate_handle_lenght")]
    pub handle: String,
    #[validate(length(min = 1), custom = "validate_name_lenght")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub gender: Gender,
    pub theme: Theme,
    #[validate(length(min = 1))]
    pub language: String,
}

fn validate_verification_code_lenght(handle: &str) -> Result<(), ValidationError> {
    if handle.len() != CONFIG.verification_code_length {
        return Err(ValidationError::new("verification_code_length_wrong"));
    }

    Ok(())
}

fn validate_handle_lenght(handle: &str) -> Result<(), ValidationError> {
    if handle.len() > CONFIG.handle_max_length {
        return Err(ValidationError::new("handle_length_exceeded"));
    }

    Ok(())
}

fn validate_name_lenght(name: &str) -> Result<(), ValidationError> {
    if name.len() > CONFIG.name_max_length {
        return Err(ValidationError::new("name_length_exceeded"));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictString {
    pub conflict: String,
}

// End region: Create Account Request Model
