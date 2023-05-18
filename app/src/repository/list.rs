use std::sync::atomic::Ordering;

use axum::async_trait;

use crate::domain::entities::TodoList;

use super::repository::InMemoryRepository;

#[derive(Debug)]
pub enum ListRepoError {
    Unknown,
    #[allow(dead_code)]
    NotFound,
    InvalidTitle,
}

#[async_trait]
pub trait ListRepository: Send + Sync {
    async fn create(&self, title: String, description: String) -> Result<TodoList, ListRepoError>;
    async fn all(&self) -> Vec<TodoList>;
    async fn find(&self, id: u16) -> Result<TodoList, ListRepoError>;
    async fn update(
        &self,
        id: u16,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError>;
    async fn remove(&self, id: u16) -> Result<(), ListRepoError>;
}

#[async_trait]
impl ListRepository for InMemoryRepository {
    async fn create(&self, title: String, description: String) -> Result<TodoList, ListRepoError> {
        let mut lock = match self.lists.write() {
            Ok(lock) => lock,
            _ => return Err(ListRepoError::Unknown),
        };

        if lock.iter().any(|list| list.title == title) {
            return Err(ListRepoError::InvalidTitle);
        }
        let id = self.last_list_id.fetch_add(1, Ordering::Relaxed);
        let list = TodoList::new(id, title, description);
        lock.push(list.clone());
        Ok(list)
    }

    async fn all(&self) -> Vec<TodoList> {
        self.lists.read().unwrap().to_vec()
    }

    async fn find(&self, id: u16) -> Result<TodoList, ListRepoError> {
        let lists = self.lists.read().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|x| x.id == id) {
            return Ok(lists[idx].clone());
        }
        return Err(ListRepoError::NotFound);
    }

    async fn update(
        &self,
        id: u16,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError> {
        let mut lists = self.lists.write().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|t| t.id == id) {
            lists[idx].title = title;
            lists[idx].description = description;
            return Ok(lists[idx].clone());
        }

        return Err(ListRepoError::NotFound);
    }

    async fn remove(&self, id: u16) -> Result<(), ListRepoError> {
        let mut lists = self.lists.write().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|t| t.id == id) {
            lists.remove(idx);
            return Ok(());
        }

        return Err(ListRepoError::NotFound);
    }
}
