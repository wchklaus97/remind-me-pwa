//! Remind Me UI Component Library
//! 
//! 基于 Dioxus 的自定义 UI 组件库，提供可复用的组件和布局系统

pub mod components;
pub mod layout;
pub mod data;
pub mod theme;

// Re-export commonly used components
pub use components::*;
pub use layout::*;
pub use data::*;
pub use theme::*;

// Convenience module for easier imports
pub mod ui {
    pub use crate::components::*;
}

/// 主题配置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// 主题上下文（未来实现）
#[derive(Clone, PartialEq, Debug)]
pub struct ThemeContext {
    pub theme: Theme,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
        }
    }
}

