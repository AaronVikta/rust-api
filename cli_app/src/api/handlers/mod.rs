pub mod hello;
use crate::{api::v1, state::ApplicationState};
use std::sync::Arc;
use axum::Router;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
