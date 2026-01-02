//! Card Component
//! 
//! A flexible card component with header, content, and footer sections.

use dioxus::prelude::*;

/// Card variant styles
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CardVariant {
    Default,
    Elevated,
    Outline,
}

impl Default for CardVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Card component props
#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    /// Card variant style
    #[props(default)]
    pub variant: CardVariant,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Card header content
    pub header: Option<Element>,
    
    /// Card footer content
    pub footer: Option<Element>,
    
    /// Card content
    pub children: Element,
}

/// Card component
/// 
/// # Example
/// ```rust
/// rsx! {
///     Card {
///         header: rsx! {
///             h3 { "Card Title" }
///         },
///         "Card content here"
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = "rounded-lg border bg-white text-gray-900 shadow-sm";
    
    let variant_classes = match props.variant {
        CardVariant::Default => "",
        CardVariant::Elevated => "shadow-lg",
        CardVariant::Outline => "border-2",
    };
    
    rsx! {
        div {
            class: "{base_classes} {variant_classes} {props.class}",
            
            if let Some(ref header) = props.header {
                div {
                    class: "flex flex-col space-y-1.5 p-6 pb-4",
                    {header.clone()}
                }
            }
            
            div {
                class: if props.header.is_some() { "p-6 pt-0" } else { "p-6" },
                {props.children}
            }
            
            if let Some(ref footer) = props.footer {
                div {
                    class: "flex items-center p-6 pt-0",
                    {footer.clone()}
                }
            }
        }
    }
}

/// Card header component
#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1.5 p-6 pb-4",
            {props.children}
        }
    }
}

/// Card title component
#[derive(PartialEq, Clone, Props)]
pub struct CardTitleProps {
    pub children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    rsx! {
        h3 {
            class: "text-2xl font-semibold leading-none tracking-tight",
            {props.children}
        }
    }
}

/// Card description component
#[derive(PartialEq, Clone, Props)]
pub struct CardDescriptionProps {
    pub children: Element,
}

#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    rsx! {
        p {
            class: "text-sm text-gray-600",
            {props.children}
        }
    }
}

/// Card content component
#[derive(PartialEq, Clone, Props)]
pub struct CardContentProps {
    pub children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    rsx! {
        div {
            class: "p-6 pt-0",
            {props.children}
        }
    }
}

/// Card footer component
#[derive(PartialEq, Clone, Props)]
pub struct CardFooterProps {
    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            class: "flex items-center p-6 pt-0",
            {props.children}
        }
    }
}
