
use std::fmt::{Debug, Display};

use tantivy::{schema::Schema, TantivyDocument, Term};

pub trait Model{
    type Error: Display + Debug;

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
    fn get_primary_key(&self) -> Term;

    /// Get the value of the primary key as a string
    fn get_primary_key_str(&self) -> String;

    fn as_document(&self) -> TantivyDocument;
}

pub trait FromDocument: Model{
    fn from_doc(
        doc: TantivyDocument,
        score: f64,
    ) -> Self;
}