use std::path::Path;

use crate::shared::models::sys_file_model::SystemFileModel;

use super::super::entities::file;

impl From<SystemFileModel> for file::Model {
    fn from(val: SystemFileModel) -> Self {
        let parent_path = get_parent_directory(&val.file_path);
        Self {
            path: val.file_path,
            parent_path,
        }
    }
}

fn get_parent_directory(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    path.parent().map(|x| x.to_string_lossy().to_string())
}
