use axum::http::StatusCode;
use axum::{Router, extract::Json, response::IntoResponse, routing::get};

use crate::middleware::real_ip::RealIp;

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "OK")
    ),
)]
pub async fn health_check(RealIp(client_ip): RealIp) -> impl IntoResponse {
    println!("received ping from {}", client_ip);
    let response = serde_json::json!({
        "message": "API is healthy",
        "client_ip": client_ip,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
    });
    (StatusCode::OK, Json(response))
}
