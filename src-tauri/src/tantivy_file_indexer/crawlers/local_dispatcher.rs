use std::{sync::Arc, time::Duration};

use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::sync::{Mutex, Semaphore};

use crate::tantivy_file_indexer::converters::date_converter::unix_time_to_tantivy_datetime;

use super::dir_walker;

pub async fn spawn_crawler(
    directory: &str,
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    batch_size: usize,
    max_concurrent_tasks:usize
) {
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut walker = dir_walker::DirWalker::new(directory);
    let mut tasks = vec![];

    while let Some(dto) = walker.next() {
        // Clone references for each async task
        let index_writer_clone = index_writer.clone();
        let schema = schema.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        // Spawn a new async task to handle the file processing
        let task = tokio::task::spawn(async move {
            // Permit is moved in here so it will be dropped when the scope ends
            let _permit = permit;
            // Remove file from index if it exists
            
            if let Err(err) = remove_file_from_index(index_writer_clone.clone(), &schema, &dto.file_path).await {
                eprintln!("Error when trying to remove file: {}", err);
            }

            // Lock the writer only for adding the document
            {
                let writer = index_writer_clone.lock().await;

                writer
                    .add_document(doc! {
                        schema.get_field("file_id").unwrap() => dto.file_id,
                        schema.get_field("name").unwrap() => dto.name,
                        schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
                        schema.get_field("path").unwrap() => dto.file_path,
                        schema.get_field("metadata").unwrap() => dto.metadata,
                        schema.get_field("popularity").unwrap() => dto.popularity,
                    })
                    .unwrap();
            }
        });

        tasks.push(task);

        // Commit the batch when reaching the batch_size
        if tasks.len() >= batch_size {
            // Await all tasks in the current batch
            for task in tasks.drain(..) {
                task.await.unwrap();
            }

            // Commit the writer asynchronously
            if let Err(err) = commit_and_retry(index_writer.clone()).await {
                eprintln!("Error committing batch: {}", err);
            }
        }
    }

    // Process any remaining tasks
    for task in tasks {
        task.await.unwrap();
    }

    // Final commit if there are remaining documents
    if let Err(err) = commit_and_retry(index_writer.clone()).await {
        eprintln!("Final writer commit attempt failed: {}", err);
    }
}

async fn commit_and_retry(writer: Arc<Mutex<IndexWriter>>) -> Result<(), TantivyError> {
    let retry_attempts = 3;
    for attempt in 1..=retry_attempts {
        match writer.lock().await.commit() {
            Ok(_) => break, // Success, exit the loop
            Err(e) if attempt < retry_attempts => {
                println!("Commit failed on attempt {}, retrying: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(500)).await; // Add delay
            }
            Err(e) => {
                println!("Commit failed after {} attempts: {:?}", retry_attempts, e);
                return Err(e);
            }
        }
    }
    println!("writer commit");
    return Ok(());
}

async fn remove_file_from_index(
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
    file_path: &str,
) -> tantivy::Result<()> {
    let index_writer = index_writer.lock().await;
    match schema.get_field("file_id") {
        Ok(field) => {
            index_writer.delete_term(tantivy::Term::from_field_text(field, file_path));
            Ok(())
        }
        Err(e) => Err(e),
    }
}
