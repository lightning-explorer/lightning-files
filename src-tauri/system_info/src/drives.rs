use sysinfo::Disks;

use crate::models::drive_model::DriveModel;

pub fn get_system_drives() -> Vec<DriveModel> {
    let mut drives = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.iter() {
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
