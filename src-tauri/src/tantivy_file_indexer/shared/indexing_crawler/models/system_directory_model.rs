use std::path::PathBuf;

use crate::shared::models::sys_file_model::SystemFileModel;

#[derive(Clone)]
/// Formerly known as `FileInputModel`
pub struct InternalSystemDirectoryModel {
    pub dtos: Vec<SystemFileModel>,
    pub path: PathBuf,
}
