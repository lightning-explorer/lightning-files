use super::{super::app_data, tables::files::api::FilesTable};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::sync::Arc;

pub struct SqlxService {
    pool: Arc<Pool<Sqlite>>,
    pub files_table: FilesTable,
}

impl SqlxService {
    // consider using a config here
    pub async fn new_async() -> Self {
        let db_path = app_data::helper_methods::create_path("file_index.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        let pool = Arc::new(SqlitePool::connect(&db_url).await.unwrap());
        let files_table = FilesTable::new_async(pool.clone()).await;

        Self { pool, files_table }
    }
    /**
     * Runs a command on the database to reclaim unused memory
     */
    pub async fn vacuum(&self) -> Result<(), String> {
        match sqlx::query("VACUUM").execute(&*self.pool.as_ref()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
