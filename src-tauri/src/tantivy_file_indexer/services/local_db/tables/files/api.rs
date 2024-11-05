use std::{borrow::Cow, collections::HashSet, sync::Arc};

use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

use super::models::FileModel;

type RowsAffected = u64;
pub struct FilesTable {
    pool: Arc<Mutex<Pool<Sqlite>>>,
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

    pub async fn upsert(&self, model: &FileModel) -> Result<(), sqlx::Error> {
        let pool = self.pool.lock().await;
        sqlx::query("INSERT OR IGNORE INTO files (path) VALUES (?)")
            .bind(&model.path)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    pub async fn remove_paths<'a, I, S>(&self, paths: I) -> Result<RowsAffected, sqlx::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + 'a,
    {
        let paths: Vec<Cow<'a, str>> = paths
            .into_iter()
            .map(|p| Cow::from(p.as_ref().to_string()))
            .collect();

        if paths.is_empty() {
            return Ok(0);
        }

        let pool = self.pool.lock().await;
        let placeholders = paths.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!("DELETE FROM files WHERE path IN ({})", placeholders);

        let mut query_builder = sqlx::query(&query);
        for path in &paths {
            query_builder = query_builder.bind(path);
        }

        let result = query_builder.execute(&*pool).await?;
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
