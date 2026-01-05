use crate::models::{Reminder, Tag};

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
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            // Try to load v2 first
            if let Ok(Some(data)) = storage.get_item(REMINDERS_V2_KEY) {
                if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
                    return reminders;
                }
            }
            
            // Migration: Load v1 and migrate to v2
            if let Ok(Some(data)) = storage.get_item(REMINDERS_V1_KEY) {
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
                        let _ = storage.set_item(REMINDERS_V2_KEY, &json);
                    }
                    
                    return migrated;
                }
            }
        }
    }
    Vec::new()
}

pub fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(reminders) {
                let _ = storage.set_item(REMINDERS_V2_KEY, &json);
            }
        }
    }
}

pub fn load_tags() -> Vec<Tag> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item(TAGS_V1_KEY) {
                if let Ok(tags) = serde_json::from_str::<Vec<Tag>>(&data) {
                    return tags;
                }
            }
        }
    }
    Vec::new()
}

pub fn save_tags(tags: &[Tag]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(tags) {
                let _ = storage.set_item(TAGS_V1_KEY, &json);
            }
        }
    }
}

