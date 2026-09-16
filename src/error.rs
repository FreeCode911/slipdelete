use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({"error": self.message});
        let body_str = serde_json::to_string(&body).unwrap_or_default();
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(body_str))
            .unwrap()
    }
}

impl<E: std::fmt::Display> From<E> for ApiError {
    fn from(err: E) -> Self {
        ApiError {
            message: err.to_string(),
        }
    }
}
