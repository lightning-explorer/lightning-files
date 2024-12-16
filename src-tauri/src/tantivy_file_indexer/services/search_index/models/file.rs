use serde::{Deserialize, Serialize};
use tantivy::schema::{Schema, FAST, INDEXED, STORED, TEXT};
use std::path::Path;

use chrono::Utc;
use tantivy::TantivyDocument;

use crate::tantivy_file_indexer::shared::search_index::{tantivy_traits, util::{field_as_date,field_as_string_or_default}};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TantivyFileModel {
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: String,
    pub score: f64,
    pub is_directory: bool,
}

impl tantivy_traits::Model for TantivyFileModel{
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

    fn get_primary_key(&self) -> Result<tantivy::Term,Self::Error> {
        let term = tantivy::Term::from_field_text(
            Self::schema()
            .get_field("path")
            .map_err(|x| format!("Field doesn't exist: {}", x))?,
             &self.file_path,
        );
        Ok(term)
    }
    
}

impl tantivy_traits::FromDocument for TantivyFileModel{
    fn from_doc(
        doc: TantivyDocument,
        schema: &Schema,
        score: f64,
    ) -> TantivyFileModel {
        let name = field_as_string_or_default(schema, &doc, "name");
        let file_path = field_as_string_or_default(schema, &doc, "path");
        let metadata = field_as_string_or_default(schema, &doc, "metadata");
        let date_modified: String = field_as_date(schema, &doc, "name").map_or(Utc::now().to_string(), |date|date.to_string());

        let is_dir: bool = Path::new(file_path.as_str()).is_dir();
        TantivyFileModel {
            name,
            file_path,
            metadata,
            date_modified,
            score,
            is_directory: is_dir,
        }
    }
}
