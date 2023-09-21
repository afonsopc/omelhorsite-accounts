use super::models::{
    Account, AccountChange, AccountChangesTable, AccountTokenClaims, AccountsTable,
};
use crate::{
    prelude::*,
    utils::{
        encryption::compare_plain_to_encrypted_string,
        jwt::{create_jwt, decode_jwt},
    },
};
use sqlx::{query_as, PgPool, Postgres};

pub async fn get_account_from_id(account_id: &String, database_pool: &PgPool) -> Result<Account> {
    let sql = r#"
        SELECT * FROM accounts WHERE account_id = $1
    "#;

    let account_row = match query_as::<Postgres, AccountsTable>(sql)
        .bind(account_id)
        .fetch_one(database_pool)
        .await
    {
        Ok(value) => value,
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountNotFound(sql.to_string())),
        Err(err) => return Err(Error::GetAccountFromIdQuery(err.to_string())),
    };

    let account = Account {
        name: account_row.name,
        email: account_row.email,
        password: account_row.password,
        language: account_row.language,
    };

    Ok(account)
}

pub async fn get_account_token(account_id: &String, database_pool: &PgPool) -> Result<String> {
    let sql = r#"
        SELECT * FROM accounts WHERE account_id = $1
    "#;

    let account_row = match query_as::<Postgres, AccountsTable>(sql)
        .bind(account_id)
        .fetch_one(database_pool)
        .await
    {
        Ok(value) => value,
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountNotFound(sql.to_string())),
        Err(err) => return Err(Error::GetAccountFromIdQuery(err.to_string())),
    };

    let account_token_claims = AccountTokenClaims {
        account_id: account_row.account_id,
        last_change_timestamp: account_row.last_change_timestamp,
        exp: usize::MAX,
    };

    let token = create_jwt::<AccountTokenClaims>(&account_token_claims)?;

    Ok(token)
}

pub async fn get_account_from_token(token: &String, database_pool: &PgPool) -> Result<Account> {
    let account_token_claims = decode_jwt::<AccountTokenClaims>(token)?;

    let account_id = &account_token_claims.account_id;
    let last_change_timestamp = &account_token_claims.last_change_timestamp;

    let sql = r#"
        SELECT * FROM accounts 
        WHERE account_id = $1 
        AND last_change_timestamp = $2
    "#;

    let account_row = match query_as::<Postgres, AccountsTable>(sql)
        .bind(account_id)
        .bind(last_change_timestamp)
        .fetch_one(database_pool)
        .await
    {
        Ok(value) => value,
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountNotFound(sql.to_string())),
        Err(err) => {
            return Err(Error::GetAccountFromIdAndLastChangeTimestampQuery(
                err.to_string(),
            ))
        }
    };

    let account = Account {
        name: account_row.name,
        email: account_row.email,
        password: account_row.password,
        language: account_row.language,
    };

    Ok(account)
}

pub async fn get_account_from_credentials(
    email: &String,
    password: &String,
    database_pool: &PgPool,
) -> Result<Account> {
    let sql = r#"
        SELECT * FROM accounts 
        WHERE email = $1 
    "#;

    let account_row = match query_as::<Postgres, AccountsTable>(sql)
        .bind(email)
        .fetch_one(database_pool)
        .await
    {
        Ok(value) => value,
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountNotFound(sql.to_string())),
        Err(err) => return Err(Error::GetAccountFromEmail(err.to_string())),
    };

    compare_plain_to_encrypted_string(password, &account_row.password).await?;

    let account = Account {
        name: account_row.name,
        email: account_row.email,
        password: account_row.password,
        language: account_row.language,
    };

    Ok(account)
}

pub async fn get_account_change(
    account_id: &String,
    account_change_id: &String,
    database_pool: &PgPool,
) -> Result<AccountChange> {
    let sql = r#"
        SELECT * FROM account_changes 
        WHERE account_change_id = $1
        AND account_id = $2
    "#;

    let account_change_row = match query_as::<Postgres, AccountChangesTable>(sql)
        .bind(account_change_id)
        .bind(account_id)
        .fetch_one(database_pool)
        .await
    {
        Ok(value) => value,
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountChangeNotFound(sql.to_string())),
        Err(err) => {
            return Err(Error::GetAccountChangeFromChangeIdAndAccountId(
                err.to_string(),
            ))
        }
    };

    let account_change = AccountChange {
        name: account_change_row.name,
        email: account_change_row.email,
        password: account_change_row.password,
        verified: account_change_row.verified,
        step: account_change_row.step,
    };

    Ok(account_change)
}
