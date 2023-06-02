mod api;
mod domain;
mod repository;
mod configuration;

use crate::api::list::DynListRepository;
use crate::repository::repository::SolanaRepository;
use crate::configuration::get_config;

use axum::Router;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let cfg = get_config().expect("Failed to read configuration.");
    // let repo = Arc::new(InMemoryRepository::new()) as DynListRepository;
    let sol_repo = Arc::new(SolanaRepository::try_new().unwrap()) as DynListRepository;

    let routes_apis = Router::new().merge(api::list::routes(sol_repo.clone()));

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
