//! i18n hooks for components
//! 
//! Components access i18n through Dioxus context.
//! Platform crates (web/mobile) provide the actual I18nContext implementation.

use dioxus::prelude::*;

// Re-export Locale from shared
pub use remind_me_shared::i18n::Locale;

// For now, provide a simple implementation that components can use
// Platform crates will override this by providing their own I18nContext

/// Simple I18nContext for components
/// Platform crates will provide their own implementation via context
#[derive(Clone)]
pub struct I18nContext {
    current_locale: Locale,
}

impl I18nContext {
    pub fn new() -> Self {
        Self {
            current_locale: Locale::En,
        }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.current_locale = locale;
    }

    pub fn current_locale_str(&self) -> &'static str {
        self.current_locale.as_str()
    }

    pub fn t(&self, key: &str) -> String {
        // Fallback: return key
        // Platform implementations will override this
        key.to_string()
    }
}

/// Hook to access I18nContext
pub fn use_i18n() -> Signal<I18nContext> {
    use_context()
}

/// Hook to get translated text
pub fn use_t(key: &str) -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.t(key)
}

/// Hook to change locale
pub fn use_set_locale() -> impl FnMut(Locale) {
    let mut i18n = use_i18n();
    move |locale: Locale| {
        let mut ctx = i18n.write();
        ctx.set_locale(locale);
    }
}

/// Get current locale as string
pub fn use_current_locale() -> String {
    let i18n = use_i18n();
    let i18n_val = i18n.read();
    i18n_val.current_locale_str().to_string()
}

/// Initialize i18n context (called by platform crates)
pub fn use_init_i18n() {
    use_context_provider(|| Signal::new(I18nContext::new()));
}
