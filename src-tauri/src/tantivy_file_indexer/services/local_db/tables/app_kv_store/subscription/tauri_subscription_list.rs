use tauri::AppHandle;

pub struct TauriSubscriptionList {
    keys: Vec<String>,
    app_handle: AppHandle,
}

impl TauriSubscriptionList {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            keys: Vec::new(),
            app_handle,
        }
    }
}
