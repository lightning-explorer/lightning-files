use std::{os::windows::fs::MetadataExt, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::shared::models::sys_file_model::SystemFileModel;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TantivyFileModel {
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: String,
    pub score: f64,
    pub is_directory: bool,
}

impl TantivyFileModel {
    pub fn to_sys_file(self) -> Result<SystemFileModel,std::io::Error> {
        let path = PathBuf::from(self.file_path.clone());
        let meta = path.metadata()?;
        Ok(
        SystemFileModel {
            name: self.name,
            file_path: self.file_path,
            date_modified: "".to_string(),
            size: meta.file_size(),
            score: self.score,
            is_directory: self.is_directory,
        })
    }
}
