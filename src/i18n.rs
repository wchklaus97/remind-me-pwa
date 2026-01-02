use std::collections::HashMap;
use i18nrs::{I18n, I18nConfig};

#[cfg(target_arch = "wasm32")]
use web_sys::console;

// Load JSON translations and convert to HashMap format expected by i18nrs
// i18nrs expects HashMap<&str, &str> where values are JSON strings
pub fn load_translations() -> HashMap<&'static str, &'static str> {
    // Include JSON files as static strings at compile time
    let en_json = include_str!("../assets/i18n/en.json");
    let zh_json = include_str!("../assets/i18n/zh.json");
    
    HashMap::from([
        ("en", en_json),
        ("zh", zh_json),
    ])
}

// Initialize i18nrs - create config with default language and translations
// CRITICAL FIX: i18nrs panics at config.rs:177 because it accesses languages[0] 
// when config.translations is empty. We MUST put translations in the config!
// Also, I18n::new() sets current_language to the FIRST language in the HashMap,
// so we ensure "en" is first to guarantee it's the default.
pub fn init_i18n() -> Result<I18n, String> {
    let mut translations = load_translations();
    
    // Ensure we have at least English translations
    if !translations.contains_key("en") {
        return Err("English translations are required".to_string());
    }
    
    // CRITICAL: Ensure "en" is the first language in the HashMap
    // I18n::new() sets current_language to languages.first(), so "en" must be first
    // We create a new HashMap with "en" first to guarantee iteration order
    let mut ordered_translations = HashMap::new();
    // Insert "en" first
    if let Some(en_val) = translations.remove("en") {
        ordered_translations.insert("en", en_val);
    }
    // Insert remaining languages
    for (key, value) in translations {
        ordered_translations.insert(key, value);
    }
    
    // CRITICAL: Put translations in the config - i18nrs accesses config.translations.keys()[0]
    // at line 177, and panics if empty. We MUST ensure config.translations is NOT empty!
    let config = I18nConfig {
        translations: ordered_translations.clone(), // Put translations in config!
    };
    
    // Verify config has translations (defensive check)
    if config.translations.is_empty() {
        return Err("Config translations cannot be empty".to_string());
    }
    
    // Create i18n instance - translations are in config, but also pass them
    // I18n::new() will set current_language to the first language ("en")
    let i18n = I18n::new(config, ordered_translations)?;
    
    // Verify current_language is set (should be "en" since it's first)
    // Note: We don't need to call set_translation_language because I18n::new()
    // already sets current_language to the first language in the HashMap
    
    Ok(i18n)
}

// Helper to get i18n instance - returns a default instance if initialization fails
pub fn get_i18n() -> I18n {
    // Try to initialize i18n, fallback to minimal instance if it fails
    init_i18n().unwrap_or_else(|e| {
        // Log error to console for debugging
        #[cfg(target_arch = "wasm32")]
        {
            console::error_1(&format!("Failed to initialize i18n: {}", e).into());
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = e; // Suppress unused variable warning for non-WASM builds
        
        // Create a minimal fallback with at least English translations
        // CRITICAL: Put translations in config to prevent panic at config.rs:177
        // Ensure "en" is first in HashMap so it becomes the default language
        let minimal_translations = HashMap::from([("en", "{}")]);
        let minimal_config = I18nConfig {
            translations: minimal_translations.clone(), // Put in config!
        };
        
        // Create i18n - I18n::new() will set current_language to "en" (first in HashMap)
        I18n::new(minimal_config, minimal_translations)
            .expect("Critical: Unable to create minimal i18n instance")
    })
}
