use sqlx::{query, PgPool};

use crate::prelude::*;

pub async fn delete_account_change(
    account_change_id: &String,
    database_pool: &PgPool,
) -> Result<()> {
    let sql = r#"
        DELETE FROM account_changes WHERE account_change_id = $1
    "#;

    match query(sql)
        .bind(account_change_id)
        .execute(database_pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(Error::AccountChangeNotFound(sql.to_string())),
        Err(err) => Err(Error::DeleteAccountChangeQuery(err.to_string())),
    }
}

pub async fn delete_expired_unverified_accounts(
    expiration: i64,
    database_pool: &PgPool,
) -> Result<()> {
    let sql = r#"
        DELETE FROM accounts
        WHERE 
            (CAST(creation_timestamp AS BIGINT)) < (EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) - $1)
            AND verified = false
    "#;

    match query(sql).bind(expiration).execute(database_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::DeleteExpiredUnverifiedAccountsQuery(err.to_string())),
    }
}

pub async fn delete_expired_account_info_changes(
    expiration: i64,
    database_pool: &PgPool,
) -> Result<()> {
    let sql = r#"
        DELETE FROM account_changes
        WHERE (CAST(creation_timestamp AS BIGINT)) < (EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) - $1)
    "#;

    match query(sql).bind(expiration).execute(database_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::DeleteExpiredAccountChangesQuery(err.to_string())),
    }
}
