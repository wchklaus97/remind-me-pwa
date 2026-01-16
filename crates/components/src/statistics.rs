use dioxus::prelude::*;
use remind_me_shared::models::Reminder;
use remind_me_shared::utils::calculate_statistics;
use crate::i18n::use_t;

#[component]
pub fn StatisticsDisplay(reminders: Vec<Reminder>) -> Element {
    let stats = calculate_statistics(&reminders);
    
    rsx! {
        section {
            class: "summary-row",
            div {
                class: "summary-card",
                div {
                    class: "summary-icon done-icon",
                    "✓"
                }
                div {
                    class: "summary-value",
                    "{stats.completed}"
                }
                div {
                    class: "summary-label",
                    "DONE TODAY"
                }
            }
            div {
                class: "summary-card",
                div {
                    class: "summary-icon pending-icon",
                    "🕒"
                }
                div {
                    class: "summary-value",
                    "{stats.active}"
                }
                div {
                    class: "summary-label",
                    "PENDING"
                }
            }
            div {
                class: "summary-card",
                div {
                    class: "summary-icon total-icon",
                    "📋"
                }
                div {
                    class: "summary-value",
                    "{stats.total}"
                }
                div {
                    class: "summary-label",
                    "TOTAL"
                }
            }
        }
    }
}
