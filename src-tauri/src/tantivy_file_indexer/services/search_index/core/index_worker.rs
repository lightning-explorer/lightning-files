use std::{
    collections::HashSet,
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use super::super::models::index_worker::file_input::FileInputModel;
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
};
use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::sync::{mpsc, Mutex};

/**
 * waits around for the MPSC channel to send it files to index, in which it will index them
 */
pub async fn spawn_worker(
    mut receiver: mpsc::Receiver<FileInputModel>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<LocalDbService>,
    vector_db_indexer: Arc<VectorDbIndexer>,
    batch_size: usize,
) {
    // Keep track of how many files (not directories) have been indexed so that the changes can be committed
    let files_processed = Arc::new(AtomicUsize::new(0));
    let mut subworker_id: u32 = 0;

    while let Some(model) = receiver.recv().await {
        let seen_paths: HashSet<String> = model.dtos.iter().map(|x| x.file_path.clone()).collect();
        let stored_paths = get_stored_paths(&db_service, &model.directory_from)
            .await
            .expect("Failed to get stored paths");
        let stale_paths = get_stale_paths(seen_paths, stored_paths);

        // Clone Arcs to pass into the threads
        let vector_db_indexer_clone = Arc::clone(&vector_db_indexer);
        let writer_clone = Arc::clone(&writer);
        let schema_clone = Arc::clone(&schema);
        let db_service_clone = Arc::clone(&db_service);
        let files_processed_clone = Arc::clone(&files_processed);

        subworker_id += 1;

        tokio::spawn(async move {
            #[cfg(feature = "index_worker_logs")]
            println!(
                "File index worker subworker has been spawned. ID: {}",
                subworker_id
            );

            #[cfg(feature = "speed_profile")]
            let time = Instant::now();

            // Ensure that the vector database gets updated
            vector_db_indexer_clone
                .index_files(&model, &stale_paths)
                .await;

            #[cfg(feature = "speed_profile")]
            println!(
                "Search Index Worker: Vector Db Indexer index files operation took {:?}",
                time.elapsed()
            );

            let dtos_len = model.dtos.len();
            files_processed_clone.fetch_add(dtos_len, Ordering::Relaxed);

            if let Err(err) = process_files(
                model.dtos,
                Arc::clone(&writer_clone),
                Arc::clone(&schema_clone),
                Arc::clone(&db_service_clone),
            )
            .await
            {
                println!("Error processing files: {}", err)
            }

            if let Err(err) = remove_unseen_entries(
                stale_paths,
                Arc::clone(&writer_clone),
                &schema_clone,
                &db_service_clone,
            )
            .await
            {
                println!("Error removing stale entries: {}", err);
            }

            if files_processed_clone.load(Ordering::Relaxed) >= batch_size {
                if let Err(err) = commit_and_retry(Arc::clone(&writer_clone)).await {
                    println!("Error committing files: {}", err);
                }
                files_processed_clone.store(0, Ordering::Relaxed);
            }
            #[cfg(feature = "index_worker_logs")]
            println!(
                "File index worker subworker has finished. ID: {}",
                subworker_id
            );
        });
    }
    println!("File index worker receiver channel closed");
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

    #[cfg(feature = "speed_profile")]
    {
        let time = Instant::now();
        let num_of_dtos = dtos.len();
    }
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

    if let Err(err) = db_service.files_table().upsert_many(&db_file_models).await {
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
    if let Err(err) = db_service.files_table().remove_paths(&stale_paths).await {
        return Err(err.to_string());
    }

    Ok(stale_paths.len())
}

async fn get_stored_paths(
    db_service: &LocalDbService,
    directory: &Path,
) -> Result<HashSet<String>, String> {
    db_service
        .files_table()
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
