use crate::models::{Reminder, Statistics};

#[cfg(not(target_arch = "wasm32"))]
use chrono::TimeZone;

pub fn format_date(date_str: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return format_date_wasm(date_str);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
            dt.format("%Y-%m-%d %H:%M").to_string()
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
            dt.format("%Y-%m-%d %H:%M").to_string()
        } else {
            date_str.to_string()
        }
    }
}

pub fn parse_date_for_sort(date_str: &str) -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        return parse_date_for_sort_wasm(date_str);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
            dt.timestamp_millis()
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
            if let Some(local_dt) = chrono::Local.from_local_datetime(&dt).single() {
                local_dt.timestamp_millis()
            } else {
                i64::MAX
            }
        } else {
            i64::MAX
        }
    }
}

pub fn calculate_statistics(reminders: &[Reminder]) -> Statistics {
    let total = reminders.len();
    let active = reminders.iter().filter(|r| !r.completed).count();
    let completed = reminders.iter().filter(|r| r.completed).count();
    let overdue = reminders
        .iter()
        .filter(|r| {
            if r.completed || r.due_date.is_empty() {
                return false;
            }
            is_overdue(&r.due_date)
        })
        .count();

    Statistics {
        total,
        active,
        completed,
        overdue,
    }
}

/// Filter and sort reminders based on filter, search query, and sort criteria.
/// 
/// Performance: This function is optimized for typical use cases (<100 reminders).
/// For larger datasets (>500 items), consider implementing virtual scrolling
/// or debounced search input. The current implementation uses efficient iterator
/// chains and in-place sorting.
pub fn get_filtered_and_sorted_reminders(
    reminders: &[Reminder],
    filter: &str,
    search_query: &str,
    sort_by: &str,
) -> Vec<Reminder> {
    let mut filtered: Vec<Reminder> = reminders
        .iter()
        .filter(|r| {
            // Apply filter
            let matches_filter = match filter {
                "active" => !r.completed,
                "completed" => r.completed,
                _ => true,
            };

            // Apply search
            let matches_search = search_query.is_empty()
                || r.title
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || r.description
                    .to_lowercase()
                    .contains(&search_query.to_lowercase());

            matches_filter && matches_search
        })
        .cloned()
        .collect();

    // Sort reminders
    match sort_by {
        "title" => {
            filtered.sort_by(|a, b| a.title.cmp(&b.title));
        }
        "status" => {
            filtered.sort_by(|a, b| {
                // Completed items last
                match (a.completed, b.completed) {
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        _ => {
            // Sort by date (default)
            filtered.sort_by(|a, b| {
                let date_a = parse_date_for_sort(&a.due_date);
                let date_b = parse_date_for_sort(&b.due_date);
                date_a.cmp(&date_b)
            });
        }
    }

    filtered
}

pub fn now_timestamp_millis() -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        return js_sys::Date::now() as i64;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        return chrono::Utc::now().timestamp_millis();
    }
}

pub fn now_rfc3339() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let d = js_sys::Date::new_0();
        return d.to_iso_string().as_string().unwrap_or_default();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        return chrono::Utc::now().to_rfc3339();
    }
}

pub fn is_overdue(date_str: &str) -> bool {
    let Some(due_ms) = parse_date_to_epoch_ms(date_str) else {
        return false;
    };
    due_ms < now_timestamp_millis()
}

pub fn to_datetime_local_value(date_str: &str) -> String {
    // If it's already a datetime-local value, keep it.
    if date_str.len() >= 16 && date_str.as_bytes().get(10) == Some(&b'T') {
        return date_str.chars().take(16).collect();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let Some(ms) = parse_date_to_epoch_ms(date_str) else {
            return date_str.to_string();
        };
        let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
        return format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            d.get_full_year(),
            d.get_month() + 1,
            d.get_date(),
            d.get_hours(),
            d.get_minutes()
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Fallback: try chrono parse for consistent formatting.
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
            return dt.format("%Y-%m-%dT%H:%M").to_string();
        }
        date_str.to_string()
    }
}

fn parse_date_to_epoch_ms(date_str: &str) -> Option<i64> {
    if date_str.trim().is_empty() {
        return None;
    }

    #[cfg(target_arch = "wasm32")]
    {
        let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(date_str));
        let ms = d.get_time();
        if ms.is_nan() {
            None
        } else {
            Some(ms as i64)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
            Some(dt.timestamp_millis())
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
            chrono::Local
                .from_local_datetime(&dt)
                .single()
                .map(|dt| dt.timestamp_millis())
        } else {
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn format_date_wasm(date_str: &str) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(date_str));
    let ms = d.get_time();
    if ms.is_nan() {
        return date_str.to_string();
    }

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        d.get_full_year(),
        d.get_month() + 1,
        d.get_date(),
        d.get_hours(),
        d.get_minutes()
    )
}

#[cfg(target_arch = "wasm32")]
fn parse_date_for_sort_wasm(date_str: &str) -> i64 {
    parse_date_to_epoch_ms(date_str).unwrap_or(i64::MAX)
}

/// Extract date part (YYYY-MM-DD) from a date string
pub fn extract_date_key(date_str: &str) -> Option<String> {
    if date_str.trim().is_empty() {
        return None;
    }

    #[cfg(target_arch = "wasm32")]
    {
        let Some(ms) = parse_date_to_epoch_ms(date_str) else {
            return None;
        };
        let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
        if d.get_time().is_nan() {
            return None;
        }
        Some(format!(
            "{:04}-{:02}-{:02}",
            d.get_full_year(),
            d.get_month() + 1,
            d.get_date()
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
            Some(dt.format("%Y-%m-%d").to_string())
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
            Some(dt.format("%Y-%m-%d").to_string())
        } else {
            None
        }
    }
}

/// Group reminders by date key (YYYY-MM-DD)
pub fn group_reminders_by_date(reminders: &[Reminder]) -> std::collections::HashMap<String, Vec<Reminder>> {
    let mut grouped = std::collections::HashMap::new();
    
    for reminder in reminders {
        if let Some(date_key) = extract_date_key(&reminder.due_date) {
            grouped.entry(date_key).or_insert_with(Vec::new).push(reminder.clone());
        }
    }
    
    // Sort reminders within each date group
    for reminders_in_date in grouped.values_mut() {
        reminders_in_date.sort_by(|a, b| {
            let date_a = parse_date_for_sort(&a.due_date);
            let date_b = parse_date_for_sort(&b.due_date);
            date_a.cmp(&date_b)
        });
    }
    
    grouped
}

/// Calendar helper: Get current date components (year, month, day)
#[cfg(target_arch = "wasm32")]
pub fn get_current_date() -> (i32, u32, u32) {
    let d = js_sys::Date::new_0();
    (
        d.get_full_year() as i32,
        (d.get_month() + 1) as u32,
        d.get_date() as u32,
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_current_date() -> (i32, u32, u32) {
    use chrono::Datelike;
    let now = chrono::Local::now();
    (now.year(), now.month(), now.day())
}

/// Calendar helper: Get number of days in a month
#[cfg(target_arch = "wasm32")]
pub fn get_days_in_month(year: i32, month: u32) -> u32 {
    // Create a date for the first day of next month, then subtract 1 day to get last day of current month
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let d = js_sys::Date::new(&format!("{}-{:02}-01", next_year, next_month).into());
    d.set_date(0); // Set to last day of previous month (current month)
    d.get_date() as u32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_days_in_month(year: i32, month: u32) -> u32 {
    use chrono::{Datelike, NaiveDate, Duration};
    // Get last day of month by going to first day of next month and subtracting 1 day
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    if let Some(first_of_next) = NaiveDate::from_ymd_opt(next_year, next_month, 1) {
        (first_of_next - Duration::days(1)).day()
    } else {
        31 // Fallback
    }
}

/// Calendar helper: Get day of week for the first day of a month (0 = Sunday, 6 = Saturday)
#[cfg(target_arch = "wasm32")]
pub fn get_first_day_of_week(year: i32, month: u32) -> u32 {
    let d = js_sys::Date::new(&format!("{}-{:02}-01", year, month).into());
    d.get_day() as u32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_first_day_of_week(year: i32, month: u32) -> u32 {
    use chrono::{Datelike, NaiveDate};
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, 1) {
        date.weekday().num_days_from_sunday() as u32
    } else {
        0 // Fallback to Sunday
    }
}

/// Format month and year for display (e.g., "January 2024")
#[cfg(target_arch = "wasm32")]
pub fn format_month_year(year: i32, month: u32) -> String {
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    if month >= 1 && month <= 12 {
        format!("{} {}", month_names[(month - 1) as usize], year)
    } else {
        format!("Month {} {}", month, year)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn format_month_year(year: i32, month: u32) -> String {
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    if month >= 1 && month <= 12 {
        format!("{} {}", month_names[(month - 1) as usize], year)
    } else {
        format!("Month {} {}", month, year)
    }
}

