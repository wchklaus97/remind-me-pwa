//! Button Component
//! 
//! A versatile button component with multiple variants and sizes.

use dioxus::prelude::*;

/// Button variant styles
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Button sizes
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Button component props
#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    /// Button variant style
    #[props(default)]
    pub variant: ButtonVariant,
    
    /// Button size
    #[props(default)]
    pub size: ButtonSize,
    
    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,
    
    /// Whether the button is in loading state
    #[props(default)]
    pub loading: bool,
    
    /// Button type (button, submit, reset)
    #[props(default)]
    pub r#type: String,
    
    /// Click event handler
    pub onclick: Option<EventHandler<()>>,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// ARIA label for accessibility
    #[props(default)]
    pub aria_label: Option<String>,
    
    /// Button content
    pub children: Element,
}

/// Button component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Button {
///         variant: ButtonVariant::Primary,
///         onclick: move |_| {
///             log::info!("Clicked!");
///         },
///         "Click me"
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Use custom CSS classes instead of Tailwind
    let base_class = "btn";
    
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Ghost => "btn-ghost",
        ButtonVariant::Danger => "btn-danger",
    };
    
    let size_class = match props.size {
        ButtonSize::Small => "btn-small",
        ButtonSize::Medium => "",
        ButtonSize::Large => "btn-large",
    };
    
    let mut class_string = format!("{} {}", base_class, variant_class);
    if !size_class.is_empty() {
        class_string.push_str(&format!(" {}", size_class));
    }
    if !props.class.is_empty() {
        class_string.push_str(&format!(" {}", props.class));
    }
    
    rsx! {
        button {
            class: "{class_string}",
            r#type: "{props.r#type}",
            disabled: props.disabled || props.loading,
            aria_label: props.aria_label.as_deref(),
            onclick: move |_| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = props.onclick.as_ref() {
                        handler.call(());
                    }
                }
            },
            
            if props.loading {
                span { "Loading..." }
            }
            
            {props.children}
        }
    }
}
