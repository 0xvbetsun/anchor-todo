use axum::Json;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
}

pub async fn check() -> Json<HealthResponse>  {
    let resp = HealthResponse{ ok: true};
    Json(resp)
}