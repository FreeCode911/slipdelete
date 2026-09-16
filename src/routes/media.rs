use std::sync::Arc;
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use axum::body::Body;
use axum::http::header;

pub async fn handle(
    state: axum::extract::State<Arc<crate::AppStateManager>>,
    query: axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
    let filepath = match query.get("path") {
        Some(path) => path,
        None => return (StatusCode::BAD_REQUEST, "Missing path parameter").into_response(),
    };

    let path = std::path::Path::new(filepath);
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    let ext = path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let image_extensions = ["jpg", "jpeg", "png", "bmp", "webp", "avif", "jxl", "heic", "heif"];

    if image_extensions.contains(&ext.as_str()) {
        if let Ok(compressed) = crate::media::compress_image(filepath) {
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/jpeg")
                .body(Body::from(compressed))
                .unwrap();
            response.headers_mut().insert(
                header::CACHE_CONTROL,
                header::HeaderValue::from_static("no-cache"),
            );
            return response;
        }
    }

    let file = match std::fs::read(filepath) {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let mime_type = mime_guess::from_path(filepath)
        .first_or_octet_stream()
        .as_ref();

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type)
        .body(Body::from(file))
        .unwrap();

    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-cache"),
    );

    response
}
