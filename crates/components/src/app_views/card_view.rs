use dioxus::prelude::*;
use remind_me_shared::models::{Reminder, Tag};
use super::ReminderCard;

#[component]
pub fn CardView(
    reminders: Vec<Reminder>,
    tags: Vec<Tag>,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    rsx! {
        section {
            class: "reminders-card-view",
            aria_label: "Card view of reminders",
            div {
                class: "card-grid",
                for (index, reminder) in reminders.iter().enumerate() {
                    div {
                        key: "{reminder.id}",
                        class: "card-view-item",
                        style: format!("animation-delay: {}ms;", index * 50),
                        ReminderCard {
                            reminder: reminder.clone(),
                            tags: tags.clone(),
                            on_toggle: move |id: String| on_toggle.call(id),
                            on_edit: move |id: String| on_edit.call(id),
                            on_delete: move |id: String| on_delete.call(id),
                        }
                    }
                }
            }
        }
    }
}

