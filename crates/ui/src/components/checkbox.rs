//! Checkbox Component
//! 
//! A checkbox component with label support.

use dioxus::prelude::*;

/// Checkbox component props
#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked
    #[props(default)]
    pub checked: bool,
    
    /// Whether the checkbox is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Checkbox label text
    #[props(default)]
    pub label: String,
    
    /// ARIA label for accessibility
    #[props(default)]
    pub aria_label: String,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Change event handler
    pub onchange: Option<EventHandler<()>>,
}

/// Checkbox component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Checkbox {
///         checked: is_checked(),
///         label: "Accept terms",
///         onchange: move |_| is_checked.set(!is_checked()),
///     }
/// }
/// ```
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let checkbox_id = format!("checkbox_{}", uuid::Uuid::new_v4().to_string());
    
    rsx! {
        div {
            class: "flex items-center space-x-2 {props.class}",
            label {
                class: "flex items-center space-x-2 cursor-pointer",
                r#for: "{checkbox_id}",
                input {
                    id: "{checkbox_id}",
                    class: "h-4 w-4 rounded border-gray-300 text-blue-600 \
                           focus:ring-2 focus:ring-blue-500 \
                           disabled:cursor-not-allowed disabled:opacity-50",
                    r#type: "checkbox",
                    checked: props.checked,
                    disabled: props.disabled,
                    aria_label: if !props.aria_label.is_empty() {
                        Some(props.aria_label.as_str())
                    } else if !props.label.is_empty() {
                        Some(props.label.as_str())
                    } else {
                        None
                    },
                    onchange: move |_| {
                        if !props.disabled {
                            if let Some(handler) = props.onchange.as_ref() {
                                handler.call(());
                            }
                        }
                    },
                }
                
                if !props.label.is_empty() {
                    span {
                        class: "text-sm font-medium text-gray-700",
                        "{props.label}"
                    }
                }
            }
        }
    }
}
