use crate::domain::entities::Status;
use crate::domain::entities::TodoList;
use anchor_client::solana_client::rpc_filter::Memcmp;
use anchor_client::solana_client::rpc_filter::MemcmpEncodedBytes;
use anchor_client::solana_client::rpc_filter::MemcmpEncoding;
use anchor_client::solana_client::rpc_filter::RpcFilterType;
use anchor_lang::prelude::Pubkey;
use axum::async_trait;
use std::str::FromStr;
use std::sync::atomic::Ordering;

use super::repository::{InMemoryRepository, SolanaRepository};
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
        let pk = Pubkey::from_str("AY2gNTezoQyY3kYUCwim8ZiaXXsGgoMZaDyXUUXbRuXJ").unwrap();
        let filters = vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            8,
            &pk.to_bytes(),
        ))];
        let lists: Vec<(_, todo_st::ListAccount)> = self
            .program
            .accounts::<todo_st::ListAccount>(filters)
            .unwrap();

        return lists
            .into_iter()
            .map(|(_, list)| TodoList {
                id: 1,
                title: list.title,
                description: list.description,
                status: Status::Active,
            })
            .collect();

        // println!("{lists:#?}");

        // TODO: should be moved to auth repo!
        // let acc = self.rpc_client.get_account(&pk).unwrap();
        // let mut data = acc.data();
        // let user = todo_st::UserProfile::try_deserialize(&mut data).unwrap();

        // println!("{user:?}");
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
