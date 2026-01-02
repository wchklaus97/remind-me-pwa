//! Empty State Component
//! 
//! A component for displaying empty states when there's no data.

use dioxus::prelude::*;
use super::button::{Button, ButtonVariant};

/// Empty state component props
#[derive(PartialEq, Clone, Props)]
pub struct EmptyStateProps {
    /// Icon or emoji
    #[props(default)]
    pub icon: String,
    
    /// Title
    #[props(default)]
    pub title: String,
    
    /// Description
    #[props(default)]
    pub description: String,
    
    /// Action button text (optional)
    #[props(default)]
    pub action_text: String,
    
    /// Action button click handler (optional)
    pub on_action: Option<EventHandler<()>>,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
}

/// Empty state component
/// 
/// # Example
/// ```rust
/// rsx! {
///     EmptyState {
///         icon: "📝",
///         title: "No reminders yet",
///         description: "Add your first reminder to get started!",
///         action_text: "Add Reminder",
///         on_action: move |_| { /* handle action */ },
///     }
/// }
/// ```
#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    let action_handler = props.on_action.clone();
    let action_text = props.action_text.clone();
    
    rsx! {
        div {
            class: "flex flex-col items-center justify-center py-12 px-4 text-center {props.class}",
            if !props.icon.is_empty() {
                div {
                    class: "text-6xl mb-4",
                    "{props.icon}"
                }
            }
            
            if !props.title.is_empty() {
                h3 {
                    class: "text-lg font-semibold text-gray-900 mb-2",
                    "{props.title}"
                }
            }
            
            if !props.description.is_empty() {
                p {
                    class: "text-sm text-gray-600 mb-4 max-w-sm",
                    "{props.description}"
                }
            }
            
            if !action_text.is_empty() {
                if let Some(handler) = action_handler {
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| handler.call(()),
                        {action_text}
                    }
                }
            }
        }
    }
}
