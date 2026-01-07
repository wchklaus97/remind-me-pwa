//! Library crate for sharing code between:
//! - `src/main.rs` (WASM SPA)
//! - `src/bin/server.rs` (SSR server)

pub mod models;
pub mod router;
pub mod storage;
pub mod storage_platform;
pub mod utils;
pub mod i18n;
pub mod components;
pub mod app;
pub mod deployment;
pub mod services;


