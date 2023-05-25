use anchor_client::{Client, Cluster, Program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};
use std::sync::{atomic::AtomicU8, Arc, RwLock};

use crate::domain::entities::{TodoItem, TodoList};

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

pub struct SolanaRepository {
    pub payer: Arc<Keypair>,
    pub program: Program<Arc<Keypair>>,
}

impl SolanaRepository {
    pub fn try_new<'a>() -> Result<Self, &'a str> {
        let cluster = Cluster::Localnet;
        let payer = match read_keypair_file("~/.config/solana/id.json") {
            Ok(kp) => kp,
            Err(_) => return Err("requires a keypair file"),
        };
        let payer = Arc::new(payer);
        let provider = Client::new_with_options(
            cluster.clone(),
            payer.clone(),
            CommitmentConfig::confirmed(),
        );

        let program = provider.program(todo::ID);
        Ok(Self { program, payer })
    }
}
