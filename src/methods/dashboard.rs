use std::sync::Arc;

use crate::{
    model::{Image, User},
    AppState,
};
use askama::Template;
use axum::{extract::State, response::Html, response::IntoResponse, Extension};
use http::StatusCode;
use tracing::error;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
    images: Vec<Link>,
    base: BaseTemplateData,
}

struct Link {
    url: String,
}

pub async fn dashboard(
    State(data): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, StatusCode> {
    // find the images uploaded by the user
    let images = sqlx::query_as!(Image, "SELECT * FROM images WHERE user_id = $1", user.id)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            error!(
                "Failed to fetch images for user {:?} with error {:?}",
                user.id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let template = DashboardTemplate {
        username: user.username,
        images: images
            .into_iter()
            .map(|image| Link {
                url: format!("/uploads/{}.webp", image.id),
            })
            .collect(),
        base: BaseTemplateData { is_logged_in: true },
    };
    Ok(Html(template.render().unwrap()))
}
