use dotenv::dotenv;
use serde_json::{json, Value};
use std::{env, net::SocketAddr};

use axum::{response::Json, routing::get, Router};

/// This app serves a router that returns a randomly generated
/// `String` suffixed with a timestamp every 5 seconds
#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT")
        .expect("Server missing `PORT` environment variable")
        .parse()
        .expect("Failed to parse `PORT` environment variable");

    let app = Router::new()
        .route("/", get(|| root()))
        .route("/json", get(|| json()));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server successfully started\nListening on {}...", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    println!("Server shutdown")
}

/// Tokio signal handler, waits for CTRL+C.
/// Used in hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    println!("\nGracefully shutting down...");
}

async fn root() -> &'static str {
    "pong"
}

async fn json() -> Json<Value> {
    Json(json!({"res": "pong"}))
}
