/*
Right now, the file crawler queue stores itself to local JSON, which works for now, but ideally it should grab its information from the
SQLite database and store stuff there.
*/

/*
 * Note that the crawler queue contains the stuff TO be indexed while the recently indexed directories table contains all of the folders
 * that have ALREADY been indexed
 */

use std::{collections::HashMap, sync::Arc};

use sea_orm::{
    prelude::Expr, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};
use sqlx::{Sqlite, Transaction};

use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;

use super::entities::indexed_dir;

#[derive(Clone)]
pub struct CrawlerQueueTable {
    db: Arc<DatabaseConnection>,
}

impl CrawlerQueueTable {
    pub async fn new_async(db: Arc<DatabaseConnection>) -> Self {
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
            INSERT INTO indexed (path, priority, taken)
            VALUES (?, ?, ?)
            ON CONFLICT(path) DO UPDATE SET
                priority = excluded.priority,
                taken = excluded.taken
        "#;

        // Execute the query for each model
        for model in models {
            sqlx::query(query)
                .bind(&model.path)
                .bind(model.priority)
                .bind(model.taken)
                .execute(&mut *transaction)
                .await?;
        }

        // Commit the transaction
        transaction.commit().await?;
        Ok(())
    }

    pub async fn pop(&self) -> Result<Option<indexed_dir::Model>, sea_orm::DbErr> {
        let mut items = self.take_many(1).await?;
        Ok(items.pop())
    }

    /// Completely removes the given models from the database
    pub async fn delete_many(&self, models: &[indexed_dir::Model]) -> Result<(), sea_orm::DbErr> {
        let txn = self.db.begin().await?;

        self.delete_many_txn(&txn, models).await?;

        txn.commit().await?;
        Ok(())
    }

    /// Retrieves the next most popular directories in the collection and removes them
    pub async fn take_many(&self, amount: u64) -> Result<Vec<indexed_dir::Model>, sea_orm::DbErr> {
        let txn = self.db.begin().await?;

        // Fetch the entry with the highest priority (biggest number)
        let next_entries = self.get_next_entries_txn(&txn, amount).await?;

        // Collect the paths of the fetched entries
        self.delete_many_txn(&txn, &next_entries).await?;

        // Commit the transaction
        txn.commit().await?;

        // Return the fetched entries
        Ok(next_entries)
    }

    /// Retrieves the next most popular directories in the collection without removing them
    pub async fn get_many(&self, amount: u64) -> Result<Vec<indexed_dir::Model>, sea_orm::DbErr> {
        let txn: DatabaseTransaction = self.db.begin().await?;

        // Fetch the entries with the highest priority (lowest number)
        let next_entries = self.get_next_entries_txn(&txn, amount).await?;

        self.mark_taken_txn(&txn, &next_entries, true).await?;

        txn.commit().await?;

        Ok(next_entries)
    }

    pub async fn count_dirs(&self) -> Result<u64, sea_orm::DbErr> {
        let count = indexed_dir::Entity::find().count(&*self.db).await?;
        Ok(count)
    }

    /**
    Retrieve the top n entries from the database
    */
    pub async fn view_all_limit(
        &self,
        limit: u64,
    ) -> Result<Vec<indexed_dir::Model>, sea_orm::DbErr> {
        indexed_dir::Entity::find().limit(limit).all(&*self.db).await
    }

    /**
    ### Example output:

    Priority 1: 2 items

    Priority 2: 1 items

    Priority 3: 3 items
     */
    pub async fn get_priority_counts(&self) -> Result<HashMap<u32, i64>, sea_orm::DbErr> {
        use indexed_dir::Entity as IndexedDir;

        let results = IndexedDir::find()
            .select_only() // Only select specific columns
            .column(indexed_dir::Column::Priority) // Select the priority column
            .column_as(Expr::col(indexed_dir::Column::Priority).count(), "count") // Count occurrences
            .group_by(indexed_dir::Column::Priority) // Group by priority
            .into_tuple::<(u32, i64)>() // Convert the result into (priority, count) tuples
            .all(&*self.db)
            .await?;

        // Convert the results into a HashMap for easier use
        let priority_counts = results.into_iter().collect::<HashMap<u32, i64>>();

        Ok(priority_counts)
    }

    pub async fn mark_all_as_not_taken(&self) -> Result<(), sea_orm::DbErr> {
        indexed_dir::Entity::update_many()
            .col_expr(indexed_dir::Column::Taken, true.into())
            .exec(&*self.db)
            .await?;
        Ok(())
    }

    /// Finds the next most popular models from the database that aren't taken
    async fn get_next_entries_txn(
        &self,
        txn: &DatabaseTransaction,
        amount: u64,
    ) -> Result<Vec<indexed_dir::Model>, sea_orm::DbErr> {
        indexed_dir::Entity::find()
            .filter(indexed_dir::Column::Taken.eq(false))
            .order_by_asc(indexed_dir::Column::Priority)
            // Order by ascending to ensure that lower numbers are nearer to the top (The lower the number, the higher the priority)
            .limit(amount)
            .all(txn)
            .await
    }

    /// Example: passing in `is_taken` as true will set the `taken` field in all of the provided models to `true`
    async fn mark_taken_txn(
        &self,
        txn: &DatabaseTransaction,
        models: &[indexed_dir::Model],
        is_taken: bool,
    ) -> Result<sea_orm::UpdateResult, sea_orm::DbErr> {
        let paths_to_modify: Vec<String> = models.iter().map(|entry| entry.path.clone()).collect();

        indexed_dir::Entity::update_many()
            .filter(indexed_dir::Column::Path.is_in(paths_to_modify))
            .col_expr(indexed_dir::Column::Taken, is_taken.into())
            .exec(txn)
            .await
    }

    async fn delete_many_txn(
        &self,
        txn: &DatabaseTransaction,
        models: &[indexed_dir::Model],
    ) -> Result<(), sea_orm::DbErr> {
        let paths_to_delete: Vec<String> = models.iter().map(|entry| entry.path.clone()).collect();

        // Delete the fetched entries using the 'IN' filter
        indexed_dir::Entity::delete_many()
            .filter(indexed_dir::Column::Path.is_in(paths_to_delete))
            .exec(txn)
            .await?;
        Ok(())
    }
}
