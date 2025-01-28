use anyhow::Context;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use super::error::DBError;

async fn get_pool() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("/home/yuzn/sqlitedb/test.db")
            .create_if_missing(true),
    )
    .await
    .with_context(|| DBError::ConnectionFailed)?;
    Ok(pool)
}

pub async fn init() -> anyhow::Result<SqlitePool> {
    let pool = get_pool().await?;
    init_table(&pool).await?;

    Ok(pool)
}

async fn init_table(pool: &SqlitePool) -> anyhow::Result<()> {
    // 创建表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NOT NULL UNIQUE,
            thumbnail VARCHAR(255) NOT NULL,
            description TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_items_name ON items (name);
        "#,
    )
    .execute(pool)
    .await
    .with_context(|| DBError::CreateTableFailed("items".to_string()))?;

    Ok(())
}
