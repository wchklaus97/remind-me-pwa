// Use deployment utilities for GitHub Pages detection and base path
use crate::deployment::is_github_pages;
#[cfg(target_arch = "wasm32")]
use crate::deployment::get_base_path;

fn path_has_locale_prefix(path: &str) -> bool {
    // Normalize:
    // - Remove leading '#' (hash-routing paths may start with "#/en/app")
    // - Remove GitHub Pages base path (currently hardcoded repo name)
    let path = path.trim_start_matches('#');
    let path = path.trim_start_matches("/remind-me-pwa");

    // Get first segment (e.g. "/en/app" -> "en", "/zh-Hans/app" -> "zh-Hans")
    let first = path
        .trim_start_matches('/')
        .split('/')
        .find(|p| !p.is_empty())
        .unwrap_or("");

    first == "en" || first == "zh" || first.starts_with("zh-")
}

#[derive(Clone, PartialEq)]
pub enum Route {
    Landing,
    App,
}

impl Route {
    pub fn from_path(path: &str) -> (Route, String) {
        // Remove base_path if present (e.g., "/remind-me-pwa/en/app" -> "/en/app")
        let path = path.trim_start_matches("/remind-me-pwa");
        let path = if path.is_empty() { "/" } else { path };
        
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').filter(|p| !p.is_empty()).collect();
        
        // Check for locale prefix: /en/app, /zh/app, /en/, /zh/, etc.
        // Valid locales: en, zh, zh-Hans, zh-Hant, etc.
        if !parts.is_empty() {
            let first_part = parts[0];
            
            // Check if first part is a valid locale code
            if first_part == "en" || first_part == "zh" || first_part.starts_with("zh-") {
                let locale = if first_part == "zh" {
                    "zh".to_string() // Keep "zh" for compatibility
                } else if first_part.starts_with("zh-") {
                    // Map zh-Hans/zh-Hant back to "zh" for i18n
                    "zh".to_string()
                } else {
                    first_part.to_string()
                };
                
                // Check if there's a route after locale
                if parts.len() >= 2 {
                    let route_str = parts[1];
                    match route_str {
                        "app" => return (Route::App, locale),
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
    
    #[allow(dead_code)] // Reserved for future use (e.g., server-side routing)
    pub fn to_path(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("/{}/", locale),
            Route::App => format!("/{}/app", locale),
        }
    }
    
    pub fn to_hash(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("#/{}/", locale),
            Route::App => format!("#/{}/app", locale),
        }
    }
}

pub fn get_initial_route() -> (Route, String) {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        
        // GitHub Pages uses hash-based routing, normal deployments use path-based routing
        // Check hash first (for GitHub Pages), then pathname (for normal deployments)
        if let Ok(hash) = location.hash() {
            let path = hash.trim_start_matches('#');
            if !path.is_empty() {
                return Route::from_path(path);
            }
        }
        
        // Extract path from URL (for normal production deployments with server-side routing)
        if let Ok(pathname) = location.pathname() {
            let (route, locale) = Route::from_path(&pathname);
            // Only use pathname if it contains a locale (not just base_path)
            if path_has_locale_prefix(&pathname) {
                return (route, locale);
            }
        }
        
        // Try to get locale from localStorage (i18nrs storage)
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(saved_locale)) = storage.get_item("remind-me-locale") {
                return (Route::Landing, saved_locale);
            }
        }
    }
    (Route::Landing, "en".to_string())
}

pub fn update_url(route: &Route, locale: &str) {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        
        // GitHub Pages only supports static hosting - use hash-based routing
        // Normal production deployments (Netlify, Vercel, etc.) support server-side routing
        // and can use path-based routing for better SEO
        if is_github_pages() {
            // Use hash-based routing for GitHub Pages (static hosting)
            // Format: #/en/app or #/zh/app
            // Hash routes work on static hosting because they don't require server-side routing
            let hash = route.to_hash(locale);
            let _ = location.set_hash(&hash);
        } else {
            // Use path-based routing for normal production deployments
            // Format: /en/app or /zh/app (with optional base_path)
            // This requires server-side routing support (Netlify, Vercel, etc.)
            // Use History API to update URL without page reload
            #[cfg(target_arch = "wasm32")]
            {
                let base_path = get_base_path();
                let path = route.to_path(locale);
                let full_path = if !base_path.is_empty() {
                    format!("{}{}", base_path, path)
                } else {
                    path
                };

                if let Ok(history) = window.history() {
                    // Use push_state_with_url to update the pathname (not hash)
                    // This creates URLs like /en/app or /remind-me-pwa/en/app
                    // Use wasm_bindgen::JsValue::NULL for null state value
                    use wasm_bindgen::JsValue;
                    match history.push_state_with_url(
                        &JsValue::NULL,
                        "",
                        Some(&full_path),
                    ) {
                        Ok(_) => {
                            // Successfully updated URL path
                            // Path-based routing works on servers that support it (Netlify, Vercel, etc.)
                        }
                        Err(_) => {
                            // If push_state fails, fallback to hash-based routing
                            let hash = route.to_hash(locale);
                            let _ = location.set_hash(&hash);
                        }
                    }
                } else {
                    // Fallback to hash-based routing if History API is unavailable
                    let hash = route.to_hash(locale);
                    let _ = location.set_hash(&hash);
                }
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Non-WASM fallback (shouldn't happen in web builds)
            }
        }
    }
}
