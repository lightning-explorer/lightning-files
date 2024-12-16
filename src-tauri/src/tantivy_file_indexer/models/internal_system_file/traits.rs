use tantivy::{doc, schema::Schema, TantivyDocument};

use crate::tantivy_file_indexer::{converters::date_converter::chrono_time_to_tantivy_datetime, shared::search_index::tantivy_traits::ToTantivyModel};

use super::model::Model;

impl ToTantivyModel for Model{

}

/*
doc! {
            //schema.get_field("file_id").unwrap() => self.file_id, // UNUSED SCHEMA FIELD
            schema.get_field("name").unwrap() => self.name.clone(),
            schema.get_field("date_modified").unwrap() => chrono_time_to_tantivy_datetime(self.date_modified), 
            schema.get_field("date_created").unwrap() => chrono_time_to_tantivy_datetime(self.date_created), 
            schema.get_field("path").unwrap() => self.file_path.clone(),
            schema.get_field("metadata").unwrap() => self.metadata.clone(),
            schema.get_field("popularity").unwrap() => self.popularity,
            } 
*/