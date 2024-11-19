pub mod service;
pub mod tauri_exports;
mod table_creator;
pub mod tables {
    pub mod files {
        pub mod api;
        pub mod entities {
            pub mod file;
        }
        pub mod tauri_exports;
    }
    pub mod crawler_queue {
        pub mod api;
        pub mod entities{
            pub mod indexed_dir;
        }
    }
    pub mod recently_indexed_dirs{
        pub mod api;
        pub mod entities{
            pub mod recently_indexed_dir;
        }
    }
}
