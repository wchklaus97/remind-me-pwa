// Main entry point - minimal, delegates to modules
mod models;
mod router;
mod storage;
mod utils;
mod i18n;
mod components;
mod app;

use app::App;

#[cfg(target_arch = "wasm32")]
fn set_panic_hook() {
    // Set a panic hook that logs to console instead of just aborting
    // This helps debug "unreachable" errors in WASM
    console_error_panic_hook::set_once();
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    set_panic_hook();
    
    dioxus::launch(App);
}

