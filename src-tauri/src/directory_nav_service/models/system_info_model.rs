use serde::{Deserialize, Serialize};

/// Information about the current user's system
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfoModel {
    pub home_directory_path: String,
}
