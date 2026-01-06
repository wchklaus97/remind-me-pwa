use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant};
use crate::models::{Reminder, Tag, ReminderFilter, ReminderSort};
use crate::components::ReminderCard;
use crate::utils::{
    get_current_date, get_days_in_month, get_first_day_of_week,
    format_month_year, group_reminders_by_date, get_filtered_and_sorted_reminders,
};
use crate::i18n::use_t;

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
    let (current_year, current_month, _) = get_current_date();
    let mut view_year = use_signal(|| current_year);
    let mut view_month = use_signal(|| current_month);
    let mut selected_date = use_signal(|| None::<String>); // YYYY-MM-DD format

    // Get reminders for selected date
    let selected_date_reminders = if let Some(date_key) = selected_date() {
        reminders_by_date.get(&date_key).cloned().unwrap_or_default()
    } else {
        Vec::new()
    };

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
                    {use_t("calendar.today")}
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
                for (is_valid, day, date_key, count) in calendar_days {
                    if is_valid {
                        div {
                            class: {
                                let is_selected = selected_date().as_ref() == Some(&date_key);
                                let is_today_date = is_today(&date_key);
                                if is_selected { "calendar-day calendar-day-selected" } else if is_today_date { "calendar-day calendar-day-today" } else { "calendar-day" }
                            },
                            onclick: move |_| {
                                let dk = date_key.clone();
                                if selected_date().as_ref() == Some(&dk) {
                                    selected_date.set(None);
                                } else {
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
                        {format!("{} {}", use_t("calendar.selected_date"), selected_date().unwrap_or_default())}
                    }
                    for reminder in selected_date_reminders.iter() {
                        ReminderCard {
                            reminder: reminder.clone(),
                            tags: tags.clone(),
                            on_toggle: move |id: String| on_toggle.call(id),
                            on_edit: move |id: String| on_edit.call(id),
                            on_delete: move |id: String| on_delete.call(id),
                        }
                    }
                }
            }

            // Unscheduled reminders section
            if !unscheduled.is_empty() {
                div {
                    class: "calendar-unscheduled",
                    h3 {
                        class: "calendar-unscheduled-title",
                        {use_t("calendar.unscheduled")}
                        span {
                            class: "calendar-unscheduled-count",
                            " ({unscheduled.len()})"
                        }
                    }
                    for reminder in unscheduled.iter() {
                        ReminderCard {
                            reminder: reminder.clone(),
                            tags: tags.clone(),
                            on_toggle: move |id: String| on_toggle.call(id),
                            on_edit: move |id: String| on_edit.call(id),
                            on_delete: move |id: String| on_delete.call(id),
                        }
                    }
                }
            }
        }
    }
}
