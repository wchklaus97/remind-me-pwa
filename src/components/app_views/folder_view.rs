use dioxus::prelude::*;
use crate::models::{Reminder, Tag};
use crate::components::ReminderCard;
use crate::i18n::use_t;

#[component]
pub fn FolderView(
    reminders: Vec<Reminder>,
    tags: Vec<Tag>,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_reorder_tags: EventHandler<(String, String)>,
    on_open: EventHandler<String>,
) -> Element {
    let mut dragged_tag_id = use_signal(|| None::<String>);
    let mut drop_target_id = use_signal(|| None::<String>);
    let mut dragged_group_id = use_signal(|| None::<String>);

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
                        class: {
                            let mut class_name = "folder-group".to_string();
                            if drop_target_id().as_ref() == Some(&tag.id) {
                                class_name.push_str(" is-drop-target");
                            }
                            if dragged_group_id().as_ref() == Some(&tag.id) {
                                class_name.push_str(" is-dragging");
                            }
                            class_name
                        },
                        draggable: "true",
                        ondragstart: move |_| {
                            dragged_tag_id.set(Some(tag.id.clone()));
                            drop_target_id.set(Some(tag.id.clone()));
                            dragged_group_id.set(Some(tag.id.clone()));
                        },
                        ondragover: move |event| {
                            event.prevent_default();
                            drop_target_id.set(Some(tag.id.clone()));
                        },
                        ondrop: move |event| {
                            event.prevent_default();
                            if let Some(drag_id) = dragged_tag_id() {
                                on_reorder_tags.call((drag_id.clone(), tag.id.clone()));
                            }
                            dragged_tag_id.set(None);
                            drop_target_id.set(None);
                            dragged_group_id.set(None);
                        },
                        ondragend: move |_| {
                            dragged_tag_id.set(None);
                            drop_target_id.set(None);
                            dragged_group_id.set(None);
                        },
                        div {
                            class: "folder-group-header",
                            span {
                                class: "folder-group-icon",
                                style: format!("background-color: {};", tag.color),
                            }
                            span { class: "folder-drag-handle", "⋮⋮" }
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
                                        on_open: move |id: String| on_open.call(id),
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
                                        on_open: move |id: String| on_open.call(id),
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

