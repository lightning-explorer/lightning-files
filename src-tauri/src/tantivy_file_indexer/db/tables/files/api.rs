use std::{collections::HashSet, sync::Arc};

use sqlx::{Pool, Sqlite};

use super::models::FileModel;

pub struct FilesTable {
    pool: Arc<Pool<Sqlite>>,
}

impl FilesTable {
    // Initialize the table with a reference to the pool
    pub async fn new_async(pool: Arc<Pool<Sqlite>>) -> Self {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                path TEXT PRIMARY KEY
            ) WITHOUT ROWID", // last_modified INTEGER NOT NULL
        )
        .execute(&*pool)
        .await
        .unwrap();

        Self { pool }
    }

    // Other methods that operate on the table can take &Pool<Sqlite> as a parameter
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

    pub async fn path_exists(&self, path: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM files WHERE path = ?)")
            .bind(path)
            .fetch_one(&*self.pool.as_ref())
            .await?;
        Ok(result)
    }

    pub async fn remove_path(&self, path: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM files WHERE path = ?")
            .bind(path)
            .execute(&*self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_all_paths(&self) -> Result<HashSet<String>, sqlx::Error> {
        let rows = sqlx::query_as::<_, FileModel>("SELECT path FROM files")
            .fetch_all(&*self.pool.as_ref())
            .await?;
        Ok(rows.into_iter().map(|model| model.path).collect())
    }
}
