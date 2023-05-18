use crate::domain::entities::TodoItem;
use axum::async_trait;
use std::sync::atomic::Ordering;

use super::repository::InMemoryRepository;

#[derive(Debug)]
pub enum TodoRepoError {
    Unknown,
    #[allow(dead_code)]
    NotFound,
    InvalidTitle,
}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, title: String, description: String) -> Result<TodoItem, TodoRepoError>;
}

#[async_trait]
impl TodoRepository for InMemoryRepository {
    async fn create(&self, title: String, description: String) -> Result<TodoItem, TodoRepoError> {
        let mut lock = match self.items.write() {
            Ok(lock) => lock,
            _ => return Err(TodoRepoError::Unknown),
        };

        if lock.iter().any(|item| item.title == title) {
            return Err(TodoRepoError::InvalidTitle);
        }
        let id = self.last_item_id.fetch_add(1, Ordering::Relaxed);
        let item = TodoItem::new(id, title, description);
        lock.push(item.clone());
        Ok(item)
    }
}
