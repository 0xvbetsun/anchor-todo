use axum::{routing::post, Json, Router};
use serde::Serialize;

use crate::repository::DynRepository;

#[derive(Clone, Debug, Serialize)]
pub struct UserResponse {
    pub id: u16,
    pub name: String,
    pub username: String,
}

pub fn routes(repo: DynRepository) -> Router {
    Router::new()
        .route("sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .with_state(repo)
}

pub async fn sign_up() -> Json<UserResponse> {
    let resp = UserResponse { id: 1, name: "John".to_owned(), username: "test".to_owned() };
    Json(resp)
}

pub async fn sign_in() -> Json<UserResponse> {
    let resp = UserResponse { id: 1, name: "John".to_owned(), username: "test".to_owned() };
    Json(resp)
}
