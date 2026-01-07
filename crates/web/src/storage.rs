//! Web storage implementation using localStorage
//! 
//! This module provides the WebStorage implementation of PlatformStorage
//! for web/WASM builds.

use remind_me_shared::storage::{PlatformStorage, StorageError};

/// Web storage implementation using browser localStorage
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

// Re-export storage functions with WebStorage implementation
use remind_me_shared::storage::{load_reminders as load_reminders_impl, save_reminders as save_reminders_impl, load_tags as load_tags_impl, save_tags as save_tags_impl};
use remind_me_shared::models::{Reminder, Tag};

/// Load reminders from localStorage
pub fn load_reminders() -> Vec<Reminder> {
    load_reminders_impl::<WebStorage>()
}

/// Save reminders to localStorage
pub fn save_reminders(reminders: &[Reminder]) {
    save_reminders_impl::<WebStorage>(reminders);
}

/// Load tags from localStorage
pub fn load_tags() -> Vec<Tag> {
    load_tags_impl::<WebStorage>()
}

/// Save tags to localStorage
pub fn save_tags(tags: &[Tag]) {
    save_tags_impl::<WebStorage>(tags);
}
