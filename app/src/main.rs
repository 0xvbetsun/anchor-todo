mod api;
mod configuration;
mod domain;
mod repository;

use crate::configuration::{get_config, Storage};
use crate::repository::in_memory::InMemoryRepository;
use crate::repository::solana::SolanaRepository;
use crate::repository::DynRepository;

use axum::Router;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let cfg = get_config().expect("Failed to read configuration.");

    let repo: DynRepository = match cfg.storage {
        Storage::InMemory => Arc::new(InMemoryRepository::new()),
        Storage::Solana => Arc::new(SolanaRepository::try_new(cfg.keypair_file).unwrap()),
    };

    let routes_apis = Router::new()
        .merge(api::auth::routes(repo.clone()))
        .merge(api::list::routes(repo.clone()));

    let routes = Router::new()
        .merge(api::health::routes())
        .nest("/api", routes_apis);
    // .route("/api/lists/:list_id/todos", get())
    // .route("/api/lists/:list_id/todos/:id", get())

    // program.
    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    println!("stopped listening");
}
