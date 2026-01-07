//! Mobile-specific code for Remind Me app
//! 
//! This crate contains platform-specific implementations for iOS and Android builds.

pub mod storage;
pub mod app;

// Re-export storage functions for convenience
pub use storage::{load_reminders, save_reminders, load_tags, save_tags};

// Re-export App component
pub use app::App;

