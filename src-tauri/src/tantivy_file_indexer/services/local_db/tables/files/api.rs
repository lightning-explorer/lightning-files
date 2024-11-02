use std::{collections::HashSet, sync::Arc};

use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

use super::models::FileModel;

type RowsAffected = u64;
pub struct FilesTable {
    pool:  Arc<Mutex<Pool<Sqlite>>>,
}

impl FilesTable {

    pub async fn new_async(pool: Arc<Mutex<Pool<Sqlite>>>) -> Self {
        let pool_clone = pool.clone();
        let pool_locked = pool_clone.lock().await;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                    path TEXT PRIMARY KEY,
                    parent_path TEXT,
                    FOREIGN KEY (parent_path) REFERENCES files(path)
                ) WITHOUT ROWID;", // last_modified INTEGER NOT NULL
        )
        .execute(&*pool_locked)
        .await
        .unwrap();

        Self { pool }
    }

     pub async fn upsert(
        &self,
        model: &FileModel,
    ) -> Result<(), sqlx::Error> {
        let pool = self.pool.lock().await;
        sqlx::query("INSERT OR IGNORE INTO files (path) VALUES (?)")
            .bind(&model.path)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    pub async fn remove_path(&self, path: &str) -> Result<RowsAffected, sqlx::Error> {
        let pool = self.pool.lock().await;
        let result = sqlx::query("DELETE FROM files WHERE path = ?")
            .bind(path)
            .execute(&*pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_paths_from_dir(&self, dir: &str) -> Result<HashSet<String>, sqlx::Error> {
        let pool = self.pool.lock().await;
        let rows = sqlx::query_as::<_, FileModel>("SELECT * FROM files WHERE parent_path = ?")
            .bind(dir)
            .fetch_all(&*pool)
            .await?;
        let set: HashSet<String> = rows.into_iter().map(|x| x.path.to_string()).collect();
        Ok(set)
    }
}
