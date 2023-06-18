use std::sync::Arc;

use crate::config::Config;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::{
    extract::MatchedPath,
    http::{ Request},
    response::{ Response},
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_http::{ cors::CorsLayer, trace::TraceLayer};
use tracing::{
    info_span,
    log::{debug, info},
    Span,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handler;
mod jwt_auth;
mod model;
mod response;
mod route;
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}
#[tokio::main]
async fn main()  {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "img_sharing=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("🐘 Connected to the database");
            pool
        }
        Err(err) => {
            info!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = route::create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {
                // You can use `_span.record("some_other_field", value)` in one of these
                // closures to attach a value to the initially empty field in the info_span
                // created above.
                // what should be done when a request is received
                info!("Request  body:- ");
                info!("{:?}", _request.body());
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                //
                info!("response received : {:?}", _response.status());
            })
    )
    .layer(cors);

    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}
