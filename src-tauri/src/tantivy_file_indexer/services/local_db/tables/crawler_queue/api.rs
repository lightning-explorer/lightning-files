/*
Right now, the file crawler queue stores itself to local JSON, which works for now, but ideally it should grab its information from the
SQLite database and store stuff there.
*/

/*
 * Note that the crawler queue contains the stuff TO be indexed while the recently indexed directories table contains all of the folders
 * that have ALREADY been indexed
 */

use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
};
use sqlx::{Sqlite, Transaction};

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

    pub async fn upsert_many(&self, models: &[indexed_dir::Model]) -> Result<(), sqlx::Error> {
        // Start a transaction
        let mut transaction: Transaction<'_, Sqlite> =
            self.db.get_sqlite_connection_pool().begin().await?;

        // Raw SQL is needed because SQLite is picky about on conflict operations
        // Prepare raw SQL for upsert
        let query = r#"
            INSERT INTO indexed (path, priority)
            VALUES (?, ?)
            ON CONFLICT(path) DO UPDATE SET
                priority = excluded.priority;
        "#;

        // Execute the query for each model
        for model in models {
            sqlx::query(query)
                .bind(&model.path)
                .bind(&model.priority)
                .execute(&mut *transaction)
                .await?;
        }

        // Commit the transaction
        transaction.commit().await?;
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
            .order_by_asc(indexed_dir::Column::Priority)
            // Order by ascending to ensure that lower numbers are nearer to the top (The lower the number, the higher the priority)
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

    pub async fn view_all(&self) -> Result<Vec<indexed_dir::Model>, sea_orm::DbErr> {
        indexed_dir::Entity::find().all(&self.db).await
    }
}
