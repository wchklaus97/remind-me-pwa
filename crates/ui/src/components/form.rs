//! Form Components
//! 
//! Form-related components for building forms.

use dioxus::prelude::*;

/// Form field component props
#[derive(PartialEq, Clone, Props)]
pub struct FormFieldProps {
    /// Field label
    #[props(default)]
    pub label: String,
    
    /// Whether the field is required
    #[props(default)]
    pub required: bool,
    
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    
    /// Help text
    #[props(default)]
    pub help: String,
    
    /// Field content (input, textarea, etc.)
    pub children: Element,
}

/// Form field component
/// 
/// Wraps form inputs with label, error message, and help text.
/// 
/// # Example
/// ```rust
/// rsx! {
///     FormField {
///         label: "Email",
///         required: true,
///         error: error_message(),
///         Input {
///             r#type: "email",
///             value: "{email()}",
///             oninput: move |e| email.set(e.value()),
///         }
///     }
/// }
/// ```
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let field_id = format!("field_{}", uuid::Uuid::new_v4().to_string());
    
    rsx! {
        div {
            class: "space-y-2",
            if !props.label.is_empty() {
                label {
                    r#for: "{field_id}",
                    class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed \
                           peer-disabled:opacity-70",
                    "{props.label}"
                    if props.required {
                        span {
                            class: "text-red-500 ml-1",
                            "*"
                        }
                    }
                }
            }
            
            div {
                id: "{field_id}",
                {props.children}
            }
            
            if let Some(error) = props.error.as_ref() {
                p {
                    class: "text-sm text-red-600",
                    "{error}"
                }
            }
            
            if !props.help.is_empty() && props.error.is_none() {
                p {
                    class: "text-sm text-gray-500",
                    "{props.help}"
                }
            }
        }
    }
}
