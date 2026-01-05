use dioxus::prelude::*;
use crate::models::{Reminder, Tag};
use crate::components::ReminderCard;

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
                for reminder in reminders {
                    div {
                        class: "card-view-item",
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

