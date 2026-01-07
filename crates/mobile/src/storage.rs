//! Mobile storage implementation using file system
//! 
//! This module provides the MobileStorage implementation of PlatformStorage
//! for iOS and Android builds.

use remind_me_shared::storage::{PlatformStorage, StorageError};
use std::path::PathBuf;
use std::fs;

/// Mobile storage implementation using file system
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
    // On iOS: This would typically be in the app's Documents directory
    // On Android: This would typically be in the app's files directory
    // For now, use a relative path that should work in most cases
    Some(PathBuf::from("data"))
}

// Re-export storage functions with MobileStorage implementation
use remind_me_shared::storage::{load_reminders as load_reminders_impl, save_reminders as save_reminders_impl, load_tags as load_tags_impl, save_tags as save_tags_impl};
use remind_me_shared::models::{Reminder, Tag};

/// Load reminders from file system
pub fn load_reminders() -> Vec<Reminder> {
    load_reminders_impl::<MobileStorage>()
}

/// Save reminders to file system
pub fn save_reminders(reminders: &[Reminder]) {
    save_reminders_impl::<MobileStorage>(reminders);
}

/// Load tags from file system
pub fn load_tags() -> Vec<Tag> {
    load_tags_impl::<MobileStorage>()
}

/// Save tags to file system
pub fn save_tags(tags: &[Tag]) {
    save_tags_impl::<MobileStorage>(tags);
}
