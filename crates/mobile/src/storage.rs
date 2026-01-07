// Mobile storage implementation using file system
use remind_me_shared::storage::{PlatformStorage, StorageError};
use std::path::PathBuf;
use std::fs;

pub struct MobileStorage;

impl PlatformStorage for MobileStorage {
    fn get(key: &str) -> Option<String> {
        if let Some(data_dir) = get_app_data_dir() {
            let file_path = data_dir.join(format!("{}.json", key));
            if let Ok(content) = fs::read_to_string(&file_path) {
                return Some(content);
            }
        }
        None
    }
    
    fn set(key: &str, value: &str) -> Result<(), StorageError> {
        if let Some(data_dir) = get_app_data_dir() {
            if let Err(_) = fs::create_dir_all(&data_dir) {
                return Err(StorageError::Unavailable);
            }
            
            let file_path = data_dir.join(format!("{}.json", key));
            if let Err(_) = fs::write(&file_path, value) {
                return Err(StorageError::SaveFailed);
            }
            return Ok(());
        }
        Err(StorageError::Unavailable)
    }
}

fn get_app_data_dir() -> Option<PathBuf> {
    // Try to get the executable directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let mut path = parent.to_path_buf();
            path.push("data");
            return Some(path);
        }
    }
    
    // Fallback: Use a standard location
    Some(PathBuf::from("data"))
}

