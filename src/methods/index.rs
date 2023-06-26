use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Extension;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    base: BaseTemplateData,
}

pub async fn index(Extension(is_logged_in): Extension<bool>) -> impl IntoResponse {
    let template = IndexTemplate {
        base: BaseTemplateData::new(is_logged_in),
    };
    Html(template.render().unwrap())
}
