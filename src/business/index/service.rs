use anyhow::Context;
use axum::extract::State;

use crate::{mdoel::error::DBError, AppState};

pub async fn get_index(State(state): State<AppState>) -> Result<(), anyhow::Error> {
    // 从数据局中获取首页条目列表
    let db_pool = state.db_pool.clone();

    // todo 数据库表还没创建
    let entries = sqlx::query("SELECT * FROM entries")
        .fetch_all(&*db_pool)
        .await
        .map_err(|e| DBError::QueryFailed(e))
        .with_context(|| "尝试获取首页列表查询数据库出现异常")?;

    Ok(())
}
