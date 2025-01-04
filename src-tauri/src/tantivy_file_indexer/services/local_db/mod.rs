pub mod service;
mod table_creator;
pub mod util {
    pub mod retry;
}
pub mod tauri_exports;
pub mod tables {
    pub mod files {
        pub mod api;
        pub mod entities {
            pub mod file;
        }
        pub mod trait_impl {
            pub mod files_model;
        }
        pub mod tauri_exports;
    }
    pub mod crawler_queue {
        pub mod api;
        pub mod entities {
            pub mod indexed_dir;
        }
    }
    pub mod indexer_queue {
        pub mod api;
        pub mod entities {
            pub mod directory_payload;
        }
    }
    pub mod recently_indexed_dirs {
        pub mod api;
        pub mod entities {
            pub mod recently_indexed_dir;
        }
    }
    pub mod app_kv_store{
        pub mod api;
        pub mod entities{
            pub mod kv_pair;
        }
    }
}
