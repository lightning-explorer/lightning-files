use std::{
    collections::HashSet,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::tantivy_file_indexer::{
    converters::date_converter::unix_time_to_tantivy_datetime,
    dtos::file_dto_input::FileDTOInput,
    services::{
        local_db::{
            service::LocalDbService,
            tables::files::{self},
        },
        vector_db::workers::indexer::VectorDbIndexer,
    },
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerReceiver,
};
use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::sync::{Mutex, Notify};

pub async fn worker_task<T>(
    mut receiver: T,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<LocalDbService>,
    vector_db_indexer: Arc<VectorDbIndexer>,
    notify: Arc<Notify>,
    batch_size: usize,
) where
    T: FileIndexerReceiver,
{
    // Keep track of how many files (not directories) have been indexed so that the changes can be committed
    let mut files_processed = 0;

    loop {
        if let Some(model) = receiver.recv().await {
            #[cfg(feature = "index_worker_logs")]
            println!("Index worker received File Input Model");

            let seen_paths: HashSet<String> =
                model.dtos.iter().map(|x| x.file_path.clone()).collect();
            let stored_paths = get_stored_paths(&db_service, &model.directory_from)
                .await
                .expect("Failed to get stored paths");
            let stale_paths = get_stale_paths(seen_paths, stored_paths);

            #[cfg(feature = "speed_profile")]
            let time = Instant::now();

            // Ensure that the vector database gets updated
            let indexing_op_handle = vector_db_indexer.index_files(&model, &stale_paths);

            #[cfg(feature = "speed_profile")]
            println!(
                "Search Index Worker: Vector Db Indexer index files operation took {:?}",
                time.elapsed()
            );

            files_processed += model.dtos.len();

            if let Err(err) = process_files(
                model.dtos,
                Arc::clone(&writer),
                Arc::clone(&schema),
                Arc::clone(&db_service),
            )
            .await
            {
                println!("Error processing files: {}", err)
            }

            if let Err(err) =
                remove_unseen_entries(stale_paths, Arc::clone(&writer), &schema, &db_service).await
            {
                println!("Error removing stale entries: {}", err);
            }

            if files_processed >= batch_size {
                if let Err(err) = commit_and_retry(Arc::clone(&writer)).await {
                    println!("Error committing files: {}", err);
                }
                files_processed = 0;
            }
        } else {
            // The indexer queue is empty. Wait for more entries
            println!("Index worker has nothing to do. Waiting for notification");
            notify.notified().await;
            println!("Index worker received notification. Resuming work");
        }
    }
}

// THIS one is the bottleneck
async fn process_files(
    dtos: Vec<FileDTOInput>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<LocalDbService>,
) -> Result<(), String> {
    let writer = writer.lock().await;
    // Remove from index and add document within a single lock

    let mut db_file_models: Vec<files::entities::file::Model> = Vec::new();

    // Only used in speed_profile feature
    let time = Instant::now();
    let num_of_dtos = dtos.len();
    //

    for dto in dtos.into_iter() {
        writer.delete_term(tantivy::Term::from_field_text(
            schema
                .get_field("file_id")
                .map_err(|x| format!("Field doesn't exist: {}", x))?,
            &dto.file_path,
        ));
        writer.add_document(doc! {
        schema.get_field("file_id").unwrap() => dto.file_id,
        schema.get_field("name").unwrap() => dto.name,
        schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
        schema.get_field("path").unwrap() => dto.file_path.clone(),
        schema.get_field("metadata").unwrap() => dto.metadata,
        schema.get_field("popularity").unwrap() => dto.popularity,
        }).map_err(|x| format!("Failed to add document: {}",x))?;

        // Create model for DTO but dont add it to DB
        let path_clone = dto.file_path.clone();
        let parent_path = get_parent_path(path_clone);
        let file_model = files::entities::file::Model {
            path: dto.file_path,
            parent_path,
        };
        db_file_models.push(file_model);
    }
    #[cfg(feature = "speed_profile")]
    println!(
        "Index Worker - Process Files: Tantivy writer adding {} entries took {:?}",
        num_of_dtos,
        time.elapsed()
    );

    #[cfg(feature = "speed_profile")]
    let time = Instant::now();

    if let Err(err) = db_service.files_table_connection().upsert_many(&db_file_models).await {
        return Err(format!("Error upserting file models: {}", err));
    }

    #[cfg(feature = "speed_profile")]
    println!(
        "Index Worker - Process Files: DB upsert took {:?}",
        time.elapsed()
    );

    Ok(())
}

fn get_stale_paths(seen_paths: HashSet<String>, stored_paths: HashSet<String>) -> HashSet<String> {
    stored_paths.difference(&seen_paths).cloned().collect()
}

async fn remove_unseen_entries(
    stale_paths: HashSet<String>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
    db_service: &LocalDbService,
) -> Result<usize, String> {
    if let Err(err) = remove_files_from_index(&stale_paths, writer.clone(), schema).await {
        return Err(err.to_string());
    }
    if let Err(err) = db_service.files_table_connection().remove_paths(&stale_paths).await {
        return Err(err.to_string());
    }

    Ok(stale_paths.len())
}

async fn get_stored_paths(
    db_service: &LocalDbService,
    directory: &Path,
) -> Result<HashSet<String>, String> {
    db_service
        .files_table_connection()
        .get_paths_from_dir(&directory.to_string_lossy())
        .await
        .map_err(|e| e.to_string())
}

async fn commit_and_retry(writer: Arc<Mutex<IndexWriter>>) -> Result<(), TantivyError> {
    let retry_attempts = 3;

    for attempt in 1..=retry_attempts {
        match writer.lock().await.commit() {
            Ok(_) => return Ok(()),
            Err(e) if attempt < retry_attempts => {
                eprintln!("Commit failed (attempt {}), retrying: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

async fn remove_files_from_index<T, S>(
    file_paths: T,
    writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
) -> tantivy::Result<()>
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let field = schema.get_field("file_id")?;
    let writer = writer.lock().await;
    for path in file_paths {
        writer.delete_term(tantivy::Term::from_field_text(field, path.as_ref()));
    }

    Ok(())
}

fn get_parent_path(path: String) -> Option<String> {
    Path::new(&path)
        .parent()
        .map(|val| val.to_string_lossy().to_string())
}