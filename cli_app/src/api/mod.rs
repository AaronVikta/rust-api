use axum::Router;

mod handlers;
mod v1;


use std::sync::Arc;
use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .nest("/v1", v1::configure(state))
}