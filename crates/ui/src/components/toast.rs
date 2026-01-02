//! Toast Component
//! 
//! A toast notification component for displaying temporary messages.

use dioxus::prelude::*;

/// Toast variant styles
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl Default for ToastVariant {
    fn default() -> Self {
        Self::Info
    }
}

/// Toast position
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

impl Default for ToastPosition {
    fn default() -> Self {
        Self::TopRight
    }
}

/// Toast component props
#[derive(PartialEq, Clone, Props)]
pub struct ToastProps {
    /// Whether the toast is visible
    pub open: Signal<bool>,
    
    /// Toast variant style
    #[props(default)]
    pub variant: ToastVariant,
    
    /// Toast position
    #[props(default)]
    pub position: ToastPosition,
    
    /// Toast title
    #[props(default)]
    pub title: String,
    
    /// Toast message
    pub message: String,
    
    /// Auto-close duration in milliseconds (0 = no auto-close)
    #[props(default = 3000)]
    pub duration: u32,
    
    /// Show close button
    #[props(default = true)]
    pub show_close: bool,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Close event handler
    pub on_close: Option<EventHandler<()>>,
}

/// Toast component
/// 
/// # Example
/// ```rust
/// let mut show_toast = use_signal(|| true);
/// 
/// rsx! {
///     Toast {
///         open: show_toast,
///         variant: ToastVariant::Success,
///         title: "Success",
///         message: "Operation completed successfully",
///         on_close: move |_| show_toast.set(false),
///     }
/// }
/// ```
#[component]
pub fn Toast(props: ToastProps) -> Element {
    if !*props.open.read() {
        return rsx! { div {} };
    }
    
    // Auto-close effect
    use_effect(move || {
        if props.duration > 0 && *props.open.read() {
            let mut open_signal = props.open;
            let on_close_handler = props.on_close.clone();
            let duration = props.duration;
            
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(duration as u32).await;
                open_signal.set(false);
                if let Some(handler) = on_close_handler.as_ref() {
                    handler.call(());
                }
            });
        }
    });
    
    let variant_classes = match props.variant {
        ToastVariant::Success => "bg-green-50 border-green-200 text-green-800",
        ToastVariant::Error => "bg-red-50 border-red-200 text-red-800",
        ToastVariant::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
        ToastVariant::Info => "bg-blue-50 border-blue-200 text-blue-800",
    };
    
    let position_classes = match props.position {
        ToastPosition::TopLeft => "top-4 left-4",
        ToastPosition::TopRight => "top-4 right-4",
        ToastPosition::TopCenter => "top-4 left-1/2 -translate-x-1/2",
        ToastPosition::BottomLeft => "bottom-4 left-4",
        ToastPosition::BottomRight => "bottom-4 right-4",
        ToastPosition::BottomCenter => "bottom-4 left-1/2 -translate-x-1/2",
    };
    
    let mut open_signal = props.open;
    let on_close_handler = props.on_close.clone();
    
    let close_toast = move |_| {
        open_signal.set(false);
        if let Some(handler) = on_close_handler.as_ref() {
            handler.call(());
        }
    };
    
    rsx! {
        div {
            class: "fixed z-50 {position_classes} {props.class}",
            div {
                class: "rounded-lg border p-4 shadow-lg {variant_classes} min-w-[300px] max-w-[500px]",
                div {
                    class: "flex items-start justify-between",
                    div {
                        class: "flex-1",
                        if !props.title.is_empty() {
                            h4 {
                                class: "font-semibold mb-1",
                                "{props.title}"
                            }
                        }
                        p {
                            class: "text-sm",
                            "{props.message}"
                        }
                    }
                    
                    if props.show_close {
                        button {
                            class: "ml-4 text-gray-400 hover:text-gray-600 transition-colors",
                            onclick: close_toast,
                            "×"
                        }
                    }
                }
            }
        }
    }
}

