use std::{io::Cursor, sync::Arc};

use crate::{
    model::{Image, ImgBody, User},
    AppState,
};

use axum::{
    extract::State,
    http::{StatusCode},
    response::{IntoResponse},
    Extension, Json,
};

use axum_typed_multipart::TypedMultipart;


use serde_json::json;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;
pub async fn image_upload_handler(
    State(data): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    TypedMultipart(ImgBody { img }): TypedMultipart<ImgBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //TODO: Make this work asynchrounously
    // as of now, this is blocking the event loop
    // because we are loading body.image into memory directly
    info!("Inside the image_upload_handler");

    let image = match image::load_from_memory(&img) {
        Ok(image) => image,
        Err(e) => {
            info!("invalid image for user {} with error {e}", user.id);
            let error_response = json!({
                "status" : "fail",
                "message" : "Invalid image",
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };
    let mut cursor = Cursor::new(Vec::new());
    image
        .write_to(&mut cursor, image::ImageOutputFormat::WebP)
        .map_err(|e| {
            info!(
                "Can't convert image to webp for user: {} with error {e}",
                user.id
            );
            let error_response = json!({
                "status" : "fail",
                "message" : "Internal server error",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    let img = sqlx::query_as!(
        Image,
        "INSERT INTO images (user_id) VALUES ($1) RETURNING *",
        user.id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        info!(
            "Can't insert image into database for user: {} with error {e}",
            user.id
        );
        let error_response = json!({
            "status" : "fail",
            "message" : "Internal server error",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let file_name = format!("{}.webp", img.id);
    //write the file to disk
    let file = File::create(format!("uploads/{}", file_name)).await;

    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            info!("Can't create file for user: {} with error:{:?}", user.id, e);
            let _ = sqlx::query!("DELETE FROM images WHERE id = $1", img.id)
                .execute(&data.db)
                .await;

            let error_response = json!({
                "status" : "fail",
                "message" : "Internal server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let vec = cursor.get_ref();

    match file.write_all(vec).await {
        Ok(_) => {}
        Err(_) => {
            info!("Can't write file for user: {}", user.id);
            // Delete the image from database
            let _ = sqlx::query!("DELETE FROM images WHERE id = $1", img.id)
                .execute(&data.db)
                .await;
            let error_response = json!({
                "status" : "fail",
                "message" : "Internal server error",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let json_response = json!({
        "status": "success",
        "data": {
            "image": {
                "id": img.id,
                "url": format!("http://localhost:8000/uploads/{}", file_name),
            }
        }
    });
    Ok(Json(json_response))
}
