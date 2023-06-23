use std::sync::Arc;

use axum::{
    extract::MatchedPath,
    http::{Request, Response},
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tower_layer::Layer;
use tracing::{info, info_span, Span};

use crate::{
    handler::{
        get_me_handler, health_checker_handler, image_upload_handler, login_user_handler,
        logout_handler, register_user_handler,
    },
    jwt_auth::{auth, check_login},
    methods::{index::index, register::register, signin::signin},
    AppState,
};
async fn root() -> &'static str {
    "Hello, World!"
}

pub fn create_router() -> NormalizePath<Router> {
    // println!("Inside  create_router ");

    let router = Router::new().route("/", get(root));
    // .route(
    //     "/login",
    //     get(signin).route_layer(from_fn_with_state(app_state.clone(), check_login)),
    // )
    // .route("/register", get(register))
    // .route(
    //     "/dashboard",
    //     get(index).route_layer(from_fn_with_state(app_state.clone(), auth)),
    // )
    // .route("/api/healthchecker", get(health_checker_handler))
    // .route("/api/auth/register", post(register_user_handler))
    // .route(
    //     "/api/auth/login",
    //     post(login_user_handler)
    //         .route_layer(from_fn_with_state(app_state.clone(), check_login)),
    // )
    // .route(
    //     "/api/auth/logout",
    //     get(logout_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
    // )
    // .route(
    //     "/api/users/me",
    //     get(get_me_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
    // )
    // .route(
    //     "/api/uploadimage",
    //     post(image_upload_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
    // )
    // .nest_service("/assets", ServeDir::new("assets"))
    // .with_state(app_state)
    // .layer(
    //     TraceLayer::new_for_http()
    //         .make_span_with(|request: &Request<_>| {
    //             let matched_path = request
    //                 .extensions()
    //                 .get::<MatchedPath>()
    //                 .map(MatchedPath::as_str);

    //             info_span!(
    //                 "http_request",
    //                 method = ?request.method(),
    //                 matched_path,
    //                 some_other_field = tracing::field::Empty,
    //             )
    //         })
    //         .on_request(|_request: &Request<_>, _span: &Span| {
    //             info!("request received at path : {:?}", _request.uri().path());
    //         })
    //         .on_response(
    //             |_response: &Response<_>, _latency: Duration, _span: &Span| {
    //                 info!("response received : {:?}", _response.status());
    //             },
    //         ),
    // );

    NormalizePathLayer::trim_trailing_slash().layer(router)
}
