use axum::{Extension, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

mod api;
mod config;
mod logging;

mod graceful_shutdown;

#[tokio::main]
async fn main() {
    logging::init();

    let config = config::init().unwrap();

    let app = Router::new()
        .layer(CorsLayer::new())
        .layer(CompressionLayer::new())
        .merge(api::router())
        .layer(Extension(config))
        .layer(logging::layer());

    println!("Server is started at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}
