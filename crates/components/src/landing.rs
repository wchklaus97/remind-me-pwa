use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::i18n::use_current_locale;
use remind_me_shared::router::Route;
// Router functions are platform-specific and will be provided by web/mobile crates
// For now, we'll need to handle this differently - these functions are in web crate
use super::landing_layout::{LandingFooter, LandingNavbar};
use super::FeaturesSection;
use super::HeroSection;
use super::WorkflowSection;
use super::TestimonialsSection;
use super::PricingSection;
use super::FAQSection;
use super::FinalCTASection;
#[cfg(target_arch = "wasm32")]
use dioxus::dioxus_core::use_hook_with_cleanup;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[component]
pub fn LandingPage(on_enter_app: EventHandler<()>, on_navigate: EventHandler<Route>) -> Element {
    let active_section = use_signal(|| "features".to_string());
    let locale = use_current_locale();

    #[cfg(target_arch = "wasm32")]
    fn base_prefix() -> String {
        crate::deployment::get_base_path()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn base_prefix() -> String {
        String::new()
    }

    let base = base_prefix();
    let app_href = format!("{}{}", base, Route::App.to_path(&locale));
    #[cfg(target_arch = "wasm32")]
    let locale_scrollspy = locale.clone();
    #[cfg(target_arch = "wasm32")]
    let locale_mount = locale.clone();

    #[cfg(target_arch = "wasm32")]
    fn scroll_to_section(id: &str) {
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(el) = document.get_element_by_id(id) {
                    // Smooth scroll to the section.
                    // We'll rely on CSS `scroll-margin-top` already present for offsets.
                    let opts = web_sys::ScrollIntoViewOptions::new();
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    opts.set_block(web_sys::ScrollLogicalPosition::Start);
                    let _ = el
                        .dyn_ref::<web_sys::Element>()
                        .map(|e| e.scroll_into_view_with_scroll_into_view_options(&opts));
                }
            }
        }
    }

    // Jump handler for nav (and reused by footer/CTAs).
    let locale_for_jump_nav = locale.clone();
    let mut active_section_nav = active_section;
    let mut jump_to_nav = move |section: &'static str| {
        active_section_nav.set(section.to_string());
        let section_param = if section == "features" { None } else { Some(section) };
        // TODO: push_landing_section_url(&locale_for_jump_nav, section_param);
        #[cfg(target_arch = "wasm32")]
        scroll_to_section(section);
    };

    let locale_for_jump_footer = locale.clone();
    let mut active_section_footer = active_section;
    let mut jump_to_footer = move |section: &'static str| {
        active_section_footer.set(section.to_string());
        let section_param = if section == "features" { None } else { Some(section) };
        // TODO: push_landing_section_url(&locale_for_jump_footer, section_param);
        #[cfg(target_arch = "wasm32")]
        scroll_to_section(section);
    };

    // (Inline CTAs used to scroll to sections; now they navigate to /app instead.)

    // Scrollspy: highlight nav item based on scroll position.
    // Uses scroll event + getBoundingClientRect (no IntersectionObserver features needed).
    #[cfg(target_arch = "wasm32")]
    let _scrollspy_listener = use_hook_with_cleanup(
        {
            let locale_scrollspy = locale_scrollspy.clone();
            move || {
                use wasm_bindgen::closure::Closure;
                use wasm_bindgen::JsCast;

                let Some(window) = web_sys::window() else {
                    return None::<(web_sys::Window, Rc<Closure<dyn FnMut(web_sys::Event)>>)>;
                };
                let Some(document) = window.document() else {
                    return None::<(web_sys::Window, Rc<Closure<dyn FnMut(web_sys::Event)>>)>;
                };

                let document_for_handler = document.clone();
                let mut active_section_signal = active_section;
                let locale_for_url = locale_scrollspy.clone();

                let handler: Rc<Closure<dyn FnMut(web_sys::Event)>> = Rc::new(Closure::wrap(
                    Box::new(move |_e: web_sys::Event| {
                        let header_offset_px = 90.0_f64; // fixed header + spacing

                        let get_top = |id: &str| -> Option<f64> {
                            let el = document_for_handler.get_element_by_id(id)?;
                            Some(el.get_bounding_client_rect().top())
                        };

                        let candidates = ["features", "how", "pricing", "faq"];
                        let mut best: Option<(&str, f64)> = None;

                        for id in candidates.iter() {
                            if let Some(top) = get_top(id) {
                                // Prefer the section closest to the header offset (top near 0).
                                // If it's already above the header, treat distance as small.
                                let dist = (top - header_offset_px).abs();
                                let is_visibleish = top < header_offset_px + 200.0;

                                if is_visibleish {
                                    match best {
                                        None => best = Some((id, dist)),
                                        Some((_best_id, best_dist)) if dist < best_dist => {
                                            best = Some((id, dist));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }

                        if let Some((id, _)) = best {
                            if active_section_signal().as_str() != id {
                                active_section_signal.set(id.to_string());
                                // Keep URL in sync with scroll position (no hash fragments).
                                let section_param = if id == "features" { None } else { Some(id) };
                                // TODO: replace_landing_section_url(&locale_for_url, section_param);
                            }
                        }
                    }) as Box<dyn FnMut(_)>));

                let cb = handler.as_ref().as_ref().unchecked_ref();
                let _ = window.add_event_listener_with_callback("scroll", cb);
                let _ = window.add_event_listener_with_callback("resize", cb);

                // Run once on mount.
                if let Ok(ev) = web_sys::Event::new("scroll") {
                    let _ = window.dispatch_event(&ev);
                }

                Some((window, handler))
            }
        },
        |state: Option<(web_sys::Window, Rc<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>>)>| {
            use wasm_bindgen::JsCast;

            let Some((window, handler)) = state else {
                return;
            };
            let cb = handler.as_ref().as_ref().unchecked_ref();
            let _ = window.remove_event_listener_with_callback("scroll", cb);
            let _ = window.remove_event_listener_with_callback("resize", cb);
        },
    );

    #[cfg(not(target_arch = "wasm32"))]
    let _scrollspy_listener = ();

    // On mount: if URL contains a section param, scroll to it.
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            // TODO: Get section from URL when router functions are available
            // let locale_for_url = locale_mount.clone();
            // if let Some(section) = get_landing_section_from_url() {
            //     let section_param = if section == "features" { None } else { Some(section.as_str()) };
            //     replace_landing_section_url(&locale_for_url, section_param);
            //     let mut active_section_signal = active_section;
            //     active_section_signal.set(section.clone());
            //     scroll_to_section(&section);
            // }
        }
    });

    // Media caching is handled by the shared MediaCacheProvider + ManagedCachedVideo widgets.

    rsx! {
        div {
            class: "landing-page",
            div {
                class: "landing-container",
                div {
                    class: "landing-shell",
                    LandingNavbar {
                        active_section: active_section(),
                        on_enter_app: on_enter_app.clone(),
                        on_jump: move |section: &'static str| {
                            jump_to_nav(section);
                        }
                    }
                    main {
                        role: "main",
                        class: "landing-main",
                        HeroSection {
                            app_href: app_href.clone(),
                            on_enter_app: on_enter_app.clone(),
                        }
                        FeaturesSection {}
                        WorkflowSection {
                            on_enter_app: on_enter_app.clone(),
                        }
                        TestimonialsSection {}
                        PricingSection {
                            on_enter_app: on_enter_app.clone(),
                        }
                        FAQSection {}
                        FinalCTASection {
                            app_href: app_href.clone(),
                            on_enter_app: on_enter_app.clone(),
                        }
                    }
                    LandingFooter {
                        active_section: active_section(),
                        on_jump: move |section: &'static str| {
                            jump_to_footer(section);
                        },
                        on_navigate: on_navigate.clone()
                    }
                }
            }
        }
    }
}

