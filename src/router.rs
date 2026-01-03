// Use deployment utilities for GitHub Pages detection and base path
#[cfg(target_arch = "wasm32")]
use crate::deployment::get_base_path;

fn strip_base_path_prefix(path: &str) -> &str {
    #[cfg(target_arch = "wasm32")]
    {
        let base_path = get_base_path();
        if !base_path.is_empty() {
            return path.strip_prefix(&base_path).unwrap_or(path);
        }
    }
    path
}

fn path_has_locale_prefix(path: &str) -> bool {
    // Normalize:
    // - Remove leading '#' (hash-routing paths may start with "#/en/app")
    let path = path.trim_start_matches('#');
    let path = strip_base_path_prefix(path);

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
    PrivacyPolicy,
    TermsOfUse,
}

impl Route {
    pub fn from_path(path: &str) -> (Route, String) {
        // Remove base_path if present (e.g., "/<repo>/en/app" -> "/en/app")
        let path = strip_base_path_prefix(path);
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
    
    pub fn to_path(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("/{}/", locale),
            Route::App => format!("/{}/app", locale),
            Route::PrivacyPolicy => format!("/{}/privacy", locale),
            Route::TermsOfUse => format!("/{}/terms", locale),
        }
    }
    
    // Used only in WASM builds as a History API fallback.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub fn to_hash(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("#/{}/", locale),
            Route::App => format!("#/{}/app", locale),
            Route::PrivacyPolicy => format!("#/{}/privacy", locale),
            Route::TermsOfUse => format!("#/{}/terms", locale),
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
        #[cfg(target_arch = "wasm32")]
        {
            // On GitHub Pages, deep links like "/<repo>/en/app" are served via 404.html
            // (often with a 404 status code), which prevents bfcache and can confuse audits.
            // Prefer hash routing there to keep the initial document a 200 OK.
            if crate::deployment::is_github_pages() {
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

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Non-WASM fallback (shouldn't happen in web builds)
            let _ = (window, route, locale);
        }
    }
}

// ---- Landing page section (scroll) URL helpers --------------------------------
//
// We avoid hash fragments (#pricing) and instead use a query parameter:
//   /en/?section=pricing
//
// This keeps URLs shareable + works with History API on GitHub Pages (SPA fallback).

#[cfg(target_arch = "wasm32")]
fn build_landing_section_url(locale: &str, section: Option<&str>) -> String {
    let base_path = get_base_path();
    // Important:
    // - On GitHub Pages, path-based deep links ("/<repo>/en/…") are served via 404.html (often 404),
    //   which causes extra reload-like behavior and prevents bfcache.
    // - Prefer hash routing there: "/<repo>/#/en/…"
    let mut url = if crate::deployment::is_github_pages() {
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
///
/// - GitHub Pages: `/<repo>/#/en/?section=how`
/// - Normal hosts: `/en/?section=how`
pub fn landing_section_href(locale: &str, section: Option<&str>) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        build_landing_section_url(locale, section)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Non-WASM builds (tests/tools) – keep it simple.
        let base = Route::Landing.to_path(locale);
        if let Some(section) = section {
            if !section.is_empty() {
                return format!("{base}?section={section}");
            }
        }
        base
    }
}

// Used by the landing navbar/footer to provide correct href fallbacks.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
const _LANDING_SECTION_HREF_USED: () = ();

/// Read the current landing section from URL (query param preferred; hash fallback).
/// Returns `None` if no valid section is present.
#[cfg(target_arch = "wasm32")]
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

/// Update the landing section in the URL without triggering navigation.
///
/// - Use **push** on click (user intent)
/// - Use **replace** on scrollspy (avoid spamming history)
pub fn push_landing_section_url(locale: &str, section: Option<&str>) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                use wasm_bindgen::JsValue;
                let url = build_landing_section_url(locale, section);
                let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (locale, section);
    }
}

// Used only on the Landing page (WASM build); allow unused in other builds.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn replace_landing_section_url(locale: &str, section: Option<&str>) {
    #[cfg(target_arch = "wasm32")]
    {
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

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (locale, section);
    }
}
