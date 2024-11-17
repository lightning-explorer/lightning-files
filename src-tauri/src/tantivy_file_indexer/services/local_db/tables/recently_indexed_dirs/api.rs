use sea_orm::DatabaseConnection;

use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;

use super::entities::recently_indexed_dir;

pub struct RecentlyIndexedDirectoriesTable {
    db: DatabaseConnection,
}

impl RecentlyIndexedDirectoriesTable {
    pub async fn new_async(db: DatabaseConnection) -> Self {
        generate_table_lenient(&db, recently_indexed_dir::Entity).await;

        Self { db }
    }

    pub async fn insert(&self, model: recently_indexed_dir::Model) {
        todo!();
    }

    pub async fn check_if_recent(&self, dir_path: String) -> bool {
        todo!();
        self.refresh().await;
        // refresh, then check
    }

    async fn refresh(&self) {}
}
