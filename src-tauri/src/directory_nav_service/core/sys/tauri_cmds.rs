use crate::directory_nav_service::models::system_info_model::SystemInfoModel;


#[tauri::command]
pub fn get_sys_info()->SystemInfoModel{
    super::info::get_sys_info()
}