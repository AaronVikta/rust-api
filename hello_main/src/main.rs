use axum::{http::StatusCode, routing::get, Json, Router};
use tokio;
use serde::Serialize;
#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500")
    .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// async fn hello()->&'static str{
//     "Hello, world!"
// }

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() ->(StatusCode, Json<Response>){
    let response = Response{
        message: "Hello,World!",
    };
    (StatusCode::OK, Json(response))
}