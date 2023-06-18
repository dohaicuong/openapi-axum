use ::entity::post;
use axum::{debug_handler, extract, routing::post, Extension, Json, Router};
use sea_orm::*;
use serde::Deserialize;
use utoipa::ToSchema;

use super::PostPayload;
use crate::db_error::ServiceDBError;

#[derive(Deserialize, ToSchema)]
pub struct CreatePostInput {
    title: String,
    text: String,
}

#[utoipa::path(
    post,
    path = "/create_post",
    tag = "post",
    responses(
        (status = 200, description = "Create a post", body = PostPayload)
    ),
    request_body(
        content = CreatePostInput,
        content_type = "application/json",
    )
)]
#[debug_handler]
pub async fn handler(
    Extension(db): Extension<DatabaseConnection>,
    extract::Json(input): extract::Json<CreatePostInput>,
) -> Result<Json<PostPayload>, ServiceDBError> {
    let new_post = post::ActiveModel {
        title: Set(input.title.to_owned()),
        text: Set(input.text.to_owned()),
        ..Default::default()
    };
    let inserted_new_post = post::Entity::insert(new_post).exec(&db).await?;

    let new_post = post::Entity::find_by_id(inserted_new_post.last_insert_id)
        .one(&db)
        .await?;

    match new_post {
        None => Err(ServiceDBError(DbErr::RecordNotFound(
            "Something went wrong!".to_string(),
        ))),
        Some(post) => Ok(Json(PostPayload {
            id: post.id,
            title: post.title,
            text: post.text,
        })),
    }
}

pub fn route() -> Router {
    Router::new().route("/create_post", post(handler))
}
