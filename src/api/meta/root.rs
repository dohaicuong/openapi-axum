use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::config::Config;

#[derive(Serialize, ToSchema)]
pub struct AppMetaPayload {
    name: String,
    version: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "meta",
    responses(
        (status = 200, description = "Service metadata", body = AppMetaPayload)
    )
)]
pub async fn handler(Extension(config): Extension<Config>) -> Json<AppMetaPayload> {
    Json(AppMetaPayload {
        name: config.package.name,
        version: config.package.version,
    })
}

pub fn route() -> Router {
    Router::new().route("/", get(handler))
}
