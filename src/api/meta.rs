use aide::{
    axum::{routing::get_with, ApiRouter},
    transform::TransformOperation,
};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize, JsonSchema)]
pub struct AppMeta {
    name: String,
    version: String,
}

pub fn doc(op: TransformOperation) -> TransformOperation {
    op.description("Service metadata")
        .tag("meta")
        .response_with::<200, Json<AppMeta>, _>(|res| {
            res.example(AppMeta {
                name: "app name".into(),
                version: "0.0.1".into(),
            })
        })
}

pub async fn handler(Extension(config): Extension<Config>) -> Json<AppMeta> {
    Json(AppMeta {
        name: config.package.name,
        version: config.package.version,
    })
}

pub fn meta_router() -> ApiRouter {
    ApiRouter::new().api_route("/", get_with(handler, doc))
}
