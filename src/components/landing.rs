use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant, ButtonSize};
use crate::i18n::use_t;
use crate::i18n::use_current_locale;
use crate::router::Route;
use crate::router::push_landing_section_url;
#[cfg(target_arch = "wasm32")]
use crate::router::{get_landing_section_from_url, replace_landing_section_url};
use crate::components::landing_layout::{LandingFooter, LandingNavbar};
use crate::components::ManagedCachedVideo;
#[cfg(target_arch = "wasm32")]
use dioxus::dioxus_core::use_hook_with_cleanup;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

static HERO_LOADING_MP4_120: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_120.mp4");
static HERO_LOADING_POSTER_160: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_160.webp");
static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");
#[allow(dead_code)]
static BRAND_LOADING_MP4_48: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_48.mp4");
#[allow(dead_code)]
static BRAND_LOADING_POSTER_64: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_64.webp");

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
        push_landing_section_url(&locale_for_jump_nav, section_param);
        #[cfg(target_arch = "wasm32")]
        scroll_to_section(section);
    };

    let locale_for_jump_footer = locale.clone();
    let mut active_section_footer = active_section;
    let mut jump_to_footer = move |section: &'static str| {
        active_section_footer.set(section.to_string());
        let section_param = if section == "features" { None } else { Some(section) };
        push_landing_section_url(&locale_for_jump_footer, section_param);
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
                                replace_landing_section_url(&locale_for_url, section_param);
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
            let locale_for_url = locale_mount.clone();
            if let Some(section) = get_landing_section_from_url() {
                // If section is "features", keep the clean base URL.
                let section_param = if section == "features" { None } else { Some(section.as_str()) };
                replace_landing_section_url(&locale_for_url, section_param);
                let mut active_section_signal = active_section;
                active_section_signal.set(section.clone());
                scroll_to_section(&section);
            }
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
                    section {
                        class: "hero-section",
                        div { class: "hero-card",
                            div {
                                class: "hero-icon-tile",
                                aria_hidden: "true",
                                ManagedCachedVideo {
                                    src: HERO_LOADING_MP4_120,
                                    poster: HERO_LOADING_POSTER_160,
                                    aria_label: Some(use_t("landing.mascot.animation")),
                                    title: Some(use_t("landing.mascot.animation")),
                                    fallback_text: Some(use_t("landing.mascot.animation")),
                                    class: None,
                                    width: "120".to_string(),
                                    height: "120".to_string(),
                                }
                            }
                            div { class: "hero-chip", {use_t("landing.hero.chip")} }
                            div {
                                class: "hero-content",
                                h2 {
                                    class: "hero-title",
                                    {use_t("landing.hero.title")}
                                    br {}
                                    span { class: "highlight", {use_t("landing.hero.highlight")} }
                                }
                                p {
                                    class: "hero-description",
                                    {use_t("landing.hero.description")}
                                }
                                div {
                                    class: "hero-actions",
                                    button {
                                        class: "hero-primary",
                                        aria_label: "Add to Home Screen",
                                        onclick: move |_| on_enter_app.call(()),
                                        span { class: "hero-home-icon", "🏠" }
                                        span { class: "hero-home-text", {use_t("landing.hero.add_to_home")} }
                                    }
                                    a {
                                        class: "hero-secondary",
                                        href: "{app_href}",
                                        onclick: move |evt| {
                                            evt.prevent_default();
                                            on_enter_app.call(());
                                        },
                                        img {
                                            class: "hero-cta-icon",
                                            src: FAVICON_32,
                                            width: "36",
                                            height: "36",
                                            alt: "",
                                            aria_hidden: "true",
                                        }
                                        span { class: "hero-cta-text", {use_t("landing.hero.try_now")} }
                                    }
                                }
                                div {
                                    class: "hero-highlights",
                                    div { class: "hero-highlight-card", {use_t("landing.hero.highlights.no_accounts")} }
                                    div { class: "hero-highlight-card", {use_t("landing.hero.highlights.works_offline")} }
                                    div { class: "hero-highlight-card", {use_t("landing.hero.highlights.add_to_home")} }
                                }
                            }
                        }
                    }
                    section {
                        id: "features",
                        class: "features-section",
                        h2 { class: "section-title", {use_t("landing.sections.features.title")} }
                        p { class: "section-subtitle", {use_t("landing.sections.features.subtitle")} }
                        div {
                            class: "features-grid",
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "✅"
                                }
                                h3 { {use_t("landing.feature.manage.title")} }
                                p { {use_t("landing.feature.manage.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "🗓️"
                                }
                                h3 { {use_t("landing.feature.due.title")} }
                                p { {use_t("landing.feature.due.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "🧪"
                                }
                                h3 { {use_t("landing.feature.filtering.title")} }
                                p { {use_t("landing.feature.filtering.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "💾"
                                }
                                h3 { {use_t("landing.feature.offline.title")} }
                                p { {use_t("landing.feature.offline.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "📱"
                                }
                                h3 { {use_t("landing.feature.pwa.title")} }
                                p { {use_t("landing.feature.pwa.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "⚠️"
                                }
                                h3 { {use_t("landing.feature.privacy.title")} }
                                p { {use_t("landing.feature.privacy.description")} }
                            }
                        }
                    }
                    section {
                        id: "how",
                        class: "workflow-section",
                        div { class: "workflow-content",
                            div { class: "workflow-left glass-panel",
                                div { class: "workflow-header", {use_t("landing.sections.how.workflow_header")} }
                                div { class: "workflow-tabs",
                                    span { class: "workflow-tab active", {use_t("landing.sections.how.workflow_tabs.all")} }
                                    span { class: "workflow-tab", {use_t("landing.sections.how.workflow_tabs.active")} }
                                    span { class: "workflow-tab", {use_t("landing.sections.how.workflow_tabs.completed")} }
                                }
                                div { class: "workflow-list",
                                    div { class: "workflow-item overdue",
                                        div { class: "dot" }
                                        div { class: "workflow-text",
                                            p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.send_weekly")} }
                                            p { class: "workflow-meta", {use_t("landing.sections.how.workflow_items.completed_2h_ago")} }
                                        }
                                        span { class: "workflow-chip", {use_t("landing.sections.how.workflow_items.completed")} }
                                    }
                                    div { class: "workflow-item",
                                        div { class: "dot open" }
                                        div { class: "workflow-text",
                                            p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.grocery_this_week")} }
                                            p { class: "workflow-meta", {use_t("landing.sections.how.workflow_items.today_5pm")} }
                                        }
                                    }
                                    div { class: "workflow-item warning",
                                        div { class: "dot alert" }
                                        div { class: "workflow-text",
                                            p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.call_mom")} }
                                            p { class: "workflow-meta", {use_t("landing.sections.how.workflow_items.recurring_weekly")} }
                                        }
                                        span { class: "workflow-chip warning", {use_t("landing.sections.how.workflow_items.overdue")} }
                                    }
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    size: ButtonSize::Large,
                                    onclick: move |_| on_enter_app.call(()),
                                    {use_t("landing.cta.button")}
                                }
                            }
                            div { class: "workflow-right",
                                div { class: "pill", {use_t("landing.sections.how.right.pill")} }
                                h3 { class: "workflow-title-main", {use_t("landing.sections.how.right.title")} }
                                p { class: "workflow-desc",
                                    {use_t("landing.sections.how.right.description")}
                                }
                                div { class: "workflow-actions",
                                    span { class: "pill ghost", {use_t("landing.sections.how.right.features.quick_input")} }
                                    span { class: "pill ghost", {use_t("landing.sections.how.right.features.date_picker")} }
                                    span { class: "pill ghost", {use_t("landing.sections.how.right.features.time_picker")} }
                                    span { class: "pill ghost", {use_t("landing.sections.how.right.features.smart_alerts")} }
                                }
                            }
                        }
                    }
                    section {
                        class: "testimonials-section",
                        h2 { class: "section-title", {use_t("landing.sections.testimonials.title")} }
                        p { class: "section-subtitle", {use_t("landing.sections.testimonials.subtitle")} }
                        div { class: "testimonials-grid",
                            div { class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t("landing.sections.testimonials.items.at.quote")} }
                                div { class: "person", {use_t("landing.sections.testimonials.items.at.person")} }
                            }
                            div { class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t("landing.sections.testimonials.items.mg.quote")} }
                                div { class: "person", {use_t("landing.sections.testimonials.items.mg.person")} }
                            }
                            div { class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t("landing.sections.testimonials.items.dp.quote")} }
                                div { class: "person", {use_t("landing.sections.testimonials.items.dp.person")} }
                            }
                        }
                    }
                    section {
                        id: "pricing",
                        class: "pricing-section",
                        h2 { class: "section-title", {use_t("landing.sections.pricing.title")} }
                        p { class: "section-subtitle", {use_t("landing.sections.pricing.subtitle")} }
                        div { class: "pricing-card",
                            div { class: "pricing-pill", {use_t("landing.sections.pricing.pill")} }
                            div { class: "pricing-icon", "🔔" }
                            h3 { class: "pricing-price", "$0" }
                            p { class: "pricing-label", {use_t("landing.sections.pricing.label")} }
                            ul {
                                li { {use_t("landing.sections.pricing.features.unlimited")} }
                                li { {use_t("landing.sections.pricing.features.due_dates")} }
                                li { {use_t("landing.sections.pricing.features.filtering")} }
                                li { {use_t("landing.sections.pricing.features.local_storage")} }
                            }
                            div { class: "pricing-actions",
                                Button {
                                    variant: ButtonVariant::Primary,
                                    size: ButtonSize::Large,
                                    onclick: move |_| on_enter_app.call(()),
                                    {use_t("landing.cta.button")}
                                }
                                a {
                                    class: "hero-secondary",
                                    href: "https://github.com/wchklaus97/remind-me-pwa",
                                    "GitHub"
                                }
                            }
                        }
                    }
                    section {
                        id: "faq",
                        class: "faq-section",
                        h2 { class: "section-title", {use_t("landing.sections.faq.title")} }
                        p { class: "section-subtitle", {use_t("landing.sections.faq.subtitle")} }
                        div { class: "faq-list",
                            div { class: "faq-item",
                                p { class: "faq-q", {use_t("landing.sections.faq.items.offline.q")} }
                                p { class: "faq-a", {use_t("landing.sections.faq.items.offline.a")} }
                            }
                            div { class: "faq-item",
                                p { class: "faq-q", {use_t("landing.sections.faq.items.storage.q")} }
                                p { class: "faq-a", {use_t("landing.sections.faq.items.storage.a")} }
                            }
                            div { class: "faq-item",
                                p { class: "faq-q", {use_t("landing.sections.faq.items.install.q")} }
                                p { class: "faq-a", {use_t("landing.sections.faq.items.install.a")} }
                            }
                            div { class: "faq-item",
                                p { class: "faq-q", {use_t("landing.sections.faq.items.free.q")} }
                                p { class: "faq-a", {use_t("landing.sections.faq.items.free.a")} }
                            }
                        }
                    }
                    section {
                        class: "cta-final",
                        div { class: "cta-final-card",
                            h2 { {use_t("landing.sections.final_cta.title")} }
                            p { {use_t("landing.sections.final_cta.description")} }
                            div { class: "cta-final-actions",
                                button {
                                    class: "hero-primary",
                                    aria_label: "Add to Home Screen",
                                    onclick: move |_| on_enter_app.call(()),
                                    span { class: "hero-home-icon", "🏠" }
                                    span { class: "hero-home-text", {use_t("landing.hero.add_to_home")} }
                                }
                                a {
                                    class: "hero-secondary",
                                    href: "{app_href}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_enter_app.call(());
                                    },
                                    img {
                                        class: "hero-cta-icon",
                                        src: FAVICON_32,
                                        width: "22",
                                        height: "22",
                                        alt: "",
                                        aria_hidden: "true",
                                    }
                                    span { class: "hero-cta-text", {use_t("landing.hero.try_now")} }
                                }
                            }
                        }
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
