// Web storage implementation using localStorage
use remind_me_shared::storage::{PlatformStorage, StorageError};

pub struct WebStorage;

impl PlatformStorage for WebStorage {
    fn get(key: &str) -> Option<String> {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(data)) = storage.get_item(key) {
                    return Some(data);
                }
            }
        }
        None
    }
    
    fn set(key: &str, value: &str) -> Result<(), StorageError> {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Err(_) = storage.set_item(key, value) {
                    return Err(StorageError::SaveFailed);
                }
                return Ok(());
            }
        }
        Err(StorageError::Unavailable)
    }
}

