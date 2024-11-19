use chrono::Utc;
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

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

    pub async fn upsert_many(
        &self,
        models: &[recently_indexed_dir::Model],
    ) -> Result<(), sea_orm::DbErr> {
        let entries: Vec<recently_indexed_dir::ActiveModel> = models
            .iter()
            .map(|model| recently_indexed_dir::ActiveModel {
                path: Set(model.path.to_owned()),
                time: Set(model.time.to_owned()),
            })
            .collect();

        recently_indexed_dir::Entity::insert_many(entries)
            .on_conflict(
                // Allow upserts
                OnConflict::column(recently_indexed_dir::Column::Path)
                    .update_columns([recently_indexed_dir::Column::Time])
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn contains_dir(&self, dir_path: String) -> Result<bool, sea_orm::DbErr> {
        let exists = recently_indexed_dir::Entity::find()
            .filter(recently_indexed_dir::Column::Path.eq(dir_path))
            .one(&self.db)
            .await?
            .is_some();
        Ok(exists)
    }

    /**
    Returns the number of files that were removed

    `cutoff_time` is a value in minutes
    */
    pub async fn refresh(&self, cutoff_time: i64) -> Result<u64, sea_orm::DbErr> {
        // removes old entries
        // Todo: add more sophisticated logic
        let now = Utc::now().timestamp();

        // Calculate the cutoff time (5 minutes ago)
        let cutoff_time = now - (cutoff_time * 60);

        let delete = recently_indexed_dir::Entity::delete_many()
            .filter(recently_indexed_dir::Column::Time.lt(cutoff_time))
            .exec(&self.db)
            .await?;

        Ok(delete.rows_affected)
    }
}
