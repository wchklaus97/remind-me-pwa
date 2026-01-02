use dioxus::prelude::*;
use crate::models::Reminder;
use crate::utils::calculate_statistics;
use crate::i18n::use_t;

#[component]
pub fn StatisticsDisplay(reminders: Vec<Reminder>) -> Element {
    let stats = calculate_statistics(&reminders);
    
    rsx! {
        section {
            class: "statistics mb-4",
            div {
                class: "stats-grid",
                div {
                    class: "stat-item",
                    span { class: "stat-value", "{stats.total}" }
                    span { class: "stat-label", {use_t("stats.total")} }
                }
                div {
                    class: "stat-item",
                    span { class: "stat-value", "{stats.active}" }
                    span { class: "stat-label", {use_t("stats.active")} }
                }
                div {
                    class: "stat-item",
                    span { class: "stat-value", "{stats.completed}" }
                    span { class: "stat-label", {use_t("stats.completed")} }
                }
                div {
                    class: "stat-item",
                    span { class: "stat-value", "{stats.overdue}" }
                    span { class: "stat-label", {use_t("stats.overdue")} }
                }
            }
        }
    }
}
