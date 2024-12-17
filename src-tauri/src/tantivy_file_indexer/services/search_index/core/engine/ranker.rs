use crate::shared::models::sys_file_model::SystemFileModel;

pub fn rank_new_file(file: &SystemFileModel) -> SystemFileModel {
    SystemFileModel {
        popularity: 2.0,
        ..file.clone()
    }
}

pub fn rank_existing_file(
    new_file: &SystemFileModel,
    old_file: &SystemFileModel,
) -> SystemFileModel {
    SystemFileModel {
        popularity: old_file.popularity + 1.0,
        ..new_file.clone()
    }
}
