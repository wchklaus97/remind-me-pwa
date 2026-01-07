use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::i18n::use_t;
use super::ManagedCachedVideo;

static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");
#[allow(dead_code)]
static BRAND_LOADING_MP4_48: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_48.mp4");
#[allow(dead_code)]
static BRAND_LOADING_POSTER_64: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_64.webp");

#[component]
pub fn PricingSection(on_enter_app: EventHandler<()>) -> Element {
    rsx! {
        section {
            id: "pricing",
            class: "pricing-section",
            h2 { class: "section-title", {use_t("landing.sections.pricing.title")} }
            p { class: "section-subtitle", {use_t("landing.sections.pricing.subtitle")} }
            div { class: "pricing-card",
                div { class: "pricing-pill", {use_t("landing.sections.pricing.pill")} }
                div { class: "pricing-icon",
                    ManagedCachedVideo {
                        src: BRAND_LOADING_MP4_48,
                        poster: BRAND_LOADING_POSTER_64,
                        fallback_text: Some("🔔".to_string()),
                        class: Some("pricing-icon-video".to_string()),
                        width: "64".to_string(),
                        height: "64".to_string(),
                    }
                }
                h3 { class: "pricing-price", "$0" span { class: "pricing-forever", {use_t("landing.sections.pricing.forever")} } }
                p { class: "pricing-label", {use_t("landing.sections.pricing.label")} }
                ul {
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.unlimited")}
                    }
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.due_dates")}
                    }
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.filtering")}
                    }
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.local_storage")}
                    }
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.offline")}
                    }
                    li {
                        span { class: "feature-check", "✓" }
                        {use_t("landing.sections.pricing.features.pwa")}
                    }
                }
                div { class: "pricing-actions",
                    button {
                        class: "pricing-button",
                        onclick: move |_| on_enter_app.call(()),
                        img {
                            class: "pricing-button-icon",
                            src: FAVICON_32,
                            width: "22",
                            height: "22",
                            alt: "",
                            aria_hidden: "true",
                        }
                        span { class: "pricing-button-text", {use_t("landing.sections.pricing.button")} }
                    }
                }
            }
        }
    }
}

