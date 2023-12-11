use crate::config::CONFIG;
use crate::error::Error;
use crate::error::TokenError;
use crate::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Deserialize;
use serde::Serialize;

pub fn create_token<T>(claims: &T) -> Result<String>
where
    T: Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let token_secret_key = &CONFIG.token_secret_key;

    let token = match encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(token_secret_key.as_ref()),
    ) {
        Ok(value) => value,
        Err(err) => return Err(Error::Token(TokenError::CreateToken(err.to_string()))),
    };

    Ok(token)
}

pub fn decode_token<T>(token: &str) -> Result<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let token_secret_key = &CONFIG.token_secret_key;

    match decode::<T>(
        token,
        &DecodingKey::from_secret(token_secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(value) => Ok(value.claims),
        Err(err) => Err(Error::Token(TokenError::DecodeToken(err.to_string()))),
    }
}
