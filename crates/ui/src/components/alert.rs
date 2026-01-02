//! Alert Component
//! 
//! An alert component for displaying important messages.

use dioxus::prelude::*;

/// Alert variant styles
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AlertVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl Default for AlertVariant {
    fn default() -> Self {
        Self::Info
    }
}

/// Alert component props
#[derive(PartialEq, Clone, Props)]
pub struct AlertProps {
    /// Alert variant style
    #[props(default)]
    pub variant: AlertVariant,
    
    /// Alert title
    #[props(default)]
    pub title: String,
    
    /// Alert message (required if children not provided)
    #[props(default)]
    pub message: String,
    
    /// Show close button
    #[props(default = false)]
    pub show_close: bool,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Close event handler
    pub on_close: Option<EventHandler<()>>,
    
}

/// Alert component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Alert {
///         variant: AlertVariant::Warning,
///         title: "Warning",
///         message: "This action cannot be undone.",
///         show_close: true,
///         on_close: move |_| { /* handle close */ },
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let variant_classes = match props.variant {
        AlertVariant::Success => "bg-green-50 border-green-200 text-green-800",
        AlertVariant::Error => "bg-red-50 border-red-200 text-red-800",
        AlertVariant::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
        AlertVariant::Info => "bg-blue-50 border-blue-200 text-blue-800",
    };
    
    let icon = match props.variant {
        AlertVariant::Success => "✓",
        AlertVariant::Error => "✕",
        AlertVariant::Warning => "⚠",
        AlertVariant::Info => "ℹ",
    };
    
    rsx! {
        div {
            class: "rounded-lg border p-4 {variant_classes} {props.class}",
            div {
                class: "flex items-start gap-3",
                div {
                    class: "flex-shrink-0 text-lg",
                    "{icon}"
                }
                div {
                    class: "flex-1",
                    if !props.title.is_empty() {
                        h4 {
                            class: "font-semibold mb-1",
                            "{props.title}"
                        }
                    }
                    p {
                        class: "text-sm",
                        "{props.message}"
                    }
                }
                
                if props.show_close {
                    button {
                        class: "flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors",
                        onclick: move |_| {
                            if let Some(handler) = props.on_close.as_ref() {
                                handler.call(());
                            }
                        },
                        "×"
                    }
                }
            }
        }
    }
}

