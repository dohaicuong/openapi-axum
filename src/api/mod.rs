use aide::axum::ApiRouter;

mod docs;
mod meta;

pub fn router() -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/", meta::meta_router())
        .nest_api_service("/docs", docs::docs_router())
}
