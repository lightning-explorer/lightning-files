use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileDTOInput {
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: u64, // UNIX timestamp
    pub popularity:f64,
}

