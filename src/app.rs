use dioxus::prelude::*;
use crate::router::{Route, get_initial_route, update_url};
use crate::i18n::{use_init_i18n, use_i18n, Locale};
use crate::components::{LandingPage, ReminderApp, PrivacyPolicyPage, TermsOfUsePage, MediaCacheProvider};
use crate::deployment::get_base_path;
#[cfg(target_arch = "wasm32")]
use crate::deployment::get_base_url;

#[cfg(target_arch = "wasm32")]
fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

// Helper function to map locale code to BCP 47 language code
// This ensures valid lang attribute values for Lighthouse
// en -> en, zh-Hans -> zh-Hans, zh-Hant -> zh-Hant
#[allow(dead_code)] // Used in #[cfg(target_arch = "wasm32")] blocks
fn locale_to_bcp47(locale: &str) -> String {
    match locale {
        "zh" | "zh-CN" => "zh-Hans".to_string(), // Default "zh" to Simplified Chinese
        "zh-Hans" => "zh-Hans".to_string(),
        "zh-Hant" | "zh-TW" => "zh-Hant".to_string(), // Traditional Chinese
        "en" => "en".to_string(),
        _ => locale.to_string(), // Fallback to original if unknown
    }
}

// Get route-specific title and description for SEO
#[allow(dead_code)] // Used in use_effect closure
fn get_route_seo(route: &Route, locale: &str) -> (String, String) {
    // Note: This function is called from use_effect which doesn't have access to i18n context
    // So we use hardcoded strings here, but they match the i18n translations
    match (route, locale) {
        (Route::Landing, "zh") => (
            "提醒我 PWA - 您的个人提醒助手".to_string(),
            "一个简单优雅的提醒应用，帮助您保持条理。支持离线使用，可安装到设备，并保护您的数据隐私。".to_string(),
        ),
        (Route::Landing, _) => (
            "Remind Me PWA - Your Personal Reminder Assistant".to_string(),
            "A beautiful and functional Progressive Web App to help you manage your reminders. Works offline, installs on your device, and keeps your data private.".to_string(),
        ),
        (Route::App, "zh") => (
            "提醒我 - 管理您的提醒事项".to_string(),
            "一个简单优雅的提醒应用，帮助您保持条理。支持离线使用，数据存储在本地设备。".to_string(),
        ),
        (Route::App, _) => (
            "Remind Me - Manage Your Reminders".to_string(),
            "A simple and elegant reminder app to help you stay organized. Works offline, data stored locally on your device.".to_string(),
        ),
        (Route::PrivacyPolicy, _) => (
            "Remind Me - Privacy Policy".to_string(),
            "Privacy Policy for Remind Me PWA. Reminders are stored locally on your device and are not collected by us.".to_string(),
        ),
        (Route::TermsOfUse, _) => (
            "Remind Me - Terms of Use".to_string(),
            "Terms of Use for Remind Me PWA. Free, open-source, offline-first reminder app.".to_string(),
        ),
    }
}

// CSS files - split for better maintainability
static BASE_CSS: Asset = asset!("/assets/css/base.css");
static COMPONENTS_CSS: Asset = asset!("/assets/css/components.css");
static APP_CSS: Asset = asset!("/assets/css/app.css");
static LANDING_CSS: Asset = asset!("/assets/css/landing.css");
static LAYOUT_CSS: Asset = asset!("/assets/css/layout.css");
static UTILITIES_CSS: Asset = asset!("/assets/css/utilities.css");
static RESPONSIVE_CSS: Asset = asset!("/assets/css/responsive.css");

// Favicon and manifest assets - registered with Dioxus asset system
// These ensure the files are included in builds and properly handled
#[allow(dead_code)] // Used implicitly by Dioxus asset system and in JavaScript injection paths
static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");
#[allow(dead_code)]
static FAVICON_16: Asset = asset!("/assets/favicon-16x16.avif");
#[allow(dead_code)]
static FAVICON_ICO: Asset = asset!("/assets/favicon.ico");
#[allow(dead_code)]
static ICON_192: Asset = asset!("/assets/icons/app/icon-192x192.avif");
#[allow(dead_code)]
static ICON_512: Asset = asset!("/assets/icons/app/icon-512x512.avif");
#[allow(dead_code)]
static MANIFEST: Asset = asset!("/assets/manifest.json");

#[component]
pub fn App() -> Element {
    // Provide the i18n context for the entire app
    use_init_i18n();

    // Get access to i18n context
    let mut i18n = use_i18n();

    // Get current locale from URL first, then localStorage, then default to "en"
    // This ensures the locale matches the URL path (e.g., /en/app -> "en", /zh/app -> "zh")
    let mut current_locale = use_signal(|| {
        // Try to get locale from URL first (path-based or hash-based routing)
        let (_, url_locale) = get_initial_route();

        // If URL has an explicit locale in the path OR hash, use it.
        // This is important for GitHub Pages hash routing (e.g. "#/zh/app"),
        // and also ensures <html lang> is correct on the very first render.
        if let Some(window) = web_sys::window() {
            let location = window.location();

            // Check hash first (GitHub Pages style)
            if let Ok(hash) = location.hash() {
                let hash_path = hash.trim_start_matches('#');
                let first = hash_path
                    .trim_start_matches('/')
                    .split('/')
                    .find(|p| !p.is_empty())
                    .unwrap_or("");
                if first == "en" || first == "zh" || first.starts_with("zh-") {
                    return url_locale;
                }
            }

            // Then check pathname (normal deployments)
            if let Ok(pathname) = location.pathname() {
                let base_path = get_base_path();
                let path = if base_path.is_empty() {
                    pathname.as_str()
                } else {
                    pathname.strip_prefix(&base_path).unwrap_or(pathname.as_str())
                };
                let first = path
                    .trim_start_matches('/')
                    .split('/')
                    .find(|p| !p.is_empty())
                    .unwrap_or("");
                if first == "en" || first == "zh" || first.starts_with("zh-") {
                    return url_locale;
                }
            }
        }

        // Fallback to localStorage if URL doesn't have locale
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(locale)) = storage.get_item("remind-me-locale") {
                    return locale;
                }
            }
        }
        "en".to_string()
    });
    
    // Set lang attribute synchronously on initial render (before Lighthouse checks)
    // This must happen immediately, not in use_effect, so Lighthouse can detect it
    // Use BCP 47 language codes (en, zh-Hans) for valid lang attribute values
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let initial_locale = current_locale();
                    let bcp47_lang = locale_to_bcp47(&initial_locale);
                    let _ = html.set_attribute("lang", &bcp47_lang);
                }
            }
        }
    }
    
    // LCP Optimization: Default to Landing immediately (no blocking)
    // Route detection happens asynchronously in use_effect to avoid blocking initial render
    let mut current_route = use_signal(|| Route::Landing);

    // Lighthouse 100%: Set HTML lang attribute, meta tags, and optimize performance
    // Note: We use web_sys here ONLY for DOM manipulation that Dioxus doesn't provide APIs for
    // (removing Google Fonts links from <head> for performance optimization)
    // Priority: Use Dioxus APIs (use_signal, use_effect, etc.) whenever possible
    // Note: Background images now use PNG format (universal browser support) instead of AVIF
    // AVIF has limited browser support and requires complex detection, so PNG is used for reliability
    // Listen for browser back/forward navigation (popstate events)
    // This ensures path-based routing works correctly with browser navigation
    // When user navigates back/forward, the URL changes and we need to update the route
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if let Some(window) = web_sys::window() {
                let mut current_route_signal = current_route;
                let mut current_locale_signal = current_locale;
                let mut i18n_signal = i18n;

                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    // When user navigates back/forward, update route from URL
                    let (detected_route, detected_locale) = get_initial_route();
                    current_route_signal.set(detected_route);

                    // Keep the "current_locale" signal in sync with URL (used for URL updates, lang attribute, etc.)
                    if detected_locale != current_locale_signal() {
                        current_locale_signal.set(detected_locale.clone());
                    }

                    // Always sync i18n with URL locale if it drifted (e.g. localStorage has a different locale).
                    let desired_locale = Locale::from_str(&detected_locale);
                    let i18n_current = i18n_signal.read().current_locale_str();
                    let desired_str = desired_locale.as_str();
                    if i18n_current != desired_str {
                        let mut ctx = i18n_signal.write();
                        ctx.set_locale(desired_locale);
                    }
                }) as Box<dyn FnMut(_)>);

                window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref()).ok();
                closure.forget(); // Keep closure alive
            }
        }
    });

    use_effect(move || {
        // LCP Optimization: Detect route asynchronously after initial render
        // This prevents blocking the initial render with web_sys calls
        let (detected_route, detected_locale) = get_initial_route();
        if detected_route != Route::Landing {
            current_route.set(detected_route);
        }

        // Keep current_locale signal in sync with URL
        if detected_locale != current_locale() {
            current_locale.set(detected_locale.clone());

            // Update URL to reflect locale change (for multi-locale path support)
            // This ensures the URL shows the correct locale path (e.g., /remind-me-pwa/en/app)
            update_url(&current_route(), &current_locale());
        }

        // Also sync the i18n context to match the URL locale even if current_locale already matched.
        // This fixes the case where localStorage had a different locale but the URL shows another.
        let desired_locale = Locale::from_str(&detected_locale);
        let i18n_current = i18n.read().current_locale_str();
        let desired_str = desired_locale.as_str();
        if i18n_current != desired_str {
            let mut ctx = i18n.write();
            ctx.set_locale(desired_locale);
        }

        // Use web_sys ONLY for removing render-blocking Google Fonts links
        // Dioxus doesn't provide APIs for manipulating <head> elements or removing external links
        // This is the minimal web_sys usage necessary for performance optimization
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // LCP Optimization: Critical operations only (lang attribute, font removal)
                    // Non-critical meta tags are deferred to reduce render delay

                    // Update lang attribute on <html> when locale changes (critical for accessibility)
                    // Use BCP 47 language codes for valid lang attribute values
                    if let Some(html) = document.document_element() {
                        let locale = current_locale();
                        let bcp47_lang = locale_to_bcp47(&locale);
                        let _ = html.set_attribute("lang", &bcp47_lang);
                    }

                    // Get head element
                    if let Ok(Some(head)) = document.query_selector("head") {
                        if let Some(head_element) = head.dyn_ref::<web_sys::HtmlElement>() {
                            // Performance: Aggressively remove Google Fonts links (render-blocking)
                            // This must be done immediately and repeatedly to prevent font loading
                            let remove_all_font_links = || {
                                // Remove font stylesheet links
                                if let Ok(font_links) = document
                                    .query_selector_all("link[href*='fonts.googleapis.com']")
                                {
                                    for i in 0..font_links.length() {
                                        if let Some(link) = font_links.get(i) {
                                            if let Some(parent) = link.parent_node() {
                                                let _ = parent.remove_child(&link);
                                            }
                                        }
                                    }
                                }
                                // Remove preconnect to fonts.googleapis.com
                                if let Ok(preconnects) = document.query_selector_all(
                                    "link[rel='preconnect'][href*='fonts.googleapis.com']",
                                ) {
                                    for i in 0..preconnects.length() {
                                        if let Some(preconnect) = preconnects.get(i) {
                                            if let Some(parent) = preconnect.parent_node() {
                                                let _ = parent.remove_child(&preconnect);
                                            }
                                        }
                                    }
                                }
                                // Remove dns-prefetch to fonts.googleapis.com
                                if let Ok(dns_prefetch) = document.query_selector_all(
                                    "link[rel='dns-prefetch'][href*='fonts.googleapis.com']",
                                ) {
                                    for i in 0..dns_prefetch.length() {
                                        if let Some(dns) = dns_prefetch.get(i) {
                                            if let Some(parent) = dns.parent_node() {
                                                let _ = parent.remove_child(&dns);
                                            }
                                        }
                                    }
                                }
                                // Also remove any links to fonts.gstatic.com
                                if let Ok(gstatic_links) =
                                    document.query_selector_all("link[href*='fonts.gstatic.com']")
                                {
                                    for i in 0..gstatic_links.length() {
                                        if let Some(link) = gstatic_links.get(i) {
                                            if let Some(parent) = link.parent_node() {
                                                let _ = parent.remove_child(&link);
                                            }
                                        }
                                    }
                                }
                            };

                            // Remove immediately (multiple times to catch any that appear)
                            remove_all_font_links();

                            // Update title and description based on current route and locale
                            let route = current_route();
                            let locale = current_locale();
                            let (title, description) = get_route_seo(&route, &locale);
                            
                            // Update document title
                            document.set_title(&title);
                            
                            // Update or create meta description
                            let existing_desc = document.query_selector("meta[name='description']");
                            if let Ok(Some(desc_meta)) = existing_desc {
                                let _ = desc_meta.set_attribute("content", &description);
                            } else if let Ok(desc_meta) = document.create_element("meta") {
                                let _ = desc_meta.set_attribute("name", "description");
                                let _ = desc_meta.set_attribute("content", &description);
                                let _ = head_element.append_child(&desc_meta);
                            }

                            // Content Security Policy (CSP) for WebAssembly support
                            // wasm-unsafe-eval is required for WebAssembly.instantiate() to work
                            let existing_csp = document.query_selector("meta[http-equiv='Content-Security-Policy']");
                            if let Ok(None) = existing_csp {
                                if let Ok(meta) = document.create_element("meta") {
                                    let _ = meta.set_attribute("http-equiv", "Content-Security-Policy");
                                    // CSP policy:
                                    // - default-src 'self': Only allow resources from same origin
                                    // - script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval': Allow scripts from same origin, WASM eval, and eval (required for WebAssembly compilation)
                                    // - style-src 'self' 'unsafe-inline': Allow styles from same origin and inline styles (needed for Dioxus)
                                    // - img-src 'self' data: Allow images from same origin and data URIs
                                    // - font-src 'self' data: Allow fonts from same origin and data URIs
                                    let _ = meta.set_attribute("content", "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:;");
                                    let _ = head_element.append_child(&meta);
                                }
                            }

                            // Get base URL and base path for absolute image URLs (Open Graph, Twitter Cards)
                            // Use utility functions for consistency across the app
                            let base_url = get_base_url();
                            let base_path = get_base_path();
                            
                            let add_link_tag = |rel: &str, href: &str, sizes: Option<&str>, link_type: Option<&str>| {
                                if let Ok(link) = document.create_element("link") {
                                    let _ = link.set_attribute("rel", rel);
                                    let _ = link.set_attribute("href", href);
                                    if let Some(sizes_val) = sizes {
                                        let _ = link.set_attribute("sizes", sizes_val);
                                    }
                                    if let Some(type_val) = link_type {
                                        let _ = link.set_attribute("type", type_val);
                                    }
                                    // For preload, also add as="style" to indicate it's a stylesheet
                                    if rel == "preload" && link_type == Some("text/css") {
                                        let _ = link.set_attribute("as", "style");
                                    }
                                    let _ = head_element.append_child(&link);
                                }
                            };

                            // Important: Dioxus serves assets with hashed filenames (e.g. favicon-32x32-dxh....png).
                            // If we link to the unhashed path (e.g. /assets/favicon-32x32.png), the dev server will
                            // fall back to index.html, and the favicon/manifest won't load.
                            //
                            // So we MUST use the Dioxus Asset URLs here.
                            let with_base_path = |href: String| -> String {
                                if base_path.is_empty() {
                                    href
                                } else if href.starts_with(&base_path) {
                                    href
                                } else if href.starts_with('/') {
                                    format!("{}{}", base_path, href)
                                } else {
                                    format!("{}/{}", base_path.trim_end_matches('/'), href)
                                }
                            };

                            let favicon_32_href = with_base_path(FAVICON_32.to_string());
                            let favicon_16_href = with_base_path(FAVICON_16.to_string());
                            let favicon_ico_href = with_base_path(FAVICON_ICO.to_string());
                            let icon_192_href = with_base_path(ICON_192.to_string());
                            let icon_512_href = with_base_path(ICON_512.to_string());
                            let manifest_href = with_base_path(MANIFEST.to_string());

                            // Preload CSS files for faster loading
                            let base_css_href = with_base_path(BASE_CSS.to_string());
                            let components_css_href = with_base_path(COMPONENTS_CSS.to_string());
                            let app_css_href = with_base_path(APP_CSS.to_string());
                            let landing_css_href = with_base_path(LANDING_CSS.to_string());
                            let layout_css_href = with_base_path(LAYOUT_CSS.to_string());
                            let utilities_css_href = with_base_path(UTILITIES_CSS.to_string());
                            let responsive_css_href = with_base_path(RESPONSIVE_CSS.to_string());

                            // Preload critical CSS files
                            add_link_tag("preload", &base_css_href, None, Some("text/css"));
                            add_link_tag("preload", &components_css_href, None, Some("text/css"));
                            add_link_tag("preload", &app_css_href, None, Some("text/css"));
                            add_link_tag("preload", &landing_css_href, None, Some("text/css"));
                            add_link_tag("preload", &layout_css_href, None, Some("text/css"));
                            add_link_tag("preload", &utilities_css_href, None, Some("text/css"));
                            add_link_tag("preload", &responsive_css_href, None, Some("text/css"));

                            // Inject stylesheet links with base_path (ensures correct paths on GitHub Pages)
                            // Dioxus document::Stylesheet should handle this, but we manually inject to ensure base_path is applied
                            add_link_tag("stylesheet", &base_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &components_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &app_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &landing_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &layout_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &utilities_css_href, None, Some("text/css"));
                            add_link_tag("stylesheet", &responsive_css_href, None, Some("text/css"));

                            // Favicons (tab icon)
                            add_link_tag("icon", &favicon_32_href, Some("32x32"), Some("image/avif"));
                            add_link_tag("icon", &favicon_16_href, Some("16x16"), Some("image/avif"));
                            add_link_tag("shortcut icon", &favicon_ico_href, None, Some("image/x-icon"));

                            // PWA / platform icons
                            add_link_tag("icon", &icon_192_href, Some("192x192"), Some("image/avif"));
                            add_link_tag("icon", &icon_512_href, Some("512x512"), Some("image/avif"));

                            // Manifest:
                            // - The file itself is hashed by Dioxus (manifest-dxh....json)
                            // - But the *contents* (icons src) should NOT hardcode hashes.
                            //   Those hashes change whenever the icon files change (and sometimes between builds),
                            //   so we generate a manifest at runtime that points at the current Asset URLs.
                            #[allow(clippy::too_many_lines)]
                            {
                                use wasm_bindgen::JsValue;

                                let manifest_json = serde_json::json!({
                                    "name": "Remind Me PWA",
                                    "short_name": "Remind Me",
                                    "description": description,
                                    "start_url": format!("{}/", base_path.trim_end_matches('/')),
                                    "display": "standalone",
                                    "background_color": "#5e5eb4",
                                    "theme_color": "#5e5eb4",
                                    "orientation": "portrait-primary",
                                    "icons": [
                                        {
                                            "src": icon_192_href,
                                            "sizes": "192x192",
                                            "type": "image/avif",
                                            "purpose": "any maskable"
                                        },
                                        {
                                            "src": icon_512_href,
                                            "sizes": "512x512",
                                            "type": "image/avif",
                                            "purpose": "any maskable"
                                        }
                                    ]
                                });

                                if let Ok(manifest_str) = serde_json::to_string(&manifest_json) {
                                    // Create a blob URL for the manifest
                                    let parts = js_sys::Array::new();
                                    parts.push(&JsValue::from_str(&manifest_str));
                                    if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&parts) {
                                        if let Ok(url) =
                                            web_sys::Url::create_object_url_with_blob(&blob)
                                        {
                                            add_link_tag(
                                                "manifest",
                                                &url,
                                                None,
                                                Some("application/manifest+json"),
                                            );
                                        } else {
                                            // Fallback to hashed manifest file URL
                                            add_link_tag(
                                                "manifest",
                                                &manifest_href,
                                                None,
                                                Some("application/manifest+json"),
                                            );
                                        }
                                    } else {
                                        // Fallback to hashed manifest file URL
                                        add_link_tag(
                                            "manifest",
                                            &manifest_href,
                                            None,
                                            Some("application/manifest+json"),
                                        );
                                    }
                                } else {
                                    // Fallback to hashed manifest file URL
                                    add_link_tag(
                                        "manifest",
                                        &manifest_href,
                                        None,
                                        Some("application/manifest+json"),
                                    );
                                }
                            }

                            // Open Graph meta tags for SEO and social sharing
                            let add_meta_tag =
                                |name: &str, property: Option<&str>, content: &str| {
                                    let selector_value = property.unwrap_or(name);
                                    let selector_attr = if property.is_some() {
                                        "property"
                                    } else {
                                        "name"
                                    };
                                    let existing = document.query_selector(&format!(
                                        "meta[{}='{}']",
                                        selector_attr,
                                        selector_value
                                    ));
                                    if let Ok(None) = existing {
                                        if let Ok(meta) = document.create_element("meta") {
                                            if let Some(prop) = property {
                                                let _ = meta.set_attribute("property", prop);
                                            } else {
                                                let _ = meta.set_attribute("name", name);
                                            }
                                            let _ = meta.set_attribute("content", content);
                                            let _ = head_element.append_child(&meta);
                                        }
                                    }
                                };

                            // Basic Open Graph tags - use dynamic title and description
                            add_meta_tag("", Some("og:type"), "website");
                            
                            // Update or create og:title
                            let existing_og_title = document.query_selector("meta[property='og:title']");
                            if let Ok(Some(og_title_meta)) = existing_og_title {
                                let _ = og_title_meta.set_attribute("content", &title);
                            } else {
                                add_meta_tag("", Some("og:title"), &title);
                            }
                            
                            // Update or create og:description
                            let existing_og_desc = document.query_selector("meta[property='og:description']");
                            if let Ok(Some(og_desc_meta)) = existing_og_desc {
                                let _ = og_desc_meta.set_attribute("content", &description);
                            } else {
                                add_meta_tag("", Some("og:description"), &description);
                            }
                            add_meta_tag("", Some("og:url"), &format!("{}{}", base_url, base_path));
                            add_meta_tag(
                                "",
                                Some("og:image"),
                                &format!(
                                    "{}{}/assets/images/backgrounds/desktop.avif",
                                    base_url, base_path
                                ),
                            );
                            add_meta_tag("", Some("og:image:type"), "image/avif");
                            add_meta_tag("", Some("og:image:width"), "1584");
                            add_meta_tag("", Some("og:image:height"), "672");
                            add_meta_tag(
                                "",
                                Some("og:image:alt"),
                                "Remind Me PWA - Beautiful gradient background",
                            );
                            add_meta_tag("", Some("og:site_name"), "Remind Me PWA");
                            add_meta_tag("", Some("og:locale"), "en_US");

                            // Twitter Card meta tags - use dynamic title and description
                            add_meta_tag("twitter:card", None, "summary_large_image");
                            
                            // Update or create twitter:title
                            let existing_twitter_title = document.query_selector("meta[name='twitter:title']");
                            if let Ok(Some(twitter_title_meta)) = existing_twitter_title {
                                let _ = twitter_title_meta.set_attribute("content", &title);
                            } else {
                                add_meta_tag("twitter:title", None, &title);
                            }
                            
                            // Update or create twitter:description
                            let existing_twitter_desc = document.query_selector("meta[name='twitter:description']");
                            if let Ok(Some(twitter_desc_meta)) = existing_twitter_desc {
                                let _ = twitter_desc_meta.set_attribute("content", &description);
                            } else {
                                add_meta_tag("twitter:description", None, &description);
                            }
                            add_meta_tag(
                                "twitter:image",
                                None,
                                &format!(
                                    "{}{}/assets/images/backgrounds/desktop.avif",
                                    base_url, base_path
                                ),
                            );
                            add_meta_tag(
                                "twitter:image:alt",
                                None,
                                "Remind Me PWA - Beautiful gradient background",
                            );

                            // Additional SEO meta tags
                            add_meta_tag("keywords", None, "reminder app, PWA, task manager, productivity, offline app, progressive web app");
                            add_meta_tag("author", None, "Remind Me PWA");
                            add_meta_tag("robots", None, "index, follow");
                            add_meta_tag("theme-color", None, "#5e5eb4");

                            // Add viewport meta tag if not exists (for SEO and mobile)
                            let existing_viewport =
                                document.query_selector("meta[name='viewport']");
                            if let Ok(None) = existing_viewport {
                                if let Ok(viewport) = document.create_element("meta") {
                                    let _ = viewport.set_attribute("name", "viewport");
                                    let _ = viewport.set_attribute(
                                        "content",
                                        "width=device-width, initial-scale=1.0",
                                    );
                                    let _ = head_element.append_child(&viewport);
                                }
                            }

                            // Add charset meta tag if not exists
                            let existing_charset = document.query_selector("meta[charset]");
                            if let Ok(None) = existing_charset {
                                if let Ok(charset) = document.create_element("meta") {
                                    let _ = charset.set_attribute("charset", "utf-8");
                                    let _ = head_element.insert_before(
                                        &charset,
                                        head_element.first_child().as_ref(),
                                    );
                                }
                            }
                        }
                    }

                    // Title is set above in the use_effect based on route and locale
                }
            }
        }
    });

    rsx! {
        // CSS stylesheets are manually injected in use_effect with base_path support
        // This ensures correct paths on both local dev (base_path = "") and GitHub Pages (base_path = "/remind-me-pwa")
        // The document::Stylesheet components were removed to avoid duplicates

        MediaCacheProvider {
            div {
                class: "app-content-wrapper",
                match current_route() {
                    Route::Landing => rsx! {
                        LandingPage {
                            on_enter_app: move |_| {
                                // Get current locale from i18n context
                                let i18n_read = i18n.read();
                                let locale = i18n_read.current_locale_str().to_string();

                                // Update current_locale signal to match i18n
                                current_locale.set(locale.clone());

                                // Update route and URL with the locale from i18n
                                current_route.set(Route::App);
                                update_url(&Route::App, &locale);
                                #[cfg(target_arch = "wasm32")]
                                scroll_to_top();
                            },
                            on_navigate: move |route: Route| {
                                let locale = current_locale();
                                current_route.set(route.clone());
                                update_url(&route, &locale);
                                #[cfg(target_arch = "wasm32")]
                                if matches!(route, Route::App | Route::PrivacyPolicy | Route::TermsOfUse) {
                                    scroll_to_top();
                                }
                            }
                        }
                    },
                    Route::App => rsx! { ReminderApp {} },
                    Route::PrivacyPolicy => rsx! {
                        PrivacyPolicyPage {
                            on_enter_app: move |_| {
                                let locale = current_locale();
                                current_route.set(Route::App);
                                update_url(&Route::App, &locale);
                                #[cfg(target_arch = "wasm32")]
                                scroll_to_top();
                            },
                            on_jump: move |section: &'static str| {
                                let locale = current_locale();
                                current_route.set(Route::Landing);
                                // LandingPage will handle scrolling on mount based on the URL.
                                crate::router::push_landing_section_url(&locale, Some(section));
                            },
                            on_navigate: move |route: Route| {
                                let locale = current_locale();
                                current_route.set(route.clone());
                                update_url(&route, &locale);
                                #[cfg(target_arch = "wasm32")]
                                if matches!(route, Route::App | Route::PrivacyPolicy | Route::TermsOfUse) {
                                    scroll_to_top();
                                }
                            }
                        }
                    },
                    Route::TermsOfUse => rsx! {
                        TermsOfUsePage {
                            on_enter_app: move |_| {
                                let locale = current_locale();
                                current_route.set(Route::App);
                                update_url(&Route::App, &locale);
                                #[cfg(target_arch = "wasm32")]
                                scroll_to_top();
                            },
                            on_jump: move |section: &'static str| {
                                let locale = current_locale();
                                current_route.set(Route::Landing);
                                crate::router::push_landing_section_url(&locale, Some(section));
                            },
                            on_navigate: move |route: Route| {
                                let locale = current_locale();
                                current_route.set(route.clone());
                                update_url(&route, &locale);
                                #[cfg(target_arch = "wasm32")]
                                if matches!(route, Route::App | Route::PrivacyPolicy | Route::TermsOfUse) {
                                    scroll_to_top();
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
