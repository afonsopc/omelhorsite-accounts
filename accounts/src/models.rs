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

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Group {
    Administrator,
    Moderator,
    Default,
}

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    NotSpecified,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub picture_id: Option<String>,
    pub handle: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub group: Group,
    pub gender: Gender,
    pub email_is_public: bool,
    pub gender_is_public: bool,
    pub country_code: String,
    pub created_at: NaiveDateTime,
    pub original_email_verification_code: Option<String>,
    pub new_email_verification_code: Option<String>,
    pub email_verification_codes_created_at: Option<NaiveDateTime>,
    pub new_password_verification_code: Option<String>,
    pub new_password_verification_code_created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPublic {
    pub id: Option<String>,
    pub picture_id: Option<String>,
    pub handle: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<Group>,
    pub gender: Option<Gender>,
    pub email_is_public: Option<bool>,
    pub gender_is_public: Option<bool>,
    pub country_code: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub account_id: String,
    pub device_name: String,
    pub device_description: String,
    pub device_type: DeviceType,
    pub expire_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenInfo {
    pub id: String,
    pub account_id: String,
    pub expire_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub session: SessionTokenInfo,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionList {
    pub sessions: Vec<Session>,
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
    pub email_is_public: bool,
    #[validate(length(min = 1))]
    pub password: String,
    pub gender: Gender,
    pub gender_is_public: bool,
    #[validate(length(min = 1))]
    pub country_code: String,
}

// End region: Create Account Request Model

// Region: Sessions Request Models

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSessionRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1), custom = "validate_device_name_max_length")]
    pub device_name: String,
    #[validate(length(min = 1), custom = "validate_device_description_max_length")]
    pub device_description: String,
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
pub struct ChangeSessionDeviceNameRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
    #[validate(length(min = 1), custom = "validate_device_name_max_length")]
    pub device_name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangeSessionDeviceDescriptionRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
    #[validate(length(min = 1), custom = "validate_device_description_max_length")]
    pub device_description: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetSessionsRequest {
    #[validate(custom = "validate_session_id_length")]
    pub session_id: String,
    pub ammount: usize,
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

// Region: Password Change Request Model

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinishPasswordChangeRequest {
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(custom = "validate_verification_code_length")]
    pub verification_code: String,
}

// End region: Password Change Request Model

// Region: Account Deletion Request Model

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinishAccountDeletionRequest {
    #[validate(custom = "validate_verification_code_length")]
    pub verification_code: String,
}

// End region: Account Deletion Request Model

// Region: Account Info Change Request Models

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AccountInfoChangeRequest {
    #[validate(length(min = 1), custom = "validate_handle_length")]
    pub handle: Option<String>,
    #[validate(length(min = 1), custom = "validate_name_length")]
    pub name: Option<String>,
    pub gender: Option<Gender>,
    #[validate(length(min = 1))]
    pub country_code: Option<String>,
}

// End region: Account Info Change Request Models

// Region: Account Get Request Models

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoToGet {
    pub id: Option<bool>,
    pub picture_id: Option<bool>,
    pub handle: Option<bool>,
    pub name: Option<bool>,
    pub email: Option<bool>,
    pub email_is_public: Option<bool>,
    pub group: Option<bool>,
    pub gender: Option<bool>,
    pub gender_is_public: Option<bool>,
    pub country_code: Option<bool>,
    pub created_at: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetAccountRequest {
    #[validate(length(min = 1), custom = "validate_account_id_length")]
    pub id: Option<String>,
    #[validate(length(min = 1), custom = "validate_handle_length")]
    pub handle: Option<String>,
    pub info_to_get: Option<AccountInfoToGet>,
}

// End region: Account Get Request Models

// Region: Account Picture Request Models

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetPictureRequest {
    #[validate(custom = "validate_picture_id_length")]
    pub picture_id: String,
}

// End region: Account Picture Request Models

fn validate_picture_id_length(picture_id: &str) -> Result<(), ValidationError> {
    if picture_id.len() != CONFIG.picture_id_length {
        return Err(ValidationError::new("picture_id_length_exceeded"));
    }

    Ok(())
}

fn validate_session_id_length(session_id: &str) -> Result<(), ValidationError> {
    if session_id.len() != CONFIG.session_id_length {
        return Err(ValidationError::new("session_id_length_exceeded"));
    }

    Ok(())
}

fn validate_device_name_max_length(device_name: &str) -> Result<(), ValidationError> {
    if device_name.len() > CONFIG.device_name_max_length {
        return Err(ValidationError::new("device_name_length_exceeded"));
    }

    Ok(())
}

fn validate_device_description_max_length(device_description: &str) -> Result<(), ValidationError> {
    if device_description.len() > CONFIG.device_description_max_length {
        return Err(ValidationError::new("device_description_length_exceeded"));
    }

    Ok(())
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

fn validate_account_id_length(account_id: &str) -> Result<(), ValidationError> {
    if account_id.len() != CONFIG.account_id_length {
        return Err(ValidationError::new("account_id_length_exceeded"));
    }

    Ok(())
}
