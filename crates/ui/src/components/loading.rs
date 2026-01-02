//! Loading/Spinner Component
//! 
//! A loading spinner component for indicating async operations.

use dioxus::prelude::*;

/// Loading size
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LoadingSize {
    Small,
    Medium,
    Large,
}

impl Default for LoadingSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Loading component props
#[derive(PartialEq, Clone, Props)]
pub struct LoadingProps {
    /// Loading size
    #[props(default)]
    pub size: LoadingSize,
    
    /// Loading text (optional)
    #[props(default)]
    pub text: String,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
}

/// Loading spinner component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Loading {
///         size: LoadingSize::Large,
///         text: "Loading...",
///     }
/// }
/// ```
#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let size_classes = match props.size {
        LoadingSize::Small => "h-4 w-4",
        LoadingSize::Medium => "h-8 w-8",
        LoadingSize::Large => "h-12 w-12",
    };
    
    rsx! {
        div {
            class: "flex flex-col items-center justify-center gap-2 {props.class}",
            svg {
                class: "{size_classes} animate-spin text-blue-600",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                circle {
                    class: "opacity-25",
                    cx: "12",
                    cy: "12",
                    r: "10",
                    stroke: "currentColor",
                    stroke_width: "4"
                }
                path {
                    class: "opacity-75",
                    fill: "currentColor",
                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                }
            }
            
            if !props.text.is_empty() {
                p {
                    class: "text-sm text-gray-600",
                    "{props.text}"
                }
            }
        }
    }
}

