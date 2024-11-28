use std::sync::Arc;

use tauri::State;

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::models::search_params_model::SearchParamsModel,
};

use super::service::SearchIndexService;

/**
 The frontent expects SystemFileModels, so we will map the Tantivy models to this
 */
#[tauri::command]
pub fn search_index_query(
    params: SearchParamsModel,
    service: State<'_, Arc<SearchIndexService>>,
) -> Result<Vec<SystemFileModel>, String> {
    match service.query(&params) {
        Ok(result) => Ok(result.into_iter().map(|x| x.into()).collect()),
        Err(err) => Err(err.to_string()),
    }
}
