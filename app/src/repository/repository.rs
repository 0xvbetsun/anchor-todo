use std::{sync::{atomic::AtomicU16, RwLock}, str::FromStr};

use anchor_client::solana_sdk::pubkey::Pubkey;

use crate::domain::entities::{TodoItem, TodoList};

pub struct InMemoryRepository {
    pub last_list_id: AtomicU16,
    pub last_item_id: AtomicU16,
    pub lists: RwLock<Vec<TodoList>>,
    pub items: RwLock<Vec<TodoItem>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let last_list_id = AtomicU16::new(1);
        let last_item_id = AtomicU16::new(1);
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
