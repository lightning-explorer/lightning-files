
use std::fmt::Display;

use tantivy::{schema::Schema, TantivyDocument, Term};

pub trait Model{
    type Error: Display;

    fn schema()->Schema;

    /// Get the term that represents the primary key
    /// 
    /// Example:
    /// ```
    /// tantivy::Term::from_field_text(
    ///        schema
    ///        .get_field("path")
    ///        .map_err(|x| format!("Field doesn't exist: {}", x))?,
    ///         &self.file_path,
    ///    )
    /// ```
    fn get_primary_key(&self) -> Result<Term,Self::Error>;
}

pub trait FromDocument: Model{
    fn from_doc(
        doc: TantivyDocument,
        schema: &Schema,
        score: f64,
    ) -> Self;
}

struct PrimaryKey{
    /// The Tantivy schema field
    key:String,
    value:String,
}

pub trait ToTantivyModel<M> where M:Model {
    fn to_model(&self, schema:&Schema) -> M;
}