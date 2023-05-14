use crate::domain::entities::{TodoItem, TodoList};
use anchor_client::solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Mutex,
};

pub enum InsertError {
    Conflict,
    Unknown,
}

pub trait Repository {
    fn create_list(&self, title: String, description: String) -> Result<TodoList, InsertError>;
    fn create_item(&self, title: String, description: String) -> Result<TodoItem, InsertError>;
}

pub struct InMemoryRepository {
    last_list_id: AtomicU16,
    last_item_id: AtomicU16,
    lists: Mutex<Vec<TodoList>>,
    items: Mutex<Vec<TodoItem>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let last_list_id = AtomicU16::new(0);
        let last_item_id = AtomicU16::new(0);
        let lists: Mutex<Vec<TodoList>> = Mutex::new(vec![]);
        let items: Mutex<Vec<TodoItem>> = Mutex::new(vec![]);

        Self {
            last_list_id,
            last_item_id,
            lists,
            items,
        }
    }
}

impl Repository for InMemoryRepository {
    fn create_list(&self, title: String, description: String) -> Result<TodoList, InsertError> {
        let mut lock = match self.lists.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        if lock.iter().any(|list| list.title == title) {
            return Err(InsertError::Conflict);
        }
        let id = self.last_list_id.fetch_add(1, Ordering::Relaxed);
        let list = TodoList::new(id, title, description);
        lock.push(list.clone());
        Ok(list)
    }

    fn create_item(&self, title: String, description: String) -> Result<TodoItem, InsertError> {
        let mut lock = match self.items.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        if lock.iter().any(|item| item.title == title) {
            return Err(InsertError::Conflict);
        }
        let id = self.last_item_id.fetch_add(1, Ordering::Relaxed);
        let item = TodoItem::new(id, title, description);
        lock.push(item.clone());
        Ok(item)
    }
}

pub struct SolanaRepository {
    program_id: Pubkey,
}

impl SolanaRepository {
    pub fn try_new() -> Result<Self, ()> {
        match Pubkey::from_str("FsgyMvD4vw6xSMNkFD14gbgRK5kadrZYzF1xGAcj2WfR") {
            Ok(program_id) => Ok(Self { program_id }),
            Err(_) => return Err(()),
        }
    }
}
