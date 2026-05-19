use axum::{Router, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", routing::get(async || "Hello Daoyi Cloud Axum !"));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, router).await.unwrap();
}
