
use axum::{extract::State, response::{IntoResponse, Response}};

use crate::AppState;
use crate::business::index::service::get_index;

pub async fn index(State(state): State<AppState>) -> Response {
    // index handler
    let err=get_index(axum::extract::State(state)).await;
    log::error!("{:?}",err);

    "todo!".into_response()
}
