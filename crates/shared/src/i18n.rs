//! Internationalization (i18n) support
//! 
//! This module provides shared i18n types and translation loading.
//! Platform-specific implementations (I18nContext, hooks) are in platform crates.

use std::collections::HashMap;
use serde_json::Value;

/// Supported locales
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Locale {
    /// English
    En,
    /// Simplified Chinese (简体中文)
    ZhHans,
    /// Traditional Chinese (繁體中文)
    ZhHant,
}

impl Locale {
    /// Convert locale to BCP 47 string
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::ZhHans => "zh-Hans",
            Locale::ZhHant => "zh-Hant",
        }
    }

    /// Create locale from string
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

/// Load translations from JSON files
/// 
/// This function loads all translation files and returns a HashMap
/// keyed by locale code. Platform-specific code should call this
/// and use the result to initialize I18nContext.
pub fn load_translations() -> HashMap<String, Value> {
    let mut translations = HashMap::new();

    // Load English translations
    let en_json: Value = serde_json::from_str(include_str!("../../../assets/i18n/en.json"))
        .expect("Failed to parse English translations");
    translations.insert("en".to_string(), en_json);

    // Load Simplified Chinese translations (zh-Hans)
    let zh_hans_json: Value = serde_json::from_str(include_str!("../../../assets/i18n/zh-Hans.json"))
        .expect("Failed to parse Simplified Chinese translations");
    translations.insert("zh-Hans".to_string(), zh_hans_json.clone());
    // Default "zh" to Simplified Chinese
    translations.insert("zh".to_string(), zh_hans_json);

    // Load Traditional Chinese translations (zh-Hant)
    let zh_hant_json: Value = serde_json::from_str(include_str!("../../../assets/i18n/zh-Hant.json"))
        .expect("Failed to parse Traditional Chinese translations");
    translations.insert("zh-Hant".to_string(), zh_hant_json.clone());
    // Also support zh-TW for Traditional Chinese
    translations.insert("zh-TW".to_string(), zh_hant_json);

    translations
}

/// Get translation value from nested JSON structure
pub fn get_translation(translations: &HashMap<String, Value>, locale: &str, key: &str) -> Option<String> {
    let translations_for_locale = translations.get(locale)?;
    let mut current = translations_for_locale;
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

/// Get translation with fallback to English
pub fn get_translation_with_fallback(translations: &HashMap<String, Value>, locale: &str, key: &str) -> String {
    // Try current locale first
    if let Some(translation) = get_translation(translations, locale, key) {
        return translation;
    }
    
    // Fallback to English
    if locale != "en" {
        if let Some(translation) = get_translation(translations, "en", key) {
            return translation;
        }
    }
    
    // Final fallback: return key
    key.to_string()
}

