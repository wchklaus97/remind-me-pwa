//! Routing types and path parsing
//! 
//! This module provides shared routing types and path parsing logic.
//! Platform-specific URL manipulation is in platform crates.

/// Application routes
#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    /// Landing page
    Landing,
    /// Main application (reminder app)
    App,
    /// Privacy policy page
    PrivacyPolicy,
    /// Terms of use page
    TermsOfUse,
}

impl Route {
    /// Parse route and locale from path string
    /// 
    /// Returns (Route, locale_string)
    /// 
    /// Examples:
    /// - "/en/app" -> (Route::App, "en")
    /// - "/zh-Hans/" -> (Route::Landing, "zh-Hans")
    /// - "/app" -> (Route::App, "en") (default locale)
    pub fn from_path(path: &str, base_path: &str) -> (Route, String) {
        // Remove base_path if present (e.g., "/<repo>/en/app" -> "/en/app")
        let path = if !base_path.is_empty() && path.starts_with(base_path) {
            path.strip_prefix(base_path).unwrap_or(path)
        } else {
            path
        };
        
        let path = if path.is_empty() { "/" } else { path };
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').filter(|p| !p.is_empty()).collect();
        
        // Check for locale prefix: /en/app, /zh/app, /en/, /zh/, etc.
        // Valid locales: en, zh, zh-Hans, zh-Hant, etc.
        if !parts.is_empty() {
            let first_part = parts[0];
            
            // Check if first part is a valid locale code
            if first_part == "en" || first_part == "zh" || first_part.starts_with("zh-") {
                let locale = if first_part == "zh" {
                    "zh-Hans".to_string() // Default "zh" to Simplified Chinese
                } else {
                    first_part.to_string() // Preserve zh-Hans, zh-Hant, etc.
                };
                
                // Check if there's a route after locale
                if parts.len() >= 2 {
                    let route_str = parts[1];
                    match route_str {
                        "app" => return (Route::App, locale),
                        "privacy" => return (Route::PrivacyPolicy, locale),
                        "terms" => return (Route::TermsOfUse, locale),
                        _ => return (Route::Landing, locale),
                    }
                } else {
                    // Just locale, no route (e.g., /en/, /zh/)
                    return (Route::Landing, locale);
                }
            }
        }
        
        // Check for simple paths: /app, /#app (without locale prefix)
        if path.contains("/app") || path.contains("#app") {
            return (Route::App, "en".to_string());
        }
        
        // Default to landing page with English
        (Route::Landing, "en".to_string())
    }
    
    /// Convert route to path string with locale
    pub fn to_path(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("/{}/", locale),
            Route::App => format!("/{}/app", locale),
            Route::PrivacyPolicy => format!("/{}/privacy", locale),
            Route::TermsOfUse => format!("/{}/terms", locale),
        }
    }
    
    /// Convert route to hash string with locale (for hash-based routing)
    pub fn to_hash(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("#/{}/", locale),
            Route::App => format!("#/{}/app", locale),
            Route::PrivacyPolicy => format!("#/{}/privacy", locale),
            Route::TermsOfUse => format!("#/{}/terms", locale),
        }
    }
}

/// Check if path has a locale prefix
pub fn path_has_locale_prefix(path: &str, base_path: &str) -> bool {
    // Normalize: Remove leading '#' (hash-routing paths may start with "#/en/app")
    let path = path.trim_start_matches('#');
    let path = if !base_path.is_empty() && path.starts_with(base_path) {
        path.strip_prefix(base_path).unwrap_or(path)
    } else {
        path
    };

    // Get first segment (e.g. "/en/app" -> "en", "/zh-Hans/app" -> "zh-Hans")
    let first = path
        .trim_start_matches('/')
        .split('/')
        .find(|p| !p.is_empty())
        .unwrap_or("");

    first == "en" || first == "zh" || first.starts_with("zh-")
}

