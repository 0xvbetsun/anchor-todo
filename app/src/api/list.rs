use std::sync::Mutex;

use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::repository::todo::Repository;

#[derive(Clone, Debug, Serialize)]
pub struct ListResponse {
    pub id: u16,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListRequest {
    pub title: String,
    pub description: String,
}

#[post("/lists", format = "json", data = "<req>")]
pub fn create(
    repo: State<Mutex<Box<dyn Repository>>>,
    req: Json<ListRequest>,
) -> Created<Json<ListResponse>> {
    let list = req.into_inner();

    let resp = ListResponse {
        id: 123,
        title: list.title,
        description: list.description,
    };
    Created(
        format!("http://0.0.0.0:8000/api/lists/{}", resp.id),
        Some(Json(resp)),
    )
}
