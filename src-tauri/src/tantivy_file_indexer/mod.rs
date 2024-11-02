pub mod service_container;
pub mod service_container_traits;
pub mod configs {
    pub mod file_indexer_config;
}
mod schemas {
    pub mod file_schema;
}
mod dtos {
    pub mod file_dto_input;
}
mod util {
    pub mod file_id_helper;
}
mod converters {
    pub mod date_converter;
    pub mod doc_to_dto;
}
pub mod services {
    pub mod search_index {
        mod core {
            pub mod index_worker;
            pub mod querier;
        }
        pub mod models {
            pub mod index_worker {
                pub mod file_input;
            }
        }
        pub mod service;
        pub mod tauri_exports;
    }
    pub mod local_db {
        pub mod service;
        pub mod tables {
            pub mod files {
                pub mod api;
                pub mod models;
            }
        }
    }
    pub mod local_crawler {
        pub mod service;
        mod core {
            pub mod crawler_queue;
            pub mod crawler_worker;
        }
    }
}
mod models {
    pub mod search_params_model;
}
mod app_data {
    pub mod helper_methods;
    pub mod json;
}
