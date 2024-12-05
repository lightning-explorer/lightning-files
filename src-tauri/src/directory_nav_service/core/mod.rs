pub mod files {
    pub mod inline_search {
        pub mod tauri_cmds;
    }
    pub mod operations {
        pub mod file_reader;
        pub mod tauri_cmds;
    }
    pub mod retrieve {
        mod helper;
        pub mod tauri_cmds;
    }
    pub mod user_input_validator {
        pub mod tauri_cmds;
    }
}
pub mod drives {
    pub mod tauri_cmds;
    mod com {
        pub mod get_sys_drives;
    }
}