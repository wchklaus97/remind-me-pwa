//! Web i18n shim.
//!
//! The canonical i18n context type lives in `remind-me-components` so that UI components and the
//! app root agree on the exact context type (Dioxus contexts are type-based).

pub use remind_me_components::i18n::{
    use_current_locale, use_i18n, use_init_i18n, use_set_locale, use_t, I18nContext, Locale,
};

