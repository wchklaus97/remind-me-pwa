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
    let base_classes = "inline-flex items-center justify-center font-medium \
                        transition-colors focus-visible:outline-none \
                        focus-visible:ring-2 focus-visible:ring-offset-2 \
                        disabled:pointer-events-none disabled:opacity-50 \
                        rounded-lg";
    
    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700 \
                                  focus-visible:ring-blue-500",
        ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300 \
                                     focus-visible:ring-gray-500",
        ButtonVariant::Outline => "border-2 border-gray-300 bg-transparent \
                                  text-gray-700 hover:bg-gray-50 \
                                  focus-visible:ring-gray-500",
        ButtonVariant::Ghost => "bg-transparent text-gray-700 hover:bg-gray-100 \
                                focus-visible:ring-gray-500",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700 \
                                 focus-visible:ring-red-500",
    };
    
    let size_classes = match props.size {
        ButtonSize::Small => "h-8 px-3 text-sm",
        ButtonSize::Medium => "h-10 px-4 py-2 text-base",
        ButtonSize::Large => "h-12 px-6 text-lg",
    };
    
    rsx! {
        button {
            class: "{base_classes} {variant_classes} {size_classes} {props.class}",
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
                svg {
                    class: "mr-2 h-4 w-4 animate-spin",
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
            }
            
            {props.children}
        }
    }
}
