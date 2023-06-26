use askama::Template;

use axum::response::Html;
use axum::response::IntoResponse;
use axum::Extension;

use super::base::BaseTemplateData;

#[derive(Template)]
#[template(path = "signin.html")]
struct LoginTemplate {
    base: BaseTemplateData,
}

#[derive(Template)]
#[template(path = "alreadysignedin.html")]
struct AlreadySignedIn {
    base: BaseTemplateData,
}

pub async fn signin(Extension(is_signed_in): Extension<bool>) -> impl IntoResponse {
    if is_signed_in {
        let template = AlreadySignedIn {
            base: BaseTemplateData::new(true),
        };
        Html(template.render().unwrap())
    } else {
        let template = LoginTemplate {
            base: BaseTemplateData::new(false),
        };
        Html(template.render().unwrap())
    }
}
