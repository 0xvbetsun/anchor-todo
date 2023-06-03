mod api;
mod configuration;
mod domain;
mod repository;

use crate::configuration::{get_config, Storage};
use crate::repository::in_memory::InMemoryRepository;
use crate::repository::solana::SolanaRepository;
use crate::repository::DynRepository;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::Router;
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = get_config().expect("Failed to read configuration.");

    let repo: DynRepository = match cfg.storage {
        Storage::InMemory => Arc::new(InMemoryRepository::new()),
        Storage::Solana => Arc::new(SolanaRepository::try_new(cfg.keypair_file).unwrap()),
    };

    let api_router = Router::new()
        .merge(api::auth::routes(repo.clone()))
        .merge(api::list::routes(repo.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let routes = Router::new()
        .merge(api::health::routes())
        .nest("/api", api_router);
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
