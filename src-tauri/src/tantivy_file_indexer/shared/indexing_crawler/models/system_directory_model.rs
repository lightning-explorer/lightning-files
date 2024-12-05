use std::path::PathBuf;

use crate::tantivy_file_indexer::models::interal_system_file::InternalSystemFileModel;

#[derive(Clone)]
/// Formerly known as `FileInputModel`
pub struct InternalSystemDirectoryModel {
    pub dtos: Vec<InternalSystemFileModel>,
    pub path: PathBuf,
}
