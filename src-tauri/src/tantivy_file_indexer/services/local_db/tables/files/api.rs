use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;
use std::collections::HashSet;

use super::entities::file::{self};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set};

pub struct FilesTable {
    db: DatabaseConnection,
}

impl FilesTable {
    pub async fn new_async(db: DatabaseConnection) -> Self {
        generate_table_lenient(&db, file::Entity).await;

        Self { db }
    }

    pub async fn upsert_many(&self, models: &[file::Model]) -> Result<(), sea_orm::DbErr> {
        let new_files: Vec<file::ActiveModel> = models
            .iter()
            .map(|model| file::ActiveModel {
                path: Set(model.path.to_owned()),
                parent_path: Set(model.parent_path.to_owned()),
            })
            .collect();

        file::Entity::insert_many(new_files).exec(&self.db).await?;
        Ok(())
    }

    pub async fn remove_paths<'a, I, S>(&self, paths: I) -> Result<(), sea_orm::DbErr>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + 'a,
    {
        let paths: Vec<String> = paths.into_iter().map(|p| p.as_ref().to_string()).collect();

        if paths.is_empty() {
            return Ok(());
        }

        file::Entity::delete_many()
            .filter(file::Column::Path.is_in(paths))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_paths_from_dir(&self, dir: &str) -> Result<HashSet<String>, sea_orm::DbErr> {
        let files = file::Entity::find()
            .filter(file::Column::ParentPath.eq(dir))
            .all(&self.db)
            .await?;
        let set: HashSet<String> = files.into_iter().map(|x| x.path.to_string()).collect();
        Ok(set)
    }

    pub async fn count_files(&self) -> Result<u64, sea_orm::DbErr> {
        let count = file::Entity::find().count(&self.db).await?;
        Ok(count)
    }
}
