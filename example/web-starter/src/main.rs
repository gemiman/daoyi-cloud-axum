pub mod config;
pub mod logger;

use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();
    let port = config::get().server.port();

    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    tracing::info!(
        "Listening on {}://{}",
        "http",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Daoyi Cloud Axum !"
}
