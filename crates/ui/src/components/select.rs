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
    let base_classes = "flex h-10 w-full rounded-md border bg-white px-3 py-2 \
                        text-sm ring-offset-white \
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
            select {
                class: "{base_classes} {border_classes} {props.class}",
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
                    class: "text-sm text-red-600",
                    "{props.error_message}"
                }
            }
        }
    }
}

