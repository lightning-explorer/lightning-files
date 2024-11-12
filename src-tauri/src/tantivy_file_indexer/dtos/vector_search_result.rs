use serde::{Deserialize, Serialize};

use crate::tantivy_file_indexer::services::vevtor::models::file_model::FileModel;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorSearchResult {
    pub file: FileModel,
    pub score: f32,
}
