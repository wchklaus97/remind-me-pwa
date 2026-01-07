//! Mobile-specific code for Remind Me app
//! 
//! This crate contains platform-specific implementations for iOS and Android builds.

pub mod storage;
pub mod app;
pub mod i18n;

// Re-export storage functions for convenience
pub use storage::{load_reminders, save_reminders, load_tags, save_tags};

// Re-export i18n hooks for convenience
pub use i18n::{use_init_i18n, use_i18n, use_t, use_set_locale, use_current_locale, I18nContext};

// Re-export App component
pub use app::App;

