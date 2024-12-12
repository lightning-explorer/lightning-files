use super::{constructor, query_organizer};
use std::collections::HashSet;

use tantivy::{
    collector::TopDocs, query::Query, schema::Schema, DocAddress, Searcher, TantivyDocument,
};

use crate::tantivy_file_indexer::{
    converters::doc_to_dto::doc_to_tantivy_file_model, dtos::search_params_dto::SearchParamsDTO,
    models::tantivy_file_model::TantivyFileModel,
};

pub struct Querier {
    schema: Schema,
    searcher: Searcher,
}

impl Querier {
    pub fn new(schema: Schema, searcher: Searcher) -> Self {
        Self { schema, searcher }
    }
    /// Where `min_results` indicated how many documents should initially be fetched, and max_results is the value to work up to
    ///
    /// `step_size`: the number of events that will get emitted in total
    pub async fn advanced_query_streamed<EmitFn>(
        &self,
        search_params: SearchParamsDTO,
        emit: EmitFn,
        step_size: usize,
        min_results: usize,
    ) where
        EmitFn: Fn(Vec<TantivyFileModel>),
    {
        let query = constructor::construct_query(&self.schema, &self.searcher, &search_params)
            .expect("Query could not be constructed");

        let max_results = search_params.num_results as usize;

        let step_by = (max_results - min_results) / step_size;
        let mut current_out_amt = min_results;
        let mut prev_docs: HashSet<DocAddress> = HashSet::new();

        for _ in 0..step_size {
            match self.execute_query(current_out_amt, &query) {
                Ok(top_docs) => {
                    let mut output_docs: Vec<TantivyFileModel> = Vec::new();
                    for (_score, address) in top_docs {
                        if prev_docs.insert(address) {
                            // Value got inserted, proceed
                            if let Ok(doc) = self.searcher.doc(address) {
                                output_docs.push(doc_to_tantivy_file_model(
                                    doc,
                                    &self.schema,
                                    _score,
                                ));
                            } else {
                                println!("Failed to retrieve document for address {:?}", address);
                            }
                        }
                    }
                    // Send the batch of documents
                    emit(output_docs);
                    current_out_amt += step_by;
                }
                Err(err) => {
                    println!("Error executing query: {}", err);
                }
            }
        }
    }

    /// Whereas the other query functions just return the items however they were presented in the index, this function adds an extra post-processing
    /// step where files belonging to the same drive, folders, sharing the same extension, etc. are grouped together.
    /// 
    /// ### NOTE: 
    /// this function slightly differs from `advanced_query_streamed` in the fact that the emit function will emit all of the organized files
    /// that get accumulated, meaning that the frontend needs to REPLACE its list of files with whatever gets emitted, as opposed to appending the
    /// emitted result.
    pub async fn organized_query_streamed<EmitFn>(
        &self,
        search_params: SearchParamsDTO,
        emit: EmitFn,
        step_size: usize,
        min_results: usize,
    ) where
        EmitFn: Fn(&[TantivyFileModel]),
    {
        let query = constructor::construct_query(&self.schema, &self.searcher, &search_params)
            .expect("Query could not be constructed");

        let max_results = search_params.num_results as usize;

        let step_by = (max_results - min_results) / step_size;
        let mut current_out_amt = min_results;
        let mut accumulated_docs: Vec<TantivyFileModel> = Vec::new();

        for _ in 0..step_size {
            match self.execute_query(current_out_amt, &query) {
                Ok(top_docs) => {
                    for (_score, address) in top_docs {
                        if let Ok(doc) = self.searcher.doc(address) {
                            accumulated_docs.push(doc_to_tantivy_file_model(
                                doc,
                                &self.schema,
                                _score,
                            ));

                        } else {
                            println!("Failed to retrieve document for address {:?}", address);
                        } 
                    }
                    // Send the batch of documents
                    Self::organize_accumulated_docs(&mut accumulated_docs);
                    emit(&accumulated_docs);
                    current_out_amt += step_by;
                }
                Err(err) => {
                    println!("Error executing query: {}", err);
                }
            }
        }
    }

    fn organize_accumulated_docs(docs:&mut Vec<TantivyFileModel>){
        let mut grouped = Vec::new();
        let groups = query_organizer::group_files(docs.to_vec());
        for (_grouping, mut files) in groups.into_iter(){
            grouped.append(&mut files);
        }
       *docs = grouped;
    }

    pub fn advanced_query(
        &self,
        search_params: &SearchParamsDTO,
    ) -> tantivy::Result<Vec<TantivyFileModel>> {
        let query = constructor::construct_query(&self.schema, &self.searcher, search_params)
            .expect("Query could not be constructed");

        // Execute the query and collect the results
        let top_docs = self.execute_query(search_params.num_results as usize, &query)?;

        let results: Vec<TantivyFileModel> = top_docs
            .into_iter()
            .map(|(_score, doc_address)| {
                let doc: TantivyDocument = self.searcher.doc(doc_address).unwrap();
                doc_to_tantivy_file_model(doc, &self.schema, _score)
            })
            .collect();

        Ok(results)
    }

    /// Execute a standard query, applying a popularity bias to the results
    fn execute_query<Q>(
        &self,
        num_results: usize,
        query: &Q,
    ) -> tantivy::Result<Vec<(f64, tantivy::DocAddress)>>
    where
        Q: Query + Sized,
    {
        self.searcher.search(
            query,
            &TopDocs::with_limit(num_results).tweak_score(
                |segment_reader: &tantivy::SegmentReader| {
                    let popularity_field = segment_reader
                        .fast_fields()
                        .f64("popularity")
                        .expect("Failed to access popularity field");
                    move |doc, original_score| {
                        // Default to 1 if no popularity
                        let pop_score = popularity_field.first(doc).unwrap_or(1.0);
                        Self::apply_popularity(original_score, pop_score)
                    }
                },
            ),
        )
    }

    fn apply_popularity(existing_score: f32, popularity_score: f64) -> f64 {
        (existing_score as f64) + popularity_score.log(10.0)
    }
}
