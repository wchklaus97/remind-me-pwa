use crate::models::{Reminder, Tag};
use crate::storage_platform::{PlatformStorage, PlatformStorageImpl, StorageError};

// Storage keys with versioning
const REMINDERS_V2_KEY: &str = "reminders_v2";
const REMINDERS_V1_KEY: &str = "reminders"; // Legacy key
const TAGS_V1_KEY: &str = "tags_v1";

// Legacy Reminder structure (v1) for migration
#[derive(serde::Deserialize)]
struct LegacyReminder {
    id: String,
    title: String,
    description: String,
    due_date: String,
    completed: bool,
    created_at: String,
}

pub fn load_reminders() -> Vec<Reminder> {
    // Try to load v2 first
    if let Some(data) = PlatformStorageImpl::get(REMINDERS_V2_KEY) {
        if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
            return reminders;
        }
    }
    
    // Migration: Load v1 and migrate to v2
    if let Some(data) = PlatformStorageImpl::get(REMINDERS_V1_KEY) {
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
                if let Err(_) = PlatformStorageImpl::set(REMINDERS_V2_KEY, &json) {
                    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
                    web_sys::console::warn_1(&"Failed to save migrated reminders".into());
                }
            }
            
            return migrated;
        }
    }
    
    Vec::new()
}

pub fn save_reminders(reminders: &[Reminder]) {
    match serde_json::to_string(reminders) {
        Ok(json) => {
            if let Err(_) = PlatformStorageImpl::set(REMINDERS_V2_KEY, &json) {
                // Log error but don't block UI (storage errors are non-critical)
                #[cfg(all(debug_assertions, target_arch = "wasm32"))]
                web_sys::console::error_1(&"Failed to save reminders".into());
            }
        }
        Err(e) => {
            // Log serialization error but don't block UI
            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            web_sys::console::error_1(&format!("Failed to serialize reminders: {}", e).into());
        }
    }
}

pub fn load_tags() -> Vec<Tag> {
    if let Some(data) = PlatformStorageImpl::get(TAGS_V1_KEY) {
        if let Ok(tags) = serde_json::from_str::<Vec<Tag>>(&data) {
            return tags;
        }
    }
    Vec::new()
}

pub fn save_tags(tags: &[Tag]) {
    match serde_json::to_string(tags) {
        Ok(json) => {
            if let Err(_) = PlatformStorageImpl::set(TAGS_V1_KEY, &json) {
                // Log error but don't block UI (storage errors are non-critical)
                #[cfg(all(debug_assertions, target_arch = "wasm32"))]
                web_sys::console::error_1(&"Failed to save tags".into());
            }
        }
        Err(e) => {
            // Log serialization error but don't block UI
            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            web_sys::console::error_1(&format!("Failed to serialize tags: {}", e).into());
        }
    }
}

