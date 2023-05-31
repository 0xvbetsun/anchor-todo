use crate::domain::entities::{TodoItem, TodoList};
use anchor_client::{solana_client::rpc_client::RpcClient, Client, Cluster, Program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
};
use std::{
    ops::Deref,
    rc::Rc,
    sync::{atomic::AtomicU8, Arc, RwLock},
};

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
    pub rpc_client: RpcClient,
}

impl SolanaRepository {
    pub fn try_new<'a>() -> Result<Self, &'a str> {
        let cluster = Cluster::Localnet;
        let payer = match read_keypair_file("/Users/vbetsun/.config/solana/id.json") {
            Ok(kp) => kp,
            Err(_) => return Err("requires a keypair file"),
        };

        let rpc_client =
            RpcClient::new_with_commitment(cluster.url(), CommitmentConfig::confirmed());

        let payer = Arc::new(payer);
        let provider =
            Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
        let program = provider.program(todo::ID);

        Ok(Self {
            program,
            payer,
            rpc_client,
        })
    }
}
