use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use super::super::models::index_worker::file_input::FileInputModel;
use crate::tantivy_file_indexer::{
    converters::date_converter::unix_time_to_tantivy_datetime,
    dtos::file_dto_input::FileDTOInput,
    services::local_db::{service::SqlxService, tables::files::models::FileModel},
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
    db_service: Arc<SqlxService>,
    batch_size: usize,
) {
    let mut batches_processed: usize = 0;
    // Each call to 'next' will return every file/directory path as a DTO
    while let Some(model) = receiver.recv().await {
        let seen_paths: HashSet<String> = model.dtos.iter().map(|x| x.file_path.clone()).collect();

        for dto in model.dtos.into_iter() {
            if let Err(err) =
                process_file(dto, writer.clone(), schema.clone(), db_service.clone()).await
            {
                println!("Error processing file:{}", err)
            }
        }
        batches_processed += 1;

        if let Err(err) = remove_unseen_entries(
            model.directory_from,
            seen_paths,
            writer.clone(),
            &schema,
            &db_service,
        )
        .await
        {
            println!("Error removing stale entries: {}", err);
        }

        if batches_processed >= batch_size {
            if let Err(err) = commit_and_retry(writer.clone()).await {
                println!("Error committing files: {}", err);
            }
            batches_processed = 0;
        }
    }
    println!("receiver channel closed");
}

async fn process_file(
    dto: FileDTOInput,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<SqlxService>,
) -> Result<(), std::io::Error> {
    // Attempt to remove file from index
    if let Err(err) = remove_file_from_index(&dto.file_path, writer.clone(), &schema).await {
        eprintln!("Error removing file from index: {}", err);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Index removal failed",
        ));
    }
    let writer = writer.lock().await;
    // Write the document to the index
    {
        if let Err(err) = writer.add_document(doc! {
                schema.get_field("file_id").unwrap() => dto.file_id,
                schema.get_field("name").unwrap() => dto.name,
                schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
                schema.get_field("path").unwrap() => dto.file_path.clone(),
                schema.get_field("metadata").unwrap() => dto.metadata,
                schema.get_field("popularity").unwrap() => dto.popularity,
            }) {
                eprintln!("Error adding document to index: {}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Document addition failed"));
            }
    }

    // Update the database
    let path_clone = dto.file_path.clone();
    let parent_path = get_parent_path(path_clone);
    let file_model = FileModel {
        path: dto.file_path,
        parent_path,
    };
    if let Err(err) = db_service.files_table.upsert(&file_model).await {
        eprintln!("Error upserting file model: {}", err);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "DB upsert failed",
        ));
    }

    Ok(())
}

async fn remove_unseen_entries(
    directory: PathBuf,
    seen_paths: HashSet<String>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
    db_service: &SqlxService,
) -> Result<usize, String> {
    let stored_paths = db_service
        .files_table
        .get_paths_from_dir(&directory.to_string_lossy())
        .await
        .map_err(|e| e.to_string())?;

    let stale_paths: HashSet<_> = stored_paths.difference(&seen_paths).cloned().collect();
    let stale_paths_len = stale_paths.len();

    for path in stale_paths {
        if let Err(err) = remove_file_from_index(&path, writer.clone(), schema).await {
            return Err(err.to_string());
        }
        if let Err(err) = db_service.files_table.remove_path(&path).await {
            return Err(err.to_string());
        }
    }
    Ok(stale_paths_len)
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

async fn remove_file_from_index(
    file_path: &str,
    writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
) -> tantivy::Result<()> {
    let field = schema.get_field("file_id")?;
    let writer = writer.lock().await;
    writer.delete_term(tantivy::Term::from_field_text(field, file_path));
    Ok(())
}

fn get_parent_path(path: String) -> Option<String> {
    Path::new(&path)
        .parent()
        .map(|val| val.to_string_lossy().to_string())
}
