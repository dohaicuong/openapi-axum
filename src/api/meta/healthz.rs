use axum::{routing::get, Router};

#[utoipa::path(
    get,
    path = "/healthz",
    tag = "meta",
    responses(
        (status = 200, description = "Service health check", body = &'static str)
    )
)]

pub async fn handler() -> &'static str {
    "ok"
}

pub fn route() -> Router {
    Router::new().route("/healthz", get(handler))
}
