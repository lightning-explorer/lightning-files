use std::sync::Arc;

use tauri::State;

use crate::directory_nav_service::{app_state::sys_info_service::SysInfoService, models::system_info_model::SystemInfoModel};


#[tauri::command]
pub fn get_sys_info(sys_state:State<'_,Arc<SysInfoService>>)->SystemInfoModel{
    sys_state.get_sys_info()
}