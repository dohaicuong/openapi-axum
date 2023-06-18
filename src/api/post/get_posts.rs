use ::entity::post;
use axum::{routing::get, Extension, Json, Router};
use sea_orm::*;

use crate::db_error::ServiceDBError;

use super::PostPayload;

#[utoipa::path(
    get,
    path = "/get_posts",
    tag = "post",
    responses(
        (status = 200, description = "List all posts", body = Vec<PostPayload>)
    )
)]
pub async fn handler(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<PostPayload>>, ServiceDBError> {
    let posts = post::Entity::find()
        .order_by(post::Column::Id, Order::Asc)
        .all(&db)
        .await?
        .iter()
        .map(|post| PostPayload {
            id: post.id,
            title: post.title.to_owned(),
            text: post.text.to_owned(),
        })
        .collect();

    Ok(Json(posts))
}

pub fn route() -> Router {
    Router::new().route("/get_posts", get(handler))
}
