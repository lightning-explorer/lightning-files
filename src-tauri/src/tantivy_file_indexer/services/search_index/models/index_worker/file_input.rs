use std::path::PathBuf;

use crate::tantivy_file_indexer::models::interal_system_file::InternalSystemFileModel;

#[derive(Clone)]
pub struct FileInputModel {
    pub dtos: Vec<InternalSystemFileModel>,
    pub directory_from: PathBuf,
}
