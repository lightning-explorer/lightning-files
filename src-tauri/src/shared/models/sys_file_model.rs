use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemFileModel {
    pub name: String,
    pub file_path: String,
    pub date_modified: String, 
    pub score:f64,
    pub is_directory:bool,
}

