
use anyhow::{Context, Result };
use axum::{extract::{Path, State}, http, response::{IntoResponse, Response}, routing::{delete, get, post, put}, Json, Router};
use sqlx::{Executor, Row};
use base64::{self, engine::general_purpose, Engine};

use crate::{mdoel::model::Index, AppState};

// 集成到 roter 里面
pub fn add_router(add_router: Router<AppState>, app_state: AppState) -> Router {
    
        // 构建路由 curd 部分
        // 根据 id 查询, id 从 url 中取 构建动态路由
        add_router.route("/cabinet/:id", get(get_cabinert_by_id))
        // 通过请求传入的 json 解析参数
        .route("/cabinet/update", post(update_cabinert_by_id))
        .route("/cabinet/addd", post(insert_cabinert_by_id))
        .route("/cabinet/del/:id", delete(del_cabinert_by_id))
        .with_state(app_state)
}


/// 插入一条数据

pub async fn insert_cabinert_by_id(
    State(state): State<AppState>,
    Json(payload): Json<UpdateCabinetRequest>,
) -> Response {
    let index_dates = add_one(axum::extract::State(state),payload.name,payload.thumbnail,payload.description).await;
    // 处理结果返回响应
    match index_dates {
        Ok(_) => (http::StatusCode::OK, "add Success").into_response(),
        Err(e) => {
            log::error!("Index Error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}



/// index handler get 请求从 url 中获取分页参数
pub async fn get_cabinert_by_id(
    State(state): State<AppState>,
    Path(id): Path<u8>,
) -> Response {
   
    let index_dates = get_by_id(axum::extract::State(state), id).await;
    // 处理结果返回响应
    match index_dates {
        Ok(dates) => Json(dates).into_response(),
        Err(e) => {
            log::error!("Index Error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}


/// 更新的路由方法
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateCabinetRequest {
    id: u8,
    name: String,
    thumbnail: Vec<u8>,
    description: String,
}

pub async fn update_cabinert_by_id(
    State(state): State<AppState>,
    Json(payload): Json<UpdateCabinetRequest>,
) -> Response {
    let index_dates = update_by_id(axum::extract::State(state), payload.id,payload.name,payload.thumbnail,payload.description).await;
    // 处理结果返回响应
    match index_dates {
        Ok(_) => (http::StatusCode::OK, "Update Success").into_response(),
        Err(e) => {
            log::error!("Index Error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}

pub async fn del_cabinert_by_id(
    State(state): State<AppState>,
    Path(id): Path<u8>,
) -> Response {
    let index_dates = del_by_id(axum::extract::State(state), id).await;
    // 处理结果返回响应
    match index_dates {
        Ok(_) => (http::StatusCode::OK, "delete Success").into_response(),
        Err(e) => {
            log::error!("Index Error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}



/// 通过 id 获取数据库中的一个条目
pub async fn get_by_id(State(state): State<AppState>,id:u8) -> Result<Index> {
    // 从数据局中获取首页条目列表
    let db_pool = state.db_pool.clone();

    // 分页查询，获取最新的10条数据,使用page和size参数
    let sql_str = format!(
        r#"
        SELECT * FROM items
        WHERE id = {}"#,id
    );
    let entries = sqlx::query(&sql_str)
        .fetch_one(&*db_pool)
        .await
        .with_context(|| "尝试获取首页列表查询数据库出现异常")?;

    // 数据库结果转换成实体并且返回
    let date = Index {
        id: entries.get(0),
        name: entries.get(1),
        thumbnail: entries.get(2),
        description: entries.get(3),
    };

    Ok(date)
}

/// 插入一条数据
pub async fn add_one(State(state): State<AppState>,name:String,thumbnail:Vec<u8>,description:String)->Result<()> {
    // 插入一条数据
    let sql_str = format!(
        r#"
        INSERT INTO items (name, thumbnail, description)
        VALUES ('{}', '{}', '{}')
        "#,
        name, 
        general_purpose::STANDARD.encode(thumbnail), // 将二进制数据转换为base64字符串
        description
    );
    
    // 执行 sql 
    state.db_pool.execute(&*sql_str)
        .await
        .with_context(|| "尝试插入一条数据到数据库出现异常")?;
    Ok(())  
}

/// 删除一条条目
pub async fn del_by_id(State(state): State<AppState>,id:u8)->Result<()>{
    // 删除一条数据
    let sql_str = format!(
        r#"
        DELETE FROM items
        WHERE id = '{}'
        "#,id
    );
    // 执行 sql
    state.db_pool.execute(&*sql_str)
        .await
        .with_context(|| "尝试删除一条数据到数据库出现异常")?;

    Ok(())
}


/// 修改条目通过 id
pub async fn update_by_id(State(state): State<AppState>,id:u8,name:String,thumbnail:Vec<u8>,description:String)->Result<()>{
    // 更新一条数据
    let sql_str = format!(
        r#"
        UPDATE items
        SET name = '{}', thumbnail = '{}', description = '{}'
        WHERE id = '{}'
        "#,
        name, 
        general_purpose::STANDARD.encode(thumbnail), // 将二进制数据转换为base64字符串
        description,
        id
    );

    // 执行 sql
    let sql_reuslt= state.db_pool.execute(&*sql_str)
        .await
        .with_context(|| "尝试删除一条数据到数据库出现异常")?;

    // 检查是否有行被更新
    if sql_reuslt.rows_affected() == 0 {
        return Err(anyhow::anyhow!("No rows were updated, please check the id"));
    }
    // 如果没有异常则返回
    log::info!("Updated {} rows", sql_reuslt.rows_affected());


    Ok(())
}
