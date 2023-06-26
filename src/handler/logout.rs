use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, String)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Redirect::to("/").into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
