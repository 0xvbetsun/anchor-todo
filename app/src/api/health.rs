use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
}

pub fn routes() -> Router {
    Router::new().route("/health", get(check))
}

pub async fn check() -> Json<HealthResponse> {
    let resp = HealthResponse { ok: true };
    Json(resp)
}
