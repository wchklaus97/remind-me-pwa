use crate::models::{Reminder, ReminderFilter, ReminderSort};
use crate::storage::{load_reminders, load_tags, save_reminders};
use crate::utils::get_filtered_and_sorted_reminders;
#[cfg(target_arch = "wasm32")]
use dioxus::dioxus_core::use_hook_with_cleanup;
use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant, EmptyState, Input, Select, SelectOption, Toast, ToastPosition,
    ToastVariant,
};
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
// Use re-exports from mod.rs to avoid clippy warnings
use crate::components::{
    AddReminderForm, CalendarView, CardView, DeleteConfirmModal, EditReminderForm, FolderView,
    ListView, StatisticsDisplay, TagManager,
};
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
    let mut detail_id = use_signal(|| None::<String>);

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

    // Keyboard shortcuts (global event listener)
    #[cfg(target_arch = "wasm32")]
    let _keyboard_listener = use_hook_with_cleanup(
        move || {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let Some(window) = web_sys::window() else {
                return None::<(
                    web_sys::Window,
                    Rc<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
                )>;
            };

            let Some(document) = window.document() else {
                return None;
            };

            let mut show_add_form_signal = show_add_form;
            let mut editing_id_signal = editing_id;
            let mut delete_confirm_id_signal = delete_confirm_id;
            let mut show_tag_manager_signal = show_tag_manager;
            let mut detail_id_signal = detail_id;

            let handler: Rc<Closure<dyn FnMut(web_sys::KeyboardEvent)>> =
                Rc::new(Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    let key = e.key();

                    // Check if user is typing in an input/textarea
                    // Don't trigger shortcuts when typing in inputs (except '/' for search)
                    let target = e.target();
                    if let Some(element) =
                        target.and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    {
                        let tag_name = element.tag_name().to_lowercase();
                        if tag_name == "input" || tag_name == "textarea" {
                            // Only allow '/' to focus search when typing in other inputs
                            // Don't allow it if already in search input
                            if key == "/" {
                                let element_id = element.id();
                                if element_id == "search_reminders" {
                                    // Don't trigger shortcut when typing '/' in search input
                                    return;
                                }
                                // Allow '/' to focus search when in other inputs
                            } else {
                                // Block all other shortcuts when in inputs
                                return;
                            }
                        }
                    }

                    match key.as_str() {
                        "n" | "N" => {
                            // Only if not already in a form and no modals open
                            if !show_add_form_signal()
                                && editing_id_signal().is_none()
                                && delete_confirm_id_signal().is_none()
                                && !show_tag_manager_signal()
                            {
                                e.prevent_default();
                                show_add_form_signal.set(true);
                            }
                        }
                        "Escape" => {
                            e.prevent_default();
                            // Close in priority order: delete confirm > tag manager > edit form > add form
                            if delete_confirm_id_signal().is_some() {
                                delete_confirm_id_signal.set(None);
                            } else if detail_id_signal().is_some() {
                                detail_id_signal.set(None);
                            } else if show_tag_manager_signal() {
                                show_tag_manager_signal.set(false);
                            } else if editing_id_signal().is_some() {
                                editing_id_signal.set(None);
                            } else if show_add_form_signal() {
                                show_add_form_signal.set(false);
                            }
                        }
                        "/" => {
                            // Only if not already in a form and not in search input
                            if !show_add_form_signal()
                                && editing_id_signal().is_none()
                                && delete_confirm_id_signal().is_none()
                                && !show_tag_manager_signal()
                            {
                                e.prevent_default();
                                // Focus search input
                                if let Some(document) = web_sys::window().and_then(|w| w.document())
                                {
                                    if let Ok(Some(search_input)) =
                                        document.query_selector("#search_reminders")
                                    {
                                        if let Some(input) = search_input
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .ok()
                                        {
                                            let _ = input.focus();
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }) as Box<dyn FnMut(_)>));

            let cb = handler.as_ref().as_ref().unchecked_ref();
            document
                .add_event_listener_with_callback("keydown", cb)
                .ok();

            Some((window, handler))
        },
        |state: Option<(
            web_sys::Window,
            Rc<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
        )>| {
            use wasm_bindgen::JsCast;

            let Some((window, handler)) = state else {
                return;
            };
            if let Some(document) = window.document() {
                let cb = handler.as_ref().as_ref().unchecked_ref();
                let _ = document.remove_event_listener_with_callback("keydown", cb);
            }
        },
    );

    rsx! {
        div {
            class: "app-container",
            a {
                href: "#main-content",
                class: "skip-nav",
                {use_t("accessibility.skip_to_content")}
            }
            header {
                class: "app-header",
                div {
                    class: "app-header-text",
                    h1 { {use_t("app.header.title")} }
                    p {
                        class: "app-header-subtitle",
                        {use_t("app.header.subtitle")}
                    }
                }
                div {
                    class: "app-header-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "btn btn-ghost icon-button".to_string(),
                        aria_label: Some(use_t("tags.manage")),
                        onclick: move |_| show_tag_manager.set(true),
                        "🏷️"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        class: "btn btn-primary".to_string(),
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
                id: "main-content",
                // Statistics section
                StatisticsDisplay { reminders: reminders() }

                if !["tags", "settings"].contains(&current_view().as_str()) {
                    section {
                        class: "controls mb-4",
                        div {
                            class: "controls-card",
                            div {
                                class: "controls-row",
                                div {
                                    class: "search-field",
                                    span { class: "search-icon", "🔍" }
                                    Input {
                                        id: "search_reminders".to_string(),
                                        name: "search".to_string(),
                                        r#type: "text",
                                        placeholder: use_t("search.placeholder"),
                                        aria_label: use_t("search.placeholder"),
                                        value: "{search_query()}",
                                        oninput: move |value| search_query.set(value),
                                    }
                                    button {
                                        class: if search_query().is_empty() {
                                            "search-clear"
                                        } else {
                                            "search-clear is-visible"
                                        },
                                        aria_label: "{use_t("search.clear")}",
                                        disabled: search_query().is_empty(),
                                        onclick: move |_| search_query.set(String::new()),
                                        "×"
                                    }
                                    kbd {
                                        class: if search_query().is_empty() {
                                            "search-kbd is-visible"
                                        } else {
                                            "search-kbd"
                                        },
                                        "/"
                                    }
                                }
                                Select {
                                    id: "sort_reminders".to_string(),
                                    name: "sort_by".to_string(),
                                    value: sort_by().as_str().to_string(),
                                    aria_label: use_t("sort.label"),
                                    onchange: move |value: String| {
                                        sort_by.set(ReminderSort::from_str(&value));
                                    },
                                    options: vec![
                                        SelectOption { value: "date".to_string(), label: use_t("sort.date") },
                                        SelectOption { value: "title".to_string(), label: use_t("sort.title") },
                                        SelectOption { value: "status".to_string(), label: use_t("sort.status") },
                                    ],
                                }
                            }
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

                if !["tags", "settings"].contains(&current_view().as_str()) {
                    nav {
                        role: "navigation",
                        aria_label: "View switcher",
                        class: "view-switcher",
                        Button {
                            variant: if current_view() == "list" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if current_view() == "list" { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("app.views.list")),
                            onclick: move |_| current_view.set("list".to_string()),
                            {use_t("app.views.list")}
                        }
                        Button {
                            variant: if current_view() == "card" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if current_view() == "card" { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("app.views.card")),
                            onclick: move |_| current_view.set("card".to_string()),
                            {use_t("app.views.card")}
                        }
                        Button {
                            variant: if current_view() == "folder" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if current_view() == "folder" { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("app.views.folder")),
                            onclick: move |_| current_view.set("folder".to_string()),
                            {use_t("app.views.folder")}
                        }
                        Button {
                            variant: if current_view() == "calendar" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if current_view() == "calendar" { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("app.views.calendar")),
                            onclick: move |_| current_view.set("calendar".to_string()),
                            {use_t("app.views.calendar")}
                        }
                    }
                }

                if !["tags", "settings"].contains(&current_view().as_str()) {
                    nav {
                        role: "navigation",
                        aria_label: "Filter reminders",
                        class: "filter-tabs",
                        Button {
                            variant: if filter() == ReminderFilter::All { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if filter() == ReminderFilter::All { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("filter.all")),
                            onclick: move |_| filter.set(ReminderFilter::All),
                            {use_t("filter.all")}
                        }
                        Button {
                            variant: if filter() == ReminderFilter::Active { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if filter() == ReminderFilter::Active { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("filter.active")),
                            onclick: move |_| filter.set(ReminderFilter::Active),
                            {use_t("filter.active")}
                        }
                        Button {
                            variant: if filter() == ReminderFilter::Completed { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            class: if filter() == ReminderFilter::Completed { "tab active".to_string() } else { "tab".to_string() },
                            aria_label: Some(use_t("filter.completed")),
                            onclick: move |_| filter.set(ReminderFilter::Completed),
                            {use_t("filter.completed")}
                        }
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
                                filter: filter(),
                                search_query: search_query(),
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
                                on_open: move |id: String| {
                                    detail_id.set(Some(id));
                                },
                                on_new_reminder: move |_| show_add_form.set(true),
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
                                on_open: move |id: String| {
                                    detail_id.set(Some(id));
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
                                on_open: move |id: String| {
                                    detail_id.set(Some(id));
                                },
                                on_reorder_tags: move |(drag_id, target_id): (String, String)| {
                                    let mut updated_tags = tags();
                                    let from_index = updated_tags.iter().position(|t| t.id == drag_id);
                                    let to_index = updated_tags.iter().position(|t| t.id == target_id);
                                    if let (Some(from), Some(to)) = (from_index, to_index) {
                                        if from != to {
                                            let tag = updated_tags.remove(from);
                                            let insert_index = if from < to { to.saturating_sub(1) } else { to };
                                            updated_tags.insert(insert_index, tag);
                                            tags.set(updated_tags.clone());
                                            save_tags(&updated_tags);
                                        }
                                    }
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
                                on_open: move |id: String| {
                                    detail_id.set(Some(id));
                                },
                                on_reschedule: move |(id, new_date): (String, Option<String>)| {
                                    let mut updated = reminders();
                                    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                        let Some(target_date) = new_date else {
                                            r.due_date = String::new();
                                            reminders.set(updated.clone());
                                            save_reminders(&updated);
                                            toast_message.set(use_t("toast.updated"));
                                            toast_variant.set(ToastVariant::Success);
                                            show_toast.set(true);
                                            return;
                                        };

                                        let next_date = if r.due_date.is_empty() {
                                            target_date.clone()
                                        } else {
                                            let local_value = to_datetime_local_value(&r.due_date);
                                            if let Some((_, time)) = local_value.split_once('T') {
                                                format!("{}T{}", target_date, time)
                                            } else {
                                                target_date.clone()
                                            }
                                        };

                                        if r.due_date == next_date {
                                            return;
                                        }
                                        r.due_date = next_date;
                                        reminders.set(updated.clone());
                                        save_reminders(&updated);

                                        toast_message.set(use_t("toast.updated"));
                                        toast_variant.set(ToastVariant::Success);
                                        show_toast.set(true);
                                    }
                                },
                            }
                        },
                        "tags" => rsx! {
                            TagManagerPanel {}
                        },
                        "settings" => rsx! {
                            SettingsView {}
                        },
                        _ => rsx! {
                            ListView {
                                reminders: filtered_reminders,
                                tags: tags(),
                                filter: filter(),
                                search_query: search_query(),
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
                                on_open: move |id: String| {
                                    detail_id.set(Some(id));
                                },
                                on_new_reminder: move |_| show_add_form.set(true),
                            }
                        },
                    }
                }

                if reminders().is_empty() && ["list", "card", "folder"].contains(&current_view().as_str()) {
                    EmptyState {
                        icon: "📝",
                        title: use_t("empty.title"),
                        description: use_t("empty.description"),
                        action_text: use_t("empty.action"),
                        class: "empty-state",
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

            if let Some(open_id) = detail_id() {
                if let Some(reminder) = reminders().iter().find(|r| r.id == open_id) {
                    ReminderDetailModal {
                        open: detail_id,
                        reminder: reminder.clone(),
                        tags: tags(),
                        on_edit: move |id: String| {
                            detail_id.set(None);
                            editing_id.set(Some(id));
                            show_add_form.set(false);
                        },
                        on_toggle: move |id: String| {
                            let mut updated = reminders();
                            if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                r.completed = !r.completed;
                                reminders.set(updated.clone());
                                save_reminders(&updated);
                                toast_message.set(use_t("toast.updated"));
                                toast_variant.set(ToastVariant::Success);
                                show_toast.set(true);
                            }
                        },
                        on_delete: move |id: String| {
                            detail_id.set(None);
                            delete_confirm_id.set(Some(id));
                        },
                    }
                }
            }

            button {
                class: "fab",
                aria_label: "{use_t("app.fab.new")}",
                onclick: move |_| {
                    current_view.set("list".to_string());
                    show_add_form.set(true);
                    editing_id.set(None);
                },
                "+"
            }

            nav {
                class: "bottom-tab-bar",
                role: "navigation",
                aria_label: use_t("app.bottom_nav.label"),
                button {
                    class: if ["list", "card", "folder"].contains(&current_view().as_str()) {
                        "bottom-tab-button active"
                    } else {
                        "bottom-tab-button"
                    },
                    aria_label: use_t("app.bottom_nav.home"),
                    onclick: move |_| current_view.set("list".to_string()),
                    span { class: "bottom-tab-icon", "🏠" }
                    span { class: "bottom-tab-label", {use_t("app.bottom_nav.home")} }
                }
                button {
                    class: if current_view() == "calendar" {
                        "bottom-tab-button active"
                    } else {
                        "bottom-tab-button"
                    },
                    aria_label: use_t("app.bottom_nav.calendar"),
                    onclick: move |_| current_view.set("calendar".to_string()),
                    span { class: "bottom-tab-icon", "📅" }
                    span { class: "bottom-tab-label", {use_t("app.bottom_nav.calendar")} }
                }
                button {
                    class: if current_view() == "tags" {
                        "bottom-tab-button active"
                    } else {
                        "bottom-tab-button"
                    },
                    aria_label: use_t("app.bottom_nav.groups"),
                    onclick: move |_| current_view.set("tags".to_string()),
                    span { class: "bottom-tab-icon", "🏷️" }
                    span { class: "bottom-tab-label", {use_t("app.bottom_nav.groups")} }
                }
                button {
                    class: if current_view() == "settings" {
                        "bottom-tab-button active"
                    } else {
                        "bottom-tab-button"
                    },
                    aria_label: use_t("app.bottom_nav.settings"),
                    onclick: move |_| current_view.set("settings".to_string()),
                    span { class: "bottom-tab-icon", "⚙️" }
                    span { class: "bottom-tab-label", {use_t("app.bottom_nav.settings")} }
                }
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

#[component]
fn ReminderDetailModal(
    open: Signal<Option<String>>,
    reminder: Reminder,
    tags: Vec<Tag>,
    on_edit: EventHandler<String>,
    on_toggle: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let mut is_open = use_signal(|| true);
    let close_modal = move |_| {
        is_open.set(false);
        open.set(None);
    };

    rsx! {
        Modal {
            open: is_open,
            size: ModalSize::Large,
            title: use_t("reminder.detail_title"),
            on_close: move |_| close_modal(()),
            div {
                class: "reminder-detail",
                h2 {
                    class: "reminder-detail-title",
                    "{reminder.title}"
                }
                if !reminder.due_date.is_empty() {
                    p {
                        class: "reminder-detail-meta",
                        "{use_t("reminder.detail_due")} {format_date(&reminder.due_date)}"
                    }
                }
                if !reminder.created_at.is_empty() {
                    p {
                        class: "reminder-detail-meta",
                        "{use_t("reminder.detail_created")} {format_date(&reminder.created_at)}"
                    }
                }
                if !reminder.tag_ids.is_empty() {
                    div {
                        class: "reminder-detail-tags",
                        for tag_id in reminder.tag_ids.iter() {
                            if let Some(tag) = tags.iter().find(|t| t.id == *tag_id) {
                                span {
                                    class: "tag-chip",
                                    style: format!("background-color: {};", tag.color),
                                    "{tag.name}"
                                }
                            }
                        }
                    }
                }
                if !reminder.description.is_empty() {
                    div {
                        class: "reminder-detail-notes",
                        h3 { class: "reminder-detail-section", {use_t("reminder.detail_notes")} }
                        p { "{reminder.description}" }
                    }
                }
                div {
                    class: "reminder-detail-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "btn btn-ghost".to_string(),
                        aria_label: Some(use_t("reminder.detail_edit")),
                        onclick: move |_| on_edit.call(reminder.id.clone()),
                        {use_t("reminder.detail_edit")}
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        class: "btn btn-primary".to_string(),
                        aria_label: Some(use_t("reminder.detail_toggle")),
                        onclick: move |_| on_toggle.call(reminder.id.clone()),
                        {
                            if reminder.completed {
                                {use_t("reminder.detail_mark_active")}
                            } else {
                                {use_t("reminder.detail_mark_done")}
                            }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Danger,
                        class: "btn btn-danger".to_string(),
                        aria_label: Some(use_t("reminder.detail_delete")),
                        onclick: move |_| on_delete.call(reminder.id.clone()),
                        {use_t("reminder.detail_delete")}
                    }
                }
            }
        }
    }
}

#[component]
fn SettingsView() -> Element {
    let mut notifications = use_signal(|| true);
    let mut week_starts_monday = use_signal(|| false);
    let mut use_24h = use_signal(|| false);

    rsx! {
        section {
            class: "settings-view",
            aria_label: use_t("settings.title"),
            h2 { class: "settings-title", {use_t("settings.title")} }
            p { class: "settings-subtitle", {use_t("settings.subtitle")} }

            div {
                class: "settings-card",
                div { class: "settings-row",
                    label { class: "settings-label", {use_t("settings.notifications")} }
                    input {
                        r#type: "checkbox",
                        aria_label: use_t("settings.notifications"),
                        checked: notifications(),
                        onchange: move |_| notifications.set(!notifications()),
                    }
                }
                div { class: "settings-row",
                    label { class: "settings-label", {use_t("settings.week_start")} }
                    input {
                        r#type: "checkbox",
                        aria_label: use_t("settings.week_start"),
                        checked: week_starts_monday(),
                        onchange: move |_| week_starts_monday.set(!week_starts_monday()),
                    }
                }
                div { class: "settings-row",
                    label { class: "settings-label", {use_t("settings.time_format")} }
                    input {
                        r#type: "checkbox",
                        aria_label: use_t("settings.time_format"),
                        checked: use_24h(),
                        onchange: move |_| use_24h.set(!use_24h()),
                    }
                }
            }
        }
    }
}
