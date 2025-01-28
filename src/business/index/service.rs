use anyhow::{Context, Result};
use axum::extract::State;
use sqlx::Row;

use crate::{mdoel::model::Index, AppState};

/// 获取首页列表的最大值   
pub async fn get_max_index(State(state): State<&AppState>) -> Result<u32> {
    // 从数据局中获取首页条目列表
    let db_pool = state.db_pool.clone();

    // 查询数据库获取最大值
    let max_index = sqlx::query("SELECT COUNT(*) FROM items")
        .fetch_one(&*db_pool)
        .await
        .with_context(|| "尝试获取首页列表的最大值查询数据库出现异常")?;

    Ok(max_index.get(0))
}

/// 获取首页列表
pub async fn get_index(State(state): State<AppState>, page: u8, size: u8) -> Result<Vec<Index>> {
    // 从数据局中获取首页条目列表
    let db_pool = state.db_pool.clone();

    // 分页查询，获取最新的10条数据,使用page和size参数
    let sql_str = format!(
        r#"
        SELECT * FROM items
        ORDER BY id DESC
        LIMIT {}
        OFFSET {}
        "#,
        size,
        (page - 1) * size
    );
    let entries = sqlx::query(&sql_str)
        .fetch_all(&*db_pool)
        .await
        .with_context(|| "尝试获取首页列表查询数据库出现异常")?;

    // 数据库结果转换成实体并且返回
    let mut dates = Vec::new();
    for entry in entries {
        dates.push(Index {
            id: entry.get(0),
            name: entry.get(1),
            thumbnail: entry.get(2),
            description: entry.get(3),
        });
    }

    Ok(dates)
}
