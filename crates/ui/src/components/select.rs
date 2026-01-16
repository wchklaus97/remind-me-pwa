//! Select Component
//! 
//! A select dropdown component with options.

use dioxus::prelude::*;

/// Select option
#[derive(PartialEq, Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

/// Select component props
#[derive(PartialEq, Clone, Props)]
pub struct SelectProps {
    /// Selected value
    pub value: String,
    
    /// Available options
    pub options: Vec<SelectOption>,
    
    /// Placeholder text (shown when no option is selected)
    #[props(default)]
    pub placeholder: String,
    
    /// Whether the select is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Whether the select is required
    #[props(default)]
    pub required: bool,
    
    /// Whether the select has an error
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
    
    /// Select ID (for label association)
    #[props(default)]
    pub id: String,
    
    /// Select name (for form submission)
    #[props(default)]
    pub name: String,
    
    /// Select change event handler
    pub onchange: Option<EventHandler<String>>,
}

/// Select component
/// 
/// # Example
/// ```rust
/// let options = vec![
///     SelectOption { value: "option1".to_string(), label: "Option 1".to_string() },
///     SelectOption { value: "option2".to_string(), label: "Option 2".to_string() },
/// ];
/// 
/// rsx! {
///     Select {
///         value: selected_value(),
///         options: options,
///         placeholder: "Choose an option",
///         onchange: move |value| selected_value.set(value),
///     }
/// }
/// ```
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut class_string = "form-input".to_string();
    if props.error {
        class_string.push_str(" form-input-error");
    }
    if !props.class.is_empty() {
        class_string.push_str(&format!(" {}", props.class));
    }
    
    rsx! {
        div {
            select {
                id: if !props.id.is_empty() { Some(props.id.as_str()) } else { None },
                name: if !props.name.is_empty() { Some(props.name.as_str()) } else { None },
                class: "{class_string}",
                disabled: props.disabled,
                required: props.required,
                aria_label: if !props.aria_label.is_empty() {
                    Some(props.aria_label.as_str())
                } else {
                    None
                },
                aria_invalid: props.error,
                value: "{props.value}",
                onchange: move |e| {
                    if let Some(handler) = props.onchange.as_ref() {
                        handler.call(e.value());
                    }
                },
                
                if !props.placeholder.is_empty() && props.value.is_empty() {
                    option {
                        value: "",
                        disabled: true,
                        selected: true,
                        "{props.placeholder}"
                    }
                }
                
                for option in props.options.iter() {
                    option {
                        value: "{option.value}",
                        selected: option.value == props.value,
                        "{option.label}"
                    }
                }
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

