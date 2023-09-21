use super::{
    create::create_account_change_id,
    models::{AccountChange, AccountChangesTable},
};
use crate::{
    prelude::*,
    utils::{encryption::encrypt_string, time::get_current_timestamp},
};
use sqlx::{query, query_as, PgPool, Postgres};

pub async fn create_account_change(
    account_id: &String,
    account_change: &AccountChange,
    database_pool: &PgPool,
) -> Result<String> {
    let current_timestamp = get_current_timestamp();

    let account_change_id = create_account_change_id(database_pool).await?;

    let account_change_password = if account_change.password.is_some() {
        let password = account_change.password.to_owned().unwrap();
        let encrypted_password = encrypt_string(&password).await?;
        Some(encrypted_password)
    } else {
        account_change.password.to_owned()
    };

    let account_change_row = AccountChangesTable {
        account_change_id,
        account_id: account_id.to_owned(),
        name: account_change.name.to_owned(),
        email: account_change.email.to_owned(),
        password: account_change_password,
        verified: account_change.verified,
        creation_timestamp: current_timestamp,
        step: account_change.step,
    };

    let sql = r#"
        INSERT INTO account_changes (
            account_change_id, account_id, name, email, password, verified, step, creation_timestamp
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8
        )
    "#;

    match query(sql)
        .bind(&account_change_row.account_change_id)
        .bind(&account_change_row.account_id)
        .bind(&account_change_row.name)
        .bind(&account_change_row.email)
        .bind(&account_change_row.password)
        .bind(account_change_row.verified)
        .bind(account_change_row.step)
        .bind(&account_change_row.creation_timestamp)
        .execute(database_pool)
        .await
    {
        Ok(_) => (),
        Err(err) => match err {
            sqlx::Error::Database(err) => {
                if err.is_unique_violation() {
                    return Err(Error::CreateAccountChangeDuplicateKey(
                        err.message().to_string(),
                    ));
                } else {
                    return Err(Error::CreateAccountChangeQuery(err.message().to_string()));
                }
            }
            err => return Err(Error::CreateAccountChangeQuery(err.to_string())),
        },
    }

    Ok(account_change_row.account_change_id)
}

pub async fn confirm_account_change(
    account_id: &String,
    account_change_id: &String,
    database_pool: &PgPool,
) -> Result<()> {
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

    let sql = r#"
            DELETE FROM account_changes 
            WHERE account_change_id = $1
            "#;

    match query(sql)
        .bind(account_change_id)
        .execute(database_pool)
        .await
    {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Err(Error::AccountChangeNotFound(sql.to_string())),
        Err(err) => return Err(Error::DeleteAccountChangeQuery(err.to_string())),
    };

    let current_timestamp = get_current_timestamp();

    let sql = r#"
            UPDATE accounts 
            SET
                name = CASE WHEN $1 IS NOT NULL THEN $1 ELSE name END,
                email = CASE WHEN $2 IS NOT NULL THEN $2 ELSE email END,
                password = CASE WHEN $3 IS NOT NULL THEN $3 ELSE password END,
                verified = CASE WHEN $4 IS NOT NULL THEN $4 ELSE verified END,
                last_change_timestamp = $5
            WHERE account_id = $6
        "#;

    match query(sql)
        .bind(&account_change_row.name)
        .bind(&account_change_row.email)
        .bind(&account_change_row.password)
        .bind(account_change_row.verified)
        .bind(&current_timestamp)
        .bind(&account_change_row.account_id)
        .execute(database_pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(Error::AccountNotFound(format!(
            "(query sql censored because of password) UPDATE accounts ... WHERE account_id = {}",
            account_id
        ))),
        Err(err) => Err(Error::UpdateAccountQuery(err.to_string())),
    }
}
