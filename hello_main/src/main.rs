use axum::{routing::get, Router};
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500")
    .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn hello()->&'static str{
    "Hello, world!"
}