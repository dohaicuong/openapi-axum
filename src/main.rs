use axum::{Extension, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

mod api;
mod config;
mod db_error;
mod graceful_shutdown;
mod logging;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::init();

    let config = config::init().unwrap();

    println!("connecting to db....");
    let connection = Database::connect(config.database_url.clone())
        .await
        .unwrap();
    println!("connected to db!");

    println!("migrating db");
    Migrator::up(&connection, None).await.unwrap();
    println!("db is migrated!");

    let app = Router::new()
        .layer(CorsLayer::new())
        .layer(CompressionLayer::new())
        .merge(api::router())
        .layer(Extension(config.clone()))
        .layer(Extension(connection))
        .layer(logging::layer());

    let url = format!("{}:{}", config.host, config.port);
    println!("Server is started at http://{url}");
    axum::Server::bind(&format!("{url}").parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}
