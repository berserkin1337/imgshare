use crate::{model::User, response::FilteredUser};

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub mod login;
pub mod logout;
pub mod register;
pub mod upload;
fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        username: user.username.to_owned(),
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    Ok(Json(json_response))
}
pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
    });

    Json(json_response)
}
