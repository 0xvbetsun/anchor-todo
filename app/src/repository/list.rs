use crate::domain::entities::TodoList;
use anchor_lang::prelude::Pubkey;
use anchor_lang::AccountDeserialize;
use axum::async_trait;
use solana_sdk::account::ReadableAccount;
use std::str::FromStr;
use std::sync::atomic::Ordering;

use super::repository::{InMemoryRepository, SolanaRepository};
use solana_sdk::signature::Signer;
use todo::accounts as todo_acc;
use todo::instruction as todo_ix;
use todo::state as todo_st;

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
    async fn find(&self, id: u8) -> Result<TodoList, ListRepoError>;
    async fn update(
        &self,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError>;
    async fn remove(&self, id: u8) -> Result<(), ListRepoError>;
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

    async fn find(&self, id: u8) -> Result<TodoList, ListRepoError> {
        let lists = self.lists.read().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|x| x.id == id) {
            return Ok(lists[idx].clone());
        }
        return Err(ListRepoError::NotFound);
    }

    async fn update(
        &self,
        id: u8,
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

    async fn remove(&self, id: u8) -> Result<(), ListRepoError> {
        let mut lists = self.lists.write().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|t| t.id == id) {
            lists.remove(idx);
            return Ok(());
        }

        return Err(ListRepoError::NotFound);
    }
}

#[async_trait]
impl ListRepository for SolanaRepository {
    async fn create(&self, title: String, description: String) -> Result<TodoList, ListRepoError> {
        unimplemented!()
    }
    async fn all(&self) -> Vec<TodoList> {
        let pk = Pubkey::from_str("9nBk7GBTf9h9Ut57NX3qyzhsCERz1myBBbWBMNU9gkN2").unwrap();
        let acc = self.rpc_client.get_account(&pk).unwrap();
        let mut data = acc.data();
        let user = todo_st::UserProfile::try_deserialize(&mut data).unwrap();

        println!("{user:?}");
        unimplemented!()
    }
    async fn find(&self, id: u8) -> Result<TodoList, ListRepoError> {
        unimplemented!()
    }
    async fn update(
        &self,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError> {
        unimplemented!()
    }

    async fn remove(&self, id: u8) -> Result<(), ListRepoError> {
        unimplemented!()
    }
}
