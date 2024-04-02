use crate::{
    database::DATABASE_POOL,
    error::{DatabaseError, Error},
    get_decode_verify_and_return_session_token, get_id_from_handle, is_account_admin_from_id,
    models::{AccountInfoToGet, AccountPublic, Gender, GetAccountRequest, Group},
};
use std::str::FromStr;
use tide::{convert::json, Response, StatusCode};
use validator::Validate;

pub async fn get_is_admin(req: tide::Request<()>) -> tide::Result {
    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            return Ok(response);
        }
    };

    let account_id = session_token.session.account_id;

    let is_admin = match is_account_admin_from_id(&account_id).await {
        Ok(is_admin) => is_admin,
        Err(Error::Database(DatabaseError::RowNotFound)) => {
            let response = Response::new(StatusCode::NotFound);
            return Ok(response);
        }
        Err(_) => {
            let response = Response::new(StatusCode::InternalServerError);
            return Ok(response);
        }
    };

    match is_admin {
        true => Ok(Response::new(StatusCode::Ok)),
        false => Ok(Response::new(StatusCode::Unauthorized)),
    }
}

pub async fn get_account(req: tide::Request<()>) -> tide::Result {
    // GET REQUEST INFO FROM QUERY PARAMS AND VALIDATE IT

    let mut info: GetAccountRequest = req.query()?;

    if info.validate().is_err() {
        let mut response = Response::new(StatusCode::UnprocessableEntity);
        response.set_error(info.validate().unwrap_err());
        return Ok(response);
    };

    info.handle = info.handle.map(|handle| handle.to_lowercase());

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET DECODE AND VERIFY TOKEN

    let (owner_of_account, account_id) = match (
        get_decode_verify_and_return_session_token(&req).await.ok(),
        info.id,
        info.handle,
    ) {
        (Some(session_token), Some(id), _) => {
            let session = session_token.session;
            let is_admin = match is_account_admin_from_id(&id).await {
                Ok(is_admin) => is_admin,
                Err(Error::Database(DatabaseError::RowNotFound)) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::NotFound);
                    return Ok(response);
                }
                Err(_) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::InternalServerError);
                    return Ok(response);
                }
            };

            ((session.account_id == id || is_admin), id)
        }
        (Some(session_token), _, Some(handle)) => {
            let session = session_token.session;

            let id = match get_id_from_handle(&handle).await {
                Ok(id) => id,
                Err(Error::Database(DatabaseError::RowNotFound)) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::NotFound);
                    return Ok(response);
                }
                Err(_) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::InternalServerError);
                    return Ok(response);
                }
            };

            let is_admin = match is_account_admin_from_id(&id).await {
                Ok(is_admin) => is_admin,
                Err(Error::Database(DatabaseError::RowNotFound)) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::NotFound);
                    return Ok(response);
                }
                Err(_) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::InternalServerError);
                    return Ok(response);
                }
            };

            ((session.account_id == id || is_admin), id)
        }
        (Some(session_token), _, _) => {
            let session = session_token.session;

            (true, session.account_id)
        }
        (_, Some(id), _) => (false, id),
        (_, _, Some(handle)) => {
            let id = match get_id_from_handle(&handle).await {
                Ok(id) => id,
                Err(Error::Database(DatabaseError::RowNotFound)) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::NotFound);
                    return Ok(response);
                }
                Err(_) => {
                    transaction.rollback().await?;
                    let response = Response::new(StatusCode::InternalServerError);
                    return Ok(response);
                }
            };

            (false, id)
        }
        _ => {
            transaction.rollback().await?;
            let response = Response::new(StatusCode::NotFound);
            return Ok(response);
        }
    };

    // SEE WHAT INFO TO GET

    let info_to_get = match info.info_to_get {
        Some(info_to_get) => info_to_get,
        None => AccountInfoToGet {
            id: None,
            handle: None,
            name: None,
            email: None,
            email_is_public: None,
            group: None,
            gender: None,
            gender_is_public: None,
            country_code: None,
            created_at: None,
        },
    };

    // GET ONLY THE INFO THAT IS SPECIFIED
    // IN THE info_to_get variable

    let get_email_is_public =
        info_to_get.email_is_public.unwrap_or(false) || info_to_get.email.unwrap_or(false);

    let get_gender_is_public =
        info_to_get.gender_is_public.unwrap_or(false) || info_to_get.gender.unwrap_or(false);

    let query = sqlx::query!(
        r#"
        SELECT
            CASE WHEN $2 THEN id ELSE NULL END AS id,
            CASE WHEN $3 THEN handle ELSE NULL END AS handle,
            CASE WHEN $4 THEN name ELSE NULL END AS name,
            CASE WHEN $5 THEN email ELSE NULL END AS email,
            CASE WHEN $6 THEN "group" ELSE NULL END AS "group",
            CASE WHEN $7 THEN gender ELSE NULL END AS gender,
            CASE WHEN $8 THEN email_is_public ELSE NULL END AS email_is_public,
            CASE WHEN $9 THEN gender_is_public ELSE NULL END AS gender_is_public,
            CASE WHEN $10 THEN country_code ELSE NULL END AS country_code,
            CASE WHEN $11 THEN created_at ELSE NULL END AS created_at
        FROM accounts
        WHERE id = $1;
    "#,
        account_id,
        info_to_get.id,
        info_to_get.handle,
        info_to_get.name,
        info_to_get.email,
        info_to_get.group,
        info_to_get.gender,
        get_email_is_public,
        get_gender_is_public,
        info_to_get.country_code,
        info_to_get.created_at,
    );

    let result = query.fetch_optional(&mut *transaction).await?;

    if result.is_none() {
        transaction.rollback().await?;

        let response = Response::new(StatusCode::NotFound);
        return Ok(response);
    }

    let result = result.unwrap();

    // TREAT EACH FIELD THAT NEEDS TREATMENT

    let treated_email = match (result.email, result.email_is_public) {
        (Some(email), Some(email_is_public)) => {
            if owner_of_account || email_is_public {
                Some(email)
            } else {
                None
            }
        }
        _ => None,
    };

    let treated_group = match result.group {
        Some(group) => Some(Group::from_str(&group)?),
        None => None,
    };

    let treated_gender = match (result.gender, result.gender_is_public) {
        (Some(gender), Some(gender_is_public)) => {
            if owner_of_account || gender_is_public {
                Some(Gender::from_str(&gender)?)
            } else {
                None
            }
        }
        _ => None,
    };

    let account_info = AccountPublic {
        id: result.id,
        handle: result.handle,
        name: result.name,
        email: treated_email,
        group: treated_group,
        gender: treated_gender,
        email_is_public: result.email_is_public,
        gender_is_public: result.gender_is_public,
        country_code: result.country_code,
        created_at: result.created_at,
    };

    // FINALY COMMIT TRANSACTION

    transaction.commit().await?;

    // SEND RESPONSE

    let response = Response::builder(StatusCode::Ok)
        .body(json!(account_info))
        .build();

    Ok(response)
}

pub async fn get_all_accounts(req: tide::Request<()>) -> tide::Result {
    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            return Ok(response);
        }
    };

    let account_id = session_token.session.account_id;

    // CHECK IF ACCOUNT IS ADMIN

    let is_admin = match is_account_admin_from_id(&account_id).await {
        Ok(is_admin) => is_admin,
        Err(Error::Database(DatabaseError::RowNotFound)) => {
            let response = Response::new(StatusCode::NotFound);
            return Ok(response);
        }
        Err(_) => {
            let response = Response::new(StatusCode::InternalServerError);
            return Ok(response);
        }
    };

    if !is_admin {
        let response = Response::new(StatusCode::Unauthorized);
        return Ok(response);
    }

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET ACCOUNTS

    let query = sqlx::query!(
        r#"
        SELECT * FROM accounts;
        "#
    );

    let result = query.fetch_all(&mut *transaction).await?;

    // TREAT EACH FIELD THAT NEEDS TREATMENT

    let accounts = result
        .iter()
        .map(|row| AccountPublic {
            id: Some(row.id.to_string()),
            handle: Some(row.handle.to_string()),
            name: Some(row.name.to_string()),
            email: Some(row.email.to_string()),
            group: Some(Group::from_str(&row.group).unwrap()),
            gender: Some(Gender::from_str(&row.gender).unwrap()),
            email_is_public: Some(row.email_is_public),
            gender_is_public: Some(row.gender_is_public),
            country_code: Some(row.country_code.to_string()),
            created_at: Some(row.created_at),
        })
        .collect::<Vec<AccountPublic>>();

    // FINALY COMMIT TRANSACTION

    let response = Response::builder(StatusCode::Ok)
        .body(json!(accounts))
        .build();

    Ok(response)
}
