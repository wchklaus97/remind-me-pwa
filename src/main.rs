use remind_me_pwa::app::App;

#[cfg(all(target_arch = "wasm32", debug_assertions))]
fn set_panic_hook() {
    // Set a panic hook that logs to console instead of just aborting
    // This helps debug "unreachable" errors in WASM
    console_error_panic_hook::set_once();
}

fn main() {
    #[cfg(all(target_arch = "wasm32", debug_assertions))]
    set_panic_hook();
    
    dioxus::launch(App);
}

