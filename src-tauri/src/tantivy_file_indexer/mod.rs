pub mod service_container;
pub mod configs{
    pub mod file_indexer_config;
}
mod schemas{
    pub mod file_schema;
}
pub mod crawlers{
    pub mod crawler;
    mod dir_walker;
    mod walker;
}
mod dtos{
    pub mod file_dto_input;
}
mod util{
    pub mod file_id_helper;
}
mod converters{
    pub mod date_converter;
    pub mod doc_to_dto;
}
pub mod service{
    pub mod search_index_service;
    mod querier;
    pub mod exports;
}
mod models{
    pub mod search_params_model;
}
pub mod db{
    pub mod sqlx_service;
    pub mod tables{
        pub mod files{
            pub mod api;
            pub mod models;
        }
    }
}
mod app_data{
    pub mod helper_methods;
    pub mod json;
}