use std::collections::HashMap;
use serde_json::Value;
use dioxus::prelude::*;

// A simple, reliable i18n implementation for Dioxus
#[derive(Clone)]
pub struct I18nContext {
    pub locale: String,
    pub translations: HashMap<String, Value>,
}

impl I18nContext {
    pub fn new() -> Self {
        let translations = load_translations();
        Self {
            locale: "en".to_string(),
            translations,
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = locale.to_string();
    }

    pub fn t(&self, key: &str) -> String {
        get_nested_value(&self.translations[&self.locale], key).unwrap_or_else(|| {
            // Fallback to English if key doesn't exist in current locale
            if self.locale != "en" {
                get_nested_value(&self.translations["en"], key).unwrap_or_else(|| key.to_string())
            } else {
                key.to_string()
            }
        })
    }
}

// Load JSON translations into HashMap
fn load_translations() -> HashMap<String, Value> {
    let mut translations = HashMap::new();
    
    // Load English translations
    let en_json: Value = serde_json::from_str(include_str!("../assets/i18n/en.json"))
        .expect("Failed to parse English translations");
    translations.insert("en".to_string(), en_json);

    // Load Chinese translations
    let zh_json: Value = serde_json::from_str(include_str!("../assets/i18n/zh.json"))
        .expect("Failed to parse Chinese translations");
    translations.insert("zh".to_string(), zh_json);

    translations
}

// Helper function to get nested translation values
fn get_nested_value(json: &Value, key: &str) -> Option<String> {
    let mut current = json;
    let mut path_parts = key.split('.');

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

// Provide the I18nContext as a Dioxus context
pub fn provide_i18n_context(cx: &ScopeState) {
    let i18n_context = I18nContext::new();
    cx.provide_context(i18n_context);
}

// Consume the I18nContext in components
pub fn use_i18n(cx: &ScopeState) -> &UseRef<I18nContext> {
    cx.use_hook(|| cx.consume_context::<I18nContext>())
        .as_ref()
        .unwrap()
}

// A hook to get translated text
pub fn t(cx: &ScopeState, key: &str) -> String {
    let i18n = use_i18n(cx);
    let i18n_val = i18n.read();
    i18n_val.t(key)
}

// A hook to change the locale
pub fn use_set_locale(cx: &ScopeState) -> impl Fn(String) {
    let i18n = use_i18n(cx);
    move |locale: String| {
        i18n.write().set_locale(&locale);
    }
}