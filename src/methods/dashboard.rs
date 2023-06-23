use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
    images: Vec<Link>,
    base: BaseTemplateData,
}

struct Link {
    name: String,
    url: String,
    title: String,
}

pub async fn dashboard() -> impl IntoResponse {
    todo!()
}
