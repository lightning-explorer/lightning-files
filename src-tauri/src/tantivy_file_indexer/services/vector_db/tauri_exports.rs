use std::sync::Arc;

use tauri::State;

use crate::tantivy_file_indexer::{
    dtos::vector_search_result::VectorSearchResult,
    models::vector_search_params_model::VectorSearchParamsModel,
};

use super::service::VectorDbService;

#[tauri::command]
pub async fn vector_db_query(
    params: VectorSearchParamsModel,
    service: State<'_, Arc<VectorDbService>>,
) -> Result<Vec<VectorSearchResult>, String> {
    service
        .query(params.query.as_str(), params.collection.as_str())
        .await
        .map(|x| {
            x.into_iter()
                .map(|(file, score)| VectorSearchResult { file, score })
                .collect()
        })
}
