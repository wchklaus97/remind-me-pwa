//! Web-specific code for Remind Me PWA
//! 
//! This crate contains platform-specific implementations for web/WASM builds.

pub mod storage;
pub mod i18n;

// Re-export storage functions for convenience
pub use storage::{load_reminders, save_reminders, load_tags, save_tags};

// Re-export i18n hooks for convenience
pub use i18n::{use_init_i18n, use_i18n, use_t, use_set_locale, use_current_locale, I18nContext};

