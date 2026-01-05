use serde::{Deserialize, Serialize};

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

