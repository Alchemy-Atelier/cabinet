
use thiserror::Error;

use crate::mdoel::error::DBError;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("数据库错误")]
    DatabaseFailed(#[from] DBError ),

    #[error("内部服务器错误: {0}")]
    InternalServerError(#[source] anyhow::Error),
}