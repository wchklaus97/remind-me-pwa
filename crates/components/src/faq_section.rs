use dioxus::prelude::*;
use remind_me_web::i18n::use_t;
use crate::components::ManagedCachedVideo;

static HERO_LOADING_MP4_120: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_120.mp4");
static HERO_LOADING_POSTER_160: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_160.webp");

#[component]
pub fn FAQSection() -> Element {
    let mut faq_expanded_0 = use_signal(|| false);
    let mut faq_expanded_1 = use_signal(|| false);
    let mut faq_expanded_2 = use_signal(|| false);
    let mut faq_expanded_3 = use_signal(|| false);
    let mut faq_expanded_4 = use_signal(|| false);
    let mut faq_expanded_5 = use_signal(|| false);
    
    rsx! {
        section {
            id: "faq",
            class: "faq-section",
            div { class: "faq-content",
                div { class: "faq-left",
                    div { class: "faq-icon",
                        ManagedCachedVideo {
                            src: HERO_LOADING_MP4_120,
                            poster: HERO_LOADING_POSTER_160,
                            aria_label: Some(use_t("landing.mascot.animation")),
                            title: Some(use_t("landing.mascot.animation")),
                            fallback_text: Some(use_t("landing.mascot.animation")),
                            class: Some("faq-icon-video".to_string()),
                            width: "120".to_string(),
                            height: "120".to_string(),
                        }
                    }
                    h2 { class: "faq-title-main", {use_t("landing.sections.faq.title")} }
                    p { class: "faq-subtitle-main", {use_t("landing.sections.faq.subtitle")} }
                }
                div { class: "faq-list",
                    // 1. What is a PWA?
                    div {
                        class: if faq_expanded_0() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_0.set(!faq_expanded_0()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.pwa.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_0() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_0() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.pwa.a")} }
                        }
                    }
                    // 2. Does Remind Me work offline?
                    div {
                        class: if faq_expanded_1() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_1.set(!faq_expanded_1()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.offline.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_1() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_1() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.offline.a")} }
                        }
                    }
                    // 3. Where is my data stored?
                    div {
                        class: if faq_expanded_2() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_2.set(!faq_expanded_2()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.storage.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_2() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_2() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.storage.a")} }
                        }
                    }
                    // 4. How do I install Remind Me on my phone?
                    div {
                        class: if faq_expanded_3() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_3.set(!faq_expanded_3()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.install.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_3() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_3() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.install.a")} }
                        }
                    }
                    // 5. Is Remind Me really free?
                    div {
                        class: if faq_expanded_4() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_4.set(!faq_expanded_4()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.free.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_4() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_4() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.free.a")} }
                        }
                    }
                    // 6. Can I contribute to the project?
                    div {
                        class: if faq_expanded_5() { "faq-item expanded" } else { "faq-item" },
                        onclick: move |_| faq_expanded_5.set(!faq_expanded_5()),
                        div { class: "faq-item-header",
                            p { class: "faq-q", {use_t("landing.sections.faq.items.contribute.q")} }
                            span { class: "faq-chevron", class: if faq_expanded_5() { "expanded" } else { "" }, "▼" }
                        }
                        if faq_expanded_5() {
                            p { class: "faq-a", {use_t("landing.sections.faq.items.contribute.a")} }
                        }
                    }
                }
            }
        }
    }
}

