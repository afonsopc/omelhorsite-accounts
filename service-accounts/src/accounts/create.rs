use crate::prelude::*;
use crate::utils::encryption::encrypt_string;
use crate::utils::random::{get_random_numbers, get_random_string};
use crate::utils::time::get_current_timestamp;
use crate::{accounts::models::Account, utils::config::AppConfig};
use sqlx::{query, query_as, PgPool, Postgres};

use super::models::AccountsTable;

pub async fn create_account_id(database_pool: &PgPool) -> Result<String> {
    let app_config = AppConfig::load_from_env().unwrap();
    let account_ids_length = app_config.account_ids_length;
    let create_id_retry_limit = app_config.create_id_retry_limit;

    let mut create_id_attempts = 0;

    let sql = r#"
        SELECT 1 FROM accounts WHERE account_id = $1 LIMIT 1
    "#;

    let account_id = loop {
        if create_id_attempts > create_id_retry_limit {
            return Err(Error::CreateIdRetryLimitExceeded());
        };

        let random_id = get_random_string(account_ids_length);

        match query_as::<Postgres, AccountsTable>(sql)
            .bind(&random_id)
            .fetch_one(database_pool)
            .await
        {
            Ok(_) => (),
            Err(sqlx::Error::RowNotFound) => break random_id,
            Err(err) => return Err(Error::CheckIfAccountIdAlreadyExistsQuery(err.to_string())),
        }

        create_id_attempts += 1;
    };

    Ok(account_id)
}

pub async fn create_account_change_id(database_pool: &PgPool) -> Result<String> {
    let app_config = AppConfig::load_from_env().unwrap();
    let account_info_change_ids_length = app_config.account_info_change_ids_length;
    let create_id_retry_limit = app_config.create_id_retry_limit;

    let mut create_id_attempts = 0;

    let sql = r#"
        SELECT 1 FROM account_changes WHERE account_change_id = $1 LIMIT 1
    "#;

    let account_change_id = loop {
        if create_id_attempts > create_id_retry_limit {
            return Err(Error::CreateIdRetryLimitExceeded());
        };

        let random_id = get_random_numbers(account_info_change_ids_length);

        match query_as::<Postgres, AccountsTable>(sql)
            .bind(&random_id)
            .fetch_one(database_pool)
            .await
        {
            Ok(_) => (),
            Err(sqlx::Error::RowNotFound) => break random_id,
            Err(err) => {
                return Err(Error::CheckIfAccountChangeIdAlreadyExistsQuery(
                    err.to_string(),
                ))
            }
        }

        create_id_attempts += 1;
    };

    Ok(account_change_id)
}

pub async fn create_account(account: &Account, database_pool: &PgPool) -> Result<String> {
    let current_timestamp = get_current_timestamp();

    let account_id = create_account_id(database_pool).await?;

    let encrypted_password = encrypt_string(&account.password).await?;

    let account_row = AccountsTable {
        account_id: account_id.clone(),
        username: account.username.to_owned(),
        email: account.email.to_owned(),
        password: encrypted_password,
        language: account.language.to_owned(),
        verified: false,
        last_change_timestamp: current_timestamp.clone(),
        creation_timestamp: current_timestamp,
    };

    let sql_query = r#"
        INSERT INTO accounts (
            account_id,
            username,
            email,
            password,
            language,
            verified,
            last_change_timestamp,
            creation_timestamp
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#;

    match query(sql_query)
        .bind(&account_row.account_id)
        .bind(&account_row.username)
        .bind(&account_row.email)
        .bind(&account_row.password)
        .bind(&account_row.language)
        .bind(account_row.verified)
        .bind(&account_row.last_change_timestamp)
        .bind(&account_row.creation_timestamp)
        .execute(database_pool)
        .await
    {
        Ok(_) => (),
        Err(err) => match err {
            sqlx::Error::Database(err) => {
                if err.is_unique_violation() {
                    return Err(Error::CreateAccountDuplicateKey(err.message().to_string()));
                } else {
                    return Err(Error::CreateAccountQuery(err.message().to_string()));
                }
            }
            err => return Err(Error::CreateAccountQuery(err.to_string())),
        },
    };

    Ok(account_id)
}
