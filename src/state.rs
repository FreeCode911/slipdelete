use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub path: String,
    pub name: String,
    pub file_type: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub kept_files: HashSet<String>,
    pub deleted_files: Vec<DeletedFile>,
    pub activity_log: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedFile {
    pub path: String,
    pub name: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            kept_files: HashSet::new(),
            deleted_files: Vec::new(),
            activity_log: Vec::new(),
        }
    }
}

pub struct Session {
    pub token: String,
}

pub struct AppStateManager {
    pub state: Mutex<AppState>,
    pub session: Mutex<Session>,
    pub current_index: Mutex<usize>,
    pub media_files: Mutex<Vec<MediaFile>>,
}

impl AppStateManager {
    pub fn new() -> Self {
        let session = Session {
            token: uuid::Uuid::new_v4().to_string(),
        };
        Self {
            state: Mutex::new(AppState::default()),
            session: Mutex::new(session),
            current_index: Mutex::new(0),
            media_files: Mutex::new(Vec::new()),
        }
    }

    pub fn log_action(&self, message: String) {
        let mut state = self.state.lock().unwrap();
        state.activity_log.insert(0, message);
        if state.activity_log.len() > 50 {
            state.activity_log.pop();
        }
    }

    pub fn get_current_file(&self) -> Option<MediaFile> {
        let mut current_index = self.current_index.lock().unwrap();
        let media_files = self.media_files.lock().unwrap();

        while *current_index < media_files.len() {
            if !media_files[*current_index].deleted {
                return Some(media_files[*current_index].clone());
            }
            *current_index += 1;
        }
        None
    }

    pub fn advance_to_next(&self) -> bool {
        let mut current_index = self.current_index.lock().unwrap();
        let media_files = self.media_files.lock().unwrap();

        while *current_index < media_files.len() - 1 {
            *current_index += 1;
            if !media_files[*current_index].deleted {
                return true;
            }
        }
        false
    }
}
