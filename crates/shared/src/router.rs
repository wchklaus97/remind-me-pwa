//! Routing types and path parsing
//! 
//! This module provides shared routing types and path parsing logic.
//! Platform-specific URL manipulation is in platform crates.

#[cfg(target_arch = "wasm32")]
use web_sys;

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

/// Build landing section URL with query parameter
/// 
/// On web platform, this automatically detects base_path and GitHub Pages deployment.
/// On other platforms, uses default values (empty base_path, not GitHub Pages).
pub fn landing_section_href(locale: &str, section: Option<&str>) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // On web platform, detect base_path and GitHub Pages automatically
        let base_path = get_base_path_web();
        let is_github_pages = is_github_pages_web();
        landing_section_href_impl(locale, section, &base_path, is_github_pages)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        landing_section_href_impl(locale, section, "", false)
    }
}

#[cfg(target_arch = "wasm32")]
fn get_base_path_web() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(hostname) = window.location().hostname() {
            if hostname.contains("github.io") {
                if let Ok(pathname) = window.location().pathname() {
                    if let Some(first) = pathname
                        .trim_start_matches('/')
                        .split('/')
                        .find(|p| !p.is_empty())
                    {
                        return format!("/{}", first);
                    }
                }
            }
        }
    }
    String::new()
}

#[cfg(target_arch = "wasm32")]
fn is_github_pages_web() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(hostname) = window.location().hostname() {
            return hostname.contains("github.io");
        }
    }
    false
}

/// Get initial route and locale from current URL
/// 
/// On web platform, reads from browser location.
/// On other platforms, returns default (Landing, "en").
pub fn get_initial_route() -> (Route, String) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let location = window.location();
            
            // Check hash first (for GitHub Pages), then pathname (for normal deployments)
            if let Ok(hash) = location.hash() {
                let path = hash.trim_start_matches('#');
                if !path.is_empty() {
                    let base_path = get_base_path_web();
                    return Route::from_path(path, &base_path);
                }
            }
            
            // Extract path from URL
            if let Ok(pathname) = location.pathname() {
                let base_path = get_base_path_web();
                let (route, locale) = Route::from_path(&pathname, &base_path);
                if path_has_locale_prefix(&pathname, &base_path) {
                    return (route, locale);
                }
            }
            
            // Try to get locale from localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_locale)) = storage.get_item("remind-me-locale") {
                    return (Route::Landing, saved_locale);
                }
            }
        }
    }
    (Route::Landing, "en".to_string())
}

/// Update URL to reflect current route and locale
/// 
/// On web platform, updates browser history.
/// On other platforms, does nothing.
#[allow(unused_variables)]
pub fn update_url(route: &Route, locale: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let base_path = get_base_path_web();
            let is_github_pages = is_github_pages_web();
            
            if is_github_pages {
                let hash = route.to_hash(locale);
                let _ = window.location().set_hash(&hash);
            } else {
                let location = window.location();
                let path = route.to_path(locale);
                let full_path = if !base_path.is_empty() {
                    format!("{}{}", base_path, path)
                } else {
                    path
                };
                if let Ok(history) = window.history() {
                    use wasm_bindgen::JsValue;
                    if history.push_state_with_url(&JsValue::NULL, "", Some(&full_path)).is_err() {
                        let hash = route.to_hash(locale);
                        let _ = location.set_hash(&hash);
                    }
                } else {
                    let hash = route.to_hash(locale);
                    let _ = location.set_hash(&hash);
                }
            }
        }
    }
}

/// Push landing section URL to browser history
/// 
/// On web platform, updates browser history.
/// On other platforms, does nothing.
pub fn push_landing_section_url(locale: &str, section: Option<&str>) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                use wasm_bindgen::JsValue;
                let base_path = get_base_path_web();
                let is_github_pages = is_github_pages_web();
                let url = landing_section_href_impl(locale, section, &base_path, is_github_pages);
                let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
            }
        }
    }
}

/// Replace landing section URL in browser history
/// 
/// On web platform, updates browser history without adding new entry.
/// On other platforms, does nothing.
pub fn replace_landing_section_url(locale: &str, section: Option<&str>) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let current = window.location();
            let base_path = get_base_path_web();
            let is_github_pages = is_github_pages_web();
            let desired = landing_section_href_impl(locale, section, &base_path, is_github_pages);
            let current_path = current.pathname().unwrap_or_default();
            let current_search = current.search().unwrap_or_default();
            let current_full = format!("{}{}", current_path, current_search);
            
            if current_full != desired {
                if let Ok(history) = window.history() {
                    use wasm_bindgen::JsValue;
                    let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&desired));
                }
            }
        }
    }
}

/// Read the current landing section from URL
/// 
/// On web platform, reads from browser location.
/// On other platforms, returns None.
pub fn get_landing_section_from_url() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        fn get_query_param(search: &str, key: &str) -> Option<String> {
            let search = search.strip_prefix('?').unwrap_or(search);
            if search.is_empty() {
                return None;
            }
            for pair in search.split('&') {
                if pair.is_empty() {
                    continue;
                }
                let mut iter = pair.splitn(2, '=');
                let k = iter.next().unwrap_or("");
                let v = iter.next().unwrap_or("");
                if k == key {
                    return Some(v.to_string());
                }
            }
            None
        }
        
        if let Some(window) = web_sys::window() {
            let location = window.location();
            if let Ok(search) = location.search() {
                if !search.is_empty() {
                    if let Some(section) = get_query_param(&search, "section") {
                        if matches!(section.as_str(), "features" | "how" | "pricing" | "faq") {
                            return Some(section);
                        }
                    }
                }
            }
            if let Ok(hash) = location.hash() {
                let hash = hash.trim_start_matches('#').trim_start_matches('/');
                if let Some((_, search)) = hash.split_once('?') {
                    if let Some(section) = get_query_param(search, "section") {
                        if matches!(section.as_str(), "features" | "how" | "pricing" | "faq") {
                            return Some(section);
                        }
                    }
                }
                let section = hash.trim_start_matches('/');
                if matches!(section, "features" | "how" | "pricing" | "faq") {
                    return Some(section.to_string());
                }
            }
        }
    }
    None
}

/// Internal implementation that accepts base_path and is_github_pages as parameters
fn landing_section_href_impl(locale: &str, section: Option<&str>, base_path: &str, is_github_pages: bool) -> String {
    let mut url = if is_github_pages {
        format!("{}{}", base_path, Route::Landing.to_hash(locale))
    } else {
        let base = Route::Landing.to_path(locale);
        if base_path.is_empty() {
            base
        } else {
            format!("{}{}", base_path, base)
        }
    };

    if let Some(section) = section {
        if !section.is_empty() {
            url.push_str("?section=");
            url.push_str(section);
        }
    }

    url
}

