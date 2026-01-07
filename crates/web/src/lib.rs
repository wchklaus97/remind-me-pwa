//! Web-specific code for Remind Me PWA
//! 
//! This crate contains platform-specific implementations for web/WASM builds.

pub mod storage;
pub mod i18n;
pub mod router;
pub mod deployment;
pub mod app;
pub mod services;

// Re-export storage functions for convenience
pub use storage::{load_reminders, save_reminders, load_tags, save_tags};

// Re-export i18n hooks for convenience
pub use i18n::{use_init_i18n, use_i18n, use_t, use_set_locale, use_current_locale, I18nContext};

// Re-export router functions for convenience
pub use router::{get_initial_route, update_url, landing_section_href, get_landing_section_from_url, push_landing_section_url, replace_landing_section_url};

// Re-export deployment utilities
pub use deployment::{get_base_path, get_base_url, is_github_pages};

// Re-export App component
pub use app::App;

// Re-export services
pub use services::*;

