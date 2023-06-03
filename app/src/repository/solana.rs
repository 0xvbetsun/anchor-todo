use anchor_client::{Cluster, Program, Client};
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_filter::{RpcFilterType, Memcmp};
use anchor_lang::prelude::{Pubkey};
use solana_sdk::signature::Signer;
use axum::async_trait;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, read_keypair_file};
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::sync::Arc;

use crate::domain::entities::{User, TodoList};

use super::{Repository, RepoError};

use todo::accounts as todo_acc;
use todo::constants as todo_const;
use todo::instruction as todo_ix;
use todo::state as todo_st;


pub struct SolanaRepository {
    pub payer: Arc<Keypair>,
    pub program: Program<Arc<Keypair>>,
    pub rpc_client: RpcClient,
}

impl SolanaRepository {
    pub fn try_new<'a>(path: String) -> Result<Self, &'a str> {
        let cluster = Cluster::Custom(
            "http://localhost:8899".to_owned(),
            "ws://localhost:8900/".to_owned(),
        );
        let payer = match read_keypair_file(path) {
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


#[async_trait]
impl Repository for SolanaRepository {
    async fn create_user(&self, _name: &str, _username: &str, _password: &str) -> Result<User, RepoError> {
        unimplemented!()
    }
    async fn find_user(&self, _username: &str, _password: &str) -> Result<User, RepoError>{
        unimplemented!()
    }
    
    async fn create_list(
        &self,
        user: &str,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError> {
        let pk = Pubkey::from_str(user).unwrap();

        let user: todo_st::UserProfile = self.program.account::<todo_st::UserProfile>(pk).unwrap();

        let (list_pda, _) = Pubkey::find_program_address(
            &[todo_const::LIST_TAG, &pk.to_bytes(), &[user.list_idx]],
            &self.program.id(),
        );

        let list_ix = self
            .program
            .request()
            .accounts(todo_acc::CreateList {
                user_profile: pk,
                list_account: list_pda,
                authority: self.payer.pubkey(),
                system_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            })
            .args(todo_ix::CreateList {
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
            id: user.list_idx,
            title,
            description,
        })
    }

    async fn all_lists(&self, user: &str) -> Result<Vec<TodoList>, RepoError> {
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
            })
            .collect());

        // println!("{lists:#?}");

        // TODO: should be moved to auth repo!
        // let acc = self.rpc_client.get_account(&pk).unwrap();
        // let mut data = acc.data();
        // let user = todo_st::UserProfile::try_deserialize(&mut data).unwrap();

        // println!("{user:?}");
    }
    async fn find_list(&self, user: &str, id: u8) -> Result<TodoList, RepoError> {
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
        })
    }

    async fn update_list(
        &self,
        user: &str,
        id: u8,
        title: String,
        description: String,
    ) -> Result<TodoList, RepoError> {
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
        })
    }

    async fn remove_list(&self, user: &str, id: u8) -> Result<(), RepoError> {
        let pk = Pubkey::from_str(user).unwrap();
        let (list_pda, _) = Pubkey::find_program_address(
            &[todo_const::LIST_TAG, &pk.to_bytes(), &[id]],
            &self.program.id(),
        );
        let list_ix = self
            .program
            .request()
            .accounts(todo_acc::RemoveList {
                user_profile: pk,
                list_account: list_pda,
                authority: self.payer.pubkey(),
                system_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            })
            .args(todo_ix::RemoveList {})
            .instructions()
            .unwrap();

        let tx = Transaction::new_signed_with_payer(
            &list_ix,
            Some(&self.payer.pubkey()),
            &[&*self.payer],
            self.rpc_client.get_latest_blockhash().unwrap(),
        );

        self.rpc_client.send_transaction(&tx).unwrap();

        Ok(())
    }
}
