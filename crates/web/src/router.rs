//! Web-specific router implementation
//! 
//! This module provides web-specific URL manipulation functions.

use remind_me_shared::router::{Route, path_has_locale_prefix};

/// Get initial route and locale from current URL
pub fn get_initial_route() -> (Route, String) {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        
        // GitHub Pages uses hash-based routing, normal deployments use path-based routing
        // Check hash first (for GitHub Pages), then pathname (for normal deployments)
        if let Ok(hash) = location.hash() {
            let path = hash.trim_start_matches('#');
            if !path.is_empty() {
                let base_path = get_base_path();
                return Route::from_path(path, &base_path);
            }
        }
        
        // Extract path from URL (for normal production deployments with server-side routing)
        if let Ok(pathname) = location.pathname() {
            let base_path = get_base_path();
            let (route, locale) = Route::from_path(&pathname, &base_path);
            // Only use pathname if it contains a locale (not just base_path)
            if path_has_locale_prefix(&pathname, &base_path) {
                return (route, locale);
            }
        }
        
        // Try to get locale from localStorage (i18n storage)
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(saved_locale)) = storage.get_item("remind-me-locale") {
                return (Route::Landing, saved_locale);
            }
        }
    }
    (Route::Landing, "en".to_string())
}

/// Update URL to reflect current route and locale
pub fn update_url(route: &Route, locale: &str) {
    if let Some(window) = web_sys::window() {
        // Prefer path-based URLs everywhere (clean shareable links):
        // - Local dev:              /en/app
        // - GitHub Pages (subdir):  /<repo>/en/app
        // - Normal production:      /en/app
        //
        // GitHub Pages is static hosting, so direct loads of /<repo>/en/app
        // are served via 404.html (SPA fallback). Our CI already generates 404.html,
        // so path-based routing is safe there too.
        //
        // If History API pushState fails for any reason, we fall back to hash routes.
        
        // On GitHub Pages, deep links like "/<repo>/en/app" are served via 404.html
        // (often with a 404 status code), which prevents bfcache and can confuse audits.
        // Prefer hash routing there to keep the initial document a 200 OK.
        if is_github_pages() {
            let hash = route.to_hash(locale);
            let _ = window.location().set_hash(&hash);
            return;
        }

        let location = window.location();
        let base_path = get_base_path();
        let path = route.to_path(locale);
        let full_path = if !base_path.is_empty() {
            format!("{}{}", base_path, path)
        } else {
            path
        };

        if let Ok(history) = window.history() {
            use wasm_bindgen::JsValue;
            if history
                .push_state_with_url(&JsValue::NULL, "", Some(&full_path))
                .is_err()
            {
                let hash = route.to_hash(locale);
                let _ = location.set_hash(&hash);
            }
        } else {
            let hash = route.to_hash(locale);
            let _ = location.set_hash(&hash);
        }
    }
}

/// Build landing section URL with query parameter
fn build_landing_section_url(locale: &str, section: Option<&str>) -> String {
    let base_path = get_base_path();
    // Important:
    // - On GitHub Pages, path-based deep links ("/<repo>/en/…") are served via 404.html (often 404),
    //   which causes extra reload-like behavior and prevents bfcache.
    // - Prefer hash routing there: "/<repo>/#/en/…"
    let mut url = if is_github_pages() {
        format!("{}{}", base_path, Route::Landing.to_hash(locale)) // "/<repo>#/en/"
    } else {
        let base = Route::Landing.to_path(locale); // "/en/"
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

/// Generate the correct, shareable landing section URL for the current environment.
pub fn landing_section_href(locale: &str, section: Option<&str>) -> String {
    build_landing_section_url(locale, section)
}

/// Read the current landing section from URL (query param preferred; hash fallback).
pub fn get_landing_section_from_url() -> Option<String> {
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

        // Prefer query parameter (section=pricing)
        if let Ok(search) = location.search() {
            if !search.is_empty() {
                if let Some(section) = get_query_param(&search, "section") {
                    if matches!(section.as_str(), "features" | "how" | "pricing" | "faq") {
                        return Some(section);
                    }
                }
            }
        }

        // Fallback: legacy hash anchors (#pricing)
        if let Ok(hash) = location.hash() {
            let hash = hash.trim_start_matches('#').trim_start_matches('/');

            // Support hash query: "#/en/?section=pricing"
            if let Some((_, search)) = hash.split_once('?') {
                if let Some(section) = get_query_param(search, "section") {
                    if matches!(section.as_str(), "features" | "how" | "pricing" | "faq") {
                        return Some(section);
                    }
                }
            }

            // Legacy: "#pricing"
            let section = hash.trim_start_matches('/');
            if matches!(section, "features" | "how" | "pricing" | "faq") {
                return Some(section.to_string());
            }
        }
    }

    None
}

/// Update the landing section in the URL without triggering navigation (push)
pub fn push_landing_section_url(locale: &str, section: Option<&str>) {
    if let Some(window) = web_sys::window() {
        if let Ok(history) = window.history() {
            use wasm_bindgen::JsValue;
            let url = build_landing_section_url(locale, section);
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
        }
    }
}

/// Update the landing section in the URL without triggering navigation (replace)
pub fn replace_landing_section_url(locale: &str, section: Option<&str>) {
    if let Some(window) = web_sys::window() {
        // Avoid unnecessary replaceState calls if already correct.
        let current = window.location();
        let desired = build_landing_section_url(locale, section);
        let current_path = current.pathname().unwrap_or_default();
        let current_search = current.search().unwrap_or_default();
        let current_full = format!("{}{}", current_path, current_search);

        // For GitHub Pages the pathname includes the base_path, which matches desired.
        if current_full != desired {
            if let Ok(history) = window.history() {
                use wasm_bindgen::JsValue;
                let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&desired));
            }
        }
    }
}

// Use deployment utilities
use crate::deployment::{get_base_path, is_github_pages};

