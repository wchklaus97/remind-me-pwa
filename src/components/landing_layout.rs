use dioxus::prelude::*;

use crate::i18n::use_t;
use crate::i18n::use_current_locale;
use crate::router::Route;
use crate::components::{ManagedCachedVideo, LanguageSwitcher};

static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");

static BRAND_LOADING_MP4_48: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_48.mp4");
static BRAND_LOADING_POSTER_64: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_64.webp");

#[component]
pub fn LandingNavbar(
    active_section: String,
    on_enter_app: EventHandler<()>,
    on_jump: EventHandler<&'static str>,
) -> Element {
    let is_features = active_section == "features";
    let is_how = active_section == "how";
    let is_pricing = active_section == "pricing";
    let is_faq = active_section == "faq";
    let mut menu_open = use_signal(|| false);

    // Lock background scroll while the fullscreen menu is open (WASM only).
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        if menu_open() {
                            let _ = html.set_attribute("data-menu-open", "true");
                        } else {
                            let _ = html.remove_attribute("data-menu-open");
                        }
                    }
                }
            }
        }
    });

    rsx! {
        header {
            role: "banner",
            class: "landing-header",
            div { class: "landing-header-inner",
                div { class: "nav-left",
                    div {
                        class: "brand-mark",
                        aria_hidden: "true",
                        ManagedCachedVideo {
                            src: BRAND_LOADING_MP4_48,
                            poster: BRAND_LOADING_POSTER_64,
                            aria_label: Some(use_t("landing.mascot.animation")),
                            title: Some(use_t("landing.mascot.animation")),
                            fallback_text: Some(use_t("landing.mascot.animation")),
                            class: None,
                            width: "48".to_string(),
                            height: "48".to_string(),
                        }
                    }
                    div { class: "brand-text",
                        h1 { "Remind Me" }
                        p { class: "tagline", {use_t("app.tagline")} }
                    }
                }
                nav {
                    role: "navigation",
                    aria_label: "Landing navigation",
                    class: "nav-links",
                    a {
                        href: "section=features",
                        class: if is_features { "active" } else { "" },
                        aria_label: "Go to Features section",
                        onclick: move |evt| {
                            evt.prevent_default();
                            on_jump.call("features");
                        },
                        {use_t("landing.nav.features")}
                    }
                    a {
                        href: "section=how",
                        class: if is_how { "active" } else { "" },
                        aria_label: "Go to How it works section",
                        onclick: move |evt| {
                            evt.prevent_default();
                            on_jump.call("how");
                        },
                        {use_t("landing.nav.how")}
                    }
                    a {
                        href: "section=pricing",
                        class: if is_pricing { "active" } else { "" },
                        aria_label: "Go to Pricing section",
                        onclick: move |evt| {
                            evt.prevent_default();
                            on_jump.call("pricing");
                        },
                        {use_t("landing.nav.pricing")}
                    }
                    a {
                        href: "section=faq",
                        class: if is_faq { "active" } else { "" },
                        aria_label: "Go to FAQ section",
                        onclick: move |evt| {
                            evt.prevent_default();
                            on_jump.call("faq");
                        },
                        {use_t("landing.nav.faq")}
                    }
                }
                div { class: "nav-right",
                    LanguageSwitcher { class: Some("nav-lang-switcher".to_string()) }
                    button {
                        class: "nav-menu-button",
                        aria_label: if menu_open() { "Close menu" } else { "Open menu" },
                        aria_expanded: if menu_open() { "true" } else { "false" },
                        aria_controls: "landing-menu",
                        onclick: move |_| menu_open.set(!menu_open()),
                        if menu_open() { "✕" } else { "☰" }
                    }
                    button {
                        class: "nav-cta",
                        aria_label: "Start using Remind Me",
                        onclick: move |_| on_enter_app.call(()),
                        img {
                            class: "cta-icon",
                            src: FAVICON_32,
                            width: "18",
                            height: "18",
                            alt: "",
                            aria_hidden: "true",
                        }
                        span { class: "cta-text", {use_t("landing.cta.button")} }
                    }
                }
            }

            // Tablet/mobile menu (shown via CSS + menu_open state)
            if menu_open() {
                div {
                    class: "nav-menu-overlay",
                    role: "dialog",
                    aria_modal: "true",
                    aria_label: "Navigation menu",
                    nav {
                        id: "landing-menu",
                        class: "nav-menu-panel",
                        role: "navigation",
                        aria_label: "Landing menu",
                        div { class: "nav-menu-top",
                            div { class: "nav-menu-brand",
                                div { class: "brand-mark", aria_hidden: "true",
                                    ManagedCachedVideo {
                                        src: BRAND_LOADING_MP4_48,
                                        poster: BRAND_LOADING_POSTER_64,
                                        aria_label: Some("Remind Me 吉祥物動畫".to_string()),
                                        title: Some("Remind Me 吉祥物動畫".to_string()),
                                        fallback_text: Some("Remind Me 吉祥物動畫".to_string()),
                                        class: None,
                                        width: "48".to_string(),
                                        height: "48".to_string(),
                                    }
                                }
                                div { class: "nav-menu-brand-text",
                                    p { class: "nav-menu-brand-title", "Remind Me" }
                                    p { class: "nav-menu-brand-sub", {use_t("app.tagline")} }
                                }
                            }
                            button {
                                class: "nav-menu-close",
                                aria_label: "Close menu",
                                onclick: move |_| menu_open.set(false),
                                "✕"
                            }
                        }

                        div { class: "nav-menu-body",
                            // Top group: navigation links
                            div { class: "nav-menu-list",
                                button {
                                    class: if is_features { "nav-menu-item active" } else { "nav-menu-item" },
                                    aria_label: "Go to Features section",
                                    onclick: move |_| {
                                        menu_open.set(false);
                                        on_jump.call("features");
                                    },
                                    {use_t("landing.nav.features")}
                                }
                                button {
                                    class: if is_how { "nav-menu-item active" } else { "nav-menu-item" },
                                    aria_label: "Go to How it works section",
                                    onclick: move |_| {
                                        menu_open.set(false);
                                        on_jump.call("how");
                                    },
                                    {use_t("landing.nav.how")}
                                }
                                button {
                                    class: if is_pricing { "nav-menu-item active" } else { "nav-menu-item" },
                                    aria_label: "Go to Pricing section",
                                    onclick: move |_| {
                                        menu_open.set(false);
                                        on_jump.call("pricing");
                                    },
                                    {use_t("landing.nav.pricing")}
                                }
                                button {
                                    class: if is_faq { "nav-menu-item active" } else { "nav-menu-item" },
                                    aria_label: "Go to FAQ section",
                                    onclick: move |_| {
                                        menu_open.set(false);
                                        on_jump.call("faq");
                                    },
                                    {use_t("landing.nav.faq")}
                                }
                            }

                            // Bottom group: primary CTA (separate concept)
                            div { class: "nav-menu-footer",
                                div { class: "nav-menu-footer-row",
                                    LanguageSwitcher { class: Some("nav-menu-lang-switcher".to_string()) }
                                }
                                button {
                                    class: "nav-menu-item nav-menu-cta",
                                    aria_label: "Try it",
                                    onclick: move |_| {
                                        menu_open.set(false);
                                        on_enter_app.call(());
                                    },
                                    img {
                                        class: "cta-icon",
                                        src: FAVICON_32,
                                        width: "18",
                                        height: "18",
                                        alt: "",
                                        aria_hidden: "true",
                                    }
                                    span { class: "cta-text", {use_t("landing.cta.button")} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LandingFooter(
    active_section: String,
    on_jump: EventHandler<&'static str>,
    on_navigate: EventHandler<Route>,
) -> Element {
    let is_features = active_section == "features";
    let is_how = active_section == "how";
    let is_pricing = active_section == "pricing";
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
    let privacy_href = format!("{}{}", base, Route::PrivacyPolicy.to_path(&locale));
    let terms_href = format!("{}{}", base, Route::TermsOfUse.to_path(&locale));

    rsx! {
        footer {
            role: "contentinfo",
            class: "landing-footer",
            div { class: "landing-footer-inner",
                div { class: "footer-grid",
                    div { class: "footer-brand",
                        div {
                            class: "brand-mark",
                            aria_hidden: "true",
                            ManagedCachedVideo {
                                src: BRAND_LOADING_MP4_48,
                                poster: BRAND_LOADING_POSTER_64,
                                aria_label: Some(use_t("landing.mascot.animation")),
                                title: Some(use_t("landing.mascot.animation")),
                                fallback_text: Some(use_t("landing.mascot.animation")),
                                class: None,
                                width: "48".to_string(),
                                height: "48".to_string(),
                            }
                        }
                        div {
                            class: "footer-brand-text",
                            p { class: "footer-brand-title", "Remind Me" }
                            p {
                                class: "footer-brand-desc",
                                {use_t("landing.footer.description")}
                            }
                        }
                    }
                    nav { class: "footer-col", aria_label: "{use_t(\"landing.footer.aria.products\")}",
                        h3 { class: "footer-col-title", {use_t("landing.footer.products")} }
                        ul { class: "footer-links",
                            li {
                                a {
                                    href: "section=features",
                                    class: if is_features { "active" } else { "" },
                                    aria_label: "{use_t(\"landing.footer.aria.features\")}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_jump.call("features");
                                    },
                                    {use_t("landing.footer.links.features")}
                                }
                            }
                            li {
                                a {
                                    href: "section=how",
                                    class: if is_how { "active" } else { "" },
                                    aria_label: "{use_t(\"landing.footer.aria.how\")}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_jump.call("how");
                                    },
                                    {use_t("landing.footer.links.how")}
                                }
                            }
                            li {
                                a {
                                    href: "section=pricing",
                                    class: if is_pricing { "active" } else { "" },
                                    aria_label: "{use_t(\"landing.footer.aria.pricing\")}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_jump.call("pricing");
                                    },
                                    {use_t("landing.footer.links.pricing")}
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/wchklaus97/remind-me-pwa/blob/main/CHANGELOG.md",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.changelog\")}",
                                    {use_t("landing.footer.links.changelog")}
                                }
                            }
                        }
                    }
                    nav { class: "footer-col", aria_label: "{use_t(\"landing.footer.aria.resources\")}",
                        h3 { class: "footer-col-title", {use_t("landing.footer.resources")} }
                        ul { class: "footer-links",
                            li {
                                a {
                                    href: "https://github.com/wchklaus97/remind-me-pwa",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.github\")}",
                                    {use_t("landing.footer.links.github")}
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/wchklaus97/remind-me-pwa#readme",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.docs\")}",
                                    {use_t("landing.footer.links.docs")}
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/wchklaus97/remind-me-pwa/issues?q=is%3Aissue+is%3Aopen",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.roadmap\")}",
                                    {use_t("landing.footer.links.roadmap")}
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/wchklaus97/remind-me-pwa/issues",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.report_issue\")}",
                                    {use_t("landing.footer.links.report_issue")}
                                }
                            }
                        }
                    }
                    nav { class: "footer-col", aria_label: "{use_t(\"landing.footer.aria.legal\")}",
                        h3 { class: "footer-col-title", {use_t("landing.footer.legal")} }
                        ul { class: "footer-links",
                            li {
                                a {
                                    href: "{privacy_href}",
                                    aria_label: "{use_t(\"landing.footer.aria.privacy\")}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_navigate.call(Route::PrivacyPolicy);
                                    },
                                    {use_t("landing.footer.links.privacy")}
                                }
                            }
                            li {
                                a {
                                    href: "{terms_href}",
                                    aria_label: "{use_t(\"landing.footer.aria.terms\")}",
                                    onclick: move |evt| {
                                        evt.prevent_default();
                                        on_navigate.call(Route::TermsOfUse);
                                    },
                                    {use_t("landing.footer.links.terms")}
                                }
                            }
                            li {
                                a {
                                    href: "https://opensource.org/license/mit/",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{use_t(\"landing.footer.aria.mit\")}",
                                    {use_t("landing.footer.links.mit")}
                                }
                            }
                        }
                    }
                }
            }
            // Footer bottom bar removed (per UI request)
        }
    }
}


