// src/database/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("数据库连接失败: {0}")]
    ConnectionFailed(#[source] sqlx::Error),

    #[error("查询失败: {0}")]
    QueryFailed(#[source] sqlx::Error),

    #[error("事务失败: {0}")]
    TransactionFailed(#[source] sqlx::Error),
}