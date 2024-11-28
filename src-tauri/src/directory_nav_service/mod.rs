mod dtos {
    pub mod inline_query_dto;
}
mod models {
    pub mod drive_model;
    pub mod get_files_model;
}
mod core {
    pub mod files {
        pub mod inline_search{
            pub mod tauri_cmds;
        }
        pub mod operations{
            pub mod tauri_cmds;
        }
        pub mod retrieve{
            pub mod tauri_cmds;
            mod helper;
        }
        pub mod user_input_validator{
            pub mod tauri_cmds;
        }
    }
    pub mod drives {
        pub mod tauri_cmds;
        mod com {
            pub mod get_sys_drives;
        }
    }
}
mod app_state {
    pub mod files_display;
}
mod util {
    pub mod metadata_inspector;
    pub mod path_ops;
}
pub mod tauri_exports;
