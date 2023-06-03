use std::sync::{
    atomic::{AtomicU8, Ordering},
    RwLock,
};
use axum::async_trait;

use crate::domain::entities::{TodoItem, TodoList, User};

use super::{RepoError, Repository};

pub struct InMemoryRepository {
    pub last_list_id: AtomicU8,
    pub last_item_id: AtomicU8,
    pub lists: RwLock<Vec<TodoList>>,
    pub items: RwLock<Vec<TodoItem>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let last_list_id = AtomicU8::new(1);
        let last_item_id = AtomicU8::new(1);
        let lists: RwLock<Vec<TodoList>> = RwLock::new(vec![]);
        let items: RwLock<Vec<TodoItem>> = RwLock::new(vec![]);

        Self {
            last_list_id,
            last_item_id,
            lists,
            items,
        }
    }
}

#[async_trait]
impl Repository for InMemoryRepository {
    async fn create_user(
        &self,
        _name: &str,
        _username: &str,
        _password: &str,
    ) -> Result<User, RepoError> {
        unimplemented!()
    }
    async fn find_user(&self, _username: &str, _password: &str) -> Result<User, RepoError> {
        unimplemented!()
    }

    async fn create_list(
        &self,
        _user: &str,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError> {
        let mut lock = match self.lists.write() {
            Ok(lock) => lock,
            _ => return Err(RepoError::Unknown),
        };

        if lock.iter().any(|list| list.title == title) {
            return Err(RepoError::InvalidTitle);
        }
        let id = self.last_list_id.fetch_add(1, Ordering::Relaxed);
        let list = TodoList::new(id, title, description);
        lock.push(list.clone());
        Ok(list)
    }

    async fn all_lists(&self, _user: &str) -> Result<Vec<TodoList>, RepoError> {
        Ok(self.lists.read().unwrap().to_vec())
    }

    async fn find_list(&self, _user: &str, id: u8) -> Result<TodoList, RepoError> {
        let lists = self.lists.read().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|x| x.id == id) {
            return Ok(lists[idx].clone());
        }
        return Err(RepoError::NotFound);
    }

    async fn update_list(
        &self,
        _user: &str,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError> {
        let mut lists = self.lists.write().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|t| t.id == id) {
            lists[idx].title = title;
            lists[idx].description = description;
            return Ok(lists[idx].clone());
        }

        return Err(RepoError::NotFound);
    }

    async fn remove_list(&self, _user: &str, id: u8) -> Result<(), RepoError> {
        let mut lists = self.lists.write().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|t| t.id == id) {
            lists.remove(idx);
            return Ok(());
        }

        return Err(RepoError::NotFound);
    }
}
