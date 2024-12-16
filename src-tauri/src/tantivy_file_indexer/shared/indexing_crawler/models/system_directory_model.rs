use std::path::PathBuf;

use crate::tantivy_file_indexer::models::internal_system_file;

#[derive(Clone)]
/// Formerly known as `FileInputModel`
pub struct InternalSystemDirectoryModel {
    pub dtos: Vec<internal_system_file::model::Model>,
    pub path: PathBuf,
}
