use crate::{
    config::CONFIG,
    error::{EncryptionError, Error},
    prelude::*,
};
use bcrypt::{hash, verify};

pub fn encrypt_string(password: &str) -> Result<String> {
    let encryption_processing_cost = CONFIG.encryption_processing_cost;

    hash(password, encryption_processing_cost)
        .map_err(|err| Error::Encryption(EncryptionError::EncryptString(err.to_string())))
}

pub fn compare_plain_to_encrypted_string(string: &String, encrypted_string: &str) -> Result<bool> {
    verify(string, encrypted_string).map_err(|err| {
        Error::Encryption(EncryptionError::ComparePlainToEncryptedString(
            err.to_string(),
        ))
    })
}
