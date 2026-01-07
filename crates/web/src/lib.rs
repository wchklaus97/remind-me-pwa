//! Web-specific code for Remind Me PWA
//! 
//! This crate contains platform-specific implementations for web/WASM builds.

pub mod storage;

// Re-export storage functions for convenience
pub use storage::{load_reminders, save_reminders, load_tags, save_tags};

