// Treat tantivy as the database for storing files

use std::{collections::HashSet, sync::Arc, time::Duration};

use tantivy::{
    collector::TopDocs,
    doc,
    query::{BooleanQuery, Occur, Query, QueryParser},
    schema::{OwnedValue, Schema},
    Document, IndexReader, IndexWriter, TantivyDocument,
};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::{
    converters::date_converter::chrono_time_to_tantivy_datetime,
    shared::indexing_crawler::traits::files_collection_api::FilesCollectionApi,
};

/// If a document is added to the file collection, that means it got indexed and is searchable
#[derive(Clone)]
pub struct TantivyFilesCollection {
    writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    index_reader: Arc<IndexReader>,
}

impl TantivyFilesCollection {
    pub fn new(
        writer: Arc<Mutex<IndexWriter>>,
        schema: Schema,
        index_reader: Arc<IndexReader>,
    ) -> Self {
        Self {
            writer,
            schema,
            index_reader,
        }
    }
}

impl FilesCollectionApi for TantivyFilesCollection {
    type Error = String;

    /// Get all of the paths that belong to the specified directory
    fn get_stored_paths(
        &self,
        directory: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<std::collections::HashSet<String>, Self::Error>> + Send
    {
        let dir_path_str = directory.to_string_lossy();
        let searcher = self.index_reader.searcher();
        let schema = self.schema.clone();
        let mut filtered_paths = HashSet::new();
        async move {
            let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

            let field = schema.get_field("path").unwrap();
            let query_parser = QueryParser::for_index(searcher.index(), vec![field]);
            let query = query_parser
                .parse_query(&dir_path_str)
                .map_err(|err| err.to_string())?;
            queries.push((Occur::Should, Box::new(query)));

            // Combine all the queries. (Even though there is just one)
            let boolean_query = BooleanQuery::new(queries);

            let top_docs = searcher
                .search(&boolean_query, &TopDocs::with_limit(1_000_000_000))
                .map_err(|err| err.to_string())?;

            // This is a vector of all of the documents whose file path includes the parent path
            // Each entry in the vec is the file path of the document
            let paths: Vec<String> = top_docs
                .into_iter()
                .map(|(_score, doc_address)| {
                    let doc: TantivyDocument = searcher.doc(doc_address).unwrap();
                    // If this fails, it means the path doesn't exist, which is problematic
                    extract_str_field_from_doc(doc, "path", &schema).unwrap()
                })
                .collect();

            for path in paths.into_iter() {
                // If the file path of the document is the actual directory we were searching for, then remove it
                if path != dir_path_str {
                    filtered_paths.insert(path);
                }
            }

            // TODO: remove debug print:
            println!("Get stored paths for tantivy returned {} entries for given path: {}", filtered_paths.len(), dir_path_str);

            Ok(filtered_paths)
        }
    }

    fn upsert_many(
        &self,
        models: &[crate::tantivy_file_indexer::models::interal_system_file::InternalSystemFileModel],
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let writer_clone = Arc::clone(&self.writer);
        let schema = self.schema.clone();

        async move {
            let writer_lock = writer_clone.lock().await;

            for model in models.iter() {
                // Use the path field as the primary key
                writer_lock.delete_term(tantivy::Term::from_field_text(
                    schema
                        .get_field("path")
                        .map_err(|x| format!("Field doesn't exist: {}", x))?,
                    &model.file_path,
                ));
                writer_lock
                .add_document(doc! {
                //schema.get_field("file_id").unwrap() => model.file_id, // UNUSED SCHEMA FIELD
                schema.get_field("name").unwrap() => model.name.clone(),
                schema.get_field("date_modified").unwrap() => chrono_time_to_tantivy_datetime(model.date_modified), 
                schema.get_field("date_created").unwrap() => chrono_time_to_tantivy_datetime(model.date_created), 
                schema.get_field("path").unwrap() => model.file_path.clone(),
                schema.get_field("metadata").unwrap() => model.metadata.clone(),
                schema.get_field("popularity").unwrap() => model.popularity,
                })
                .map_err(|x| format!("Failed to add document: {}", x))?;
            }
            Ok(())
        }
    }

    fn remove_paths(
        &self,
        paths: &std::collections::HashSet<String>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let paths_clone = paths.clone();
        let writer_clone = Arc::clone(&self.writer);
        let schema = self.schema.clone();
        async move {
            let writer_lock = writer_clone.lock().await;
            for path in paths_clone.into_iter() {
                writer_lock.delete_term(tantivy::Term::from_field_text(
                    schema
                        .get_field("path")
                        .map_err(|x| format!("Field doesn't exist: {}", x))?,
                    &path,
                ));
            }
            Ok(())
        }
    }

    /// If an error occurs, it will attempt to commit the documents two more times before returning an error
    fn commit_all(&self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let writer_clone = Arc::clone(&self.writer);
        async move {
            let retry_attempts = 3;

            for attempt in 1..=retry_attempts {
                match writer_clone.lock().await.commit() {
                    Ok(_) => return Ok(()),
                    Err(e) if attempt < retry_attempts => {
                        eprintln!("Commit failed (attempt {}), retrying: {:?}", attempt, e);
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            Ok(())
        }
    }
}

fn extract_str_field_from_doc(
    doc: TantivyDocument,
    field_name: &str,
    schema: &Schema,
) -> Result<String, String> {
    for (field, value) in doc.iter_fields_and_values() {
        let name = schema.get_field_name(field);
        if name == field_name {
            if let OwnedValue::Str(text) = value {
                return Ok(text.clone());
            }
        }
    }
    Err(format!(
        "Field with name {} does not exist in document",
        field_name
    ))
}
