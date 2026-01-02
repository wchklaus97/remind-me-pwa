use crate::models::Reminder;

pub fn load_reminders() -> Vec<Reminder> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item("reminders") {
                if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
                    return reminders;
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
                let _ = storage.set_item("reminders", &json);
            }
        }
    }
}

