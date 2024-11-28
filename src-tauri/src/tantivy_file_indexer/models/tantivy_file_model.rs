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

impl From<TantivyFileModel> for SystemFileModel{
    fn from(val: TantivyFileModel) -> Self {
        SystemFileModel{
            name:val.name,
            file_path:val.file_path,
            date_modified:"".to_string(),
            score:val.score,
            is_directory:val.is_directory
        }
    }
}