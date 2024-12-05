use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};
use tokio::sync::watch;

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

// TODO: make the task handling logic more efficient and make results get streamed to the frontend faster
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

    // Create a new cancellation channel
    let (cancel_sender, mut cancel_receiver) = watch::channel(());

    // Cancel the current task if one exists
    if let Some((old_cancel_sender, old_handle)) = task_manager.current_task.write().await.take() {
        let _ = old_cancel_sender.send(()); // Signal cancellation
        let _ = old_handle.await; // Wait for the task to finish
    }

    // Use a one-shot channel to notify task completion
    let (completion_sender, completion_receiver) = tokio::sync::oneshot::channel();

    // Spawn the new task
    let handle = tokio::spawn(async move {
        let result = tokio::select! {
            emit_result = async {
                let streaming_task = search_service_clone.streaming_query(params, move |files| {
                    let num_results = files.len();
                    match app_handle.emit(&event_name, files){
                        Ok(_)=>println!("emitted results: {}", num_results),
                        Err(err)=>println!("{}",err)
                    }
                });
                _ = streaming_task.await;
                Ok(())
            } => emit_result,
            _ = cancel_receiver.changed() => {
                Err("Cancellation received. Streaming query task exited")
            }
        };

        // Notify task completion
        println!("Task completed");
        let _ = completion_sender.send(result);
    });

    // Store the new task in the TaskManager
    task_manager
        .current_task
        .write()
        .await
        .replace((cancel_sender, handle));

    // Wait for the task to complete
    match completion_receiver.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(err.to_string()),
        Err(_) => Err("Task completion channel was dropped".into()),
    }
}
