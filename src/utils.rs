use chrono::TimeZone;
use crate::models::{Reminder, Statistics};

/// Convert datetime-local format (YYYY-MM-DDTHH:MM) to RFC3339 format
/// Returns the RFC3339 string, or the original string if parsing fails
pub fn convert_datetime_local_to_rfc3339(date_str: &str) -> String {
    if date_str.is_empty() {
        return String::new();
    }
    
    // Try parsing as datetime-local format (YYYY-MM-DDTHH:MM)
    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
        // Convert to local timezone, then to UTC
        if let Some(local_dt) = chrono::Local.from_local_datetime(&naive_dt).single() {
            return local_dt.with_timezone(&chrono::Utc).to_rfc3339();
        }
    }
    
    // If already in RFC3339 format or parsing fails, return as-is
    date_str.to_string()
}

pub fn format_date(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else {
        date_str.to_string()
    }
}

pub fn parse_date_for_sort(date_str: &str) -> chrono::DateTime<chrono::Utc> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.with_timezone(&chrono::Utc)
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
        if let Some(local_dt) = chrono::Local.from_local_datetime(&dt).single() {
            local_dt.with_timezone(&chrono::Utc)
        } else {
            chrono::Utc::now()
        }
    } else {
        chrono::Utc::now()
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
            if let Ok(due) = chrono::DateTime::parse_from_rfc3339(&r.due_date) {
                due < chrono::Utc::now()
            } else if let Ok(due) =
                chrono::NaiveDateTime::parse_from_str(&r.due_date, "%Y-%m-%dT%H:%M")
            {
                if let Some(local_dt) = chrono::Local.from_local_datetime(&due).single() {
                    local_dt < chrono::Local::now()
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count();

    Statistics {
        total,
        active,
        completed,
        overdue,
    }
}

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

