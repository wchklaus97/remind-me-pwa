use dioxus::prelude::*;
use remind_me_shared::models::{Reminder, Tag};
use crate::components::ReminderCard;
use remind_me_web::i18n::use_t;

#[component]
pub fn FolderView(
    reminders: Vec<Reminder>,
    tags: Vec<Tag>,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    // Group reminders by tag
    let tag_groups: Vec<(Tag, Vec<Reminder>)> = tags
        .iter()
        .map(|tag| {
            let tag_reminders: Vec<Reminder> = reminders
                .iter()
                .filter(|r| r.tag_ids.contains(&tag.id))
                .cloned()
                .collect();
            (tag.clone(), tag_reminders)
        })
        .filter(|(_, reminders)| !reminders.is_empty())
        .collect();

    // Reminders without tags (untagged)
    let untagged: Vec<Reminder> = reminders
        .iter()
        .filter(|r| r.tag_ids.is_empty())
        .cloned()
        .collect();

    rsx! {
        section {
            class: "reminders-folder-view",
            aria_label: "Folder view of reminders grouped by tags",
            div {
                class: "folder-groups",
                // Render tag groups
                for (tag, tag_reminders) in tag_groups {
                    div {
                        class: "folder-group",
                        div {
                            class: "folder-group-header",
                            span {
                                class: "folder-group-icon",
                                style: format!("background-color: {};", tag.color),
                            }
                            h3 {
                                class: "folder-group-title",
                                "{tag.name}"
                                span {
                                    class: "folder-group-count",
                                    " ({tag_reminders.len()})"
                                }
                            }
                        }
                        div {
                            class: "folder-group-items",
                            for (index, reminder) in tag_reminders.iter().enumerate() {
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

                // Render untagged reminders if any
                if !untagged.is_empty() {
                    div {
                        key: "{format!(\"untagged\")}",
                        class: "folder-group",
                        div {
                            class: "folder-group-header",
                            h3 {
                                class: "folder-group-title",
                                {use_t("app.views.untagged")}
                                span {
                                    class: "folder-group-count",
                                    " ({untagged.len()})"
                                }
                            }
                        }
                        div {
                            class: "folder-group-items",
                            for (index, reminder) in untagged.iter().enumerate() {
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
        }
    }
}

