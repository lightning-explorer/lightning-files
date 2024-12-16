use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tantivy::{
    doc,
    schema::{Schema, FAST, INDEXED, STORED, TEXT},
};

use tantivy::TantivyDocument;

use crate::{
    get_parent_directory,
    tantivy_file_indexer::{
        converters::date_converter::chrono_time_to_tantivy_datetime,
        models::internal_system_file,
        shared::search_index::{
            tantivy_traits::{self, ToTantivyModel},
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

        schema_builder.add_text_field("parent_directory", TEXT | FAST);

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
    fn from_doc(doc: TantivyDocument, schema: &Schema, score: f64) -> TantivyFileModel {
        // Unwrap all of the values because if the fields do not exist, then there is an underlying problem and the app can't continue
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

impl ToTantivyModel<TantivyFileModel> for internal_system_file::model::Model {
    fn to_model(self) -> TantivyFileModel {
        let is_directory = PathBuf::from(self.file_path.clone()).is_dir();
        let parent_directory = get_parent_directory(&self.file_path);
        TantivyFileModel {
            name: self.name,
            file_path: self.file_path,
            parent_directory,
            metadata: self.metadata,
            date_modified: chrono_time_to_tantivy_datetime(self.date_modified),
            date_created: chrono_time_to_tantivy_datetime(self.date_created),
            score: 0.0,
            popularity: self.popularity,
            is_directory,
        }
    }
}
