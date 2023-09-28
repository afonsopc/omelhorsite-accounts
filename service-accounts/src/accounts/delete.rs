use sqlx::{query, query_as, PgPool, Postgres};

use crate::prelude::*;

use super::models::{AccountEmail, AccountId};

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
) -> Result<Vec<String>> {
    let sql = r#"
        DELETE FROM accounts
        WHERE 
            (CAST(creation_timestamp AS BIGINT)) < (EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) - $1)
            AND verified = false
        RETURNING email
    "#;

    let mut removed_emails: Vec<String> = Vec::new();

    match query_as::<Postgres, AccountEmail>(sql)
        .bind(expiration)
        .fetch_all(database_pool)
        .await
    {
        Ok(rows) => {
            for row in rows {
                removed_emails.push(row.email);
            }
            Ok(removed_emails)
        }
        Err(err) => Err(Error::DeleteExpiredUnverifiedAccountsQuery(err.to_string())),
    }
}

pub async fn delete_expired_account_changes(
    expiration: i64,
    database_pool: &PgPool,
) -> Result<Vec<String>> {
    let sql = r#"
        DELETE FROM account_changes
        WHERE (CAST(creation_timestamp AS BIGINT)) < (EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) - $1)
        RETURNING account_id
    "#;

    let mut removed_account_ids: Vec<String> = Vec::new();

    match query_as::<Postgres, AccountId>(sql)
        .bind(expiration)
        .fetch_all(database_pool)
        .await
    {
        Ok(rows) => {
            for row in rows {
                removed_account_ids.push(row.account_id);
            }
            Ok(removed_account_ids)
        }
        Err(err) => Err(Error::DeleteExpiredAccountChangesQuery(err.to_string())),
    }
}
