use std::{borrow::Cow, collections::HashSet, sync::Arc};

use super::entities::recently_indexed_dir_model::RecentlyIndexedDirModel;
use sqlx::{Pool, Sqlite};
type RowsAffected = u64;
pub struct RecentlyIndexedDirectoriesTable {
    pool: Arc<Pool<Sqlite>>,
}

impl RecentlyIndexedDirectoriesTable {
    pub async fn new_async(pool: Arc<Pool<Sqlite>>) -> Self {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS recent_indexed (
                    path TEXT PRIMARY KEY,
                    indexed_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
                ) WITHOUT ROWID;", // last_modified INTEGER NOT NULL
        )
        .execute(&*pool)
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn insert(&self, model: RecentlyIndexedDirModel) {
        todo!();
    }

    pub async fn check_if_recent(&self, dir_path: String) -> bool {
        todo!();
        self.refresh();
        // refresh, then check
    }

    async fn refresh(&self) {}
}
