use super::super::ReminderCard;
use crate::i18n::use_t;
use dioxus::prelude::*;
use remind_me_shared::models::{Reminder, ReminderFilter, ReminderSort, Tag};
use remind_me_shared::utils::{
    format_month_year, get_current_date, get_days_in_month, get_filtered_and_sorted_reminders,
    get_first_day_of_week, group_reminders_by_date,
};
use remind_me_ui::{Button, ButtonVariant};

#[cfg(not(target_arch = "wasm32"))]
fn debug_timestamp_ms() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(not(target_arch = "wasm32"))]
fn debug_log(hypothesis_id: &str, location: &str, message: &str, data_json: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let payload = format!(
        r#"{{"sessionId":"debug-session","runId":"run1","hypothesisId":"{}","location":"{}","message":"{}","data":{},"timestamp":{}}}"#,
        hypothesis_id,
        json_escape(location),
        json_escape(message),
        data_json,
        debug_timestamp_ms()
    );
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/klaus_mac/Desktop/vibe_code/remind-me-pwa/.cursor/debug.log")
    {
        let _ = writeln!(file, "{}", payload);
    }
}

#[cfg(target_arch = "wasm32")]
fn debug_log(hypothesis_id: &str, location: &str, message: &str, data_json: &str) {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::{spawn_local, JsFuture};
    use web_sys::{Headers, RequestInit};

    let payload = format!(
        r#"{{"sessionId":"debug-session","runId":"run1","hypothesisId":"{}","location":"{}","message":"{}","data":{},"timestamp":{}}}"#,
        hypothesis_id,
        json_escape(location),
        json_escape(message),
        data_json,
        js_sys::Date::now() as u64
    );
    let endpoint = "http://127.0.0.1:7246/ingest/f5e997b2-7920-42e9-8705-dee81e872e30";
    let mut opts = RequestInit::new();
    opts.set_method("POST");
    let body_value = JsValue::from_str(&payload);
    // Use body() method which takes Option<&JsValue>
    opts.body(Some(&body_value));
    let headers = Headers::new().ok();
    if let Some(headers) = &headers {
        let _ = headers.set("Content-Type", "application/json");
    }
    if let Some(headers) = headers {
        opts.set_headers(&headers);
    }
    if let Some(window) = web_sys::window() {
        spawn_local(async move {
            let promise = window.fetch_with_str_and_init(endpoint, &opts);
            let _ = JsFuture::from(promise).await;
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn build_drag_ghost(title: &str) -> Option<web_sys::Element> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let element = document.create_element("div").ok()?;
    element.set_class_name("calendar-drag-ghost");
    element.set_text_content(Some(title));
    if let Some(body) = document.body() {
        let _ = body.append_child(&element);
    }
    Some(element)
}

#[cfg(target_arch = "wasm32")]
fn ensure_touch_ghost(title: &str) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let document = window.document()?;
    if let Ok(Some(existing)) = document.query_selector("#calendar-touch-ghost") {
        let element = existing.dyn_into::<web_sys::HtmlElement>().ok()?;
        element.set_text_content(Some(title));
        return Some(element);
    }
    let element = document
        .create_element("div")
        .ok()?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;
    element.set_id("calendar-touch-ghost");
    element.set_class_name("calendar-touch-ghost");
    element.set_text_content(Some(title));
    if let Some(body) = document.body() {
        let _ = body.append_child(&element);
    }
    Some(element)
}

#[cfg(target_arch = "wasm32")]
fn update_touch_ghost_position(x: f64, y: f64) {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(Some(element)) = document.query_selector("#calendar-touch-ghost") else {
        return;
    };
    let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    let style = html.style();
    let _ = style.set_property(
        "transform",
        &format!("translate3d({}px, {}px, 0)", x + 14.0, y + 14.0),
    ).ok();
    let _ = style.set_property("opacity", "1").ok();
}

#[cfg(target_arch = "wasm32")]
fn hide_touch_ghost() {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(Some(element)) = document.query_selector("#calendar-touch-ghost") else {
        return;
    };
    let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    let style = html.style();
    let _ = style.set_property("opacity", "0").ok();
}

#[cfg(target_arch = "wasm32")]
fn resolve_drop_target(x: f64, y: f64) -> (Option<String>, bool) {
    let Some(window) = web_sys::window() else {
        return (None, false);
    };
    let Some(document) = window.document() else {
        return (None, false);
    };
    let Some(element) = document.element_from_point(x as f32, y as f32) else {
        return (None, false);
    };

    if let Ok(Some(day)) = element.closest(".calendar-day") {
        if let Some(date) = day.get_attribute("data-date") {
            return (Some(date), false);
        }
    }

    if let Ok(Some(_)) = element.closest(".calendar-unscheduled") {
        return (None, true);
    }

    (None, false)
}

#[component]
pub fn CalendarView(
    reminders: Vec<Reminder>,
    tags: Vec<Tag>,
    filter: ReminderFilter,
    search_query: String,
    sort_by: ReminderSort,
    on_toggle: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_open: EventHandler<String>,
    on_reschedule: EventHandler<(String, Option<String>)>,
) -> Element {
    // Get filtered and sorted reminders
    let filtered_reminders =
        get_filtered_and_sorted_reminders(&reminders, &filter, &search_query, &sort_by);

    // Group reminders by date
    let reminders_by_date = group_reminders_by_date(&filtered_reminders);

    // Get reminders without dates (unscheduled)
    let unscheduled: Vec<Reminder> = filtered_reminders
        .iter()
        .filter(|r| r.due_date.is_empty())
        .cloned()
        .collect();

    // Calendar state: current month and year
    let (current_year, current_month, _) = get_current_date();
    let mut view_year = use_signal(|| current_year);
    let mut view_month = use_signal(|| current_month);
    let mut selected_date = use_signal(|| None::<String>); // YYYY-MM-DD format
    let mut dragging_id = use_signal(|| None::<String>);
    let mut drop_target_date = use_signal(|| None::<String>);
    let mut drop_target_unscheduled = use_signal(|| false);
    let mut pointer_drag_active = use_signal(|| false);

    // Get reminders for selected date
    let selected_date_reminders = if let Some(date_key) = selected_date() {
        reminders_by_date
            .get(&date_key)
            .cloned()
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    // #region agent log
    debug_log(
        "A",
        "calendar_view.rs:170",
        "calendar_view_state",
        &format!(
            r#"{{"total":{},"filtered":{},"unscheduled":{},"selected":"{}"}}"#,
            reminders.len(),
            filtered_reminders.len(),
            unscheduled.len(),
            json_escape(&selected_date().unwrap_or_default())
        ),
    );
    // #endregion

    // #region agent log
    debug_log(
        "B",
        "calendar_view.rs:205",
        "calendar_nodes_counts",
        &format!(
            r#"{{"selected_nodes":{},"unscheduled_nodes":{}}}"#,
            selected_date_reminders.len(),
            unscheduled.len()
        ),
    );
    // #endregion

    // Navigation handlers
    let handle_prev_month = move |_| {
        let mut month = view_month();
        let mut year = view_year();
        if month == 1 {
            month = 12;
            year -= 1;
        } else {
            month -= 1;
        }
        view_month.set(month);
        view_year.set(year);
        selected_date.set(None);
    };

    let handle_next_month = move |_| {
        let mut month = view_month();
        let mut year = view_year();
        if month == 12 {
            month = 1;
            year += 1;
        } else {
            month += 1;
        }
        view_month.set(month);
        view_year.set(year);
        selected_date.set(None);
    };

    let handle_today = move |_| {
        let (year, month, _) = get_current_date();
        view_year.set(year);
        view_month.set(month);
        selected_date.set(None);
    };

    // Calendar grid generation
    let days_in_month = get_days_in_month(view_year(), view_month());
    let first_day = get_first_day_of_week(view_year(), view_month());

    // Generate calendar days (using a flat structure to avoid Option pattern matching in rsx!)
    // Include multiple clones of date_key - one for each closure that needs it
    let calendar_days: Vec<(
        bool,
        u32,
        String,
        usize,
        String,
        String,
        String,
        String,
        String,
    )> = {
        let mut days = Vec::new();

        // Empty cells for days before month starts
        for _ in 0..first_day {
            days.push((
                false,
                0,
                String::new(),
                0,
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ));
        }

        // Days of the month - clone date_key for each closure
        for day in 1..=days_in_month {
            let date_key = format!("{:04}-{:02}-{:02}", view_year(), view_month(), day);
            let count = reminders_by_date
                .get(&date_key)
                .map(|r| r.len())
                .unwrap_or(0);
            days.push((
                true,
                day,
                date_key.clone(), // For attributes
                count,
                date_key.clone(), // For onclick
                date_key.clone(), // For ondragenter
                date_key.clone(), // For ondragleave
                date_key.clone(), // For ondragover
                date_key.clone(), // For ondrop
            ));
        }

        days
    };

    // Check if a date is today
    let is_today = move |date_key: &str| -> bool {
        let (year, month, day) = get_current_date();
        date_key == format!("{:04}-{:02}-{:02}", year, month, day)
    };

    // Weekday headers
    let weekday_headers = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    rsx! {
        section {
            class: "reminders-calendar-view",
            aria_label: "Calendar view of reminders",

            // Calendar header with navigation
            div {
                class: "calendar-header",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: handle_prev_month,
                    "←"
                }
                div {
                    class: "calendar-month-year",
                    h2 {
                        {format_month_year(view_year(), view_month())}
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: handle_next_month,
                    "→"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: handle_today,
                    {use_t("app.calendar.today")}
                }
            }

            // Calendar grid
            div {
                class: "calendar-grid",
                // Weekday headers
                for header in weekday_headers.iter() {
                    div {
                        class: "calendar-weekday-header",
                        {*header}
                    }
                }

                // Calendar days
                for (is_valid, day, date_key, count, date_key_onclick, date_key_ondragenter, date_key_ondragleave, date_key_ondragover, date_key_ondrop) in calendar_days {
                    if is_valid {
                        div {
                            class: {
                                let is_selected = selected_date().as_ref() == Some(&date_key);
                                let is_today_date = is_today(&date_key);
                                let is_drop_target = drop_target_date().as_ref() == Some(&date_key);
                                let mut classes = vec!["calendar-day"];
                                if is_selected {
                                    classes.push("calendar-day-selected");
                                } else if is_today_date {
                                    classes.push("calendar-day-today");
                                }
                                if is_drop_target {
                                    classes.push("is-drop-target");
                                }
                                classes.join(" ")
                            },
                            role: "button",
                            aria_label: format!("{} {}", use_t("app.calendar.day_label"), date_key),
                            r#"data-date"#: "{date_key}",
                            onclick: move |_| {
                                let dk = date_key_onclick.clone();
                                if selected_date().as_ref() == Some(&dk) {
                                    selected_date.set(None);
                                } else {
                                    selected_date.set(Some(dk));
                                }
                            },
                            ondragenter: move |_| {
                                let dk = date_key_ondragenter.clone();
                                drop_target_date.set(Some(dk));
                            },
                            ondragleave: move |_| {
                                let dk = date_key_ondragleave.clone();
                                if drop_target_date().as_ref() == Some(&dk) {
                                    drop_target_date.set(None);
                                }
                            },
                            ondragover: move |evt| {
                                evt.prevent_default();
                                let dk = date_key_ondragover.clone();
                                drop_target_date.set(Some(dk));
                            },
                            ondrop: move |evt| {
                                evt.prevent_default();
                                drop_target_date.set(None);
                                let dk = date_key_ondrop.clone();
                                let dt = evt.data_transfer();
                                let mut dropped_id = dt.get_data("text/plain").unwrap_or_default();
                                if dropped_id.is_empty() {
                                    if let Some(current) = dragging_id() {
                                        dropped_id = current;
                                    }
                                }
                                if !dropped_id.is_empty() {
                                    // #region agent log
                                    debug_log(
                                        "C",
                                        "calendar_view.rs:508",
                                        "drop_on_day",
                                        &format!(
                                            r#"{{"id":"{}","date":"{}"}}"#,
                                            json_escape(&dropped_id),
                                            json_escape(&dk)
                                        ),
                                    );
                                    // #endregion
                                    let dk_for_reschedule = dk.clone();
                                    on_reschedule.call((dropped_id, Some(dk_for_reschedule)));
                                    selected_date.set(Some(dk));
                                }
                            },
                            div {
                                class: "calendar-day-number",
                                {day.to_string()}
                            }
                            if count > 0 {
                                div {
                                    class: "calendar-day-count",
                                    {count.to_string()}
                                }
                            }
                        }
                    } else {
                        div {
                            class: "calendar-day calendar-day-empty"
                        }
                    }
                }
            }

            // Selected date reminders
            if !selected_date_reminders.is_empty() {
                div {
                    class: "calendar-selected-date-reminders",
                    h3 {
                        class: "calendar-selected-date-title",
                        {format!("{} {}", use_t("app.calendar.selected_date"), selected_date().unwrap_or_default())}
                    }
                    div {
                        class: "calendar-reminders-list",
                        // Create tuples with multiple clones of reminder for each closure
                        for (reminder, reminder_id, reminder_pointerdown, reminder_pointerup, reminder_dragstart) in selected_date_reminders.clone().into_iter().map(|r| {
                            (
                                r.clone(), // For ReminderCard
                                r.id.clone(), // For attributes
                                r.clone(), // For onpointerdown
                                r.clone(), // For onpointerup
                                r.clone(), // For ondragstart
                            )
                        }) {
                            div {
                                key: "{reminder_id.clone()}",
                                class: {
                                    if dragging_id().as_ref() == Some(&reminder_id) { "calendar-reminder-card is-dragging" } else { "calendar-reminder-card" }
                                },
                                draggable: "true",
                                aria_label: use_t("app.calendar.drag_reminder"),
                                aria_grabbed: {
                                    if dragging_id().as_ref() == Some(&reminder_id) { "true" } else { "false" }
                                },
                                onpointerdown: move |evt| {
                                    let reminder_id = reminder_pointerdown.id.clone();
                                    if evt.pointer_type() != "touch" {
                                        return;
                                    }
                                    evt.prevent_default();
                                    pointer_drag_active.set(true);
                                    dragging_id.set(Some(reminder_id.clone()));
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        if let Some(_ghost) = ensure_touch_ghost(&reminder_pointerdown.title) {
                                            update_touch_ghost_position(coords.x as f64, coords.y as f64);
                                        }
                                    }
                                },
                                onpointermove: move |evt| {
                                    if evt.pointer_type() != "touch" || !pointer_drag_active() {
                                        return;
                                    }
                                    evt.prevent_default();
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        update_touch_ghost_position(coords.x as f64, coords.y as f64);
                                        let (date, is_unscheduled) = resolve_drop_target(coords.x as f64, coords.y as f64);
                                        drop_target_date.set(date);
                                        drop_target_unscheduled.set(is_unscheduled);
                                    }
                                },
                                onpointerup: move |evt| {
                                    if evt.pointer_type() != "touch" || !pointer_drag_active() {
                                        return;
                                    }
                                    let reminder_id = reminder_pointerup.id.clone();
                                    evt.prevent_default();
                                    pointer_drag_active.set(false);
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        hide_touch_ghost();
                                        let coords = evt.client_coordinates();
                                        let (date, is_unscheduled) = resolve_drop_target(coords.x as f64, coords.y as f64);
                                        if is_unscheduled {
                                            on_reschedule.call((reminder_id.clone(), None));
                                        } else if let Some(d) = date {
                                            let d_clone = d.clone();
                                            on_reschedule.call((reminder_id.clone(), Some(d_clone)));
                                            selected_date.set(Some(d));
                                        }
                                    }
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ondragstart: move |evt| {
                                    let reminder_id = reminder_dragstart.id.clone();
                                    let dt = evt.data_transfer();
                                    let _ = dt.set_data("text/plain", &reminder_id);
                                    // Note: Dioxus DataTransfer doesn't support set_drag_image
                                    // The drag will work without a custom drag image
                                    dragging_id.set(Some(reminder_id.clone()));
                                },
                                ondragend: move |_| {
                                    dragging_id.set(None);
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ReminderCard {
                                    reminder: reminder,
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

            // Unscheduled reminders section
            if !unscheduled.is_empty() {
                div {
                    class: if drop_target_unscheduled() { "calendar-unscheduled is-drop-target" } else { "calendar-unscheduled" },
                    role: "region",
                    aria_label: use_t("app.calendar.unscheduled"),
                    ondragenter: move |_| {
                        drop_target_unscheduled.set(true);
                    },
                    ondragleave: move |_| {
                        drop_target_unscheduled.set(false);
                    },
                    ondragover: move |evt| {
                        evt.prevent_default();
                        drop_target_unscheduled.set(true);
                    },
                    ondrop: move |evt| {
                        evt.prevent_default();
                        drop_target_unscheduled.set(false);
                        let dt = evt.data_transfer();
                        let mut dropped_id = dt.get_data("text/plain").unwrap_or_default();
                        if dropped_id.is_empty() {
                            if let Some(current) = dragging_id() {
                                dropped_id = current;
                            }
                        }
                        if !dropped_id.is_empty() {
                            // #region agent log
                            debug_log(
                                "C",
                                "calendar_view.rs:557",
                                "drop_unscheduled",
                                &format!(r#"{{"id":"{}"}}"#, json_escape(&dropped_id)),
                            );
                            // #endregion
                            on_reschedule.call((dropped_id, None));
                            selected_date.set(None);
                        }
                    },
                    h3 {
                        class: "calendar-unscheduled-title",
                        {use_t("app.calendar.unscheduled")}
                        span {
                            class: "calendar-unscheduled-count",
                            " ({unscheduled.len()})"
                        }
                    }
                    div {
                        class: "calendar-reminders-list",
                        // Create tuples with multiple clones of reminder for each closure
                        for (reminder, reminder_id, reminder_pointerdown, reminder_pointerup, reminder_dragstart) in unscheduled.clone().into_iter().map(|r| {
                            (
                                r.clone(), // For ReminderCard
                                r.id.clone(), // For attributes
                                r.clone(), // For onpointerdown
                                r.clone(), // For onpointerup
                                r.clone(), // For ondragstart
                            )
                        }) {
                            div {
                                key: "{reminder_id.clone()}",
                                class: {
                                    if dragging_id().as_ref() == Some(&reminder_id) { "calendar-reminder-card is-dragging" } else { "calendar-reminder-card" }
                                },
                                draggable: "true",
                                aria_label: use_t("app.calendar.drag_reminder"),
                                aria_grabbed: {
                                    if dragging_id().as_ref() == Some(&reminder_id) { "true" } else { "false" }
                                },
                                onpointerdown: move |evt| {
                                    let reminder_id = reminder_pointerdown.id.clone();
                                    if evt.pointer_type() != "touch" {
                                        return;
                                    }
                                    evt.prevent_default();
                                    pointer_drag_active.set(true);
                                    dragging_id.set(Some(reminder_id.clone()));
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        if let Some(_ghost) = ensure_touch_ghost(&reminder_pointerdown.title) {
                                            update_touch_ghost_position(coords.x as f64, coords.y as f64);
                                        }
                                    }
                                },
                                onpointermove: move |evt| {
                                    if evt.pointer_type() != "touch" || !pointer_drag_active() {
                                        return;
                                    }
                                    evt.prevent_default();
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        update_touch_ghost_position(coords.x as f64, coords.y as f64);
                                        let (date, is_unscheduled) = resolve_drop_target(coords.x as f64, coords.y as f64);
                                        drop_target_date.set(date);
                                        drop_target_unscheduled.set(is_unscheduled);
                                    }
                                },
                                onpointerup: move |evt| {
                                    if evt.pointer_type() != "touch" || !pointer_drag_active() {
                                        return;
                                    }
                                    let reminder_id = reminder_pointerup.id.clone();
                                    evt.prevent_default();
                                    pointer_drag_active.set(false);
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        hide_touch_ghost();
                                        let coords = evt.client_coordinates();
                                        let (date, is_unscheduled) = resolve_drop_target(coords.x as f64, coords.y as f64);
                                        if is_unscheduled {
                                            on_reschedule.call((reminder_id.clone(), None));
                                        } else if let Some(d) = date {
                                            let d_clone = d.clone();
                                            on_reschedule.call((reminder_id.clone(), Some(d_clone)));
                                            selected_date.set(Some(d));
                                        }
                                    }
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ondragstart: move |evt| {
                                    let reminder_id = reminder_dragstart.id.clone();
                                    let dt = evt.data_transfer();
                                    let _ = dt.set_data("text/plain", &reminder_id);
                                    // Note: Dioxus DataTransfer doesn't support set_drag_image
                                    // The drag will work without a custom drag image
                                    dragging_id.set(Some(reminder_id.clone()));
                                },
                                ondragend: move |_| {
                                    dragging_id.set(None);
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ReminderCard {
                                    reminder: reminder,
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
