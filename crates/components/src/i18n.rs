//! i18n hooks for UI components.
//!
//! Important: **This is the canonical I18n context type** used by all components.
//! Platform crates (web/mobile) should provide this same type via Dioxus context.

use dioxus::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use remind_me_shared::i18n::{get_translation_with_fallback, load_translations};

// Re-export Locale from shared for convenience
pub use remind_me_shared::i18n::Locale;

/// Canonical I18nContext used by all components.
#[derive(Clone)]
pub struct I18nContext {
    current_locale: Locale,
    translations: Arc<HashMap<String, Value>>,
}

impl I18nContext {
    pub fn new() -> Self {
        let translations = load_translations();

        // Default locale
        let mut current_locale = Locale::En;

        // On web/WASM, try to restore the saved locale from localStorage.
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(locale)) = storage.get_item("remind-me-locale") {
                        current_locale = Locale::from_str(&locale);
                    }
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

        // Persist locale preference on web/WASM.
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("remind-me-locale", self.current_locale.as_str());
                }
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

/// Initialize i18n context provider (call once near app root).
pub fn use_init_i18n() {
    use_context_provider(|| Signal::new(I18nContext::new()));
}

/// Hook to access I18nContext
pub fn use_i18n() -> Signal<I18nContext> {
    use_context()
}

/// Hook to get translated text
pub fn use_t(key: &str) -> String {
    let i18n = use_i18n();
    // Avoid holding the read guard across the end of the function (drop order).
    let out = i18n.read().t(key);
    out
}

/// Hook to change locale
pub fn use_set_locale() -> impl FnMut(Locale) {
    let mut i18n = use_i18n();
    move |locale: Locale| {
        i18n.write().set_locale(locale);
    }
}

/// Get current locale as string
pub fn use_current_locale() -> String {
    let i18n = use_i18n();
    // Avoid holding the read guard across the end of the function (drop order).
    let out = i18n.read().current_locale_str().to_string();
    out
}
