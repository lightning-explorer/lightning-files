use std::{os::windows::fs::MetadataExt, path::PathBuf};

use crate::shared::models::sys_file_model::SystemFileModel;

use super::file::TantivyFileModel;



impl TryFrom<TantivyFileModel> for SystemFileModel{
    type Error = std::io::Error;

    fn try_from(value: TantivyFileModel) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value.file_path.clone());
        let meta = path.metadata()?;
        Ok(
        SystemFileModel {
            name: value.name,
            file_path: value.file_path,
            date_modified: "".to_string(),
            size: meta.file_size(),
            score: value.score,
            is_directory: value.is_directory,
        })
    }
}