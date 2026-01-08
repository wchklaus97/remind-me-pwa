//! Web app entry point
//! 
//! This is the main entry point for the web/WASM build of Remind Me PWA.

use remind_me_web::App;

#[cfg(all(target_arch = "wasm32", debug_assertions))]
fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn main() {
    #[cfg(all(target_arch = "wasm32", debug_assertions))]
    set_panic_hook();
    
    dioxus::launch(App);
}

