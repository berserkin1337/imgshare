use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "signin.html")]
struct LoginTemplate {}

pub async fn signin() -> impl IntoResponse {
    let template = LoginTemplate {};
    Html(template.render().unwrap())
}
