//! Main App component for mobile platform
//! 
//! This module contains the root App component with routing and i18n
//! specific to iOS and Android platforms.

use dioxus::prelude::*;
use remind_me_shared::router::Route;
use remind_me_components::{ReminderApp, MediaCacheProvider, PrivacyPolicyPage, TermsOfUsePage};
use crate::i18n::{use_init_i18n, use_i18n};

/// Main App component for mobile platforms
#[component]
pub fn App() -> Element {
    // Initialize i18n context provider for the entire app
    use_init_i18n();

    // Get access to i18n context
    let _i18n = use_i18n();

    // Mobile apps typically start with the main app (not landing page)
    let mut current_route = use_signal(|| Route::App);

    rsx! {
        MediaCacheProvider {
            div {
                class: "mobile-app",
                match current_route() {
                    Route::App => rsx! {
                        ReminderApp {}
                    },
                    Route::Landing => rsx! {
                        // Mobile apps typically don't need a landing page
                        // Redirect to app
                        ReminderApp {}
                    },
                    Route::PrivacyPolicy => rsx! {
                        PrivacyPolicyPage {
                            on_enter_app: move |_| {
                                current_route.set(Route::App);
                            },
                            on_jump: move |_section: &'static str| {
                                // No-op for mobile - landing page sections not used
                            },
                            on_navigate: move |nav_route: Route| {
                                current_route.set(nav_route);
                            }
                        }
                    },
                    Route::TermsOfUse => rsx! {
                        TermsOfUsePage {
                            on_enter_app: move |_| {
                                current_route.set(Route::App);
                            },
                            on_jump: move |_section: &'static str| {
                                // No-op for mobile - landing page sections not used
                            },
                            on_navigate: move |nav_route: Route| {
                                current_route.set(nav_route);
                            }
                        }
                    }
                }
            }
        }
    }
}

