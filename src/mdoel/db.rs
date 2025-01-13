use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

pub async fn get_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("/home/yuzn/sqlitedb/test.db")
            .create_if_missing(true),
    )
    .await?;
    Ok(pool)
}
