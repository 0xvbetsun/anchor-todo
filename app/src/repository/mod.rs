use std::sync::Arc;

use axum::async_trait;

use crate::domain::entities::{User, TodoList};

pub mod in_memory;
pub mod solana;


pub type DynRepository = Arc<dyn Repository + Send + Sync>;

#[derive(Debug)]
pub enum RepoError {
    Unknown,
    #[allow(dead_code)]
    NotFound,
    InvalidTitle,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_user(
        &self,
        name: &str,
        username: &str,
        password: &str,
    ) -> Result<User, RepoError>;
    async fn find_user(&self, username: &str, password: &str) -> Result<User, RepoError>;

    async fn create_list(
        &self,
        user: &str,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError>;
    async fn all_lists(&self, user: &str) -> Result<Vec<TodoList>, RepoError>;
    async fn find_list(&self, user: &str, id: u8) -> Result<TodoList, RepoError>;
    async fn update_list(
        &self,
        user: &str,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError>;
    async fn remove_list(&self, user: &str, id: u8) -> Result<(), RepoError>;
}
