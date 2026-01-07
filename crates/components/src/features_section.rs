use dioxus::prelude::*;
use remind_me_web::i18n::use_t;

#[component]
pub fn FeaturesSection() -> Element {
    rsx! {
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
    }
}

