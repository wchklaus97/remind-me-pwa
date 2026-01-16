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
    let mut class_string = "form-input".to_string();
    if props.error {
        class_string.push_str(" form-input-error");
    }
    if !props.class.is_empty() {
        class_string.push_str(&format!(" {}", props.class));
    }
    
    rsx! {
        div {
            input {
                id: if !props.id.is_empty() { Some(props.id.as_str()) } else { None },
                name: if !props.name.is_empty() { Some(props.name.as_str()) } else { None },
                class: "{class_string}",
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
                    class: "form-error-message",
                    "{props.error_message}"
                }
            }
        }
    }
}
