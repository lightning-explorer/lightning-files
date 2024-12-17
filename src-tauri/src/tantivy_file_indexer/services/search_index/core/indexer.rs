use std::{sync::Arc, time::Duration};

use tantivy::{IndexWriter, TantivyError};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::shared::search_index::tantivy_traits::{self};

pub async fn add_entries_to_index<M>(
    models: &[M],
    writer: Arc<Mutex<IndexWriter>>,
) -> Result<(), String>
where
    M: tantivy_traits::Model,
{
    let mut writer_lock = writer.lock().await;
    for tantivy_model in models.iter() {
        // Use the path field as the primary key
        remove_entry_from_index(tantivy_model, &writer_lock)
            .await
            .map_err(|err| err.to_string())?;

        writer_lock
            .add_document(tantivy_model.as_document())
            .map_err(|x| format!("Failed to add document: {}", x))?;
    }
    commit(&mut writer_lock, 3)
        .await
        .map_err(|err| err.to_string())?;
    // Writer lock is dropped here
    Ok(())
}

pub async fn remove_entries_from_index<M>(
    models: Vec<M>,
    writer: Arc<Mutex<IndexWriter>>,
) -> tantivy::Result<()>
where
    M: tantivy_traits::Model,
{
    let writer = writer.lock().await;
    for model in models.into_iter() {
        if let Err(err) = remove_entry_from_index(&model, &writer).await {
            println!("error removing file from Tantivy index: {}", err);
        }
    }

    Ok(())
}

/// Commit all staged files and deletions to the index, retrying if there was an error
async fn commit(
    writer_lock: &mut tokio::sync::MutexGuard<'_, IndexWriter>,
    retry_attempts: i32,
) -> Result<(), TantivyError> {
    for attempt in 1..=retry_attempts {
        match writer_lock.commit() {
            Ok(_) => return Ok(()),
            Err(e) if attempt < retry_attempts => {
                eprintln!("Commit failed (attempt {}), retrying: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => return Err(e),
        }
    }
    Err(TantivyError::InternalError(format!(
        "Error committing files to index after trying {} times",
        retry_attempts
    )))
}

async fn remove_entry_from_index<M>(
    model: &M,
    writer_lock: &tokio::sync::MutexGuard<'_, IndexWriter>,
) -> tantivy::Result<()>
where
    M: tantivy_traits::Model,
{
    writer_lock.delete_term(model.get_primary_key());

    Ok(())
}
