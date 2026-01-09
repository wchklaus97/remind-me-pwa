//! Web app entry point
//! 
//! This is the main entry point for the web/WASM build of Remind Me PWA.

use remind_me_web::App;

// In WASM release builds, Rust panics show up as `RuntimeError: unreachable`
// unless we install a panic hook. Enable it for all wasm32 builds.
#[cfg(target_arch = "wasm32")]
fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    set_panic_hook();
    
    dioxus::launch(App);
}

