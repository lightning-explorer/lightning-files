use tantivy::{schema::{OwnedValue, Schema}, time::OffsetDateTime, Document, TantivyDocument};

/// Extracts the field from the document as a String
pub fn field_as_string(schema: &Schema, doc: &TantivyDocument, field_name: &str) -> Option<String> {
    for (field, value) in doc.iter_fields_and_values() {
        if schema.get_field_name(field) == field_name {
            if let OwnedValue::Str(text) = value {
                return Some(text.to_string());
            }
        }
    }
    None
}

pub fn field_as_string_or_default(schema: &Schema, doc: &TantivyDocument, field_name: &str) -> String{
    field_as_string(schema, doc, field_name).unwrap_or_default()
}

/// Extracts the field from the document as a UTC date
pub fn field_as_date(schema: &Schema, doc: &TantivyDocument, field_name: &str) -> Option<tantivy::DateTime> {
    for (field, value) in doc.iter_fields_and_values() {
        if schema.get_field_name(field) == field_name {
            if let OwnedValue::Date(date) = value {
                return Some(*date)
            }
        }
    }
    None
}

/// The default would be the current UTC time
pub fn field_as_date_or_default(schema: &Schema, doc: &TantivyDocument, field_name: &str)->tantivy::DateTime{
    field_as_date(schema,doc,field_name).unwrap_or(tantivy::DateTime::from_utc(OffsetDateTime::now_utc()))
}

pub fn field_as_f64(schema: &Schema, doc: &TantivyDocument, field_name: &str)->Option<f64>{
    for (field, value) in doc.iter_fields_and_values() {
        if schema.get_field_name(field) == field_name {
            if let OwnedValue::F64(val) = value {
                return Some(*val)
            }
        }
    }
    None
}