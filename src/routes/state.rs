use std::sync::Arc;
use axum::{http::StatusCode, response::{IntoResponse, Json}};

pub async fn handle(
    state: axum::extract::State<Arc<crate::state::AppStateManager>>,
    cookies: axum::http::header::HeaderMap,
) -> impl IntoResponse {
    let session = state.session.lock().unwrap();
    let expected_token = session.token.clone();

    let client_cookie = cookies
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .find(|part| part.trim().starts_with("session_id="))
                .map(|part| part.trim().strip_prefix("session_id=").unwrap_or(""))
        });

    if client_cookie != Some(expected_token.as_str()) {
        return (StatusCode::LOCKED, "Server is in use by another session").into_response();
    }

    let current_file = state.get_current_file();
    let media_files = state.media_files.lock().unwrap();
    let app_state = state.state.lock().unwrap();

    let total = media_files.len();
    let deleted = media_files.iter().filter(|f| f.deleted).count();
    let kept = app_state.kept_files.len();
    let remaining = total - deleted;

    let response = serde_json::json!({
        "current_index": *state.current_index.lock().unwrap(),
        "total_files": total,
        "progress": if total > 0 { format!("{} / {}", *state.current_index.lock().unwrap() + 1, total) } else { "0 / 0".to_string() },
        "filename": current_file.as_ref().map(|f| f.name.clone()).unwrap_or_else(|| "No file selected".to_string()),
        "current_file": current_file.as_ref().map(|f| serde_json::json!({
            "name": f.name,
            "path": f.path,
            "type": f.file_type
        })),
        "stats": {
            "total": total,
            "kept": kept,
            "deleted": deleted,
            "remaining": remaining
        },
        "activity_log": &app_state.activity_log[..app_state.activity_log.len().min(10)]
    });

    Json(response).into_response()
}
