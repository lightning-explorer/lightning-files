mod core {
    pub mod file_indexer {
        mod index_worker;
        pub mod index_worker_manager;
    }
    pub mod querier;
}
pub mod models {
    pub mod index_worker {
        pub mod file_input;
    }
}
pub mod service;
pub mod tauri_exports;
