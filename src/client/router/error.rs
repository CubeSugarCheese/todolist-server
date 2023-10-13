use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("require login")]
    RequireLogin,
    #[error("invalid token")]
    InvalidToken,
    #[error("you have not permission to access this request")]
    PermissionDeny,
    #[error("internal error: {0}")]
    InternalServerError(String)
}

impl ApiError {
    pub const fn status_code(&self) -> StatusCode {
        match self {
            ApiError::RequireLogin => StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken => StatusCode::FORBIDDEN,
            ApiError::PermissionDeny => StatusCode::FORBIDDEN,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    pub fn to_json_string(&self) -> String {
        let error_msg = self.to_string();
        json!({
            "msg": error_msg,
            "data": {}
        }).to_string()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = body::boxed(body::Full::from(self.to_json_string()));
        Response::builder()
            .status(self.status_code())
            .header("content-type","application/json")
            .body(body)
            .unwrap()
    }
}
