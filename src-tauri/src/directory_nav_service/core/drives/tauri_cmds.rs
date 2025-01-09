use crate::directory_nav_service::models::drive_model::DriveModel;
use super::get_sys_drives::get_system_drives;

#[tauri::command]
pub fn get_drives() -> Vec<DriveModel> {
    get_system_drives()
}
