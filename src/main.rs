use remind_me_pwa::app::App;

// In WASM release builds, Rust panics show up as `RuntimeError: unreachable`
// unless we install a panic hook. Enable it for all wasm32 builds.
#[cfg(target_arch = "wasm32")]
fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// Web (WASM) launch
#[cfg(target_arch = "wasm32")]
fn main() {
    #[cfg(target_arch = "wasm32")]
    set_panic_hook();
    
    dioxus::launch(App);
}

// Mobile (iOS/Android) launch
#[cfg(any(target_os = "ios", target_os = "android"))]
fn main() {
    dioxus_mobile::launch(App);
}

