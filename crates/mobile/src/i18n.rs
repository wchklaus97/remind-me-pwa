//! Mobile-specific i18n implementation
//! 
//! This module provides Dioxus hooks and context for internationalization on mobile platforms.

use dioxus::prelude::*;
use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;
use remind_me_shared::i18n::{Locale, load_translations, get_translation_with_fallback};

/// I18n context for mobile platform
#[derive(Clone)]
pub struct I18nContext {
    current_locale: Locale,
    translations: Arc<HashMap<String, Value>>,
}

impl I18nContext {
    pub fn new() -> Self {
        let translations = load_translations();

        // Try to load locale from mobile storage first, fallback to default
        let mut current_locale = Locale::En;

        // Load saved locale from storage (using a special key)
        // For mobile, we can use the same storage mechanism as reminders
        // Store locale preference in a simple JSON file
        if let Some(locale_str) = load_locale_from_storage() {
            current_locale = Locale::from_str(&locale_str);
        }

        Self {
            current_locale,
            translations: Arc::new(translations),
        }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.current_locale = locale;

        // Persist locale preference to mobile storage
        save_locale_to_storage(self.current_locale.as_str());
    }

    pub fn current_locale_str(&self) -> &'static str {
        self.current_locale.as_str()
    }

    pub fn t(&self, key: &str) -> String {
        let current_locale = self.current_locale.as_str();
        get_translation_with_fallback(&self.translations, current_locale, key)
    }
}

/// Load locale from mobile storage
fn load_locale_from_storage() -> Option<String> {
    // Use a simple approach: try to read from a preferences file
    // For now, we'll use the same storage mechanism as reminders
    // In a real implementation, you might want to use platform-specific preferences APIs
    use std::fs;
    
    if let Some(data_dir) = get_app_data_dir() {
        let prefs_file = data_dir.join("preferences.json");
        if let Ok(content) = fs::read_to_string(&prefs_file) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(locale) = json.get("locale").and_then(|v| v.as_str()) {
                    return Some(locale.to_string());
                }
            }
        }
    }
    None
}

/// Save locale to mobile storage
fn save_locale_to_storage(locale: &str) {
    use std::fs;
    
    if let Some(data_dir) = get_app_data_dir() {
        if let Err(_) = fs::create_dir_all(&data_dir) {
            return;
        }
        
        let prefs_file = data_dir.join("preferences.json");
        let prefs = serde_json::json!({
            "locale": locale
        });
        
        if let Ok(json_str) = serde_json::to_string(&prefs) {
            let _ = fs::write(&prefs_file, json_str);
        }
    }
}

/// Get app data directory for mobile platforms
fn get_app_data_dir() -> Option<std::path::PathBuf> {
    // For mobile platforms, use a platform-specific data directory
    // This is a simplified implementation - in production, you'd use
    // platform-specific APIs (e.g., NSDocumentDirectory on iOS, getFilesDir on Android)
    
    // For now, use a relative "data" directory
    // In production, this should be replaced with platform-specific paths
    Some(std::path::PathBuf::from("data"))
}

/// Initialize i18n context provider
pub fn use_init_i18n() {
    use_context_provider(|| Signal::new(I18nContext::new()));
}

/// Hook to access the I18nContext
pub fn use_i18n() -> Signal<I18nContext> {
    use_context()
}

/// Hook to get translated text
pub fn use_t(key: &str) -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.t(key)
}

/// Hook to change the locale
pub fn use_set_locale() -> impl FnMut(Locale) {
    let mut i18n = use_i18n();
    move |locale: Locale| {
        let mut ctx = i18n.write();
        ctx.set_locale(locale);
    }
}

/// Helper to get the current locale as string
pub fn use_current_locale() -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.current_locale_str().to_string()
}

