use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
    images: Vec<Link>,
}

struct Link {
    name: String,
    url: String,
    title: String,
}

pub async fn dashboard() -> impl IntoResponse {
    todo!()
}
