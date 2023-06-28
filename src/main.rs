use crate::{config::Config, route::create_router};
use axum::ServiceExt;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tracing::log::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod config;
mod handler;
mod jwt_auth;
mod layers;
mod logbody;
mod methods;
mod model;
mod response;
mod route;
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}
#[tokio::main]
async fn main() {
    //make a directory ./uploads if it does not exist
    match std::fs::read_dir("./uploads") {
        Ok(_) => (),
        Err(_) => {
            std::fs::create_dir("./uploads").unwrap();
        }
    }

    // set up logging
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "img_sharing=debug,tower_http=debug,axum::rejection=debug".into()
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

    // let cors = CorsLayer::new()
    //     .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_credentials(true)
    //     .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let app = create_router(Arc::new(AppState {
        db: pool,
        env: config,
    }));

    info!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C received, exiting");
        },
        _ = terminate => {
            println!("SIGTERM received, exiting");
        },
    }

    println!("signal received, starting graceful shutdown");
}
