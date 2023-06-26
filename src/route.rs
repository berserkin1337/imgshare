use axum::{middleware, routing::post, Router};
use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::get};

use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    services::ServeDir,
};
use tower_layer::Layer;

use crate::{
    handler::{
        get_me_handler, health_checker_handler,
        login::login_user_handler,
        logout::logout_handler,
        register::register_handler,
        upload::{self, image_upload_handler},
    },
    jwt_auth::{auth, check_login},
    layers::log_request_response,
    methods::{
        dashboard::dashboard, index::index, register::register, signin::signin, upload::upload,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> NormalizePath<Router> {
    // println!("Inside  create_router ");

    let router = Router::new()
        .route(
            "/",
            get(index).route_layer(from_fn_with_state(app_state.clone(), check_login)),
        )
        .route(
            "/login",
            get(signin).route_layer(from_fn_with_state(app_state.clone(), check_login)),
        )
        .route(
            "/register",
            get(register).route_layer(from_fn_with_state(app_state.clone(), check_login)),
        )
        .route(
            "/dashboard",
            get(dashboard).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/upload",
            get(upload).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .route(
            "/api/auth/login",
            post(login_user_handler)
                .route_layer(from_fn_with_state(app_state.clone(), check_login)),
        )
        .route(
            "/api/auth/logout",
            get(logout_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/users/me",
            get(get_me_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/uploadimage",
            post(image_upload_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/uploads/", ServeDir::new("uploads"))
        .with_state(app_state)
        .layer(middleware::from_fn(log_request_response));
    NormalizePathLayer::trim_trailing_slash().layer(router)
}
