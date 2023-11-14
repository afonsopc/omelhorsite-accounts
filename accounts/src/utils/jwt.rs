use crate::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Deserialize;
use serde::Serialize;

use super::config;

pub fn create_jwt<T>(claims: &T) -> Result<String>
where
    T: Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let app_config = config::AppConfig::load_from_env().unwrap();
    let jwt_secret_key = app_config.jwt_secret_key;

    let token = match encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret_key.as_ref()),
    ) {
        Ok(value) => value,
        Err(err) => return Err(Error::CreateJwt(err.to_string())),
    };

    Ok(token)
}

pub fn decode_jwt<T>(token: &str) -> Result<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let app_config = config::AppConfig::load_from_env().unwrap();
    let jwt_secret_key = app_config.jwt_secret_key;

    match decode::<T>(
        token,
        &DecodingKey::from_secret(jwt_secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(value) => Ok(value.claims),
        Err(err) => Err(Error::DecodeJwt(err.to_string())),
    }
}
