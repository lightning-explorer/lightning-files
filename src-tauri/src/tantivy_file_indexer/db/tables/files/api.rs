use std::{collections::HashSet, sync::Arc};

use sqlx::{Pool, Sqlite};

use super::models::FileModel;

pub struct FilesTable {
    pool: Arc<Pool<Sqlite>>,
}

impl FilesTable {

    pub async fn new_async(pool: Arc<Pool<Sqlite>>) -> Self {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                    path TEXT PRIMARY KEY,
                    parent_path TEXT,
                    FOREIGN KEY (parent_path) REFERENCES files(path)
                ) WITHOUT ROWID;", // last_modified INTEGER NOT NULL
        )
        .execute(&*pool)
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn upsert(
        &self,
        model: &FileModel, // Accepts a reference to a FileRecord
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO files (path) VALUES (?)")
            .bind(&model.path)
            .execute(&*self.pool.as_ref())
            .await?;
        Ok(())
    }

    /**
     * Returns the number of rows that were affected
     */
    pub async fn remove_path(&self, path: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM files WHERE path = ?")
            .bind(path)
            .execute(&*self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_paths_from_dir(&self, dir: &str) -> Result<HashSet<String>, sqlx::Error> {
        let rows = sqlx::query_as::<_, FileModel>("SELECT * FROM files WHERE parent_path = ?")
            .bind(dir)
            .fetch_all(&*self.pool.as_ref())
            .await?;
        let set:HashSet<String> = rows.into_iter().map(|x|x.path.to_string()).collect();
        Ok(set)
    }
}
