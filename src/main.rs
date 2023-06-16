use axum::{response::IntoResponse, routing::get, Json, Router};
mod config;
pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthchecker", get(health_checker_handler));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
