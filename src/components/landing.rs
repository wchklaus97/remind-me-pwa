use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant, ButtonSize};
use crate::i18n::use_t;

#[component]
pub fn LandingPage(on_enter_app: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "landing-page",
            div {
                class: "landing-container",
                header {
                    role: "banner",
                    class: "landing-header",
                    div {
                        class: "logo-section",
                        div {
                            class: "logo-icon",
                            "🔔"
                        }
                        div {
                            class: "logo-text",
                            h1 { "Remind Me" }
                            p { class: "tagline", {use_t("app.tagline")} }
                        }
                    }
                }
                main {
                    role: "main",
                    class: "landing-main",
                    section {
                        class: "hero-section",
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
                        }
                    }
                    section {
                        class: "features-section",
                        div {
                            class: "features-grid",
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
                                    "💾"
                                }
                                h3 { {use_t("landing.feature.offline.title")} }
                                p { {use_t("landing.feature.offline.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "🔒"
                                }
                                h3 { {use_t("landing.feature.privacy.title")} }
                                p { {use_t("landing.feature.privacy.description")} }
                            }
                        }
                    }
                    section {
                        class: "cta-section",
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Large,
                            onclick: move |_| on_enter_app.call(()),
                            {use_t("landing.cta.button")}
                        }
                        p {
                            class: "cta-subtitle",
                            {use_t("landing.cta.subtitle")}
                        }
                    }
                }
            }
        }
    }
}
