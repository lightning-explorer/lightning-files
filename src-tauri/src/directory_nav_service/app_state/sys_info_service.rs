use crate::directory_nav_service::models::system_info_model::SystemInfoModel;
use dirs;

pub struct SysInfoService {}

impl SysInfoService {

    pub fn get_sys_info(&self)->SystemInfoModel{
        // TODO: this may need to be conditional
        Self::sys_info_windows()
    }

    fn sys_info_windows() -> SystemInfoModel {
        let system_directory_path =
            std::env::var("SystemDrive").expect("Failed to get system drive path");

        let home_directory_path: Option<String> =
        dirs::home_dir().map(|x| x.to_string_lossy().into_owned());

        let desktop_directory_path: Option<String> =
        dirs::desktop_dir().map(|x| x.to_string_lossy().into_owned());

        let downloads_directory_path: Option<String> =
        dirs::download_dir().map(|x| x.to_string_lossy().into_owned());

        let documents_directory_path: Option<String> =
        dirs::document_dir().map(|x| x.to_string_lossy().into_owned());

        let pictures_directory_path: Option<String> =
        dirs::picture_dir().map(|x| x.to_string_lossy().into_owned());

        SystemInfoModel {
            system_directory_path,
            home_directory_path,
            desktop_directory_path,
            downloads_directory_path,
            documents_directory_path,
            pictures_directory_path
        }
    }
}
