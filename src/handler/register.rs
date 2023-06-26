use axum::extract::State;

use std::sync::Arc;

use crate::{
    handler::filter_user_record,
    model::{RegisterUserSchema, User},
    AppState,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{http::StatusCode, response::IntoResponse, Form, Json};

use rand_core::OsRng;
use serde_json::json;
use sqlx::{query_as, query_scalar};
pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Form(body): Form<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //Create  a generic response
    let user_exists = query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        body.email
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        println!("Error when fetching email from db: {}", e);
        let error_response = json!({
            "status" : "fail",
            "message" : "Internal Error",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    if let Some(exists) = user_exists {
        if exists {
            let error_response = json!({
                "status" : "fail",
                "message" : "User already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            println!("Error when hashing password: {}", e);
            let error_response = json!({
                "status" : "fail",
                "message" : "Internal Error",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;
    let user = query_as!(
        User,
        "Insert into users (username,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.username,
        body.email,
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let user_response = serde_json::json!({"status": "success","data": json!({
        "user": filter_user_record(&user)
    })});
    Ok(Json(user_response))
}
