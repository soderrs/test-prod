use middlewares::authorize::AppState;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

mod auth;
mod countries;
mod friends;
mod me;
mod middlewares;
mod profiles;
mod routes;
mod posts;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        revoked_tokens: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = routes::app(state).await;
    let listener = tokio::net::TcpListener::bind("localhost:7878")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "<h1>Hello, World!</h1"
}

async fn ping() -> &'static str {
    "ok"
}
