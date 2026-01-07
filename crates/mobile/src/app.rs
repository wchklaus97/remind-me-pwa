//! Main App component for mobile platform
//! 
//! This module contains the root App component with routing and i18n
//! specific to iOS and Android platforms.

use dioxus::prelude::*;
use remind_me_shared::router::Route;
use remind_me_components::{ReminderApp, MediaCacheProvider};
use remind_me_shared::i18n::Locale;

/// Main App component for mobile platforms
#[component]
pub fn App() -> Element {
    // Initialize i18n with default locale
    let locale = use_signal(|| Locale::En);
    let mut current_route = use_signal(|| Route::App); // Mobile apps typically start with the main app

    // Load saved locale preference (if any)
    use_effect(move || {
        // TODO: Load from mobile storage preferences
        // For now, default to English
        locale.set(Locale::En);
    });

    rsx! {
        MediaCacheProvider {
            div {
                class: "mobile-app",
                match current_route() {
                    Route::App => {
                        ReminderApp {}
                    }
                    Route::Landing => {
                        // Mobile apps typically don't need a landing page
                        // Redirect to app
                        ReminderApp {}
                    }
                    Route::PrivacyPolicy => {
                        // TODO: Add privacy policy page for mobile
                        div {
                            "Privacy Policy"
                        }
                    }
                    Route::TermsOfUse => {
                        // TODO: Add terms of use page for mobile
                        div {
                            "Terms of Use"
                        }
                    }
                }
            }
        }
    }
}

