use dioxus::prelude::*;
use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;

// Define supported locales
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Locale {
    En,
    ZhHans,  // Simplified Chinese (简体中文)
    ZhHant,  // Traditional Chinese (繁體中文)
}

impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::ZhHans => "zh-Hans",
            Locale::ZhHant => "zh-Hant",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "zh-Hans" | "zh-CN" | "zh" => Locale::ZhHans, // Default "zh" to Simplified Chinese
            "zh-Hant" | "zh-TW" => Locale::ZhHant,
            _ => Locale::En, // Default to English for any other value
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::En
    }
}

// Simple I18n context that loads translations from JSON files
#[derive(Clone)]
pub struct I18nContext {
    current_locale: Locale,
    translations: Arc<HashMap<String, Value>>,
}

impl I18nContext {
    pub fn new() -> Self {
        let translations = Self::load_translations();

        // Try to load locale from localStorage first, fallback to default
        #[cfg(target_arch = "wasm32")]
        {
            let mut current_locale = Locale::En;

            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(locale)) = storage.get_item("remind-me-locale") {
                        current_locale = Locale::from_str(&locale);
                    }
                }
            }

            return Self {
                current_locale,
                translations: Arc::new(translations),
            };
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                current_locale: Locale::En,
                translations: Arc::new(translations),
            }
        }
    }

    fn load_translations() -> HashMap<String, Value> {
        let mut translations = HashMap::new();

        // Load English translations
        let en_json: Value = serde_json::from_str(include_str!("../assets/i18n/en.json"))
            .expect("Failed to parse English translations");
        translations.insert("en".to_string(), en_json);

        // Load Simplified Chinese translations (zh-Hans)
        let zh_hans_json: Value = serde_json::from_str(include_str!("../assets/i18n/zh-Hans.json"))
            .expect("Failed to parse Simplified Chinese translations");
        translations.insert("zh-Hans".to_string(), zh_hans_json.clone());
        // Default "zh" to Simplified Chinese
        translations.insert("zh".to_string(), zh_hans_json);

        // Load Traditional Chinese translations (zh-Hant)
        let zh_hant_json: Value = serde_json::from_str(include_str!("../assets/i18n/zh-Hant.json"))
            .expect("Failed to parse Traditional Chinese translations");
        translations.insert("zh-Hant".to_string(), zh_hant_json.clone());
        // Also support zh-TW for Traditional Chinese
        translations.insert("zh-TW".to_string(), zh_hant_json);

        translations
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.current_locale = locale;

        // Persist locale preference to localStorage
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
        let translations = &self.translations[current_locale];

        // Navigate through the nested JSON structure to find the translation
        let mut current = translations;
        let path_parts = key.split('.');

        for path_part in path_parts {
            match current.get(path_part) {
                Some(value) => current = value,
                None => {
                    // Fallback to English if key doesn't exist in current locale
                    if current_locale != "en" {
                        let en_translations = &self.translations["en"];
                        return self.get_nested_value(en_translations, key)
                            .unwrap_or_else(|| key.to_string());
                    }
                    return key.to_string();
                }
            }
        }

        match current {
            Value::String(s) => s.clone(),
            _ => {
                // Fallback to English if value is not a string
                if current_locale != "en" {
                    let en_translations = &self.translations["en"];
                    return self.get_nested_value(en_translations, key)
                        .unwrap_or_else(|| key.to_string());
                }
                key.to_string()
            }
        }
    }

    fn get_nested_value(&self, json: &Value, key: &str) -> Option<String> {
        let mut current = json;
        let path_parts = key.split('.');

        for path_part in path_parts {
            current = match current.get(path_part) {
                Some(value) => value,
                None => return None,
            };
        }

        match current {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

// Provide the I18nContext as a Dioxus context - this should be called once in the App component
pub fn use_init_i18n() {
    use_context_provider(|| Signal::new(I18nContext::new()));
}

// Hook to access the I18nContext
pub fn use_i18n() -> Signal<I18nContext> {
    use_context()
}

// Hook to get translated text
pub fn use_t(key: &str) -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.t(key)
}

// Hook to change the locale
pub fn use_set_locale() -> impl FnMut(Locale) {
    let mut i18n = use_i18n();
    move |locale: Locale| {
        let mut ctx = i18n.write();
        ctx.set_locale(locale);
    }
}

// Helper to get the current locale as string
pub fn use_current_locale() -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.current_locale_str().to_string()
}
