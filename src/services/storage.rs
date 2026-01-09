use crate::models::file::StoredFile;
use crate::utils::constants::STORAGE_KEY;
use web_sys::window;

pub struct StorageService;

impl StorageService {
    pub fn load_files() -> Vec<StoredFile> {
        let window = match window() {
            Some(w) => w,
            None => return Vec::new(),
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return Vec::new(),
        };

        storage
            .get_item(STORAGE_KEY)
            .ok()
            .flatten()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save_files(files: &[StoredFile]) {
        let window = match window() {
            Some(w) => w,
            None => return,
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        if let Ok(json) = serde_json::to_string(files) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }

    pub fn clear_files() {
        let window = match window() {
            Some(w) => w,
            None => return,
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        let _ = storage.remove_item(STORAGE_KEY);
    }
}