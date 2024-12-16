use std::{path::PathBuf, sync::Arc, time::Duration};

use tantivy::{IndexWriter, TantivyError};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::{
    models::internal_system_file,
    shared::{
        indexing_crawler::traits::files_collection_api::FilesCollectionApi,
        search_index::tantivy_traits::{self, Model, ToTantivyModel},
    },
};

/// The files also get committed to the Tantivy index and database at the end of this function
pub async fn index_files<F>(
    files: &[internal_system_file::model::Model],
    writer: Arc<Mutex<IndexWriter>>,
    parent_path: PathBuf,
    files_collection: Arc<F>,
) -> Result<(), String>
where
    F: FilesCollectionApi,
{
    let writer_clone = Arc::clone(&writer);
    let files_collection_clone = Arc::clone(&files_collection);

    let stored_paths = files_collection
        .get_stored_paths(&parent_path)
        .await
        .expect("Failed to get stored paths");

    let stale_paths: Vec<internal_system_file::model::Model> = files
        .iter()
        .filter(|x| !stored_paths.contains(&x.file_path))
        .map(|x| x.clone())
        .collect();

    process_files_and_commit(files, writer, files_collection).await?;

    remove_unseen_entries(stale_paths, writer_clone, files_collection_clone).await?;

    println!("Files successfully committed to index");
    Ok(())
}

async fn process_files_and_commit<F>(
    files: &[internal_system_file::model::Model],
    writer: Arc<Mutex<IndexWriter>>,
    files_collection: Arc<F>,
) -> Result<(), String>
where
    F: FilesCollectionApi,
{
    {
        let writer_lock = writer.lock().await;

        for file in files.iter() {
            // Use the path field as the primary key
            let tantivy_model = file.clone().to_model();
            writer_lock.delete_term(
                tantivy_model
                    .get_primary_key()
                    .expect("Failed to get primary key from tantivy document"),
            );
            writer_lock
                .add_document(tantivy_model.as_document())
                .map_err(|x| format!("Failed to add document: {}", x))?;
        }
        // Writer lock is dropped here
    }

    // Writer lock must be dropped so this function can use it
    if let Err(err) = commit_and_retry(Arc::clone(&writer)).await {
        return Err(format!("Error committing files to Tantivy index: {}", err));
    }

    if let Err(err) = files_collection.upsert_many(files).await {
        return Err(format!("Error upserting file models: {}", err));
    }

    Ok(())
}

async fn remove_unseen_entries<F>(
    stale_paths: Vec<internal_system_file::model::Model>,
    writer: Arc<Mutex<IndexWriter>>,
    files_collection: Arc<F>,
) -> Result<usize, String>
where
    F: FilesCollectionApi,
{
    // Remove the files from the Tantivy index
    if let Err(err) = remove_files_from_index(
        stale_paths
            .iter()
            .map(|file| file.clone().to_model())
            .collect(),
        writer.clone(),
    )
    .await
    {
        return Err(err.to_string());
    }
    let len = stale_paths.len();
    // Remove the files from the database
    if let Err(err) = files_collection
        .remove_paths(&stale_paths.into_iter().map(|file| file.file_path).collect())
        .await
    {
        return Err(err.to_string());
    }

    Ok(len)
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

async fn remove_files_from_index<M>(
    models: Vec<M>,
    writer: Arc<Mutex<IndexWriter>>,
) -> tantivy::Result<()>
where
    M: tantivy_traits::Model,
{
    let writer = writer.lock().await;
    for model in models.into_iter() {
        writer.delete_term(
            model
                .get_primary_key()
                .expect("Could not find primary key for model"),
        );
    }

    Ok(())
}
