//! Data models for the Remind Me application
//! 
//! This module contains all shared data structures used across platforms.

use serde::{Deserialize, Serialize};

/// Filter type for reminders
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReminderFilter {
    All,
    Active,
    Completed,
}

impl ReminderFilter {
    /// Convert filter to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderFilter::All => "all",
            ReminderFilter::Active => "active",
            ReminderFilter::Completed => "completed",
        }
    }

    /// Create filter from string
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
    /// Convert sort to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderSort::Date => "date",
            ReminderSort::Title => "title",
            ReminderSort::Status => "status",
        }
    }

    /// Create sort from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "title" => ReminderSort::Title,
            "status" => ReminderSort::Status,
            _ => ReminderSort::Date,
        }
    }
}

/// Reminder data structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    /// Unique identifier for the reminder
    pub id: String,
    /// Title of the reminder
    pub title: String,
    /// Description/details of the reminder
    pub description: String,
    /// Due date in ISO 8601 format
    pub due_date: String,
    /// Whether the reminder is completed
    pub completed: bool,
    /// Creation timestamp in ISO 8601 format
    pub created_at: String,
    /// List of tag IDs associated with this reminder
    #[serde(default)]
    pub tag_ids: Vec<String>,
}

/// Tag data structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// Unique identifier for the tag
    pub id: String,
    /// Name of the tag
    pub name: String,
    /// Hex color code (e.g., "#FA8A59")
    pub color: String,
}

/// Statistics data structure
#[derive(Clone, Debug)]
pub struct Statistics {
    /// Total number of reminders
    pub total: usize,
    /// Number of active (incomplete) reminders
    pub active: usize,
    /// Number of completed reminders
    pub completed: usize,
    /// Number of overdue reminders
    pub overdue: usize,
}

