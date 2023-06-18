use axum::Router;
use serde::Serialize;
use utoipa::ToSchema;

pub mod create_post;
pub mod get_posts;

#[derive(Serialize, ToSchema)]
pub struct PostPayload {
    pub id: i32,
    pub title: String,
    pub text: String,
}

pub fn router() -> Router {
    Router::new()
        .merge(get_posts::route())
        .merge(create_post::route())
}
