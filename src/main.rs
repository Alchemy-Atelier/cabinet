mod api;
mod business;
mod error;
mod mdoel;

use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};
use env_logger::{Builder, Target};

// 定义一个共享的结构体
#[derive(Clone)]
pub struct AppState {
    // 这里可以包含任何你需要共享的数据
    // 例如数据库连接池、配置信息等
    db_pool: Arc<sqlx::SqlitePool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // log init
    logger();

    // db init
    let _pool = mdoel::db::init().await.unwrap();

    log::info!("Database connected");

    // server init
    let app_state = AppState {
        db_pool: Arc::new(_pool),
    };
    let app = router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9512").await.unwrap();

    log::info!("Server running on");

    // start server
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// build router
fn router(app_state: AppState) -> Router {
    let router=Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .route(
            "/",
            get(|| async {
                // 重定向到 /index/1/10
                // 构造一个重定向的响应出现异常就返回 500
                axum::http::Response::builder()
                    .status(axum::http::StatusCode::FOUND)
                    .header("location", "/index/1/10")
                    .body(axum::body::Body::empty())
                    .unwrap_or_else(|_e| {
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error",
                        )
                            .into_response()
                    })
            }),
        )
        .route("/index/:page/:size", get(api::index::index))
        .with_state(app_state.clone());

     // todo! 构建其他的部分的路由,比如获取单个示例以及对示例的增删改查
     business::curd::service::add_router(router, app_state)

}

/// init logger
fn logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(log::LevelFilter::Debug);
    builder.init();
}
