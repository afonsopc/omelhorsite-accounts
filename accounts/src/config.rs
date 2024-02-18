use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "SERVER_HOST")]
    pub server_host: String,

    #[envconfig(from = "HANDLE_MAX_LENGTH")]
    pub handle_max_length: usize,

    #[envconfig(from = "NAME_MAX_LENGTH")]
    pub name_max_length: usize,

    #[envconfig(from = "VERIFICATION_CODE_LENGTH")]
    pub verification_code_length: usize,

    #[envconfig(from = "ENCRYPTION_PROCESSING_COST")]
    pub encryption_processing_cost: u32,

    #[envconfig(from = "TOKEN_SECRET_KEY")]
    pub token_secret_key: String,

    #[envconfig(from = "SESSION_ID_LENGTH")]
    pub session_id_length: usize,

    #[envconfig(from = "ACCOUNT_ID_LENGTH")]
    pub account_id_length: usize,

    #[envconfig(from = "DEVICE_NAME_MAX_LENGTH")]
    pub device_name_max_length: usize,

    #[envconfig(from = "DEVICE_DESCRIPTION_MAX_LENGTH")]
    pub device_description_max_length: usize,

    #[envconfig(from = "PICTURES_DIRECTORY")]
    pub pictures_directory: String,

    #[envconfig(from = "PICTURE_ID_LENGTH")]
    pub picture_id_length: usize,

    #[envconfig(from = "PICTURE_MAX_DIMENTION")]
    pub picture_max_dimention: u32,

    #[envconfig(from = "PICTURE_COMPRESSION")]
    pub picture_compression: f32,

    #[envconfig(from = "PICTURE_MAX_SIZE_IN_MEGABYTES")]
    pub picture_max_size_in_megabytes: u64,

    #[envconfig(from = "MANAGER_EMAIL_ADDRESS")]
    pub manager_email_address: String,

    #[envconfig(from = "SEND_TEST_STARTUP_EMAIL")]
    pub send_test_startup_email: bool,

    #[envconfig(from = "EMAIL_ADDRESS")]
    pub email_address: String,

    #[envconfig(from = "EMAIL_USERNAME")]
    pub email_username: String,

    #[envconfig(from = "SMTP_RELAY")]
    pub smtp_relay: String,

    #[envconfig(from = "SMTP_STARTTLS")]
    pub smtp_starttls: bool,

    #[envconfig(from = "SMTP_PORT")]
    pub smtp_port: u16,

    #[envconfig(from = "SMTP_USERNAME")]
    pub smtp_username: String,

    #[envconfig(from = "SMTP_PASSWORD")]
    pub smtp_password: String,

    #[envconfig(from = "EMAIL_PLACEHOLDER_MARKER")]
    pub email_placeholder_marker: String,

    #[envconfig(from = "SERVICE_STARTUP_EMAIL_HTML")]
    pub service_startup_email_html: bool,

    #[envconfig(from = "SERVICE_STARTUP_EMAIL_SUBJECT")]
    pub service_startup_email_subject: String,

    #[envconfig(from = "SERVICE_STARTUP_EMAIL_BODY")]
    pub service_startup_email_body: String,

    #[envconfig(from = "ACCOUNT_CREATION_VERIFICATION_EMAIL_HTML")]
    pub account_creation_verification_email_html: bool,

    #[envconfig(from = "ACCOUNT_CREATION_VERIFICATION_EMAIL_SUBJECT")]
    pub account_creation_verification_email_subject: String,

    #[envconfig(from = "ACCOUNT_CREATION_VERIFICATION_EMAIL_BODY")]
    pub account_creation_verification_email_body: String,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_ORIGINAL_EMAIL_VERIFICATION_HTML")]
    pub account_email_change_original_email_verification_html: bool,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_ORIGINAL_EMAIL_VERIFICATION_SUBJECT")]
    pub account_email_change_original_email_verification_subject: String,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_ORIGINAL_EMAIL_VERIFICATION_BODY")]
    pub account_email_change_original_email_verification_body: String,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_NEW_EMAIL_VERIFICATION_HTML")]
    pub account_email_change_new_email_verification_html: bool,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_NEW_EMAIL_VERIFICATION_SUBJECT")]
    pub account_email_change_new_email_verification_subject: String,

    #[envconfig(from = "ACCOUNT_EMAIL_CHANGE_NEW_EMAIL_VERIFICATION_BODY")]
    pub account_email_change_new_email_verification_body: String,

    #[envconfig(from = "ACCOUNT_PASSWORD_CHANGE_VERIFICATION_HTML")]
    pub account_password_change_verification_html: bool,

    #[envconfig(from = "ACCOUNT_PASSWORD_CHANGE_VERIFICATION_SUBJECT")]
    pub account_password_change_verification_subject: String,

    #[envconfig(from = "ACCOUNT_PASSWORD_CHANGE_VERIFICATION_BODY")]
    pub account_password_change_verification_body: String,

    #[envconfig(from = "ACCOUNT_DELETION_VERIFICATION_HTML")]
    pub account_deletion_verification_html: bool,

    #[envconfig(from = "ACCOUNT_DELETION_VERIFICATION_SUBJECT")]
    pub account_deletion_verification_subject: String,

    #[envconfig(from = "ACCOUNT_DELETION_VERIFICATION_BODY")]
    pub account_deletion_verification_body: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}
