use serde::{Deserialize, Serialize};

use crate::tantivy_file_indexer::services::vector_db::models::embeddable_file_model::EmbeddableFileModel;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorSearchResult {
    pub file: EmbeddableFileModel,
    pub score: f32,
}
