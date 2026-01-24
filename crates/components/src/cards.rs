use dioxus::prelude::*;
use remind_me_ui::{
    Badge, BadgeVariant,
    Button, ButtonVariant, ButtonSize,
    Card, CardContent,
    Checkbox,
};
use remind_me_shared::models::{Reminder, Tag, Priority};
use remind_me_shared::utils::{format_date, is_overdue};
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

    // Swipe gesture state
    let mut swipe_start_x = use_signal(|| None::<f64>);
    let mut swipe_current_x = use_signal(|| 0.0);
    let mut is_swiping = use_signal(|| false);
    let mut swipe_action = use_signal(|| None::<String>); // "complete" or "delete"

    let reminder_id_toggle = reminder.id.clone();
    let reminder_id_edit = reminder.id.clone();
    let reminder_id_delete = reminder.id.clone();
    let reminder_id_open = reminder.id.clone();
    let reminder_id_swipe_complete = reminder.id.clone();
    let reminder_id_swipe_delete = reminder.id.clone();

    // Swipe threshold (minimum distance to trigger action)
    const SWIPE_THRESHOLD: f64 = 100.0;

    // Calculate swipe indicator values
    let swipe_opacity = if is_swiping() {
        if swipe_current_x() > 0.0 {
            (swipe_current_x() / SWIPE_THRESHOLD).min(1.0)
        } else {
            (-swipe_current_x() / SWIPE_THRESHOLD).min(1.0)
        }
    } else {
        0.0
    };
    
    let swipe_bg_gradient = if is_swiping() && swipe_current_x() > 0.0 {
        "linear-gradient(to right, rgba(34, 197, 94, 0.2), transparent)"
    } else if is_swiping() {
        "linear-gradient(to left, rgba(239, 68, 68, 0.2), transparent)"
    } else {
        "transparent"
    };

    rsx! {
        div {
            class: "reminder-card-wrapper",
            style: if is_swiping() {
                format!(
                    "transform: translateX({}px); transition: transform 0.2s ease-out;",
                    swipe_current_x()
                )
            } else {
                    "transform: translateX(0); transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);".to_string()
                },
            onpointerdown: move |e| {
                let coords = e.client_coordinates();
                swipe_start_x.set(Some(coords.x));
                is_swiping.set(true);
            },
            onpointermove: move |e| {
                if let Some(start_x) = swipe_start_x() {
                    let coords = e.client_coordinates();
                    let delta = coords.x - start_x;
                    swipe_current_x.set(delta);
                    
                    // Determine action based on swipe direction
                    if delta > SWIPE_THRESHOLD {
                        swipe_action.set(Some("complete".to_string()));
                    } else if delta < -SWIPE_THRESHOLD {
                        swipe_action.set(Some("delete".to_string()));
                    } else {
                        swipe_action.set(None);
                    }
                }
            },
            onpointerup: move |_| {
                if let Some(action) = swipe_action() {
                    if action == "complete" && !reminder.completed {
                        on_toggle.call(reminder_id_swipe_complete.clone());
                    } else if action == "delete" {
                        on_delete.call(reminder_id_swipe_delete.clone());
                    }
                }
                // Reset swipe state
                swipe_start_x.set(None);
                swipe_current_x.set(0.0);
                is_swiping.set(false);
                swipe_action.set(None);
            },
            onpointercancel: move |_| {
                // Reset on cancel (e.g., scroll)
                swipe_start_x.set(None);
                swipe_current_x.set(0.0);
                is_swiping.set(false);
                swipe_action.set(None);
            },
            // Swipe action background indicators
            if is_swiping() {
                div {
                    class: "swipe-action-indicator",
                    style: format!("background: {}; opacity: {};", swipe_bg_gradient, swipe_opacity),
                    if swipe_current_x() > SWIPE_THRESHOLD {
                        span {
                            class: "swipe-action-text swipe-complete",
                            "✓ Complete"
                        }
                    } else if swipe_current_x() < -SWIPE_THRESHOLD {
                        span {
                            class: "swipe-action-text swipe-delete",
                            "🗑 Delete"
                        }
                    }
                }
            }
            Card {
                variant: if is_overdue { remind_me_ui::CardVariant::Outline } else { remind_me_ui::CardVariant::Default },
                class: format!(
                    "{} swipeable-card {}",
                    card_class,
                    if is_swiping() {
                        if swipe_current_x() > SWIPE_THRESHOLD {
                            "swipe-complete-bg"
                        } else if swipe_current_x() < -SWIPE_THRESHOLD {
                            "swipe-delete-bg"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    }
                ),
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
                                            {use_t("reminder.overdue")}
                                        }
                                    }
                                    span {
                                        class: "text-sm text-gray-500",
                                        {
                                            format!("{} {}", use_t("reminder.due"), format_date(&reminder.due_date))
                                        }
                                    }
                                }
                            }
                            div {
                                class: "mt-2 flex items-center gap-2 flex-wrap",
                                // Priority badge
                                span {
                                    class: format!("priority-badge priority-{}", reminder.priority.as_str()),
                                    match reminder.priority {
                                        Priority::Low => "🔵 Low",
                                        Priority::Medium => "🟡 Med",
                                        Priority::High => "🔴 High",
                                    }
                                }
                                // Tags
                                if !reminder.tag_ids.is_empty() {
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
                        class: "flex gap-2",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            aria_label: Some(format!("{} {}", use_t("reminder.details"), reminder.title.clone())),
                            onclick: move |_| {
                                on_open.call(reminder_id_open.clone());
                            },
                            "ℹ️"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            aria_label: Some(format!("{} {}", use_t("tags.edit"), reminder.title.clone())),
                            onclick: move |_| {
                                on_edit.call(reminder_id_edit.clone());
                            },
                            "✏️"
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Small,
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
}
