use crate::directory_nav_service::models::drive_model::DriveModel;

use super::com::get_sys_drives::get_system_drives;

#[tauri::command]
pub fn get_drives() -> Result<Vec<DriveModel>, String> {
    match get_system_drives() {
        Ok(drives) => Ok(drives
            .iter()
            .map(|x| DriveModel {
                name: x.device_id.to_string(),
                label: x.volume_name.to_string(),
            })
            .collect()),
        Err(err) => Err(err),
    }
}
