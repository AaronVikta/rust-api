use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tokio;
use serde::Serialize;
use anyhow::Context;


#[tokio::main]
async fn main()-> anyhow::Result<()> {
    let app = Router::new()
    .route("/", get(hello_json))
    .layer(tower_http::catch_panic::CatchPanicLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500")
    .await
    .context("Failed to bind TCP Listener")?;
    axum::serve(listener, app).await
    .context("Axum::serve filed")?;

    Ok(())
}


// async fn hello()->&'static str{
//     "Hello, world!"
// }

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError{
    fn from(value: anyhow::Error) -> Self{
        Self(value)
    }
}

impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR,
        self.0.to_string()).into_response()
    }
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: generate_message().context("failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(response)))
}


fn generate_message()->anyhow::Result<&'static str>{
    if rand::random(){
        anyhow::bail!("No Message for You");
    }
    Ok("Hello, World!")
}