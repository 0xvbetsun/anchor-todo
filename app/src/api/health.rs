use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
}

#[get("/health")]
pub fn check() -> Json<HealthResponse>  {
    let resp = HealthResponse{ ok: true};
    Json(resp)
}