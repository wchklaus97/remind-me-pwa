use dioxus::prelude::*;
use crate::i18n::use_t;
use crate::components::ManagedCachedVideo;

static HERO_LOADING_MP4_120: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_120.mp4");
static HERO_LOADING_POSTER_160: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_160.webp");
static FAVICON_32: Asset = asset!("/assets/favicon-32x32.avif");

#[component]
pub fn HeroSection(
    app_href: String,
    on_enter_app: EventHandler<()>,
) -> Element {
    rsx! {
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
                        div {
                            class: "hero-highlight-card",
                            span { class: "highlight-icon", "✓" }
                            {use_t("landing.hero.highlights.works_offline")}
                        }
                        div {
                            class: "hero-highlight-card",
                            span { class: "highlight-icon", "📅" }
                            {use_t("landing.hero.highlights.due_dates_times")}
                        }
                        div {
                            class: "hero-highlight-card",
                            span { class: "highlight-icon", "📥" }
                            {use_t("landing.hero.highlights.install_any_device")}
                        }
                    }
                }
            }
            div {
                class: "hero-example-card",
                h3 { {use_t("landing.hero.example_card.title")} }
                div {
                    class: "hero-example-list",
                    div {
                        class: "hero-example-item",
                        span { class: "hero-example-item-icon", "✓" }
                        div {
                            class: "hero-example-item-content",
                            p {
                                class: "hero-example-item-title",
                                {use_t("landing.hero.example_card.items.complete_proposal")}
                            }
                            p {
                                class: "hero-example-item-meta",
                                {use_t("landing.hero.example_card.meta.today")}
                            }
                        }
                    }
                    div {
                        class: "hero-example-item",
                        span { class: "hero-example-item-icon", "🔔" }
                        div {
                            class: "hero-example-item-content",
                            p {
                                class: "hero-example-item-title",
                                {use_t("landing.hero.example_card.items.team_meeting")}
                            }
                            p {
                                class: "hero-example-item-meta",
                                {use_t("landing.hero.example_card.meta.time_2_45")}
                            }
                        }
                    }
                    div {
                        class: "hero-example-item overdue",
                        span { class: "hero-example-item-icon", "⏰" }
                        div {
                            class: "hero-example-item-content",
                            p {
                                class: "hero-example-item-title",
                                {use_t("landing.hero.example_card.items.pay_bill")}
                            }
                            p {
                                class: "hero-example-item-meta",
                                {use_t("landing.hero.example_card.meta.overdue")}
                            }
                        }
                    }
                }
            }
        }
    }
}

