// src/database/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("数据库连接失败")]
    ConnectionFailed,

    #[error("创建表 :{0} 失败")]
    CreateTableFailed(String),

    #[error("查询失败: {0}")]
    QueryFailed(#[source] sqlx::Error),

    #[error("事务失败: {0}")]
    TransactionFailed(#[source] sqlx::Error),
}
