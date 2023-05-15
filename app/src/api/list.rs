use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::{response, Request, State};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

extern crate serde_json;
use crate::repository::todo::Repository;

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

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
pub fn create(repo: State<Mutex<Box<dyn Repository>>>, req: Json<ListRequest>) -> ApiResponse {
    let list_req = req.into_inner();

    let list = match repo
        .lock()
        .unwrap()
        .create_list(list_req.title, list_req.description)
    {
        Ok(val) => val,
        Err(err) => {
            return ApiResponse {
                json: json!({ "error": "oops"}),
                status: Status::UnprocessableEntity,
            }
        }
    };
    return ApiResponse {
        json: json!(ListResponse {
            id: list.id,
            title: list.title,
            description: list.description,
        }),
        status: Status::Created,
    }
    
}

#[get("/lists", format = "json")]
pub fn all(repo: State<Mutex<Box<dyn Repository>>>) -> ApiResponse {
    let resp: Vec<ListResponse> = repo
        .lock()
        .unwrap()
        .all_lists()
        .into_iter()
        .map(|list| ListResponse {
            id: list.id,
            title: list.title,
            description: list.description,
        })
        .collect();

    ApiResponse {
        json: json!(resp),
        status: Status::Ok,
    }
}
