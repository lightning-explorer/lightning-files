use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: move this to the models folder
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: DateTime<Utc>,
    pub date_created: DateTime<Utc>,
    /// The size of the file, in bytes
    pub size: u64,
    pub popularity: f64, // Consider making popularity more elaborate
}
