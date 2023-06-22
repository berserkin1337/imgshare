use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "register.html")]
struct LTemplate {}

pub async fn register() -> impl IntoResponse {
    let template = LTemplate {};
    Html(template.render().unwrap())
}
