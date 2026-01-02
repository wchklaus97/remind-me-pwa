//! Badge Component
//! 
//! A badge component for displaying status, labels, or counts.

use dioxus::prelude::*;

/// Badge variant styles
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for BadgeVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge component props
#[derive(PartialEq, Clone, Props)]
pub struct BadgeProps {
    /// Badge variant style
    #[props(default)]
    pub variant: BadgeVariant,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Badge content
    pub children: Element,
}

/// Badge component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Badge {
///         variant: BadgeVariant::Success,
///         "Active"
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_classes = "inline-flex items-center rounded-full px-2.5 py-0.5 \
                       text-xs font-semibold transition-colors";
    
    let variant_classes = match props.variant {
        BadgeVariant::Default => "bg-gray-100 text-gray-800",
        BadgeVariant::Primary => "bg-blue-100 text-blue-800",
        BadgeVariant::Success => "bg-green-100 text-green-800",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-800",
        BadgeVariant::Info => "bg-cyan-100 text-cyan-800",
    };
    
    rsx! {
        span {
            class: "{base_classes} {variant_classes} {props.class}",
            {props.children}
        }
    }
}
