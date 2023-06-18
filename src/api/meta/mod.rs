use axum::Router;

pub mod healthz;
pub mod root;

pub fn router() -> Router {
    Router::new().merge(root::route()).merge(healthz::route())
}
