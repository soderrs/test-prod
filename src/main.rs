mod auth;
mod countries;
mod me;
mod middlewares;
mod routes;
mod profiles;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = routes::app().await;
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
