/*
Right now, the file crawler queue stores itself to local JSON, which works for now, but ideally it should grab its information from the
SQLite database and store stuff there.
*/

/*
 * Note that the crawler queue contains the stuff TO be indexed while the recently indexed directories table contains all of the folders
 * that have ALREADY been indexed
 */

use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait
};

use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;

use super::entities::indexed_dir;

pub struct CrawlerQueueTable {
    db: DatabaseConnection,
}

impl CrawlerQueueTable {
    pub async fn new_async(db: DatabaseConnection) -> Self {
        generate_table_lenient(&db, indexed_dir::Entity).await;

        Self { db }
    }

    pub async fn upsert_many(&self, models: &[indexed_dir::Model]) -> Result<(), sea_orm::DbErr> {
        //println!("debug models: {:?}", models);

        let entries: Vec<indexed_dir::ActiveModel> = models
            .iter()
            .map(|model| indexed_dir::ActiveModel {
                path: Set(model.path.to_owned()),
                priority: Set(model.priority.to_owned()),
            })
            .collect();
        let entries_dbg = format!("{:?}", entries);

        //println!("debug entries: {:?}", entries);

        // To get rid of all this error handling, just add ? after the await and remove the rest
        indexed_dir::Entity::insert_many(entries)
            .on_conflict(
                // Allow upserts
                OnConflict::column(indexed_dir::Column::Path)
                    .update_columns([indexed_dir::Column::Priority]).to_owned()
            )
            .exec(&self.db)
            .await
            .map_err(|err| {
                sea_orm::DbErr::Custom(format!(
                    "Error upserting: {}. Here are the models: {:?}. Here are the entries: {:?}",
                    err, models, entries_dbg
                ))
            })?;
        Ok(())
    }

    /**
     * Retrieves the next most popular directory in the collection
     */
    pub async fn pop(&self) -> Result<Option<indexed_dir::Model>, sea_orm::DbErr> {
        // Begin a transaction
        let txn = self.db.begin().await?;

        // Fetch the entry with the highest priority (biggest number)
        if let Some(next_entry) = indexed_dir::Entity::find()
            .order_by_desc(indexed_dir::Column::Priority) // Order by priority descending
            .one(&txn)
            .await?
        {
            // Delete the fetched entry
            indexed_dir::Entity::delete_many()
                .filter(indexed_dir::Column::Path.eq(next_entry.path.clone()))
                .exec(&txn)
                .await?;

            // Commit the transaction
            txn.commit().await?;

            // Return the fetched entry
            Ok(Some(next_entry))
        } else {
            // No entry found, roll back the transaction
            txn.rollback().await?;
            Ok(None)
        }
    }

    pub async fn count_dirs(&self) -> Result<u64, sea_orm::DbErr> {
        let count = indexed_dir::Entity::find().count(&self.db).await?;
        Ok(count)
    }
}
