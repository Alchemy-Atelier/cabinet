use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use crate::business::index::service::{get_index, get_max_index};
use crate::AppState;

/// index handler get 请求从 url 中获取分页参数
pub async fn index(
    State(state): State<AppState>,
    Path(page): Path<u8>,
    Path(size): Path<u8>,
) -> Response {
    // 校验参数
    let mut page = page;
    let mut size = size;
    if page == 0 || size == 0 {
        page = 1;
        size = 10;
    }

    // 校验是否超过最大值
    // 获取首页列表的最大值
    let max_index = get_max_index(axum::extract::State(&state)).await.unwrap();

    if (page * size) as u32 > max_index {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Page is greater than the maximum value",
        )
            .into_response();
    }

    let index_dates = get_index(axum::extract::State(state), page, size).await;
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
