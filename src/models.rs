use serde::{Deserialize, Serialize};

/// Filter type for reminders
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReminderFilter {
    All,
    Active,
    Completed,
}

impl ReminderFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderFilter::All => "all",
            ReminderFilter::Active => "active",
            ReminderFilter::Completed => "completed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => ReminderFilter::Active,
            "completed" => ReminderFilter::Completed,
            _ => ReminderFilter::All,
        }
    }
}

/// Sort type for reminders
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReminderSort {
    Date,
    Title,
    Status,
}

impl ReminderSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderSort::Date => "date",
            ReminderSort::Title => "title",
            ReminderSort::Status => "status",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "title" => ReminderSort::Title,
            "status" => ReminderSort::Status,
            _ => ReminderSort::Date,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
    pub created_at: String,
    #[serde(default)]
    pub tag_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String, // Hex color code (e.g., "#FA8A59")
}

#[derive(Clone, Debug)]
pub struct Statistics {
    pub total: usize,
    pub active: usize,
    pub completed: usize,
    pub overdue: usize,
}

