use std::{fs, path::PathBuf};
use tantivy::{schema::Schema, Index, IndexReader, IndexWriter};
use crate::tantivy_file_indexer::schemas::file_schema;

/**
 Creates the Tantivy index at the given directory
 */
pub fn initialize_tantity(buffer_size:usize, index_path:PathBuf)->(Schema,IndexReader,IndexWriter){

    let schema = file_schema::create_schema();
    // Create the Tantivy index
    let index = if index_path.exists() {
        // If the index directory exists, open the existing index
        println!("Opening existing index at {:?}", index_path);
        Index::open_in_dir(index_path)
    } else {
        // If the index directory doesn't exist, create a new index
        println!("Creating a new index at {:?}", index_path);
        fs::create_dir_all(index_path.clone()).expect("could not create output directory");
        Index::create_in_dir(index_path, schema.clone())
    };
    let index = index.unwrap();
    let index_writer: IndexWriter = index.writer(buffer_size).unwrap();

    let index_reader = index.reader().unwrap();
    (schema,index_reader,index_writer)
}