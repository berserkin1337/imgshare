use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use tracing::info;

use crate::{
    model::{TokenClaims, User},
    AppState,
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    // if auth_value.starts_with("Bearer ") {
                    //     Some(auth_value[7..].to_owned())
                    auth_value
                        .strip_prefix("Bearer ")
                        .map(|auth_val| auth_val.to_owned())
                })
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;
    let user = user.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

pub async fn check_login<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    // if auth_value.starts_with("Bearer ") {
                    //     Some(auth_value[7..].to_owned())
                    auth_value
                        .strip_prefix("Bearer ")
                        .map(|auth_val| auth_val.to_owned())
                })
        });

    if token.is_none() {
        info!("Not logged in");
        req.extensions_mut().insert(false);
        return next.run(req).await;
    }

    let token = token.unwrap();
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    );

    if claims.is_err() {
        req.extensions_mut().insert(false);
        return next.run(req).await;
    }
    let claims = claims.unwrap().claims;
    let user_id = uuid::Uuid::parse_str(&claims.sub);
    if user_id.is_err() {
        req.extensions_mut().insert(false);
        return next.run(req).await;
    }
    let user_id = user_id.unwrap();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&data.db)
        .await;
    if user.is_err() {
        req.extensions_mut().insert(false);
        return next.run(req).await;
    }
    let user = user.unwrap();
    if user.is_none() {
        req.extensions_mut().insert(false);
        return next.run(req).await;
    }
    // let user = user.unwrap();
    req.extensions_mut().insert(true);
    dbg!("user is logged in ");
    next.run(req).await
}
