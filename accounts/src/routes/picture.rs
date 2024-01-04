use crate::{
    config::CONFIG, database::DATABASE_POOL, get_decode_verify_and_return_session_token,
    prelude::*, random::get_random_string,
};
use tide::{Response, StatusCode};

#[tracing::instrument]
pub async fn upload_picture(mut req: tide::Request<()>) -> tide::Result {
    // GET DECODE AND VERIFY TOKEN

    let session_token = match get_decode_verify_and_return_session_token(&req).await {
        Ok(session_token) => session_token,
        Err(err) => {
            let mut response = Response::new(StatusCode::Unauthorized);
            response.set_error(err);
            return Ok(response);
        }
    };

    let session = session_token.session;

    // GET ACCOUNT ID FROM TOKEN

    let account_id = session.account_id;

    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // DELETE PREVIOUS PICTURE

    let query = sqlx::query!(
        r#"
            SELECT picture_id 
            FROM accounts 
            WHERE id = $1
        "#,
        account_id
    );

    let result = query.fetch_optional(&mut *transaction).await?;
    let previous_picture_id = result.and_then(|row| row.picture_id);

    // GET IMAGE BYTES FROM REQUEST BODY

    let bytes: Vec<u8> = req.body_bytes().await?;

    let mut img = image::load_from_memory(&bytes)?;

    // RESIZE IMAGE
    img = img.resize_exact(
        CONFIG.picture_width,
        CONFIG.picture_height,
        image::imageops::FilterType::Nearest,
    );

    // CONVERT IMAGE TO RGB

    let rgb_img = img.to_rgb8();

    // ENCODE THE IMAGE

    let webp_bytes = async_std::task::spawn_blocking(move || {
        let webp_encoder =
            webp::Encoder::new(&rgb_img, webp::PixelLayout::Rgb, img.width(), img.height());
        let webp_memory = webp_encoder.encode(CONFIG.picture_quality);
        webp_memory.to_vec()
    })
    .await;

    // GENERATE A NEW RANDOM PICTURE ID

    let new_picture_id = get_random_string(CONFIG.picture_id_length);

    // UPDATE ACCOUNT WITH NEW PICTURE ID

    let query = sqlx::query!(
        r#"
            UPDATE accounts
            SET picture_id = $1
            WHERE id = $2
        "#,
        new_picture_id,
        account_id
    );

    let result = query.execute(&mut *transaction).await?;

    if result.rows_affected() != 1 {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::InternalServerError);
        return Ok(response);
    }

    // WRITE THE WEBP BYTES TO A FILE

    let new_image_file_path = f!("{}/{}.webp", CONFIG.pictures_directory, new_picture_id);
    std::fs::write(new_image_file_path, &*webp_bytes)?;

    // DELETE THE PREVIOUS PICTURE IF THERE WAS ONE

    if let Some(previous_picture_id) = previous_picture_id {
        let previous_image_file_path =
            f!("{}/{}.webp", CONFIG.pictures_directory, previous_picture_id);
        std::fs::remove_file(previous_image_file_path)?;
    }

    // COMMIT DATABASE TRANSACTION

    transaction.commit().await?;

    // OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}

#[tracing::instrument]
pub async fn get_picture(req: tide::Request<()>) -> tide::Result {
    // GET PICTURE ID FROM URL

    let picture_id: String = match req.param("picture_id") {
        Ok(picture_id) => picture_id.to_string(),
        Err(err) => {
            let mut response = Response::new(StatusCode::UnprocessableEntity);
            response.set_error(err);

            return Ok(response);
        }
    };
    // BEGIN DATABASE TRANSACTION

    let mut transaction = DATABASE_POOL.begin().await?;

    // GET PICTURE BY ID

    let query = sqlx::query!(
        r#"
            SELECT EXISTS(
                SELECT 1
                FROM accounts
                WHERE picture_id = $1
            ) AS exists
        "#,
        picture_id
    );

    let result = query.fetch_one(&mut *transaction).await?;

    if let Some(false) = result.exists {
        transaction.rollback().await?;
        let response = Response::new(StatusCode::NotFound);
        return Ok(response);
    }

    // GET PICTURE BY ID

    let picture_file_path = f!("{}/{}.webp", CONFIG.pictures_directory, picture_id);

    let picture_bytes = std::fs::read(picture_file_path)?;

    // CREATE A RESPONSE WITH THE PICTURE BYTES
    // AND THE CORRECT CONTENT TYPE

    let response = Response::builder(StatusCode::Ok)
        .body(picture_bytes)
        .content_type("image/webp")
        .build();

    Ok(response)
}
