use super::{configs::file_indexer_config::FileIndexerConfig, crawlers::local_dispatcher, schemas::file_schema};
use dirs::data_dir;
use tantivy::{schema::Schema, Index, IndexReader, IndexWriter};
use tokio::sync::Mutex;
use std::{fs, sync::Arc};

pub struct SearchIndexService {
    schema: Schema,
    index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: IndexReader,
    config: FileIndexerConfig,
}

impl SearchIndexService {
    pub fn new(config: &FileIndexerConfig) -> Self {

    let app_data = data_dir().expect("Could not find AppData directory");
    let app_path = app_data.join("DesktopSearch");
    let index_path = app_path.join("TantivyOut");

    let schema = file_schema::create_schema();
    // Ensure that the App's AppData directory is there
    if !app_path.exists(){
        fs::create_dir_all("DesktopSearch").expect("could not create DesktopSearch directory");
    }
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
        let index_writer = index.writer(config.buffer_size).unwrap();

        let index_reader = index.reader().unwrap();

        Self {
            config: config.clone(),
            schema,
            index_writer: Arc::new(Mutex::new(index_writer)),
            index_reader,
        }
    }

    pub fn spawn_crawler<'a>(&self, directory: String) {
        let index_writer_clone = Arc::clone(&self.index_writer);
        let schema_clone = self.schema.clone();
        let batch_size = self.config.indexer_batch_size;
        let tasks_limit = self.config.indexer_tasks_limit;
    
        tokio::spawn(async move {
            local_dispatcher::spawn_crawler(&directory, index_writer_clone, schema_clone, batch_size, tasks_limit).await
        });
    }

}