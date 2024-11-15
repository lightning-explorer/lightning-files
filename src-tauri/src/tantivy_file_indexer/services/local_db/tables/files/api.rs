use std::{borrow::Cow, collections::HashSet, sync::Arc};

use super::entities::file_model::FileModel;
use sqlx::{Pool, Sqlite};
type RowsAffected = u64;
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

    pub async fn upsert_many(&self, models: &Vec<FileModel>) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        for model in models {
            sqlx::query("INSERT OR IGNORE INTO files (path, parent_path) VALUES (?, ?)")
                .bind(&model.path)
                .bind(&model.parent_path)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await?;
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

        let placeholders = paths.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!("DELETE FROM files WHERE path IN ({})", placeholders);
        let mut query_builder = sqlx::query(&query);
        for path in &paths {
            query_builder = query_builder.bind(path);
        }

        let result = query_builder.execute(&*self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_paths_from_dir(&self, dir: &str) -> Result<HashSet<String>, sqlx::Error> {
        let rows = sqlx::query_as::<_, FileModel>("SELECT * FROM files WHERE parent_path = ?")
            .bind(dir)
            .fetch_all(&*self.pool)
            .await?;
        let set: HashSet<String> = rows.into_iter().map(|x| x.path.to_string()).collect();
        Ok(set)
    }

    pub async fn count_files(&self) -> Result<u64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM files")
            .fetch_one(&*self.pool)
            .await?;
        Ok(row.0 as u64)
    }
}
