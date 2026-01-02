use dioxus::prelude::*;
use crate::router::{Route, get_initial_route, update_url};
use crate::i18n::{use_init_i18n, use_i18n, Locale};
use crate::components::{LandingPage, ReminderApp};
#[cfg(target_arch = "wasm32")]
use crate::deployment::{get_base_path, get_base_url};

// Helper function to map locale code to BCP 47 language code
// This ensures valid lang attribute values for Lighthouse
// en -> en, zh -> zh-Hans (Simplified Chinese)
#[allow(dead_code)] // Used in #[cfg(target_arch = "wasm32")] blocks
fn locale_to_bcp47(locale: &str) -> String {
    match locale {
        "zh" => "zh-Hans".to_string(), // Simplified Chinese
        "zh-CN" => "zh-Hans".to_string(),
        "zh-TW" => "zh-Hant".to_string(), // Traditional Chinese
        "en" => "en".to_string(),
        _ => locale.to_string(), // Fallback to original if unknown
    }
}

// Background images using Dioxus 0.7 asset! macro for proper bundling
// Using AVIF format for better performance (smaller file size, better compression)
// Modern browsers support AVIF, with PNG as fallback if needed
static MOBILE_BG: Asset = asset!("/assets/images/backgrounds/mobile.avif");
static DESKTOP_BG: Asset = asset!("/assets/images/backgrounds/desktop.avif");

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
                let path = pathname.trim_start_matches("/remind-me-pwa");
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
                    let desired_locale = if detected_locale == "zh" {
                        Locale::Zh
                    } else {
                        Locale::En
                    };
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
        // This fixes the case where localStorage had "zh" but the URL is "/en/app".
        let desired_locale = if detected_locale == "zh" {
            Locale::Zh
        } else {
            Locale::En
        };
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

                            // LCP Optimization: Non-critical meta tags (these don't block rendering)
                            // Set meta description if not exists
                            let existing_meta = document.query_selector("meta[name='description']");
                            if let Ok(None) = existing_meta {
                                if let Ok(meta) = document.create_element("meta") {
                                    let _ = meta.set_attribute("name", "description");
                                    let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
                                    let _ = head_element.append_child(&meta);
                                }
                            }

                            // Content Security Policy (CSP) for WebAssembly support
                            // wasm-unsafe-eval is required for WebAssembly.instantiate() to work
                            let existing_csp = document.query_selector("meta[http-equiv='Content-Security-Policy']");
                            if let Ok(None) = existing_csp {
                                if let Ok(meta) = document.create_element("meta") {
                                    let _ = meta.set_attribute("http-equiv", "Content-Security-Policy");
                                    // CSP policy:
                                    // - default-src 'self': Only allow resources from same origin
                                    // - script-src 'self' 'wasm-unsafe-eval': Allow scripts from same origin and WASM eval (required for WebAssembly)
                                    // - style-src 'self' 'unsafe-inline': Allow styles from same origin and inline styles (needed for Dioxus)
                                    let _ = meta.set_attribute("content", "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline';");
                                    let _ = head_element.append_child(&meta);
                                }
                            }

                            // Get base URL and base path for absolute image URLs (Open Graph, Twitter Cards)
                            // Use utility functions for consistency across the app
                            let base_url = get_base_url();
                            let base_path = get_base_path();

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

                            // Basic Open Graph tags
                            add_meta_tag("", Some("og:type"), "website");
                            add_meta_tag(
                                "",
                                Some("og:title"),
                                "Remind Me PWA - Your Personal Reminder Assistant",
                            );
                            add_meta_tag(
                                "",
                                Some("og:description"),
                                "A simple and elegant reminder app to help you stay organized",
                            );
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

                            // Twitter Card meta tags
                            add_meta_tag("twitter:card", None, "summary_large_image");
                            add_meta_tag(
                                "twitter:title",
                                None,
                                "Remind Me PWA - Your Personal Reminder Assistant",
                            );
                            add_meta_tag(
                                "twitter:description",
                                None,
                                "A simple and elegant reminder app to help you stay organized",
                            );
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

                    // Set title if not exists
                    if document.title().is_empty() {
                        document.set_title("Remind Me PWA - Your Personal Reminder Assistant");
                    }
                }
            }
        }
    });

    // Background images using Dioxus 0.7 asset! macro - properly bundled and optimized
    // Dioxus asset! macro automatically handles base_path when set in Dioxus.toml during build
    // When base_path = "/remind-me-pwa" is set in CI/CD (via GITHUB_PAGES_BASE_PATH env var),
    // Dioxus will automatically prepend it to all asset URLs at build time.
    // 
    // Build-time behavior:
    // - Local dev (base_path = ""): "/assets/mobile-xxx.avif"
    // - GitHub Pages (base_path = "/remind-me-pwa"): "/remind-me-pwa/assets/mobile-xxx.avif"
    //
    // We trust Dioxus to handle this correctly, so we use the asset URLs directly.
    let mobile_bg_str = format!("{}", MOBILE_BG);
    let desktop_bg_str = format!("{}", DESKTOP_BG);
    
    rsx! {
        div {
            class: "dioxus-root app-background",
            style: "position: fixed !important; top: 0 !important; left: 0 !important; right: 0 !important; bottom: 0 !important; width: 100vw !important; height: 100vh !important; margin: 0 !important; padding: 0 !important; z-index: -999 !important; background-color: #5e5eb4 !important; background-image: url('{mobile_bg_str}') !important; background-size: cover !important; background-position: center center !important; background-repeat: no-repeat !important; background-attachment: fixed !important; pointer-events: none !important; display: block !important; visibility: visible !important; opacity: 1 !important; --mobile-bg: url('{mobile_bg_str}'); --desktop-bg: url('{desktop_bg_str}');",
        }
        div {
            class: "app-content-wrapper",
            if current_route() == Route::Landing {
                LandingPage {
                    on_enter_app: move |_| {
                        // Get current locale from i18n context
                        let i18n_read = i18n.read();
                        let locale = i18n_read.current_locale_str().to_string();

                        // Update current_locale signal to match i18n
                        current_locale.set(locale.clone());

                        // Update route and URL with the locale from i18n
                        // The URL will show the locale (e.g., /en/app or /zh/app)
                        current_route.set(Route::App);
                        update_url(&Route::App, &locale);
                    },
                }
            } else {
                ReminderApp {}
            }
        }
    }
}
