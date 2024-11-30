use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::{
    converters::date_converter::unix_time_to_tantivy_datetime, dtos::file_dto_input::FileDTOInput,
    shared::indexing_crawler::{models::file_model::FileModel, traits::files_collection_api::FilesCollectionApi},
};

use super::worker_manager::TantivyInput;

pub async fn index_files<F>(
    files: &Vec<FileDTOInput>,
    tantivy: TantivyInput,
    parent_path: PathBuf,
    files_collection: Arc<F>,
) -> Result<(), String>
where
    F: FilesCollectionApi,
{
    let (writer, schema) = tantivy;
    let writer_clone = Arc::clone(&writer);
    let files_collection_clone = Arc::clone(&files_collection);

    let seen_paths: HashSet<String> = files.iter().map(|x| x.file_path.clone()).collect();
    let stored_paths = files_collection
        .get_stored_paths(&parent_path)
        .await
        .expect("Failed to get stored paths");
    let stale_paths: HashSet<String> = stored_paths.difference(&seen_paths).cloned().collect();

    process_files_and_commit(files, writer, schema.clone(), files_collection).await?;

    remove_unseen_entries(stale_paths, writer_clone, schema, files_collection_clone).await?;

    Ok(())
}

async fn process_files_and_commit<F>(
    dtos: &Vec<FileDTOInput>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    files_collection: Arc<F>,
) -> Result<(), String>
where
    F: FilesCollectionApi,
{
    let writer_lock = writer.lock().await;
    // Remove from index and add document within a single lock
    let mut db_file_models = Vec::new();

    for dto in dtos.into_iter() {
        // Use the name field as the primary key
        writer_lock.delete_term(tantivy::Term::from_field_text(
            schema
                .get_field("name")
                .map_err(|x| format!("Field doesn't exist: {}", x))?,
            &dto.file_path,
        ));
        writer_lock.add_document(doc! {
        //schema.get_field("file_id").unwrap() => dto.file_id,
        schema.get_field("name").unwrap() => dto.name.clone(),
        schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
        schema.get_field("path").unwrap() => dto.file_path.clone(),
        schema.get_field("metadata").unwrap() => dto.metadata.clone(),
        schema.get_field("popularity").unwrap() => dto.popularity,
        }).map_err(|x| format!("Failed to add document: {}",x))?;

        // Create model for DTO but dont add it to DB
        let path_clone = dto.file_path.clone();
        let parent_path = get_parent_path(path_clone);
        let file_model = FileModel {
            path: dto.file_path.clone(),
            parent_path,
        };
        db_file_models.push(file_model);
    }

    if let Err(err) = commit_and_retry(Arc::clone(&writer)).await {
        return Err(format!("Error committing files to Tantivy index: {}", err));
    }

    if let Err(err) = files_collection.upsert_many(db_file_models).await {
        return Err(format!("Error upserting file models: {}", err));
    }

    Ok(())
}

async fn remove_unseen_entries<F>(
    stale_paths: HashSet<String>,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    files_collection: Arc<F>,
) -> Result<usize, String>
where
    F: FilesCollectionApi,
{
    if let Err(err) = remove_files_from_index(&stale_paths, writer.clone(), &schema).await {
        return Err(err.to_string());
    }
    if let Err(err) = files_collection.remove_paths(&stale_paths).await {
        return Err(err.to_string());
    }

    Ok(stale_paths.len())
}

fn get_parent_path(path: String) -> Option<String> {
    Path::new(&path)
        .parent()
        .map(|val| val.to_string_lossy().to_string())
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
    let field = schema.get_field("name")?;
    let writer = writer.lock().await;
    for path in file_paths {
        writer.delete_term(tantivy::Term::from_field_text(field, path.as_ref()));
    }

    Ok(())
}
