use axum::http::StatusCode;

pub async fn hello() -> Result<String,StatusCode>{
    Ok("Hello from API!".to_string())
}