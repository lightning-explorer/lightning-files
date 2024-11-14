pub mod service;
pub mod tables {
    pub mod files {
        pub mod api;
        pub mod entities {
            pub mod file_model;
        }
        pub mod tauri_exports;
    }
    pub mod crawler_queue {
        pub mod api;
        pub mod models;
    }
}
