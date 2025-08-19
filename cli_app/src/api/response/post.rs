use crate::model::Post;
use serde::Serialize;

#[derive(Serialize)]
pub struct SinglePostResponse{
    pub data:Post,
}

#[derive(Serialize)]
pub struct ListPostResponse{
    pub data: Vec<Post>,
}