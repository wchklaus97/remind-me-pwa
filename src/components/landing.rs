use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant, ButtonSize};
use i18nrs::I18n;

#[component]
pub fn LandingPage(on_enter_app: EventHandler<()>, i18n: I18n) -> Element {
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
                            p { class: "tagline", {i18n.t("app.tagline")} }
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
                                {i18n.t("landing.hero.title")}
                                br {}
                                span { class: "highlight", {i18n.t("landing.hero.highlight")} }
                            }
                            p {
                                class: "hero-description",
                                {i18n.t("landing.hero.description")}
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
                                h3 { {i18n.t("landing.feature.pwa.title")} }
                                p { {i18n.t("landing.feature.pwa.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "💾"
                                }
                                h3 { {i18n.t("landing.feature.offline.title")} }
                                p { {i18n.t("landing.feature.offline.description")} }
                            }
                            div {
                                class: "feature-card",
                                div {
                                    class: "feature-icon",
                                    "🔒"
                                }
                                h3 { {i18n.t("landing.feature.privacy.title")} }
                                p { {i18n.t("landing.feature.privacy.description")} }
                            }
                        }
                    }
                    section {
                        class: "cta-section",
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Large,
                            onclick: move |_| on_enter_app.call(()),
                            {i18n.t("landing.cta.button")}
                        }
                        p {
                            class: "cta-subtitle",
                            {i18n.t("landing.cta.subtitle")}
                        }
                    }
                }
            }
        }
    }
}
