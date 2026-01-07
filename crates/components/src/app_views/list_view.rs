use dioxus::prelude::*;
use remind_me_ui::EmptyState;
use remind_me_shared::models::{Reminder, Tag, ReminderFilter};
use crate::components::ReminderCard;
use remind_me_web::i18n::use_t;

#[component]
pub fn ListView(
    reminders: Vec<Reminder>,
    tags: Vec<Tag>,
    filter: ReminderFilter,
    search_query: String,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_new_reminder: EventHandler<()>,
) -> Element {
    rsx! {
        section {
            class: "reminders-list",
            aria_label: "List of reminders",
            if reminders.is_empty() {
                EmptyState {
                    icon: if search_query.is_empty() {
                        match filter {
                            ReminderFilter::Active => "✅",
                            ReminderFilter::Completed => "🎉",
                            ReminderFilter::All => "📝",
                        }
                    } else {
                        "🔍"
                    },
                    title: if search_query.is_empty() {
                        match filter {
                            ReminderFilter::Active => use_t("empty.active_title"),
                            ReminderFilter::Completed => use_t("empty.completed_title"),
                            ReminderFilter::All => use_t("empty.title"),
                        }
                    } else {
                        use_t("empty.search_title")
                    },
                    description: if search_query.is_empty() {
                        match filter {
                            ReminderFilter::Active => use_t("empty.active_description"),
                            ReminderFilter::Completed => use_t("empty.completed_description"),
                            ReminderFilter::All => use_t("empty.description"),
                        }
                    } else {
                        use_t("empty.search_description")
                    },
                    action_text: if search_query.is_empty() {
                        use_t("empty.action")
                    } else {
                        use_t("empty.clear_search")
                    },
                    on_action: move |_| on_new_reminder.call(()),
                }
            } else {
                for (index, reminder) in reminders.iter().enumerate() {
                    div {
                        key: "{reminder.id}",
                        class: "reminder-item",
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

