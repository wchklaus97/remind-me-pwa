//! Textarea Component
//! 
//! A textarea component with validation and error states.

use dioxus::prelude::*;

/// Textarea component props
#[derive(PartialEq, Clone, Props)]
pub struct TextareaProps {
    /// Textarea value
    pub value: String,
    
    /// Placeholder text
    #[props(default)]
    pub placeholder: String,
    
    /// Number of rows
    #[props(default = 4)]
    pub rows: u32,
    
    /// Whether the textarea is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Whether the textarea is required
    #[props(default)]
    pub required: bool,
    
    /// Whether the textarea has an error
    #[props(default)]
    pub error: bool,
    
    /// Error message (shown when error is true)
    #[props(default)]
    pub error_message: String,
    
    /// ARIA label for accessibility
    #[props(default)]
    pub aria_label: String,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Textarea change event handler
    pub oninput: Option<EventHandler<String>>,
}

/// Textarea component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Textarea {
///         placeholder: "Enter description",
///         value: "{value()}",
///         rows: 5,
///         oninput: move |e| value.set(e),
///     }
/// }
/// ```
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let base_classes = "flex min-h-[80px] w-full rounded-md border bg-white px-3 py-2 \
                        text-sm ring-offset-white \
                        placeholder:text-gray-500 \
                        focus-visible:outline-none focus-visible:ring-2 \
                        focus-visible:ring-offset-2 \
                        disabled:cursor-not-allowed disabled:opacity-50 \
                        resize-vertical";
    
    let border_classes = if props.error {
        "border-red-500 focus-visible:ring-red-500"
    } else {
        "border-gray-300 focus-visible:ring-blue-500"
    };
    
    rsx! {
        div {
            class: "space-y-2",
            textarea {
                class: "{base_classes} {border_classes} {props.class}",
                placeholder: "{props.placeholder}",
                rows: "{props.rows}",
                value: "{props.value}",
                disabled: props.disabled,
                required: props.required,
                aria_label: if !props.aria_label.is_empty() {
                    Some(props.aria_label.as_str())
                } else {
                    None
                },
                aria_invalid: props.error,
                oninput: move |e| {
                    if let Some(handler) = props.oninput.as_ref() {
                        handler.call(e.value());
                    }
                },
            }
            
            if props.error && !props.error_message.is_empty() {
                p {
                    class: "text-sm text-red-600",
                    "{props.error_message}"
                }
            }
        }
    }
}

