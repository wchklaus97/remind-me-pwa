use dioxus::prelude::*;

use remind_me_shared::router::Route;
use super::landing_layout::{LandingFooter, LandingNavbar};

#[component]
pub fn PublicPageTemplate(
    title: String,
    active_section: String,
    on_enter_app: EventHandler<()>,
    on_jump: EventHandler<&'static str>,
    on_navigate: EventHandler<Route>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "landing-page",
            div {
                class: "landing-container",
                div {
                    class: "landing-shell",
                    LandingNavbar {
                        active_section,
                        on_enter_app,
                        on_jump
                    }
                    main {
                        role: "main",
                        class: "public-page-main",
                        section {
                            class: "public-page-card",
                            h2 { class: "public-page-title", "{title}" }
                            div { class: "public-page-content", {children} }
                        }
                    }
                    LandingFooter {
                        active_section: "".to_string(),
                        on_jump,
                        on_navigate
                    }
                }
            }
        }
    }
}


