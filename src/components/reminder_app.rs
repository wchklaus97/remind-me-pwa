use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant, Input, Select, SelectOption,
    EmptyState, Toast, ToastPosition, ToastVariant,
};
use crate::models::{Reminder, ReminderFilter, ReminderSort};
use crate::storage::{load_reminders, save_reminders, load_tags};
use crate::utils::get_filtered_and_sorted_reminders;
// Use re-exports from mod.rs to avoid clippy warnings
use crate::components::{StatisticsDisplay, AddReminderForm, EditReminderForm, DeleteConfirmModal, ListView, CardView, FolderView, CalendarView, TagManager};
use crate::i18n::use_t;

#[component]
pub fn ReminderApp() -> Element {
    let mut reminders = use_signal(load_reminders);
    let mut tags = use_signal(load_tags);
    let mut show_add_form = use_signal(|| false);
    let mut filter = use_signal(|| ReminderFilter::All);
    let mut search_query = use_signal(String::new);
    let mut sort_by = use_signal(|| ReminderSort::Date);
    let mut editing_id = use_signal(|| None::<String>);

    // Toast notification state
    let mut show_toast = use_signal(|| false);
    let mut toast_message = use_signal(String::new);
    let mut toast_variant = use_signal(|| ToastVariant::Success);

    // Delete confirmation state
    let mut delete_confirm_id = use_signal(|| None::<String>);

    // View state (list, card, folder)
    let mut current_view = use_signal(|| "list".to_string());

    // Tag manager modal state
    let mut show_tag_manager = use_signal(|| false);

    rsx! {
        div {
            class: "app-container",
            header {
                role: "banner",
                class: "app-header",
                h1 { {use_t("app.header.title")} }
                div {
                    class: "app-header-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        aria_label: Some(use_t("tags.manage")),
                        onclick: move |_| show_tag_manager.set(true),
                        "🏷️"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        aria_label: Some(if show_add_form() {
                            use_t("app.header.cancel")
                        } else {
                            use_t("app.header.new_reminder")
                        }),
                        onclick: move |_| show_add_form.set(!show_add_form()),
                        {
                            if show_add_form() {
                                {use_t("app.header.cancel")}
                            } else {
                                {use_t("app.header.new_reminder")}
                            }
                        }
                    }
                }
            }

            main {
                role: "main",
                // Statistics section
                StatisticsDisplay { reminders: reminders() }

                // Search and sort controls
                section {
                    class: "controls mb-4",
                    div {
                        class: "flex gap-2 mb-2",
                        Input {
                            id: "search_reminders".to_string(),
                            name: "search".to_string(),
                            r#type: "text",
                            placeholder: use_t("search.placeholder"),
                            value: "{search_query()}",
                            oninput: move |value| search_query.set(value),
                        }
                        Select {
                            id: "sort_reminders".to_string(),
                            name: "sort_by".to_string(),
                            value: sort_by().as_str().to_string(),
                            onchange: move |value: String| sort_by.set(ReminderSort::from_str(value.as_str())),
                            options: vec![
                                SelectOption { value: "date".to_string(), label: use_t("sort.date") },
                                SelectOption { value: "title".to_string(), label: use_t("sort.title") },
                                SelectOption { value: "status".to_string(), label: use_t("sort.status") },
                            ],
                        }
                    }
                }

                if show_add_form() || editing_id().is_some() {
                    if let Some(edit_id) = editing_id() {
                        if let Some(reminder) = reminders().iter().find(|r| r.id == edit_id) {
                            EditReminderForm {
                                reminder: reminder.clone(),
                                tags: tags(),
                                on_save: move |updated: Reminder| {
                                    let mut updated_reminders = reminders();
                                    if let Some(r) = updated_reminders.iter_mut().find(|r| r.id == updated.id) {
                                        *r = updated.clone();
                                    }
                                    reminders.set(updated_reminders);
                                    save_reminders(&reminders());
                                    editing_id.set(None);

                                    toast_message.set(use_t("toast.updated"));
                                    toast_variant.set(ToastVariant::Success);
                                    show_toast.set(true);
                                },
                                on_cancel: move |_| editing_id.set(None),
                            }
                        }
                    } else {
                        AddReminderForm {
                            tags: tags(),
                            on_add: move |reminder: Reminder| {
                                let mut new_reminders = reminders();
                                new_reminders.push(reminder);
                                reminders.set(new_reminders);
                                save_reminders(&reminders());
                                show_add_form.set(false);

                                toast_message.set(use_t("toast.added"));
                                toast_variant.set(ToastVariant::Success);
                                show_toast.set(true);
                            },
                        }
                    }
                }

                // View switcher
                nav {
                    role: "navigation",
                    aria_label: "View switcher",
                    class: "view-switcher",
                    Button {
                        variant: if current_view() == "list" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("app.views.list")),
                        onclick: move |_| current_view.set("list".to_string()),
                        {use_t("app.views.list")}
                    }
                    Button {
                        variant: if current_view() == "card" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("app.views.card")),
                        onclick: move |_| current_view.set("card".to_string()),
                        {use_t("app.views.card")}
                    }
                    Button {
                        variant: if current_view() == "folder" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("app.views.folder")),
                        onclick: move |_| current_view.set("folder".to_string()),
                        {use_t("app.views.folder")}
                    }
                }

                nav {
                    role: "navigation",
                    aria_label: "Filter reminders",
                    class: "filter-tabs",
                    Button {
                        variant: if filter() == ReminderFilter::All { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("filter.all")),
                        onclick: move |_| filter.set(ReminderFilter::All),
                        {use_t("filter.all")}
                    }
                    Button {
                        variant: if filter() == ReminderFilter::Active { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("filter.active")),
                        onclick: move |_| filter.set(ReminderFilter::Active),
                        {use_t("filter.active")}
                    }
                    Button {
                        variant: if filter() == ReminderFilter::Completed { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                        aria_label: Some(use_t("filter.completed")),
                        onclick: move |_| filter.set(ReminderFilter::Completed),
                        {use_t("filter.completed")}
                    }
                }

                // Render view based on current_view state
                {
                    let filtered_reminders = get_filtered_and_sorted_reminders(
                        &reminders(),
                        &filter(),
                        &search_query(),
                        &sort_by()
                    );

                    match current_view().as_str() {
                        "list" => rsx! {
                            ListView {
                                reminders: filtered_reminders,
                                tags: tags(),
                                on_toggle: move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { use_t("toast.completed") } else { use_t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", use_t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                },
                                on_edit: move |id: String| {
                                    editing_id.set(Some(id));
                                    show_add_form.set(false);
                                },
                                on_delete: move |id: String| {
                                    delete_confirm_id.set(Some(id));
                                },
                            }
                        },
                        "card" => rsx! {
                            CardView {
                                reminders: filtered_reminders,
                                tags: tags(),
                                on_toggle: move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { use_t("toast.completed") } else { use_t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", use_t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                },
                                on_edit: move |id: String| {
                                    editing_id.set(Some(id));
                                    show_add_form.set(false);
                                },
                                on_delete: move |id: String| {
                                    delete_confirm_id.set(Some(id));
                                },
                            }
                        },
                        "folder" => rsx! {
                            FolderView {
                                reminders: filtered_reminders,
                                tags: tags(),
                                on_toggle: move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { use_t("toast.completed") } else { use_t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", use_t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                },
                                on_edit: move |id: String| {
                                    editing_id.set(Some(id));
                                    show_add_form.set(false);
                                },
                                on_delete: move |id: String| {
                                    delete_confirm_id.set(Some(id));
                                },
                            }
                        },
                        "calendar" => rsx! {
                            CalendarView {
                                reminders: reminders(),
                                tags: tags(),
                                filter: filter(),
                                search_query: search_query(),
                                sort_by: sort_by(),
                                on_toggle: move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { use_t("toast.completed") } else { use_t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", use_t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                },
                                on_edit: move |id: String| {
                                    editing_id.set(Some(id));
                                    show_add_form.set(false);
                                },
                                on_delete: move |id: String| {
                                    delete_confirm_id.set(Some(id));
                                },
                            }
                        },
                        _ => rsx! {
                            ListView {
                                reminders: filtered_reminders,
                                tags: tags(),
                                on_toggle: move |id: String| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        r.completed = !r.completed;
                                        let status = if r.completed { use_t("toast.completed") } else { use_t("toast.marked_active") };
                                        reminders.set(updated);
                                        save_reminders(&reminders());

                                        toast_message.set(format!("{} {}", use_t("toast.info"), status));
                                        toast_variant.set(ToastVariant::Info);
                                        show_toast.set(true);
                                    }
                                },
                                on_edit: move |id: String| {
                                    editing_id.set(Some(id));
                                    show_add_form.set(false);
                                },
                                on_delete: move |id: String| {
                                    delete_confirm_id.set(Some(id));
                                },
                            }
                        },
                    }
                }

                if reminders().is_empty() {
                    EmptyState {
                        icon: "📝",
                        title: use_t("empty.title"),
                        description: use_t("empty.description"),
                        action_text: use_t("empty.action"),
                        on_action: move |_| show_add_form.set(true),
                    }
                }
            }

            // Delete confirmation modal
            if let Some(delete_id) = delete_confirm_id() {
                DeleteConfirmModal {
                    open: delete_confirm_id,
                    reminder_id: delete_id.clone(),
                    on_confirm: move |id: String| {
                        let mut updated = reminders();
                        updated.retain(|r| r.id != id);
                        reminders.set(updated);
                        save_reminders(&reminders());
                        delete_confirm_id.set(None);

                        toast_message.set(use_t("toast.deleted"));
                        toast_variant.set(ToastVariant::Success);
                        show_toast.set(true);
                    },
                    on_cancel: move |_| delete_confirm_id.set(None),
                }
            }

            // Tag Manager Modal
            TagManager {
                open: show_tag_manager,
                on_close: move |_| {
                    show_tag_manager.set(false);
                    // Reload tags after closing tag manager
                    tags.set(load_tags());
                },
            }

            // Toast notification
            Toast {
                open: show_toast,
                variant: toast_variant(),
                title: {
                    match toast_variant() {
                        ToastVariant::Success => use_t("toast.success"),
                        ToastVariant::Error => use_t("toast.error"),
                        ToastVariant::Warning => use_t("toast.warning"),
                        ToastVariant::Info => use_t("toast.info"),
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
