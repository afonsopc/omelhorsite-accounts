use crate::config::CONFIG;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Tablet,
    Console,
    Fridge,
    Teapot,
    Toaster,
    AirConditioner,
    Car,
    Blender,
    VacuumCleaner,
    WashingMachine,
    LawnMower,
    Microwave,
    HairDryer,
    ElectricToothbrush,
    Desktop,
    Laptop,
    Television,
    Mobile,
    SpaceShip,
    TimeMachine,
    Hoverboard,
    Teleporter,
    MagicCarpet,
    Unicorn,
    FlyingBroom,
    Submarine,
    HotAirBalloon,
    Keychain,
    AlarmClock,
    Radio,
    RecordPlayer,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Group {
    Administrator,
    Moderator,
    Default,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    NotSpecified,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Dark,
    Light,
    Automatic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
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
    pub new_email_verification_code: Option<String>,
    pub email_verification_codes_created_at: Option<NaiveDateTime>,
    pub new_password_verification_code: Option<String>,
    pub new_password_verification_code_created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSafe {
    pub id: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub account_id: String,
    pub device: String,
    pub device_type: DeviceType,
    pub expire_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub session: Session,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictString {
    pub conflict: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
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
    #[validate(length(min = 1), custom = "validate_handle_length")]
    pub handle: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinishAccountCreationRequest {
    #[validate(custom = "validate_verification_code_length")]
    pub verification_code: String,
    #[validate(length(min = 1), custom = "validate_handle_length")]
    pub handle: String,
    #[validate(length(min = 1), custom = "validate_name_length")]
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

fn validate_verification_code_length(handle: &str) -> Result<(), ValidationError> {
    if handle.len() != CONFIG.verification_code_length {
        return Err(ValidationError::new("verification_code_length_wrong"));
    }

    Ok(())
}

fn validate_handle_length(handle: &str) -> Result<(), ValidationError> {
    if handle.len() > CONFIG.handle_max_length {
        return Err(ValidationError::new("handle_length_exceeded"));
    }

    Ok(())
}

fn validate_name_length(name: &str) -> Result<(), ValidationError> {
    if name.len() > CONFIG.name_max_length {
        return Err(ValidationError::new("name_length_exceeded"));
    }

    Ok(())
}

// End region: Create Account Request Model

// Region: Sessions Request Models

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSessionRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1), custom = "validate_device_max_length")]
    pub device: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteSessionRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangeSessionDeviceTypeRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetSessionsRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
    pub ammount: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSessionsResponse {
    pub sessions: Vec<Session>,
}

fn validate_session_id_length(session_id: &str) -> Result<(), ValidationError> {
    if session_id.len() != CONFIG.session_id_length {
        return Err(ValidationError::new("session_id_length_exceeded"));
    }

    Ok(())
}

fn validate_device_max_length(device: &str) -> Result<(), ValidationError> {
    if device.len() > CONFIG.device_name_max_length {
        return Err(ValidationError::new("device_name_length_exceeded"));
    }

    Ok(())
}

// End region: Sessions Request Models

// Region: Email Change Request Model

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BeginEmailChangeRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinishEmailChangeRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_verification_code_length")]
    pub original_email_verification_code: String,
    #[validate(custom = "validate_verification_code_length")]
    pub new_email_verification_code: String,
}

// End region: Email Change Request Model
