use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::Extension;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

mod api;
mod config;
mod graceful_shutdown;
mod logging;

#[tokio::main]
async fn main() {
    logging::init();

    let config = config::init().unwrap();
    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .layer(CorsLayer::new())
        .layer(CompressionLayer::new())
        .merge(api::router())
        // .merge(api::router())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(config))
        .layer(Extension(api))
        .layer(logging::layer());

    println!("Server is started at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("OpenAPI Axum")
        .summary("Summary of the API")
        .description("Description of the API")
        .security_scheme(
            "APIKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "Bearer".into(),
                description: Some("A key that is ignored".into()),
                extensions: Default::default(),
            },
        )
}
