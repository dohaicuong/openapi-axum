use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::config::Config;

#[derive(Serialize, ToSchema)]
pub struct AppMeta {
    name: String,
    version: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "meta",
    responses(
        (status = 200, description = "Service metadata", body = AppMeta)
    )
)]
pub async fn handler(Extension(config): Extension<Config>) -> Json<AppMeta> {
    Json(AppMeta {
        name: config.package.name,
        version: config.package.version,
    })
}

pub fn route() -> Router {
    Router::new().route("/", get(handler))
}
