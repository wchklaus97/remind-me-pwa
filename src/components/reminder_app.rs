use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant, Input, Select, SelectOption,
    EmptyState, Toast, ToastPosition, ToastVariant,
};
use crate::models::Reminder;
use crate::storage::{load_reminders, save_reminders};
use crate::utils::get_filtered_and_sorted_reminders;
// Use re-exports from mod.rs to avoid clippy warnings
use crate::components::{StatisticsDisplay, AddReminderForm, EditReminderForm, ReminderCard, DeleteConfirmModal};
use i18nrs::I18n;

#[component]
pub fn ReminderApp(i18n: I18n) -> Element {
    let mut reminders = use_signal(load_reminders);
    let mut show_add_form = use_signal(|| false);
    let mut filter = use_signal(|| "all".to_string());
    let mut search_query = use_signal(String::new);
    let mut sort_by = use_signal(|| "date".to_string());
    let mut editing_id = use_signal(|| None::<String>);

    // Toast notification state
    let mut show_toast = use_signal(|| false);
    let mut toast_message = use_signal(String::new);
    let mut toast_variant = use_signal(|| ToastVariant::Success);

    // Delete confirmation state
    let mut delete_confirm_id = use_signal(|| None::<String>);

    // Clone i18n for use in closures - each closure needs its own clone
    let i18n_stats = i18n.clone();
    let i18n_search = i18n.clone();
    let i18n_sort = i18n.clone();
    let i18n_header = i18n.clone();
    let i18n_filter_all = i18n.clone();
    let i18n_filter_active = i18n.clone();
    let i18n_filter_completed = i18n.clone();
    let i18n_empty = i18n.clone();
    let i18n_toggle_card = i18n.clone();
    let i18n_toast = i18n.clone();

    rsx! {
        div {
            class: "app-container",
            header {
                role: "banner",
                class: "app-header",
                h1 { {i18n_header.t("app.header.title")} }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| show_add_form.set(!show_add_form()),
                    {
                        if show_add_form() {
                            {i18n_header.t("app.header.cancel")}
                        } else {
                            {i18n_header.t("app.header.new_reminder")}
                        }
                    }
                }
            }

            main {
                role: "main",
                // Statistics section
                StatisticsDisplay { reminders: reminders(), i18n: i18n_stats }

                // Search and sort controls
                section {
                    class: "controls mb-4",
                    div {
                        class: "flex gap-2 mb-2",
                        Input {
                            id: "search_reminders".to_string(),
                            name: "search".to_string(),
                            r#type: "text",
                            placeholder: i18n_search.t("search.placeholder"),
                            value: "{search_query()}",
                            oninput: move |value| search_query.set(value),
                        }
                        Select {
                            id: "sort_reminders".to_string(),
                            name: "sort_by".to_string(),
                            value: sort_by(),
                            onchange: move |value| sort_by.set(value),
                            options: vec![
                                SelectOption { value: "date".to_string(), label: i18n_sort.t("sort.date") },
                                SelectOption { value: "title".to_string(), label: i18n_sort.t("sort.title") },
                                SelectOption { value: "status".to_string(), label: i18n_sort.t("sort.status") },
                            ],
                        }
                    }
                }

                if show_add_form() || editing_id().is_some() {
                    if let Some(edit_id) = editing_id() {
                        if let Some(reminder) = reminders().iter().find(|r| r.id == edit_id) {
                            EditReminderForm {
                                reminder: reminder.clone(),
                                on_save: {
                                    let i18n_save = i18n.clone();
                                    move |updated: Reminder| {
                                        let mut updated_reminders = reminders();
                                        if let Some(r) = updated_reminders.iter_mut().find(|r| r.id == updated.id) {
                                            *r = updated.clone();
                                        }
                                        reminders.set(updated_reminders);
                                        save_reminders(&reminders());
                                        editing_id.set(None);

                                        toast_message.set(i18n_save.t("toast.updated"));
                                        toast_variant.set(ToastVariant::Success);
                                        show_toast.set(true);
                                    }
                                },
                                on_cancel: move |_| editing_id.set(None),
                                i18n: {
                                    let i18n_edit = i18n.clone();
                                    i18n_edit
                                },
                            }
                        }
                    } else {
                        AddReminderForm {
                            on_add: {
                                let i18n_add = i18n.clone();
                                move |reminder: Reminder| {
                                    let mut new_reminders = reminders();
                                    new_reminders.push(reminder);
                                    reminders.set(new_reminders);
                                    save_reminders(&reminders());
                                    show_add_form.set(false);

                                    toast_message.set(i18n_add.t("toast.added"));
                                    toast_variant.set(ToastVariant::Success);
                                    show_toast.set(true);
                                }
                            },
                            i18n: {
                                let i18n_add_form = i18n.clone();
                                i18n_add_form
                            },
                        }
                    }
                }

                nav {
                    role: "navigation",
                    aria_label: "Filter reminders",
                    class: "filter-tabs",
                    Button {
                        variant: if filter() == "all" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        onclick: move |_| filter.set("all".to_string()),
                        {i18n_filter_all.t("filter.all")}
                    }
                    Button {
                        variant: if filter() == "active" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        onclick: move |_| filter.set("active".to_string()),
                        {i18n_filter_active.t("filter.active")}
                    }
                    Button {
                        variant: if filter() == "completed" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        onclick: move |_| filter.set("completed".to_string()),
                        {i18n_filter_completed.t("filter.completed")}
                    }
                }

                section {
                    class: "reminders-list",
                    aria_label: "List of reminders",
                    for reminder in get_filtered_and_sorted_reminders(
                        &reminders(),
                        &filter(),
                        &search_query(),
                        &sort_by()
                    ) {
                        ReminderCard {
                            reminder: reminder.clone(),
                            on_toggle: {
                                let i18n_toggle = i18n.clone();
                                move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { i18n_toggle.t("toast.completed") } else { i18n_toggle.t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", i18n_toggle.t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                }
                            },
                            on_edit: move |id: String| {
                                editing_id.set(Some(id));
                                show_add_form.set(false);
                            },
                            on_delete: move |id: String| {
                                delete_confirm_id.set(Some(id));
                            },
                            i18n: i18n_toggle_card.clone(),
                        }
                    }
                }

                if reminders().is_empty() {
                    EmptyState {
                        icon: "📝",
                        title: i18n_empty.t("empty.title"),
                        description: i18n_empty.t("empty.description"),
                        action_text: i18n_empty.t("empty.action"),
                        on_action: move |_| show_add_form.set(true),
                    }
                }
            }

            // Delete confirmation modal
            if let Some(delete_id) = delete_confirm_id() {
                DeleteConfirmModal {
                    open: delete_confirm_id,
                    reminder_id: delete_id.clone(),
                    on_confirm: {
                        let i18n_delete = i18n.clone();
                        move |id: String| {
                            let mut updated = reminders();
                            updated.retain(|r| r.id != id);
                            reminders.set(updated);
                            save_reminders(&reminders());
                            delete_confirm_id.set(None);

                            toast_message.set(i18n_delete.t("toast.deleted"));
                            toast_variant.set(ToastVariant::Success);
                            show_toast.set(true);
                        }
                    },
                    on_cancel: move |_| delete_confirm_id.set(None),
                    i18n: {
                        let i18n_modal = i18n.clone();
                        i18n_modal
                    },
                }
            }

            // Toast notification
            Toast {
                open: show_toast,
                variant: toast_variant(),
                title: {
                    match toast_variant() {
                        ToastVariant::Success => i18n_toast.t("toast.success"),
                        ToastVariant::Error => i18n_toast.t("toast.error"),
                        ToastVariant::Warning => i18n_toast.t("toast.warning"),
                        ToastVariant::Info => i18n_toast.t("toast.info"),
                    }
                },
                message: toast_message(),
                position: ToastPosition::TopRight,
                duration: 3000,
                on_close: move |_| show_toast.set(false),
            }
        }
    }
}
