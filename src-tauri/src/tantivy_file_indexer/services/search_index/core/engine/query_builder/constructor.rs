use tantivy::{
    query::{BooleanQuery, FuzzyTermQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::Schema,
    DateTime, IndexReader, TantivyError, Term,
};

use crate::tantivy_file_indexer::{
    dtos::search_params_dto::{DateRange, SearchParamsDTO},
    enums::search_query_type::SearchQueryType,
    services::search_index::models::file::TantivyFileModel,
};

pub struct QueryConstructor {
    schema: Schema,
    reader: IndexReader,
}

impl QueryConstructor {
    pub fn new(schema: Schema, reader: IndexReader) -> Self {
        Self { schema, reader }
    }

    /// Construct a query according to the query type specified in the parameters
    pub fn construct_query(
        &self,
        search_params: &SearchParamsDTO,
    ) -> tantivy::Result<Box<dyn Query>> {
        match search_params.query_type {
            SearchQueryType::Term => self
                .construct_standard_query(search_params)
                .map(|x| Box::new(BooleanQuery::new(x)) as Box<dyn Query>),
            SearchQueryType::Fuzzy => self
                .construct_fuzzy_query(search_params)
                .map(|x| Box::new(x) as Box<dyn Query>),
            SearchQueryType::Hybrid => {
                let mut terms = self.construct_standard_query(search_params)?;
                let fuzzy = self.construct_fuzzy_query(search_params)?;
                terms.push((Occur::Should, Box::new(fuzzy)));
                Ok(Box::new(BooleanQuery::new(terms)))
            }
        }
    }

    /// Construct a query to retrieve files, based off of the files' schema
    fn construct_standard_query(
        &self,
        search_params: &SearchParamsDTO,
    ) -> tantivy::Result<Vec<(Occur, Box<dyn Query>)>> {
        let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

        if let Some(file_path) = &search_params.file_path {
            let field_name: String = TantivyFileModel::file_path_field().into();
            let query = self.create_standard_query(&field_name, file_path, Occur::Should)?;
            queries.push(query);
        }

        // if let Some(query_str) = &search_params.name {
        //     let query = self.create_standard_query("name", query_str, Occur::Should)?;
        //     queries.push(query);
        // }

        if let Some(date_range) = &search_params.date_modified_range {
            let field_name: String = TantivyFileModel::date_modified_field().into();
            queries.push(self.create_date_query(&field_name, date_range, Occur::Must));
        }

        if let Some(date_range) = &search_params.date_created_range {
            let field_name: String = TantivyFileModel::date_created_field().into();
            queries.push(self.create_date_query(&field_name, date_range, Occur::Must));
        }

        // if let Some(metadata) = &search_params.metadata {
        //     let query = self.create_term_query("metadata", metadata, Occur::Must)?;
        //     queries.push(query);
        // }

        Ok(queries)
    }

    fn construct_fuzzy_query(
        &self,
        search_params: &SearchParamsDTO,
    ) -> tantivy::Result<FuzzyTermQuery> {
        let term = if let Some(file_path) = &search_params.file_path {
            let field_name: String = TantivyFileModel::file_path_field().into();
            let field = self.schema.get_field(&field_name)?;
            let term = Term::from_field_text(field, file_path);
            Ok(term)
        } else {
            Err(TantivyError::InvalidArgument(
                "Fuzzy query only works on file_path field".to_string(),
            ))
        }?;
        // Distance must be less than 3 or else Tantivy throws an error
        let query = FuzzyTermQuery::new(term, 2, true);
        Ok(query)
    }

    fn create_standard_query(
        &self,
        field_name: &str,
        query: &str,
        occur: Occur,
    ) -> Result<(Occur, Box<dyn Query>), TantivyError> {
        let field = self.schema.get_field(field_name)?;
        let mut query_parser = QueryParser::for_index(self.reader.searcher().index(), vec![field]);
        query_parser.set_conjunction_by_default();
        let query = query_parser.parse_query(query)?;

        Ok((occur, Box::new(query)))
    }

    fn create_date_query(
        &self,
        field_name: &str,
        range: &DateRange,
        occur: Occur,
    ) -> (Occur, Box<dyn Query>) {
        let start_date = DateTime::from_utc(range.start);
        let end_date = DateTime::from_utc(range.end);
        let query = RangeQuery::new_date(field_name.to_string(), start_date..end_date);
        (occur, Box::new(query))
    }

    /// More rigid than a standard query. Checks for exact matches
    fn create_term_query(
        &self,
        field_name: &str,
        query: &str,
        occur: Occur,
    ) -> Result<(Occur, Box<dyn Query>), TantivyError> {
        let field = self.schema.get_field(field_name)?;
        let term = Term::from_field_text(field, query);
        let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::WithFreqs);
        Ok((occur, Box::new(query)))
    }
}
