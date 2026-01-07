//! Storage abstraction for cross-platform data persistence
//! 
//! This module provides a platform-agnostic storage trait and storage functions
//! for reminders and tags. Platform-specific implementations are provided in
//! `crates/web/src/storage.rs` and `crates/mobile/src/storage.rs`.

use crate::models::{Reminder, Tag};

/// Storage error type
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Storage is unavailable (e.g., localStorage not supported)
    Unavailable,
    /// Failed to serialize/deserialize data
    SerializationFailed,
    /// Failed to save data to storage
    SaveFailed,
}

/// Platform storage trait for key-value storage
/// 
/// This trait is implemented by platform-specific storage backends:
/// - `WebStorage` in `crates/web/src/storage.rs` (uses localStorage)
/// - `MobileStorage` in `crates/mobile/src/storage.rs` (uses file system)
pub trait PlatformStorage {
    /// Get a value from storage by key
    fn get(key: &str) -> Option<String>;
    
    /// Set a value in storage by key
    fn set(key: &str, value: &str) -> Result<(), StorageError>;
}

// Storage keys
pub const REMINDERS_V2_KEY: &str = "reminders_v2";
pub const REMINDERS_V1_KEY: &str = "reminders"; // Legacy key
pub const TAGS_V1_KEY: &str = "tags_v1";

// Legacy Reminder structure (v1) for migration
#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct LegacyReminder {
    id: String,
    title: String,
    description: String,
    due_date: String,
    completed: bool,
    created_at: String,
}

/// Internal implementation with generic PlatformStorage
#[allow(dead_code)]
fn load_reminders_impl<S: PlatformStorage>() -> Vec<Reminder> {
    // Try to load v2 first
    if let Some(data) = S::get(REMINDERS_V2_KEY) {
        if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
            return reminders;
        }
    }
    
    // Migration: Load v1 and migrate to v2
    if let Some(data) = S::get(REMINDERS_V1_KEY) {
        if let Ok(legacy_reminders) = serde_json::from_str::<Vec<LegacyReminder>>(&data) {
            let migrated: Vec<Reminder> = legacy_reminders
                .into_iter()
                .map(|r| Reminder {
                    id: r.id,
                    title: r.title,
                    description: r.description,
                    due_date: r.due_date,
                    completed: r.completed,
                    created_at: r.created_at,
                    tag_ids: Vec::new(), // Initialize with empty tags
                })
                .collect();
            
            // Save migrated data to v2
            if let Ok(json) = serde_json::to_string(&migrated) {
                if let Err(_) = S::set(REMINDERS_V2_KEY, &json) {
                    // Log error but don't block (storage errors are non-critical)
                    #[cfg(debug_assertions)]
                    eprintln!("Failed to save migrated reminders");
                }
            }
            
            return migrated;
        }
    }
    
    Vec::new()
}

/// Internal implementation with generic PlatformStorage
#[allow(dead_code)]
fn save_reminders_impl<S: PlatformStorage>(reminders: &[Reminder]) {
    match serde_json::to_string(reminders) {
        Ok(json) => {
            if let Err(_) = S::set(REMINDERS_V2_KEY, &json) {
                // Log error but don't block UI (storage errors are non-critical)
                #[cfg(debug_assertions)]
                eprintln!("Failed to save reminders");
            }
        }
        Err(e) => {
            // Log serialization error but don't block UI
            #[cfg(debug_assertions)]
            eprintln!("Failed to serialize reminders: {}", e);
        }
    }
}

/// Internal implementation with generic PlatformStorage
#[allow(dead_code)]
fn load_tags_impl<S: PlatformStorage>() -> Vec<Tag> {
    if let Some(data) = S::get(TAGS_V1_KEY) {
        if let Ok(tags) = serde_json::from_str::<Vec<Tag>>(&data) {
            return tags;
        }
    }
    Vec::new()
}

/// Internal implementation with generic PlatformStorage
#[allow(dead_code)]
fn save_tags_impl<S: PlatformStorage>(tags: &[Tag]) {
    match serde_json::to_string(tags) {
        Ok(json) => {
            if let Err(_) = S::set(TAGS_V1_KEY, &json) {
                // Log error but don't block UI (storage errors are non-critical)
                #[cfg(debug_assertions)]
                eprintln!("Failed to save tags");
            }
        }
        Err(e) => {
            // Log serialization error but don't block UI
            #[cfg(debug_assertions)]
            eprintln!("Failed to serialize tags: {}", e);
        }
    }
}
