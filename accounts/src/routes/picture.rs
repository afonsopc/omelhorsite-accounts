use crate::error::Error;
use crate::error::S3Error;
use crate::get_decode_verify_and_return_session_token;
use crate::prelude::*;
use crate::CONFIG;
use image::ImageError;
use s3::creds::Credentials;
use s3::Bucket;
use s3::Region;
use tide::{Response, StatusCode};

pub async fn put_webp_picture_in_bucket(picture: Vec<u8>, picture_name: &str) -> Result<()> {
    // INSTANTIATE BUCKET

    let bucket = Bucket::new(
        &CONFIG.s3_pictures_bucket,
        Region::Custom {
            region: CONFIG.s3_region.to_owned(),
            endpoint: CONFIG.s3_endpoint.to_owned(),
        },
        Credentials {
            access_key: Some(CONFIG.s3_access_key.to_owned()),
            secret_key: Some(CONFIG.s3_secret_key.to_owned()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .map_err(|err| Error::S3(S3Error::InstantiateBucket(err.to_string())))?
    .with_path_style();

    // PUT OBJECT IN BUCKET

    let response = bucket
        .put_object_with_content_type(picture_name, &picture, "image/webp")
        .await
        .map_err(|err| Error::S3(S3Error::PutObject(err.to_string())))?;

    if response.status_code() == 404 {
        return Err(Error::S3(S3Error::BucketNotFound));
    }

    Ok(())
}

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

    // GET IMAGE BYTES FROM REQUEST BODY

    let bytes = req.body_bytes().await?;

    let mut img = match image::load_from_memory(&bytes) {
        Ok(img) => img,
        Err(ImageError::Unsupported(err)) => {
            let mut response = Response::new(StatusCode::UnsupportedMediaType);
            response.set_error(err);
            return Ok(response);
        }
        Err(err) => {
            let mut response = Response::new(StatusCode::InternalServerError);
            response.set_error(err);
            return Ok(response);
        }
    };

    // GET THE SMALLER DIMENSION OF THE IMAGE
    // CHECK IF IT'S BIGGER THAN THE MAXIMUM ALLOWED
    // IF IT IS, RESIZE THE IMAGE TO THE MAXIMUM ALLOWED
    // ELSE, AND MAKE BOTH HEIGHT AND WIDTH THAT SIZE

    let smallest_dimention = img.height().min(img.width());
    let new_size = if smallest_dimention > CONFIG.picture_max_dimention {
        CONFIG.picture_max_dimention
    } else {
        smallest_dimention
    };

    // RESIZE THE IMAGE

    img = img.resize_exact(new_size, new_size, image::imageops::FilterType::Nearest);

    // CONVERT IMAGE TO RGB

    let rgb_img = img.to_rgb8();

    // GET IMAGE SIZE IN MEGABYTES

    let img_size = rgb_img.len() as u64 / 1024 / 1024;

    // ENCODE THE IMAGE

    let webp_bytes = async_std::task::spawn_blocking(move || {
        let webp_encoder =
            webp::Encoder::new(&rgb_img, webp::PixelLayout::Rgb, img.width(), img.height());

        // IF THE IMAGE SIZE IS BIGGER THAN THE MAXIMUM ALLOWED
        // ENCODE THE IMAGE WITH LOSSY COMPRESSION
        if img_size > CONFIG.picture_max_size_in_megabytes {
            let webp_memory = webp_encoder.encode(CONFIG.picture_compression);
            return webp_memory.to_vec();
        }

        let webp_memory = webp_encoder.encode_lossless();
        webp_memory.to_vec()
    })
    .await;

    // PUT IMAGE IN S3 BUCKET

    let picture_name = f!("{}.webp", account_id);

    match put_webp_picture_in_bucket(webp_bytes, &picture_name).await {
        Ok(_) => (),
        Err(err) => {
            let mut response = Response::new(StatusCode::InternalServerError);
            response.set_error(err);
            return Ok(response);
        }
    }

    // OK RESPONSE

    Ok(Response::new(StatusCode::Ok))
}
