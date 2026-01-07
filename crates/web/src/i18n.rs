//! Web-specific i18n implementation
//! 
//! This module provides Dioxus hooks and context for internationalization on web platform.

use dioxus::prelude::*;
use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;
use remind_me_shared::i18n::{Locale, load_translations, get_translation_with_fallback};

/// I18n context for web platform
#[derive(Clone)]
pub struct I18nContext {
    current_locale: Locale,
    translations: Arc<HashMap<String, Value>>,
}

impl I18nContext {
    pub fn new() -> Self {
        let translations = load_translations();

        // Try to load locale from localStorage first, fallback to default
        let mut current_locale = Locale::En;

        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(locale)) = storage.get_item("remind-me-locale") {
                    current_locale = Locale::from_str(&locale);
                }
            }
        }

        Self {
            current_locale,
            translations: Arc::new(translations),
        }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.current_locale = locale;

        // Persist locale preference to localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("remind-me-locale", self.current_locale.as_str());
            }
        }
    }

    pub fn current_locale_str(&self) -> &'static str {
        self.current_locale.as_str()
    }

    pub fn t(&self, key: &str) -> String {
        let current_locale = self.current_locale.as_str();
        get_translation_with_fallback(&self.translations, current_locale, key)
    }
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

