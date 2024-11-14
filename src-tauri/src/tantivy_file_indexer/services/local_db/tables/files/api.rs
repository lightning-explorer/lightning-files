use super::entities::file_model;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use std::collections::HashSet;

pub struct FilesTable {
    connection: DatabaseConnection,
}

impl FilesTable {
    pub async fn new_async(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn upsert_many(&self, models: &[file_model::Model]) -> Result<(), sea_orm::DbErr> {
        let new_files: Vec<file_model::ActiveModel> = models
            .iter()
            .map(|model| file_model::ActiveModel {
                path: Set(model.path.to_owned()),
                parent_path: Set(model.parent_path.to_owned()),
            })
            .collect();

        // Insert all models in a single batch insert
        file_model::Entity::insert_many(new_files)
            .exec(&self.connection)
            .await?;

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

        file_model::Entity::delete_many()
            .filter(file_model::Column::Path.is_in(paths))
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn get_paths_from_dir(&self, dir: &str) -> Result<HashSet<String>, sea_orm::DbErr> {
        let files = file_model::Entity::find()
            .filter(file_model::Column::ParentPath.eq(dir))
            .all(&self.connection)
            .await?;
        let set: HashSet<String> = files.into_iter().map(|x| x.path.to_string()).collect();
        Ok(set)
    }

    pub async fn count_files(&self) -> Result<u64, sea_orm::DbErr> {
        let count = file_model::Entity::find().count(&self.connection).await?;
        Ok(count)
    }
}
