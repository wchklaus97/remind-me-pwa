use dioxus::prelude::*;
use remind_me_web::i18n::use_t;
use crate::components::ManagedCachedVideo;

static HERO_LOADING_MP4_120: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_120.mp4");
static HERO_LOADING_POSTER_160: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_160.webp");
static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");

#[component]
pub fn FinalCTASection(
    app_href: String,
    on_enter_app: EventHandler<()>,
) -> Element {
    rsx! {
        section {
            class: "cta-final",
            div { class: "cta-final-card",
                div { class: "cta-final-icon",
                    ManagedCachedVideo {
                        src: HERO_LOADING_MP4_120,
                        poster: HERO_LOADING_POSTER_160,
                        aria_label: Some(use_t("landing.mascot.animation")),
                        title: Some(use_t("landing.mascot.animation")),
                        fallback_text: Some(use_t("landing.mascot.animation")),
                        class: Some("cta-final-video".to_string()),
                        width: "120".to_string(),
                        height: "120".to_string(),
                    }
                }
                h2 { class: "cta-final-title", {use_t("landing.sections.final_cta.title")} }
                p { class: "cta-final-description", {use_t("landing.sections.final_cta.description")} }
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
}

