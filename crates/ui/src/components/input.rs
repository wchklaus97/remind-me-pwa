//! Input Component
//! 
//! A flexible input component with validation and error states.

use dioxus::prelude::*;

/// Input component props
#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    /// Input type (text, email, password, number, etc.)
    #[props(default)]
    pub r#type: String,
    
    /// Input value
    pub value: String,
    
    /// Placeholder text
    #[props(default)]
    pub placeholder: String,
    
    /// Whether the input is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Whether the input is required
    #[props(default)]
    pub required: bool,
    
    /// Whether the input has an error
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
    
    /// Input ID (for label association)
    #[props(default)]
    pub id: String,
    
    /// Input name (for form submission)
    #[props(default)]
    pub name: String,
    
    /// Input change event handler
    pub oninput: Option<EventHandler<String>>,
}

/// Input component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Input {
///         r#type: "text",
///         placeholder: "Enter text",
///         value: "{value()}",
///         oninput: move |e| value.set(e.value()),
///     }
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    let base_classes = "flex h-10 w-full rounded-md border bg-white px-3 py-2 \
                        text-sm ring-offset-white \
                        file:border-0 file:bg-transparent file:text-sm file:font-medium \
                        placeholder:text-gray-500 \
                        focus-visible:outline-none focus-visible:ring-2 \
                        focus-visible:ring-offset-2 \
                        disabled:cursor-not-allowed disabled:opacity-50";
    
    let border_classes = if props.error {
        "border-red-500 focus-visible:ring-red-500"
    } else {
        "border-gray-300 focus-visible:ring-blue-500"
    };
    
    rsx! {
        div {
            class: "space-y-2",
            input {
                id: if !props.id.is_empty() { Some(props.id.as_str()) } else { None },
                name: if !props.name.is_empty() { Some(props.name.as_str()) } else { None },
                class: "{base_classes} {border_classes} {props.class}",
                r#type: "{props.r#type}",
                placeholder: "{props.placeholder}",
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
