//! Modal Component
//! 
//! A modal dialog component with backdrop and close functionality.

use dioxus::prelude::*;

/// Modal sizes
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Fullscreen,
}

impl Default for ModalSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Modal component props
#[derive(PartialEq, Clone, Props)]
pub struct ModalProps {
    /// Whether the modal is open
    pub open: Signal<bool>,
    
    /// Modal size
    #[props(default)]
    pub size: ModalSize,
    
    /// Whether to close on backdrop click
    #[props(default = true)]
    pub close_on_backdrop: bool,
    
    /// Modal title
    #[props(default)]
    pub title: String,
    
    /// Show close button
    #[props(default = true)]
    pub show_close: bool,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Close event handler
    pub on_close: Option<EventHandler<()>>,
    
    /// Modal content
    pub children: Element,
}

/// Modal component
/// 
/// # Example
/// ```rust
/// let mut is_open = use_signal(|| true);
/// 
/// rsx! {
///     Modal {
///         open: is_open,
///         title: "Confirm Action",
///         on_close: move |_| is_open.set(false),
///         "Modal content here"
///     }
/// }
/// ```
#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !*props.open.read() {
        return rsx! { div {} };
    }
    
    let backdrop_classes = "fixed inset-0 z-50 bg-black/80 \
                            data-[state=open]:animate-in \
                            data-[state=closed]:animate-out \
                            data-[state=closed]:fade-out-0 \
                            data-[state=open]:fade-in-0";
    
    let size_classes = match props.size {
        ModalSize::Small => "sm:max-w-sm",
        ModalSize::Medium => "sm:max-w-md",
        ModalSize::Large => "sm:max-w-lg",
        ModalSize::Fullscreen => "sm:max-w-[95vw] sm:max-h-[95vh]",
    };
    
    let mut open_signal = props.open;
    let on_close_handler = props.on_close.clone();
    
    let close_modal = move |_| {
        open_signal.set(false);
        if let Some(handler) = on_close_handler.as_ref() {
            handler.call(());
        }
    };
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center",
            
            // Backdrop
            if props.close_on_backdrop {
                div {
                    class: backdrop_classes,
                    onclick: close_modal,
                }
            } else {
                div {
                    class: backdrop_classes,
                }
            }
            
            // Modal content
            div {
                class: "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg \
                       translate-x-[-50%] translate-y-[-50%] gap-4 border \
                       bg-white p-6 shadow-lg duration-200 \
                       sm:rounded-lg md:w-full {size_classes} {props.class}",
                
                // Header
                if !props.title.is_empty() || props.show_close {
                    div {
                        class: "flex flex-col space-y-1.5 text-center sm:text-left",
                        if !props.title.is_empty() {
                            h2 {
                                class: "text-lg font-semibold leading-none tracking-tight",
                                "{props.title}"
                            }
                        }
                        
                        if props.show_close {
                            button {
                                class: "absolute right-4 top-4 rounded-sm opacity-70 \
                                       ring-offset-white transition-opacity \
                                       hover:opacity-100 focus:outline-none \
                                       disabled:pointer-events-none",
                                onclick: close_modal,
                                span {
                                    class: "sr-only",
                                    "Close"
                                }
                                "×"
                            }
                        }
                    }
                }
                
                // Content
                div {
                    class: "py-4",
                    {props.children}
                }
            }
        }
    }
}
