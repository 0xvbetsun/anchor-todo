use crate::domain::entities::Status;
use crate::domain::entities::TodoList;
use anchor_client::solana_client::rpc_filter::Memcmp;
use anchor_client::solana_client::rpc_filter::RpcFilterType;
use anchor_lang::prelude::Pubkey;
use axum::async_trait;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::borrow::Borrow;
use std::sync::atomic::Ordering;

use super::repository::{InMemoryRepository, SolanaRepository};
use todo::accounts as todo_acc;
use todo::constants as todo_const;
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
    async fn all(&self, user: &str) -> Result<Vec<TodoList>, ListRepoError>;
    async fn find(&self, user: &str, id: u8) -> Result<TodoList, ListRepoError>;
    async fn update(
        &self,
        user: &str,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError>;
    async fn remove(&self, user: &str, id: u8) -> Result<(), ListRepoError>;
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

    async fn all(&self, user: &str) -> Result<Vec<TodoList>, ListRepoError> {
        Ok(self.lists.read().unwrap().to_vec())
    }

    async fn find(&self, user: &str, id: u8) -> Result<TodoList, ListRepoError> {
        let lists = self.lists.read().expect("mutex poisoned");

        if let Some(idx) = lists.iter().position(|x| x.id == id) {
            return Ok(lists[idx].clone());
        }
        return Err(ListRepoError::NotFound);
    }

    async fn update(
        &self,
        user: &str,
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

    async fn remove(&self, user: &str, id: u8) -> Result<(), ListRepoError> {
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

    async fn all(&self, user: &str) -> Result<Vec<TodoList>, ListRepoError> {
        let pk = Pubkey::from_str(user).unwrap();
        let filters = vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            8,
            &pk.to_bytes(),
        ))];
        let lists: Vec<(_, todo_st::ListAccount)> = self
            .program
            .accounts::<todo_st::ListAccount>(filters)
            .unwrap();

        return Ok(lists
            .into_iter()
            .map(|(_, list)| TodoList {
                id: list.id,
                title: list.title,
                description: list.description,
                status: Status::Active,
            })
            .collect());

        // println!("{lists:#?}");

        // TODO: should be moved to auth repo!
        // let acc = self.rpc_client.get_account(&pk).unwrap();
        // let mut data = acc.data();
        // let user = todo_st::UserProfile::try_deserialize(&mut data).unwrap();

        // println!("{user:?}");
    }
    async fn find(&self, user: &str, id: u8) -> Result<TodoList, ListRepoError> {
        let pk = Pubkey::from_str(user).unwrap();
        let (list_pda, _) = Pubkey::find_program_address(
            &[todo_const::LIST_TAG, &pk.to_bytes(), &[id]],
            &self.program.id(),
        );
        let list: todo_st::ListAccount = self
            .program
            .account::<todo_st::ListAccount>(list_pda)
            .unwrap();

        Ok(TodoList {
            id: list.id,
            title: list.title,
            description: list.description,
            status: Status::Active,
        })
    }

    async fn update(
        &self,
        user: &str,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, ListRepoError> {
        let pk = Pubkey::from_str(user).unwrap();
        let (list_pda, _) = Pubkey::find_program_address(
            &[todo_const::LIST_TAG, &pk.to_bytes(), &[id]],
            &self.program.id(),
        );
        let list_ix = self
            .program
            .request()
            .accounts(todo_acc::UpdateList {
                user_profile: pk,
                list_account: list_pda,
                authority: self.payer.pubkey(),
                system_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            })
            .args(todo_ix::UpdateList {
                id,
                title: title.to_owned(),
                description: description.to_owned(),
            })
            .instructions()
            .unwrap();

        let tx = Transaction::new_signed_with_payer(
            &list_ix,
            Some(&self.payer.pubkey()),
            &[&*self.payer],
            self.rpc_client.get_latest_blockhash().unwrap(),
        );

        self.rpc_client.send_transaction(&tx).unwrap();

        Ok(TodoList {
            id,
            title,
            description,
            status: Status::Active,
        })
    }

    async fn remove(&self, user: &str, id: u8) -> Result<(), ListRepoError> {
        unimplemented!()
    }
}
