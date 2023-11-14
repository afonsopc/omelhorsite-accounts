use crate::prelude::*;
use bcrypt::{hash, verify};

use super::config;

pub async fn encrypt_string(password: &str) -> Result<String> {
    let app_config = config::AppConfig::load_from_env().unwrap();
    let string_encryption_processing_cost = app_config.string_encryption_processing_cost;

    match hash(password, string_encryption_processing_cost) {
        Ok(value) => Ok(value),
        Err(err) => Err(Error::StringEncrypt(err.to_string())),
    }
}

pub async fn compare_plain_to_encrypted_string(
    string: &String,
    encrypted_string: &str,
) -> Result<()> {
    match verify(string, encrypted_string) {
        Ok(value) => match value {
            true => Ok(()),
            false => Err(Error::PlainNotEqualToEncryptedString()),
        },
        Err(err) => Err(Error::ComparePlainToEncryptedString(err.to_string())),
    }
}
