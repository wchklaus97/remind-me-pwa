// Web app entry point
use remind_me_web::*;

#[cfg(all(target_arch = "wasm32", debug_assertions))]
fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn main() {
    #[cfg(all(target_arch = "wasm32", debug_assertions))]
    set_panic_hook();
    
    // TODO: Launch web app
    // dioxus::launch(App);
}

