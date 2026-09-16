use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};

pub async fn delete(state: axum::extract::State<Arc<crate::AppStateManager>>) -> impl IntoResponse {
    let current = match state.get_current_file() {
        Some(f) => f,
        None => return (StatusCode::OK, Json(serde_json::json!({"success": false, "message": "No file to delete"}))).into_response(),
    };

    let file_path = current.path.clone();
    let file_name = current.name.clone();

    match crate::recycle_bin::move_to_recycle_bin(&file_path) {
        Ok(_) => {
            {
                let mut media_files = state.media_files.lock().unwrap();
                if let Some(f) = media_files.iter_mut().find(|f| f.path == file_path) {
                    f.deleted = true;
                }
            }
            {
                let mut app_state = state.state.lock().unwrap();
                app_state.deleted_files.push(crate::state::DeletedFile {
                    path: file_path.clone(),
                    name: file_name.clone(),
                });
            }
            state.log_action(format!("DELETED: {}", file_name));
            state.advance_to_next();
            (StatusCode::OK, Json(serde_json::json!({"success": true, "name": file_name}))).into_response()
        }
        Err(e) => {
            (StatusCode::OK, Json(serde_json::json!({"success": false, "message": e}))).into_response()
        }
    }
}

pub async fn keep(state: axum::extract::State<Arc<crate::AppStateManager>>) -> impl IntoResponse {
    let current = match state.get_current_file() {
        Some(f) => f,
        None => return (StatusCode::OK, Json(serde_json::json!({"success": false, "message": "No file to keep"}))).into_response(),
    };

    let file_name = current.name.clone();

    {
        let mut app_state = state.state.lock().unwrap();
        app_state.kept_files.insert(current.path.clone());
    }

    state.log_action(format!("KEPT: {}", file_name));
    state.advance_to_next();

    (StatusCode::OK, Json(serde_json::json!({"success": true, "name": file_name}))).into_response()
}

pub async fn restore(state: axum::extract::State<Arc<crate::AppStateManager>>) -> impl IntoResponse {
    let mut app_state = state.state.lock().unwrap();

    if let Some(last) = app_state.deleted_files.pop() {
        {
            let mut media_files = state.media_files.lock().unwrap();
            if let Some(f) = media_files.iter_mut().find(|f| f.path == last.path) {
                f.deleted = false;
            }
        }
        state.log_action(format!("RESTORED: {}", last.name));
        (StatusCode::OK, Json(serde_json::json!({"success": true, "name": last.name}))).into_response()
    } else {
        (StatusCode::OK, Json(serde_json::json!({"success": false, "message": "Nothing to restore"}))).into_response()
    }
}

pub async fn reset(state: axum::extract::State<Arc<crate::AppStateManager>>) -> impl IntoResponse {
    {
        let mut app_state = state.state.lock().unwrap();
        app_state.kept_files.clear();
        app_state.deleted_files.clear();
        app_state.activity_log.clear();
    }
    {
        let mut current_index = state.current_index.lock().unwrap();
        *current_index = 0;
    }
    {
        let mut media_files = state.media_files.lock().unwrap();
        for f in media_files.iter_mut() {
            f.deleted = false;
        }
    }
    state.log_action("HISTORY CLEARED".to_string());
    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}
