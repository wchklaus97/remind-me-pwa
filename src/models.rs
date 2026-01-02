use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct Statistics {
    pub total: usize,
    pub active: usize,
    pub completed: usize,
    pub overdue: usize,
}

