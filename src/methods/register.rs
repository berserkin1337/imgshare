use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Extension;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    base: BaseTemplateData,
}

pub async fn register(Extension(is_logged_in): Extension<bool>) -> impl IntoResponse {
    let template = RegisterTemplate {
        base: BaseTemplateData::new(is_logged_in),
    };

    Html(template.render().unwrap())
}
