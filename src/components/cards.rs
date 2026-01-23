use dioxus::prelude::*;
use remind_me_ui::{
    Badge, BadgeVariant,
    Button, ButtonVariant, ButtonSize,
    Card, CardContent,
    Checkbox,
};
use crate::models::{Reminder, Tag};
use crate::utils::{format_date, is_overdue};
use crate::i18n::use_t;

#[component]
pub fn ReminderCard(
    reminder: Reminder,
    tags: Vec<Tag>,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_open: EventHandler<String>,
) -> Element {
    let is_overdue = !reminder.completed
        && !reminder.due_date.is_empty()
        && is_overdue(&reminder.due_date);

    let card_class = if reminder.completed {
        "reminder-card completed"
    } else if is_overdue {
        "reminder-card overdue"
    } else {
        "reminder-card"
    };

    let reminder_id_toggle = reminder.id.clone();
    let reminder_id_edit = reminder.id.clone();
    let reminder_id_delete = reminder.id.clone();
    let reminder_id_open = reminder.id.clone();

    rsx! {
        Card {
            variant: if is_overdue { remind_me_ui::CardVariant::Outline } else { remind_me_ui::CardVariant::Default },
            class: "{card_class}",
            CardContent {
                div {
                    class: "reminder-card-layout",
                    div {
                        class: "reminder-card-info",
                        Checkbox {
                            checked: reminder.completed,
                            aria_label: format!("{} {}", use_t("reminder.toggle"), reminder.title),
                            onchange: move |_| {
                                on_toggle.call(reminder_id_toggle.clone());
                            },
                        }
                        div {
                            class: "reminder-card-body",
                            h3 {
                                class: if reminder.completed { "reminder-title is-completed" } else { "reminder-title" },
                                "{reminder.title}"
                            }
                            if !reminder.description.is_empty() {
                                p {
                                    class: "reminder-description",
                                    "{reminder.description}"
                                }
                            }
                            if !reminder.due_date.is_empty() {
                                div {
                                    class: "reminder-meta",
                                    if is_overdue {
                                        Badge {
                                            variant: BadgeVariant::Danger,
                                            {use_t("reminder.overdue")}
                                        }
                                    }
                                    span {
                                        class: "reminder-due",
                                        {
                                            format!("{} {}", use_t("reminder.due"), format_date(&reminder.due_date))
                                        }
                                    }
                                }
                            }
                            if !reminder.tag_ids.is_empty() {
                                div {
                                    class: "reminder-tags",
                                    for tag_id in reminder.tag_ids.iter() {
                                        if let Some(tag) = tags.iter().find(|t| t.id == *tag_id) {
                                            span {
                                                class: "tag-chip",
                                                style: format!("background-color: {};", tag.color),
                                                {tag.name.clone()}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "reminder-actions",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            class: "btn btn-ghost btn-icon".to_string(),
                            aria_label: Some(format!("{} {}", use_t("reminder.details"), reminder.title.clone())),
                            onclick: move |_| {
                                on_open.call(reminder_id_open.clone());
                            },
                            "ℹ️"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            class: "btn btn-ghost btn-icon".to_string(),
                            aria_label: Some(format!("{} {}", use_t("tags.edit"), reminder.title.clone())),
                            onclick: move |_| {
                                on_edit.call(reminder_id_edit.clone());
                            },
                            "✏️"
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Small,
                            class: "btn btn-danger btn-icon".to_string(),
                            aria_label: Some(format!("{} {}", use_t("tags.delete"), reminder.title.clone())),
                            onclick: move |_| {
                                on_delete.call(reminder_id_delete.clone());
                            },
                            "🗑️"
                        }
                    }
                }
            }
        }
    }
}
