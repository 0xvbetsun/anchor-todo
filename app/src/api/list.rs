use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    domain::entities::TodoList,
    repository::{DynRepository, RepoError},
};

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

const USER_KEY: &str = "EtNhzsHYkYHWX4dWia7snqubw4wLYcQrRuzHYGDof9nP";

pub fn routes(repo: DynRepository) -> Router {
    Router::new()
        .route("/lists", get(all).post(create))
        .route("/lists/:id", get(find).patch(update).delete(remove))
        .with_state(repo)
}

pub async fn create(
    State(repo): State<DynRepository>,
    Json(req): Json<ListRequest>,
) -> Result<(StatusCode, Json<TodoList>), AppError> {
    let list = repo
        .create_list(USER_KEY, req.title, req.description)
        .await?;

    Ok((StatusCode::CREATED, list.into()))
}

pub async fn all(State(repo): State<DynRepository>) -> Json<Vec<TodoList>> {
    // TODO: extract key from header
    let lists: Vec<TodoList> = repo.all_lists(USER_KEY).await.unwrap();

    lists.into()
}

pub async fn find(
    Path(id): Path<u8>,
    State(repo): State<DynRepository>,
) -> Result<Json<TodoList>, AppError> {
    let list: TodoList = repo.find_list(USER_KEY, id).await?;

    Ok(list.into())
}

pub async fn update(
    Path(id): Path<u8>,
    State(repo): State<DynRepository>,
    Json(req): Json<ListRequest>,
) -> Result<Json<TodoList>, AppError> {
    let list: TodoList = repo
        .update_list(USER_KEY, id, req.title, req.description)
        .await?;

    Ok(list.into())
}

pub async fn remove(
    Path(id): Path<u8>,
    State(repo): State<DynRepository>,
) -> Result<impl IntoResponse, AppError> {
    repo.remove_list(USER_KEY, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub enum AppError {
    Repo(RepoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Repo(RepoError::Unknown) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Repo(RepoError::NotFound) => (StatusCode::NOT_FOUND, "List not found"),
            AppError::Repo(RepoError::InvalidTitle) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid title")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<RepoError> for AppError {
    fn from(inner: RepoError) -> Self {
        AppError::Repo(inner)
    }
}
