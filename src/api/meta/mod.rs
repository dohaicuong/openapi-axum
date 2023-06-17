use axum::Router;

pub mod healthz;
pub mod meta;

pub fn router() -> Router {
    Router::new().merge(meta::route()).merge(healthz::route())
}
