use std::path::Path;

use crate::tantivy_file_indexer::models::interal_system_file::InternalSystemFileModel;

use super::super::entities::file;

impl From<InternalSystemFileModel> for file::Model {
    fn from(val: InternalSystemFileModel) -> Self {
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
