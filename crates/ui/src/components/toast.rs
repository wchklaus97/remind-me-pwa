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
        ToastVariant::Success => "toast toast-success",
        ToastVariant::Error => "toast toast-error",
        ToastVariant::Warning => "toast toast-warning",
        ToastVariant::Info => "toast toast-info",
    };
    
    let position_classes = match props.position {
        ToastPosition::TopLeft => "toast-top-left",
        ToastPosition::TopRight => "toast-top-right",
        ToastPosition::TopCenter => "toast-top-center",
        ToastPosition::BottomLeft => "toast-bottom-left",
        ToastPosition::BottomRight => "toast-bottom-right",
        ToastPosition::BottomCenter => "toast-bottom-center",
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
            class: "toast-container {position_classes} {props.class}",
            div {
                class: "toast-card {variant_classes}",
                div {
                    class: "toast-body",
                    div {
                        class: "toast-content",
                        if !props.title.is_empty() {
                            h4 {
                                class: "toast-title",
                                "{props.title}"
                            }
                        }
                        p {
                            class: "toast-message",
                            "{props.message}"
                        }
                    }
                    
                    if props.show_close {
                        button {
                            class: "toast-close",
                            onclick: close_toast,
                            "×"
                        }
                    }
                }
            }
        }
    }
}

