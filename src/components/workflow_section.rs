use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant, ButtonSize};
use crate::i18n::use_t;
use crate::components::ManagedCachedVideo;

#[allow(dead_code)]
static BRAND_LOADING_MP4_48: Asset = asset!("/assets/videos/optimized/clay_crab_loading_peek_48.mp4");
#[allow(dead_code)]
static BRAND_LOADING_POSTER_64: Asset =
    asset!("/assets/videos/optimized/clay_crab_loading_peek_poster_64.webp");

#[component]
pub fn WorkflowSection(on_enter_app: EventHandler<()>) -> Element {
    rsx! {
        section {
            id: "how",
            class: "workflow-section",
            div { class: "workflow-content",
                div { class: "workflow-left glass-panel",
                    div {
                        class: "workflow-header",
                        div {
                            class: "workflow-header-text",
                            div { class: "workflow-header-icon",
                                ManagedCachedVideo {
                                    src: BRAND_LOADING_MP4_48,
                                    poster: BRAND_LOADING_POSTER_64,
                                    fallback_text: Some("🔔".to_string()),
                                    class: Some("workflow-header-video".to_string()),
                                    width: "32".to_string(),
                                    height: "32".to_string(),
                                }
                            }
                            {use_t("landing.sections.how.workflow_header")}
                        }
                        span { class: "workflow-item-count", {use_t("landing.sections.how.workflow_item_count")} }
                    }
                    div { class: "workflow-tabs",
                        span { class: "workflow-tab active", {use_t("landing.sections.how.workflow_tabs.all")} }
                        span { class: "workflow-tab", {use_t("landing.sections.how.workflow_tabs.active")} }
                        span { class: "workflow-tab", {use_t("landing.sections.how.workflow_tabs.completed")} }
                    }
                    div { class: "workflow-list",
                        div { class: "workflow-item",
                            span { class: "workflow-item-icon", "○" }
                            div { class: "workflow-text",
                                p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.buy_groceries")} }
                                p { class: "workflow-meta", "📅 " {use_t("landing.sections.how.workflow_items.today_5pm")} }
                            }
                        }
                        div { class: "workflow-item",
                            span { class: "workflow-item-icon", "✓" }
                            div { class: "workflow-text",
                                p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.send_weekly")} }
                                p { class: "workflow-meta", "⏰ " {use_t("landing.sections.how.workflow_items.completed_2h_ago")} }
                            }
                            span { class: "workflow-chip completed", {use_t("landing.sections.how.workflow_items.done")} }
                        }
                        div { class: "workflow-item",
                            span { class: "workflow-item-icon", "🔔" }
                            div { class: "workflow-text",
                                p { class: "workflow-title", {use_t("landing.sections.how.workflow_items.call_mom")} }
                                p { class: "workflow-meta", "⏰ " {use_t("landing.sections.how.workflow_items.days_overdue")} }
                            }
                            span { class: "workflow-chip warning", {use_t("landing.sections.how.workflow_items.overdue")} }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Large,
                        onclick: move |_| on_enter_app.call(()),
                        {use_t("app.header.new_reminder")}
                    }
                }
                div { class: "workflow-right",
                    div { class: "pill", {use_t("landing.sections.how.right.pill")} }
                    h3 { class: "workflow-title-main", {use_t("landing.sections.how.right.title")} }
                    p { class: "workflow-desc",
                        {use_t("landing.sections.how.right.description")}
                    }
                    div { class: "workflow-actions",
                        span { class: "pill ghost", "+ " {use_t("landing.sections.how.right.features.quick_input")} }
                        span { class: "pill ghost", "📅 " {use_t("landing.sections.how.right.features.date_picker")} }
                        span { class: "pill ghost highlighted", "⏰ " {use_t("landing.sections.how.right.features.time_picker")} }
                        span { class: "pill ghost", "🔔 " {use_t("landing.sections.how.right.features.smart_alerts")} }
                    }
                }
            }
        }
    }
}

