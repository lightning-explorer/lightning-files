use std::sync::Arc;

use tauri::State;

use crate::tantivy_file_indexer::models::{search_params_model::SearchParamsModel, tantivy_file_model::TantivyFileModel};

use super::service::SearchIndexService;

#[tauri::command]
pub fn search_index_query(
    params: SearchParamsModel,
    service: State<'_, Arc<SearchIndexService>>,
) -> Result<Vec<TantivyFileModel>, String> {
    match service.query(&params) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
