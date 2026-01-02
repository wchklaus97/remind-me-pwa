use chrono::TimeZone;
use dioxus::prelude::*;
use remind_me_ui::{
    Badge, BadgeVariant,
    Button, ButtonVariant, ButtonSize,
    Card, CardContent,
    Checkbox,
};
use crate::models::Reminder;
use crate::utils::format_date;
use i18nrs::I18n;

#[component]
pub fn ReminderCard(
    reminder: Reminder,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    i18n: I18n,
) -> Element {
    let is_overdue = if !reminder.completed && !reminder.due_date.is_empty() {
        if let Ok(due) = chrono::DateTime::parse_from_rfc3339(&reminder.due_date) {
            due < chrono::Utc::now()
        } else if let Ok(due) =
            chrono::NaiveDateTime::parse_from_str(&reminder.due_date, "%Y-%m-%dT%H:%M")
        {
            if let Some(local_dt) = chrono::Local.from_local_datetime(&due).single() {
                local_dt < chrono::Local::now()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

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

    rsx! {
        Card {
            variant: if is_overdue { remind_me_ui::CardVariant::Outline } else { remind_me_ui::CardVariant::Default },
            class: "{card_class}",
            CardContent {
                div {
                    class: "flex items-start justify-between gap-4",
                    div {
                        class: "flex items-start gap-3 flex-1",
                        Checkbox {
                            checked: reminder.completed,
                            onchange: move |_| {
                                on_toggle.call(reminder_id_toggle.clone());
                            },
                        }
                        div {
                            class: "flex-1",
                            h3 {
                                class: if reminder.completed { "line-through text-gray-500" } else { "font-semibold" },
                                "{reminder.title}"
                            }
                            if !reminder.description.is_empty() {
                                p {
                                    class: "text-sm text-gray-600 mt-1",
                                    "{reminder.description}"
                                }
                            }
                            if !reminder.due_date.is_empty() {
                                div {
                                    class: "mt-2 flex items-center gap-2",
                                    if is_overdue {
                                        Badge {
                                            variant: BadgeVariant::Danger,
                                            {i18n.t("reminder.overdue")}
                                        }
                                    }
                                    span {
                                        class: "text-sm text-gray-500",
                                        {
                                            format!("{} {}", i18n.t("reminder.due"), format_date(&reminder.due_date))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex gap-2",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                on_edit.call(reminder_id_edit.clone());
                            },
                            "✏️"
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Small,
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
