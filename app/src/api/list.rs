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
    repository::list::{ListRepoError, ListRepository},
};

#[derive(Clone, Debug, Serialize)]
pub struct ListResponse {
    pub id: u16,
    pub title: String,
    pub description: String,
}

pub type DynListRepository = std::sync::Arc<dyn ListRepository + Send + Sync>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListRequest {
    pub title: String,
    pub description: String,
}

const USER_KEY: &str = "7jX6B2esEYECF9jVe4rEPSaEEhj75JMYeAgRZzWMt885";

pub fn routes(repo: DynListRepository) -> Router {
    Router::new()
        .route("/lists", get(all).post(create))
        .route("/lists/:id", get(find).patch(update).delete(remove))
        .with_state(repo)
}

pub async fn create(
    State(repo): State<DynListRepository>,
    Json(req): Json<ListRequest>,
) -> Result<(StatusCode, Json<TodoList>), AppError> {
    let list = repo.create(req.title, req.description).await?;

    Ok((StatusCode::CREATED, list.into()))
}

pub async fn all(State(repo): State<DynListRepository>) -> Json<Vec<TodoList>> {
    // TODO: extract key from header
    let lists: Vec<TodoList> = repo.all(USER_KEY).await.unwrap();

    lists.into()
}

pub async fn find(
    Path(id): Path<u8>,
    State(repo): State<DynListRepository>,
) -> Result<Json<TodoList>, AppError> {
    let list: TodoList = repo.find(USER_KEY, id).await?;

    Ok(list.into())
}

pub async fn update(
    Path(id): Path<u8>,
    State(repo): State<DynListRepository>,
    Json(req): Json<ListRequest>,
) -> Result<Json<TodoList>, AppError> {
    let list: TodoList = repo.update(id, req.title, req.description).await?;

    Ok(list.into())
}

pub async fn remove(
    Path(id): Path<u8>,
    State(repo): State<DynListRepository>,
) -> Result<impl IntoResponse, AppError> {
    repo.remove(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub enum AppError {
    Repo(ListRepoError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Repo(ListRepoError::Unknown) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Repo(ListRepoError::NotFound) => (StatusCode::NOT_FOUND, "List not found"),
            AppError::Repo(ListRepoError::InvalidTitle) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid title")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<ListRepoError> for AppError {
    fn from(inner: ListRepoError) -> Self {
        AppError::Repo(inner)
    }
}
