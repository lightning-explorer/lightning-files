use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::dtos::{
        search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO,
    },
};

use super::{service::SearchIndexService, services::task_manager::TaskManagerService};

/**
The frontent expects SystemFileModels, so we will map the Tantivy models to this
*/
#[tauri::command]
pub async fn search_index_query(
    params: SearchParamsDTO,
    service: State<'_, Arc<SearchIndexService>>,
) -> Result<Vec<SystemFileModel>, String> {
    let service = Arc::clone(&service);

    let handle = tokio::task::spawn(async move {
        match service.query(&params) {
            Ok(result) => Ok(result
                .into_iter()
                .map(|x| x.to_sys_file())
                .flatten() // Ignore conversion errors
                .collect::<Vec<SystemFileModel>>()),
            Err(err) => Err(err.to_string()),
        }
    });

    // Await the result from the spawned task
    match handle.await {
        Ok(res) => res,
        Err(err) => Err(format!("Task failed: {}", err)),
    }
}

// TODO: you may be able to remove this if organized querying is superior
/// Emits an event in the format {STREAM_IDENTIFIER}:search_result to the frontend
#[tauri::command]
pub async fn search_index_query_streaming(
    params: StreamingSearchParamsDTO,
    app_handle: AppHandle,
    search_service: State<'_, Arc<SearchIndexService>>,
    task_manager: State<'_, Arc<TaskManagerService>>,
) -> Result<(), String> {
    let event_name = format!("{}:search_result", params.stream_identifier);
    let search_service_clone = Arc::clone(&search_service);

    task_manager
        .task
        .run(search_service_clone.streaming_query(params, move |files| {
            match app_handle.emit(&event_name, files) {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            }
        }))
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn search_index_query_streaming_organized(
    params: StreamingSearchParamsDTO,
    app_handle: AppHandle,
    search_service: State<'_, Arc<SearchIndexService>>,
    task_manager: State<'_, Arc<TaskManagerService>>,
) -> Result<(), String> {
    let event_name = format!("{}:search_result", params.stream_identifier);
    let search_service_clone = Arc::clone(&search_service);

    task_manager
        .task
        .run(search_service_clone.streaming_query_organized(params, move |files| {
            match app_handle.emit(&event_name, files) {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            }
        }))
        .await?;

    Ok(())
}
