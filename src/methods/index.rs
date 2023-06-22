use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}
