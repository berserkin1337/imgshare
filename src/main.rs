use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;
mod config;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}
#[tokio::main]
async fn main() {
    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState {
            db: pool,
            env: config,
        }));
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
    });

    Json(json_response)
}
