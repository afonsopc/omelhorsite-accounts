mod accounts;
mod error;
mod prelude;
mod utils;

use accounts::delete::{delete_expired_account_info_changes, delete_expired_unverified_accounts};
use prelude::*;
use sqlx::{migrate, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use utils::config::AppConfig;

use crate::accounts::{
    create::create_account,
    delete::{
        delete_account, delete_account_change, delete_all_account_changes, delete_all_accounts,
    },
    get::get_account_change,
    models::{Account, AccountChange},
    update::{confirm_account_change, create_account_change},
};

#[derive(Clone)]
pub struct AppState {
    pub database_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = AppConfig::load_from_env().unwrap();
    let database_url = &app_config.database_url;
    let account_confirmation_lifespan = app_config.account_confirmation_lifespan;
    let check_timeout = app_config.check_timeout;

    println!("Connecting to Database...");
    let database_pool = PgPool::connect(database_url).await.unwrap();
    println!("Connected to Database.");

    migrate!().run(&database_pool).await.unwrap();

    // Apagar todas as contas
    delete_all_account_changes(&database_pool).await.unwrap();
    delete_all_accounts(&database_pool).await.unwrap();

    // Criar conta
    let account = Account {
        name: "Afonso".to_owned(),
        email: "afonso@mail.pt".to_owned(),
        password: "teste123".to_owned(),
        language: "pt".to_owned(),
    };
    let account_id = create_account(&account, &database_pool).await.unwrap();

    // Create verification process
    let account_confirmation_change = AccountChange {
        name: None,
        email: None,
        password: None,
        verified: Some(true),
        step: None,
    };
    let account_confirmation_code =
        create_account_change(&account_id, &account_confirmation_change, &database_pool)
            .await
            .unwrap();

    // Cofirm account
    confirm_account_change(&account_id, &account_confirmation_code, &database_pool)
        .await
        .unwrap();

    // Change the name
    let account_name_change = AccountChange {
        name: Some("Carlos".to_owned()),
        email: None,
        password: None,
        verified: None,
        step: None,
    };
    let account_name_change_confirmation_code =
        create_account_change(&account_id, &account_name_change, &database_pool)
            .await
            .unwrap();

    confirm_account_change(
        &account_id,
        &account_name_change_confirmation_code,
        &database_pool,
    )
    .await
    .unwrap();

    // Change the password
    let account_password_change = AccountChange {
        name: None,
        email: None,
        password: Some("carlos123".to_owned()),
        verified: None,
        step: None,
    };
    let account_password_change_confirmation_code =
        create_account_change(&account_id, &account_password_change, &database_pool)
            .await
            .unwrap();

    confirm_account_change(
        &account_id,
        &account_password_change_confirmation_code,
        &database_pool,
    )
    .await
    .unwrap();

    // Change the email
    let account_email_change = AccountChange {
        name: None,
        email: Some("carlos@pagman.org".to_owned()),
        password: None,
        verified: None,
        step: Some(1),
    };
    let account_email_change_confirmation_code =
        create_account_change(&account_id, &account_email_change, &database_pool)
            .await
            .unwrap();

    let account_email_change_step_1 = get_account_change(
        &account_id,
        &account_email_change_confirmation_code,
        &database_pool,
    )
    .await
    .unwrap();

    let account_email_change_step_2_confirmation_code: String =
        if let Some(step) = account_email_change_step_1.step {
            if step == 1 {
                let account_email_change_step_2 = AccountChange {
                    name: None,
                    email: account_email_change_step_1.email.clone(),
                    password: None,
                    verified: None,
                    step: Some(2),
                };
                delete_account_change(&account_email_change_confirmation_code, &database_pool)
                    .await
                    .unwrap();
                create_account_change(&account_id, &account_email_change_step_2, &database_pool)
                    .await
                    .unwrap()
            } else {
                panic!("Account email change is not in step 1");
            }
        } else {
            panic!("Account email change step is not Some");
        };

    confirm_account_change(
        &account_id,
        &account_email_change_step_2_confirmation_code,
        &database_pool,
    )
    .await
    .unwrap();

    // Delete account
    delete_account(&account_id, &database_pool).await.unwrap();

    // Check loop
    tokio::spawn(async move {
        let timeout: Duration = Duration::from_secs(check_timeout);
        loop {
            println!("Ran Check");
            sleep(timeout).await;

            check_loop(account_confirmation_lifespan, &database_pool)
                .await
                .unwrap()
        }
    });

    Ok(())
}

async fn check_loop(account_confirmation_lifespan: i64, database_pool: &PgPool) -> Result<()> {
    delete_expired_unverified_accounts(account_confirmation_lifespan, database_pool).await?;
    delete_expired_account_info_changes(account_confirmation_lifespan, database_pool).await?;

    Ok(())
}

// let account_basic_info = AccountBasicInfo {
//     name: "Afonso".to_string(),
//     email: "afonso@mail.pt".to_string(),
//     password: "passwordsecreta".to_string(),
//     language: "pt".to_string(),
// };

// println!("Deleting all accounts.");
// delete_all_account(&database_pool).await.unwrap();

// println!("Deleting all account info changes.");
// delete_all_account_info_changes(&database_pool)
//     .await
//     .unwrap();

// println!("Creating account.");
// let account_id = create_account(&account_basic_info, &database_pool)
//     .await
//     .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// let confirmed_account_info = Account {
//     account_id: None,
//     name: None,
//     email: None,
//     password: None,
//     language: None,
//     verified: Some(true),
//     last_change_timestamp: None,
//     creation_timestamp: None,
// };

// println!("Creating account info change to verify account.");
// let account_verification_change_id =
//     create_account_change(&account_id, &confirmed_account_info, &database_pool)
//         .await
//         .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// println!("Confirming change account inf0 to verify account.");
// confirm_account_change(&account_id, &account_verification_change_id, &database_pool)
//     .await
//     .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(5);
// sleep(_timeout).await;

// let new_account_info = Account {
//     account_id: None,
//     name: None,
//     email: Some("afonso@pagman.org".to_string()),
//     password: None,
//     language: None,
//     verified: None,
//     last_change_timestamp: None,
//     creation_timestamp: None,
// };

// println!("Creating change account info.");
// let account_info_change_id =
//     create_account_change(&account_id, &new_account_info, &database_pool)
//         .await
//         .unwrap();

// println!("Waiting 15 seconds.");
// let _timeout: Duration = Duration::from_secs(15);
// sleep(_timeout).await;

// println!("Confirming change account info.");
// confirm_account_change(&account_id, &account_info_change_id, &database_pool)
//     .await
//     .unwrap();

// let current_timestamp = Utc::now().timestamp().to_string();
// let account = Account {
//     account_id: "1232123422223".to_string(),
//     name: "Fernando".to_string(),
//     email: "fernando@222342mail.pt".to_string(),
//     password: "fernando123".to_string(),
//     language: "pt".to_string(),
//     verified: false,
//     creation_timestamp: current_timestamp
// };

// create_account(&account, &database_pool).await.unwrap();

// let account_record = get_account(&account.account_id, &database_pool).await.unwrap();
// println!("Before: {account_record:?}");

// let new_account = Account {
//     account_id: "".to_string(),
//     name: "Manuel O Maluco".to_string(),
//     email: "".to_string(),
//     password: "".to_string(),
//     language: "".to_string(),
//     verified: true,
//     creation_timestamp: "".to_string()
// };

// update_account(&account.account_id, &new_account, &database_pool).await.unwrap();

// let new_account_record = get_account(&account.account_id, &database_pool).await.unwrap();
// println!("After: {new_account_record:?}");

// delete_account(&account.account_id, &database_pool).await.unwrap();
