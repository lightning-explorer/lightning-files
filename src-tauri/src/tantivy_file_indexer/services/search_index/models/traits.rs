use std::{os::windows::fs::MetadataExt, path::PathBuf};

use tantivy::time::UtcOffset;

use crate::{shared::models::sys_file_model::SystemFileModel, tantivy_file_indexer::converters::date_converter::tantivy_time_to_chrono_datetime};

use super::file::TantivyFileModel;

impl TryFrom<TantivyFileModel> for SystemFileModel {
    type Error = std::io::Error;

    fn try_from(value: TantivyFileModel) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value.file_path.clone());
        let meta = path.metadata()?;
        Ok(SystemFileModel {
            name: value.name,
            file_path: value.file_path,
            date_modified: tantivy_time_to_chrono_datetime(value.date_modified),
            date_created: tantivy_time_to_chrono_datetime(value.date_modified),
            size: meta.file_size(),
            metadata: value.metadata,
            popularity: value.popularity
        })
    }
}
