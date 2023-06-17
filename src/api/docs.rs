use aide::{
    axum::{routing::get_with, ApiRouter},
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{routing::get, Extension, Json};

pub fn docs_router() -> ApiRouter {
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Redoc::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<OpenApi>) -> Json<aide::openapi::OpenApi> {
    Json(api)
}
