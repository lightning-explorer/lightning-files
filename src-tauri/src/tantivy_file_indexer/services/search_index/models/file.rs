use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tantivy::{
    doc,
    schema::{Schema, FAST, INDEXED, STORED, STRING, TEXT},
};

use tantivy::TantivyDocument;

use crate::{
    get_parent_directory,
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::{
        converters::date_converter::{chrono_time_to_tantivy_datetime, tantivy_time_to_chrono_datetime},
        shared::search_index::{
            tantivy_traits::{self, Model},
            util::{field_as_date, field_as_f64, field_as_string},
        },
    },
};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TantivyFileModel {
    pub name: String,
    pub file_path: String,
    pub parent_directory: String,
    pub metadata: String,
    pub date_modified: tantivy::DateTime,
    pub date_created: tantivy::DateTime,
    pub score: f64,
    pub is_directory: bool,
    pub popularity: f64,
}

impl TantivyFileModel {
    pub fn make_term(field: &str, value: &str) -> Result<tantivy::Term, String> {
        let term = tantivy::Term::from_field_text(
            Self::schema()
                .get_field(field)
                .map_err(|x| format!("Field doesn't exist: {}", x))?,
            value,
        );
        Ok(term)
    }
}

impl tantivy_traits::Model for TantivyFileModel {
    type Error = String;

    /// Builds and returns the schema for the model
    fn schema() -> Schema {
        let mut schema_builder = Schema::builder();

        //schema_builder.add_text_field("file_id", TEXT | STORED); // UID
        schema_builder.add_text_field("name", TEXT | STORED);
        schema_builder.add_date_field("date_modified", INDEXED | STORED);
        schema_builder.add_date_field("date_created", INDEXED | STORED);
        schema_builder.add_text_field("metadata", TEXT | STORED);
        // PRIMARY KEY
        schema_builder.add_text_field("path", TEXT | STORED);

        schema_builder.add_text_field("parent_directory", STRING | FAST | STORED);

        schema_builder.add_f64_field("popularity", FAST | STORED);
        schema_builder.build()
    }

    fn get_primary_key(&self) -> Result<tantivy::Term, Self::Error> {
        let term = tantivy::Term::from_field_text(
            Self::schema()
                .get_field("path")
                .map_err(|x| format!("Field doesn't exist: {}", x))?,
            &self.file_path,
        );
        Ok(term)
    }

    fn as_document(&self) -> TantivyDocument {
        let schema = Self::schema();
        doc! {
        schema.get_field("name").unwrap() => self.name.clone(),
        schema.get_field("parent_directory").unwrap() => self.parent_directory.clone(),
        schema.get_field("date_modified").unwrap() => self.date_modified,
        schema.get_field("date_created").unwrap() => self.date_created,
        schema.get_field("path").unwrap() => self.file_path.clone(),
        schema.get_field("metadata").unwrap() => self.metadata.clone(),
        schema.get_field("popularity").unwrap() => self.popularity,
        }
    }
}

impl tantivy_traits::FromDocument for TantivyFileModel {
    fn from_doc(doc: TantivyDocument, score: f64) -> TantivyFileModel {
        let schema = &TantivyFileModel::schema();

        let name = field_as_string(schema, &doc, "name").unwrap();
        let file_path = field_as_string(schema, &doc, "path").unwrap();
        let parent_directory = field_as_string(schema, &doc, "parent_directory").unwrap();
        let metadata = field_as_string(schema, &doc, "metadata").unwrap();

        let date_modified = field_as_date(schema, &doc, "date_modified").unwrap();
        let date_created = field_as_date(schema, &doc, "date_created").unwrap();

        let popularity = field_as_f64(schema, &doc, "popularity").unwrap();

        let is_dir: bool = Path::new(file_path.as_str()).is_dir();
        TantivyFileModel {
            name,
            file_path,
            parent_directory,
            metadata,
            date_modified,
            date_created,
            score,
            popularity,
            is_directory: is_dir,
        }
    }
}

impl From<SystemFileModel> for TantivyFileModel {
    fn from(value:SystemFileModel) -> TantivyFileModel {
        let is_directory = PathBuf::from(value.file_path.clone()).is_dir();
        let parent_directory = get_parent_directory(&value.file_path);
        TantivyFileModel {
            name: value.name,
            file_path: value.file_path,
            parent_directory,
            metadata: value.metadata,
            date_modified: chrono_time_to_tantivy_datetime(value.date_modified),
            date_created: chrono_time_to_tantivy_datetime(value.date_created),
            score: 0.0,
            popularity: value.popularity,
            is_directory,
        }
    }
}

impl From<TantivyFileModel> for SystemFileModel {
    fn from(value:TantivyFileModel) ->  SystemFileModel {
        SystemFileModel {
            name: value.name,
            file_path: value.file_path,
            metadata: value.metadata,
            date_modified: tantivy_time_to_chrono_datetime(value.date_modified),
            date_created: tantivy_time_to_chrono_datetime(value.date_created),
            popularity: value.popularity,
            size: 0, //TODO: ensure this is not needed
            is_directory: value.is_directory
        }
    }
}