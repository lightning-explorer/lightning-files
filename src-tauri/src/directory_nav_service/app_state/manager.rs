use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::FilesDisplayState;

pub fn manage_state(handle:&AppHandle){
    handle.manage(Arc::new(FilesDisplayState::new()));
}