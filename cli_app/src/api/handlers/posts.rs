use std::sync::Arc;

use axum::{extract::{Path, State}, Json};

use crate::{services::post::{CreatePostRequest, PostService, UpdatePostRequest}, state::ApplicationState};

use crate::api::response::post::{SinglePostResponse, ListPostResponse}; // Update this import to the correct path
use crate::api::errors::AppError; // Add this import if AppError is defined elsewhere
use axum::http::StatusCode; // Add this import for StatusCode

pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>
)->Result<Json<SinglePostResponse>, AppError>{

    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse{data:post};

    Ok(Json(response))
}

pub async fn update(
    State(state):State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>
) -> Result<Json<SinglePostResponse>, AppError>{

    let post = state.post_service.update_post(id, payload).await?;

    let response = SinglePostResponse{data:post};

    Ok(Json(response))

}

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
)->Result<Json<()>, AppError>{
    state.post_service.delete_post(id).await?;

    Ok(Json(()))

}

pub async fn list (
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostResponse>,AppError>{

    let posts = state.post_service.get_all_posts().await?;

    let response = ListPostResponse{data:posts};

    Ok(Json(response))
}

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(slug): Path<String>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_slug(&slug).await;

    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };

            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((StatusCode::NOT_FOUND, e))),
    }
}
