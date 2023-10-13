use crate::client::model::ApiResponse;
use crate::client::router::error::ApiError;
use axum::Json;
use serde::Serialize;

pub mod error;
pub mod jwt;
pub mod task;
pub mod user;

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiError>;

const fn ok<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(Json(ApiResponse::new(data)))
}
