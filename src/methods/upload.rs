use askama::Template;

use axum::response::Html;
use axum::response::IntoResponse;
use axum::Extension;

use crate::model::User;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
    base: BaseTemplateData,
}

pub async fn upload(Extension(_): Extension<User>) -> impl IntoResponse {
    let template = UploadTemplate {
        base: BaseTemplateData { is_logged_in: true },
    };
    Html(template.render().unwrap())
}
