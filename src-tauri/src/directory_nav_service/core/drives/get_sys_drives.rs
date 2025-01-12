use crate::directory_nav_service::models::drive_model::DriveModel;
use sysinfo::{DiskExt, System, SystemExt};

pub fn get_system_drives() -> Vec<DriveModel> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut drives = Vec::new();

    for disk in system.disks() {
        let model = DriveModel {
            name: disk.mount_point().to_string_lossy().to_string(),
            label: None, // TODO: fill in if the user sets a custom label
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        };
        drives.push(model);
    }
    drives
}
