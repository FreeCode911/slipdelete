use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({"error": self.message}));
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(serde_json::to_string(&body).unwrap_or_default()))
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
