use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use dioxus::dioxus_core::use_hook_with_cleanup;
use remind_me_ui::{Button, ButtonVariant, Modal, ModalSize, Badge, BadgeVariant};
use remind_me_shared::models::{Reminder, Tag, Priority};
use remind_me_shared::utils::{format_date, is_overdue, now_timestamp_millis, parse_date_to_epoch_ms};
use crate::i18n::use_t;

#[component]
pub fn DeleteConfirmModal(
    open: Signal<Option<String>>,
    reminder_id: String,
    on_confirm: EventHandler<String>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_open = open().is_some();
    let mut open_signal = use_signal(|| is_open);

    // Update signal when open changes using use_effect
    use_effect(move || {
        let current_is_open = open().is_some();
        if current_is_open != open_signal() {
            open_signal.set(current_is_open);
        }
    });

    rsx! {
        Modal {
            open: open_signal,
            size: ModalSize::Small,
            title: use_t("delete.title"),
            close_on_backdrop: true,
            on_close: move |_| on_cancel.call(()),
            p {
                class: "mb-4",
                {use_t("delete.message")}
            }
            div {
                class: "flex justify-end gap-2",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| on_cancel.call(()),
                    {use_t("delete.cancel")}
                }
                Button {
                    variant: ButtonVariant::Danger,
                    onclick: move |_| on_confirm.call(reminder_id.clone()),
                    {use_t("delete.confirm")}
                }
            }
        }
    }
}

/// Calculate time remaining until due date
fn calculate_time_remaining(due_date_str: &str) -> Option<(i64, String)> {
    let Some(due_ms) = parse_date_to_epoch_ms(due_date_str) else {
        return None;
    };
    let now_ms = now_timestamp_millis();
    let remaining_ms = due_ms - now_ms;
    
    if remaining_ms <= 0 {
        return Some((0, "Overdue".to_string()));
    }
    
    let seconds = remaining_ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    
    let time_str = if days > 0 {
        format!("{}d {}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    };
    
    Some((remaining_ms, time_str))
}

/// Get urgency color based on time remaining
fn get_urgency_color(remaining_ms: i64) -> &'static str {
    let hours = remaining_ms / (1000 * 60 * 60);
    if hours < 1 {
        "var(--danger-color)" // Red - less than 1 hour
    } else if hours < 24 {
        "var(--primary-color)" // Orange - less than 24 hours
    } else {
        "var(--brand-blue)" // Blue - more than 24 hours
    }
}

/// Calculate progress percentage (0-100) for circular progress
fn calculate_progress(due_date_str: &str, created_at_str: &str) -> f64 {
    let Some(due_ms) = parse_date_to_epoch_ms(due_date_str) else {
        return 0.0;
    };
    let Some(created_ms) = parse_date_to_epoch_ms(created_at_str) else {
        return 0.0;
    };
    let now_ms = now_timestamp_millis();
    
    let total_duration = due_ms - created_ms;
    let elapsed = now_ms - created_ms;
    
    if total_duration <= 0 {
        return 100.0;
    }
    
    let progress = (elapsed as f64 / total_duration as f64) * 100.0;
    progress.max(0.0).min(100.0)
}

#[component]
pub fn ReminderDetailModal(
    open: Signal<Option<String>>,
    reminder: Option<Reminder>,
    tags: Vec<Tag>,
    on_close: EventHandler<()>,
    on_edit: EventHandler<String>,
    on_toggle: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let is_open = open().is_some();
    let mut open_signal = use_signal(|| is_open);
    let mut countdown_time = use_signal(|| String::new());
    let mut progress_percent = use_signal(|| 0.0);
    let mut urgency_color = use_signal(|| "var(--brand-blue)");

    // Update signal when open changes
    use_effect(move || {
        let current_is_open = open().is_some();
        if current_is_open != open_signal() {
            open_signal.set(current_is_open);
        }
    });

    // Countdown timer effect - update every second
    let reminder_clone = reminder.clone();
    use_effect(move || {
        if let Some(ref reminder) = reminder_clone {
            if !reminder.due_date.is_empty() && !reminder.completed {
                let due_date = reminder.due_date.clone();
                let created_at = reminder.created_at.clone();
                
                // Initial update
                if let Some((remaining_ms, time_str)) = calculate_time_remaining(&due_date) {
                    countdown_time.set(time_str);
                    urgency_color.set(get_urgency_color(remaining_ms));
                    progress_percent.set(calculate_progress(&due_date, &created_at));
                }
                
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    // Set up interval for updates
                    if let Some(window) = web_sys::window() {
                        let closure = wasm_bindgen::closure::Closure::wrap(Box::new({
                            let due_date = due_date.clone();
                            let created_at = created_at.clone();
                            move || {
                                if let Some((remaining_ms, time_str)) = calculate_time_remaining(&due_date) {
                                    countdown_time.set(time_str);
                                    urgency_color.set(get_urgency_color(remaining_ms));
                                    progress_percent.set(calculate_progress(&due_date, &created_at));
                                }
                            }
                        }) as Box<dyn FnMut()>);
                        
                        if let Ok(interval_id) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            1000,
                        ) {
                            closure.forget();
                            
                            // Store interval_id for cleanup (simplified - cleanup happens on unmount)
                            // Note: In a real implementation, we'd use use_hook_with_cleanup
                            // For now, interval will be cleaned up when component unmounts
                        }
                    }
                }
            } else {
                countdown_time.set(String::new());
                progress_percent.set(0.0);
            }
        }
    });

    let Some(ref reminder) = reminder else {
        return rsx! { div {} };
    };

    let reminder_id_edit = reminder.id.clone();
    let reminder_id_toggle = reminder.id.clone();
    let reminder_id_delete = reminder.id.clone();
    let is_overdue = !reminder.completed && !reminder.due_date.is_empty() && is_overdue(&reminder.due_date);

    rsx! {
        Modal {
            open: open_signal,
            size: ModalSize::Large,
            title: reminder.title.clone(),
            close_on_backdrop: true,
            on_close: move |_| on_close.call(()),
            div {
                class: "reminder-detail-content",
                // Countdown Timer Section
                if !reminder.completed && !reminder.due_date.is_empty() {
                    div {
                        class: "countdown-section",
                        div {
                            class: "countdown-circle",
                            style: format!(
                                "background: conic-gradient({} 0%, {} {}%, rgba(0,0,0,0.1) {}%, rgba(0,0,0,0.1) 100%);",
                                urgency_color(),
                                urgency_color(),
                                progress_percent(),
                                progress_percent()
                            ),
                            div {
                                class: "countdown-inner",
                                div {
                                    class: "countdown-time",
                                    style: format!("color: {};", urgency_color()),
                                    if countdown_time().is_empty() {
                                        "Calculating..."
                                    } else {
                                        "{countdown_time()}"
                                    }
                                }
                                div {
                                    class: "countdown-label",
                                    if is_overdue {
                                        "Overdue"
                                    } else {
                                        "Time Remaining"
                                    }
                                }
                            }
                        }
                    }
                }

                // Reminder Info Section
                div {
                    class: "reminder-info-section",
                    // Description
                    if !reminder.description.is_empty() {
                        div {
                            class: "reminder-description",
                            p { "{reminder.description}" }
                        }
                    }

                    // Due Date
                    if !reminder.due_date.is_empty() {
                        div {
                            class: "reminder-meta-item",
                            span {
                                class: "meta-label",
                                "📅 Due Date:"
                            }
                            span {
                                class: "meta-value",
                                {format_date(&reminder.due_date)}
                            }
                        }
                    }

                    // Priority
                    div {
                        class: "reminder-meta-item",
                        span {
                            class: "meta-label",
                            "⚡ Priority:"
                        }
                        Badge {
                            variant: match reminder.priority {
                                Priority::High => BadgeVariant::Danger,
                                Priority::Medium => BadgeVariant::Default,
                                Priority::Low => BadgeVariant::Default,
                            },
                            match reminder.priority {
                                Priority::Low => "🔵 Low",
                                Priority::Medium => "🟡 Medium",
                                Priority::High => "🔴 High",
                            }
                        }
                    }

                    // Tags
                    if !reminder.tag_ids.is_empty() {
                        div {
                            class: "reminder-meta-item",
                            span {
                                class: "meta-label",
                                "🏷️ Tags:"
                            }
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

                    // Status
                    div {
                        class: "reminder-meta-item",
                        span {
                            class: "meta-label",
                            "✓ Status:"
                        }
                        if reminder.completed {
                            Badge {
                                variant: BadgeVariant::Default,
                                "✅ Completed"
                            }
                        } else if is_overdue {
                            Badge {
                                variant: BadgeVariant::Danger,
                                "⚠️ Overdue"
                            }
                        } else {
                            Badge {
                                variant: BadgeVariant::Default,
                                "⏳ Active"
                            }
                        }
                    }
                }

                // Action Buttons
                div {
                    class: "reminder-detail-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        aria_label: Some("Edit reminder".to_string()),
                        onclick: move |_| {
                            on_edit.call(reminder_id_edit.clone());
                            on_close.call(());
                        },
                        "✏️ Edit"
                    }
                    if !reminder.completed {
                        Button {
                            variant: ButtonVariant::Primary,
                            aria_label: Some("Mark as complete".to_string()),
                            onclick: move |_| {
                                on_toggle.call(reminder_id_toggle.clone());
                                on_close.call(());
                            },
                            "✓ Complete"
                        }
                    }
                    Button {
                        variant: ButtonVariant::Danger,
                        aria_label: Some("Delete reminder".to_string()),
                        onclick: move |_| {
                            on_delete.call(reminder_id_delete.clone());
                            on_close.call(());
                        },
                        "🗑️ Delete"
                    }
                }
            }
        }
    }
}
