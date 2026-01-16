use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant};
use crate::models::{Reminder, Tag, ReminderFilter, ReminderSort};
use crate::components::ReminderCard;
use crate::utils::{
    get_current_date, get_days_in_month, get_first_day_of_week,
    format_month_year, group_reminders_by_date, get_filtered_and_sorted_reminders,
};
use crate::i18n::use_t;

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

fn parse_date_key(date_key: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = date_key.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;
    Some((year, month, day))
}

#[cfg(target_arch = "wasm32")]
fn add_days(date_key: &str, delta_days: i32) -> Option<String> {
    let d = js_sys::Date::new(&date_key.into());
    let ms = d.get_time() + (delta_days as f64) * 86_400_000.0;
    let next = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
    Some(format!(
        "{:04}-{:02}-{:02}",
        next.get_full_year(),
        next.get_month() + 1,
        next.get_date()
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn add_days(date_key: &str, delta_days: i32) -> Option<String> {
    use chrono::{NaiveDate, Duration, Datelike};
    let (year, month, day) = parse_date_key(date_key)?;
    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    let next = date.checked_add_signed(Duration::days(delta_days as i64))?;
    Some(format!("{:04}-{:02}-{:02}", next.year(), next.month(), next.day()))
}

#[cfg(target_arch = "wasm32")]
fn build_week_dates(anchor: &str) -> Vec<String> {
    let d = js_sys::Date::new(&anchor.into());
    let weekday = d.get_day() as i32;
    let base_ms = d.get_time();
    (0..7)
        .map(|i| {
            let offset = i - weekday;
            let ms = base_ms + (offset as f64) * 86_400_000.0;
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
            format!(
                "{:04}-{:02}-{:02}",
                date.get_full_year(),
                date.get_month() + 1,
                date.get_date()
            )
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn build_week_dates(anchor: &str) -> Vec<String> {
    use chrono::{Datelike, NaiveDate, Duration};
    let (year, month, day) = match parse_date_key(anchor) {
        Some(parts) => parts,
        None => return Vec::new(),
    };
    let date = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => return Vec::new(),
    };
    let weekday = date.weekday().num_days_from_sunday() as i64;
    (0..7)
        .map(|i| {
            let offset = i as i64 - weekday;
            let next = date + Duration::days(offset);
            format!("{:04}-{:02}-{:02}", next.year(), next.month(), next.day())
        })
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn ensure_touch_ghost(title: &str) -> Option<web_sys::HtmlElement> {
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
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Ok(Some(element)) = document.query_selector("#calendar-touch-ghost") else { return };
    let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else { return };
    let style = html.style();
    let _ = style.set_property("transform", &format!("translate3d({}px, {}px, 0)", x + 14.0, y + 14.0));
    let _ = style.set_property("opacity", "1");
}

#[cfg(target_arch = "wasm32")]
fn hide_touch_ghost() {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Ok(Some(element)) = document.query_selector("#calendar-touch-ghost") else { return };
    let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else { return };
    let style = html.style();
    let _ = style.set_property("opacity", "0");
}

#[cfg(target_arch = "wasm32")]
fn resolve_drop_target(x: f64, y: f64) -> (Option<String>, bool) {
    let Some(window) = web_sys::window() else { return (None, false) };
    let Some(document) = window.document() else { return (None, false) };
    let Some(element) = document.element_from_point(x, y) else { return (None, false) };

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
    let filtered_reminders = get_filtered_and_sorted_reminders(
        &reminders,
        &filter,
        &search_query,
        &sort_by,
    );

    // Group reminders by date
    let reminders_by_date = group_reminders_by_date(&filtered_reminders);

    // Get reminders without dates (unscheduled)
    let unscheduled: Vec<Reminder> = filtered_reminders
        .iter()
        .filter(|r| r.due_date.is_empty())
        .cloned()
        .collect();

    // Calendar state: current month and year
    let (current_year, current_month, current_day) = get_current_date();
    let mut view_year = use_signal(|| current_year);
    let mut view_month = use_signal(|| current_month);
    let mut selected_date = use_signal(|| None::<String>); // YYYY-MM-DD format
    let mut dragging_id = use_signal(|| None::<String>);
    let mut drop_target_date = use_signal(|| None::<String>);
    let mut drop_target_unscheduled = use_signal(|| false);
    let mut pointer_drag_active = use_signal(|| false);
    let mut calendar_mode = use_signal(|| "month".to_string());

    // Get reminders for selected date
    let selected_date_reminders = if let Some(date_key) = selected_date() {
        reminders_by_date.get(&date_key).cloned().unwrap_or_default()
    } else {
        Vec::new()
    };

    // Navigation handlers
    let handle_prev_month = move |_| {
        if calendar_mode() == "week" {
            let anchor = selected_date()
                .unwrap_or_else(|| format!("{:04}-{:02}-{:02}", current_year, current_month, current_day));
            if let Some(next) = add_days(&anchor, -7) {
                if let Some((year, month, _)) = parse_date_key(&next) {
                    view_year.set(year);
                    view_month.set(month);
                }
                selected_date.set(Some(next));
            }
            return;
        }
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
        if calendar_mode() == "week" {
            let anchor = selected_date()
                .unwrap_or_else(|| format!("{:04}-{:02}-{:02}", current_year, current_month, current_day));
            if let Some(next) = add_days(&anchor, 7) {
                if let Some((year, month, _)) = parse_date_key(&next) {
                    view_year.set(year);
                    view_month.set(month);
                }
                selected_date.set(Some(next));
            }
            return;
        }
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
        let (year, month, day) = get_current_date();
        view_year.set(year);
        view_month.set(month);
        if calendar_mode() == "week" {
            selected_date.set(Some(format!("{:04}-{:02}-{:02}", year, month, day)));
        } else {
            selected_date.set(None);
        }
    };

    // Calendar grid generation
    let days_in_month = get_days_in_month(view_year(), view_month());
    let first_day = get_first_day_of_week(view_year(), view_month());

    // Generate calendar days (using a flat structure to avoid Option pattern matching in rsx!)
    let calendar_days: Vec<(bool, u32, String, usize)> = {
        let mut days = Vec::new();

        // Empty cells for days before month starts
        for _ in 0..first_day {
            days.push((false, 0, String::new(), 0));
        }

        // Days of the month
        for day in 1..=days_in_month {
            let date_key = format!("{:04}-{:02}-{:02}", view_year(), view_month(), day);
            let count = reminders_by_date.get(&date_key).map(|r| r.len()).unwrap_or(0);
            days.push((true, day, date_key, count));
        }

        days
    };

    let week_anchor = selected_date()
        .unwrap_or_else(|| format!("{:04}-{:02}-{:02}", current_year, current_month, current_day));
    let week_dates = build_week_dates(&week_anchor);

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
                div {
                    class: "calendar-nav",
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "btn btn-ghost btn-icon".to_string(),
                        aria_label: Some(use_t("app.calendar.prev_month")),
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
                        class: "btn btn-ghost btn-icon".to_string(),
                        aria_label: Some(use_t("app.calendar.next_month")),
                        onclick: handle_next_month,
                        "→"
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    class: "btn btn-ghost calendar-today".to_string(),
                    aria_label: Some(use_t("app.calendar.today")),
                    onclick: handle_today,
                    {use_t("app.calendar.today")}
                }
            }

            div {
                class: "calendar-mode-toggle",
                Button {
                    variant: if calendar_mode() == "month" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    class: if calendar_mode() == "month" { "tab active".to_string() } else { "tab".to_string() },
                    aria_label: Some(use_t("app.calendar.month_view")),
                    onclick: move |_| calendar_mode.set("month".to_string()),
                    {use_t("app.calendar.month_view")}
                }
                Button {
                    variant: if calendar_mode() == "week" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    class: if calendar_mode() == "week" { "tab active".to_string() } else { "tab".to_string() },
                    aria_label: Some(use_t("app.calendar.week_view")),
                    onclick: move |_| {
                        if selected_date().is_none() {
                            selected_date.set(Some(format!("{:04}-{:02}-{:02}", current_year, current_month, current_day)));
                        }
                        calendar_mode.set("week".to_string());
                    },
                    {use_t("app.calendar.week_view")}
                }
            }

            // Calendar grid
            div {
                class: if calendar_mode() == "week" { "calendar-grid calendar-week" } else { "calendar-grid" },
                // Weekday headers
                for header in weekday_headers.iter() {
                    div {
                        class: "calendar-weekday-header",
                        {*header}
                    }
                }

                if calendar_mode() == "week" {
                    for date_key in week_dates {
                        let day = parse_date_key(&date_key).map(|(_, _, d)| d).unwrap_or(0);
                        let count = reminders_by_date.get(&date_key).map(|r| r.len()).unwrap_or(0);
                        let is_drop_target = drop_target_date().as_ref() == Some(&date_key);
                        div {
                            class: {
                                let is_selected = selected_date().as_ref() == Some(&date_key);
                                let is_today_date = is_today(&date_key);
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
                            data_date: "{date_key}",
                            onclick: move |_| {
                                let dk = date_key.clone();
                                selected_date.set(Some(dk));
                            },
                            ondragenter: move |_| {
                                drop_target_date.set(Some(date_key.clone()));
                            },
                            ondragleave: move |_| {
                                if drop_target_date().as_ref() == Some(&date_key) {
                                    drop_target_date.set(None);
                                }
                            },
                            ondragover: move |evt| {
                                evt.prevent_default();
                                drop_target_date.set(Some(date_key.clone()));
                            },
                            ondrop: move |evt| {
                                evt.prevent_default();
                                drop_target_date.set(None);
                                let mut dropped_id = String::new();
                                if let Some(dt) = evt.data_transfer() {
                                    if let Ok(value) = dt.get_data("text/plain") {
                                        dropped_id = value;
                                    }
                                }
                                if dropped_id.is_empty() {
                                    if let Some(current) = dragging_id() {
                                        dropped_id = current;
                                    }
                                }
                                if !dropped_id.is_empty() {
                                    on_reschedule.call((dropped_id, Some(date_key.clone())));
                                    selected_date.set(Some(date_key.clone()));
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
                    }
                } else {
                    // Calendar days
                    for (is_valid, day, date_key, count) in calendar_days {
                        if is_valid {
                            let is_drop_target = drop_target_date().as_ref() == Some(&date_key);
                            div {
                                class: {
                                    let is_selected = selected_date().as_ref() == Some(&date_key);
                                    let is_today_date = is_today(&date_key);
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
                                data_date: "{date_key}",
                                onclick: move |_| {
                                    let dk = date_key.clone();
                                    if selected_date().as_ref() == Some(&dk) {
                                        selected_date.set(None);
                                    } else {
                                        selected_date.set(Some(dk));
                                    }
                                },
                                ondragenter: move |_| {
                                    drop_target_date.set(Some(date_key.clone()));
                                },
                                ondragleave: move |_| {
                                    if drop_target_date().as_ref() == Some(&date_key) {
                                        drop_target_date.set(None);
                                    }
                                },
                                ondragover: move |evt| {
                                    evt.prevent_default();
                                    drop_target_date.set(Some(date_key.clone()));
                                },
                                ondrop: move |evt| {
                                    evt.prevent_default();
                                    drop_target_date.set(None);
                                    let mut dropped_id = String::new();
                                    if let Some(dt) = evt.data_transfer() {
                                        if let Ok(value) = dt.get_data("text/plain") {
                                            dropped_id = value;
                                        }
                                    }
                                    if dropped_id.is_empty() {
                                        if let Some(current) = dragging_id() {
                                            dropped_id = current;
                                        }
                                    }
                                    if !dropped_id.is_empty() {
                                        on_reschedule.call((dropped_id, Some(date_key.clone())));
                                        selected_date.set(Some(date_key.clone()));
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
                        for reminder in selected_date_reminders.iter() {
                            let reminder_id = reminder.id.clone();
                            let is_dragging = dragging_id().as_ref() == Some(&reminder_id);
                            div {
                                class: if is_dragging { "calendar-reminder-card is-dragging" } else { "calendar-reminder-card" },
                                draggable: "true",
                                aria_label: use_t("app.calendar.drag_reminder"),
                                aria_grabbed: if is_dragging { "true" } else { "false" },
                                onpointerdown: move |evt| {
                                    if evt.pointer_type() != "touch" {
                                        return;
                                    }
                                    evt.prevent_default();
                                    pointer_drag_active.set(true);
                                    dragging_id.set(Some(reminder_id.clone()));
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        if let Some(_ghost) = ensure_touch_ghost(&reminder.title) {
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
                                            on_reschedule.call((reminder_id.clone(), Some(d)));
                                            selected_date.set(Some(d));
                                        }
                                    }
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ondragstart: move |evt| {
                                    if let Some(dt) = evt.data_transfer() {
                                        let _ = dt.set_data("text/plain", &reminder_id);
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(ghost) = build_drag_ghost(&reminder.title) {
                                                dt.set_drag_image(&ghost, 16, 16);
                                                let _ = ghost.remove();
                                            }
                                        }
                                    }
                                    dragging_id.set(Some(reminder_id.clone()));
                                },
                                ondragend: move |_| {
                                    dragging_id.set(None);
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
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
                        let mut dropped_id = String::new();
                        if let Some(dt) = evt.data_transfer() {
                            if let Ok(value) = dt.get_data("text/plain") {
                                dropped_id = value;
                            }
                        }
                        if dropped_id.is_empty() {
                            if let Some(current) = dragging_id() {
                                dropped_id = current;
                            }
                        }
                        if !dropped_id.is_empty() {
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
                        for reminder in unscheduled.iter() {
                            let reminder_id = reminder.id.clone();
                            let is_dragging = dragging_id().as_ref() == Some(&reminder_id);
                            div {
                                class: if is_dragging { "calendar-reminder-card is-dragging" } else { "calendar-reminder-card" },
                                draggable: "true",
                                aria_label: use_t("app.calendar.drag_reminder"),
                                aria_grabbed: if is_dragging { "true" } else { "false" },
                                onpointerdown: move |evt| {
                                    if evt.pointer_type() != "touch" {
                                        return;
                                    }
                                    evt.prevent_default();
                                    pointer_drag_active.set(true);
                                    dragging_id.set(Some(reminder_id.clone()));
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let coords = evt.client_coordinates();
                                        if let Some(_ghost) = ensure_touch_ghost(&reminder.title) {
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
                                            on_reschedule.call((reminder_id.clone(), Some(d)));
                                            selected_date.set(Some(d));
                                        }
                                    }
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
                                ondragstart: move |evt| {
                                    if let Some(dt) = evt.data_transfer() {
                                        let _ = dt.set_data("text/plain", &reminder_id);
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(ghost) = build_drag_ghost(&reminder.title) {
                                                dt.set_drag_image(&ghost, 16, 16);
                                                let _ = ghost.remove();
                                            }
                                        }
                                    }
                                    dragging_id.set(Some(reminder_id.clone()));
                                },
                                ondragend: move |_| {
                                    dragging_id.set(None);
                                    drop_target_date.set(None);
                                    drop_target_unscheduled.set(false);
                                },
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
