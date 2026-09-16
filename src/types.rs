use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub path: String,
    pub name: String,
    pub media_type: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedFileRecord {
    pub path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct AppState {
    pub session_token: String,
    pub folder_path: String,
    pub media_files: Vec<MediaFile>,
    pub current_index: usize,
    pub kept_files: HashSet<String>,
    pub deleted_files: Vec<DeletedFileRecord>,
    pub activity_log: Vec<String>,
}
