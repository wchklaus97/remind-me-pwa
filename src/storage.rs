use crate::models::Reminder;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

pub fn load_reminders() -> Vec<Reminder> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item("reminders") {
                match serde_json::from_str::<Vec<Reminder>>(&data) {
                    Ok(reminders) => return reminders,
                    Err(e) => {
                        #[cfg(target_arch = "wasm32")]
                        console::warn_1(&format!("Failed to parse reminders from storage: {}", e).into());
                    }
                }
            }
        }
    }
    Vec::new()
}

pub fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            match serde_json::to_string(reminders) {
                Ok(json) => {
                    match storage.set_item("reminders", &json) {
                        Ok(_) => {
                            // Successfully saved
                        }
                        Err(e) => {
                            // Check if it's a quota exceeded error
                            let error_name = e.name();
                            #[cfg(target_arch = "wasm32")]
                            {
                                if error_name == "QuotaExceededError" {
                                    console::error_1(&"Storage quota exceeded. Please clear some data or use fewer reminders.".into());
                                } else {
                                    console::error_1(&format!("Failed to save reminders to storage: {}", error_name).into());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    #[cfg(target_arch = "wasm32")]
                    console::error_1(&format!("Failed to serialize reminders: {}", e).into());
                }
            }
        } else {
            #[cfg(target_arch = "wasm32")]
            console::warn_1(&"localStorage is not available. Reminders will not be persisted.".into());
        }
    }
}

