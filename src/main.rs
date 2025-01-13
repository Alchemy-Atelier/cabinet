mod api;
mod mdoel;

use axum::{routing::get, Router};
use env_logger::{Builder, Target};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // log init
    logger();

    // db init
    let _pool = mdoel::db::get_pool().await.unwrap();

    log::info!("Database connected");

    // server init
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9512").await.unwrap();

    log::info!("Server running on");

    // start server
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// build router
fn router() -> Router {
    Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .route("/", get(api::index::index))
}

/// init logger
fn logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(log::LevelFilter::Debug);
    builder.init();
}
