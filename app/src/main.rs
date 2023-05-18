mod api;
mod domain;
mod repository;

use crate::{api::list::DynListRepository, repository::repository::InMemoryRepository};

use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(api::health::check))
        .route("/api/lists", get(api::list::all).post(api::list::create))
        .route(
            "/api/lists/:id",
            get(api::list::find)
                .patch(api::list::update)
                .delete(api::list::remove),
        )
        .with_state(Arc::new(InMemoryRepository::new()) as DynListRepository);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("stopped listening");
}
