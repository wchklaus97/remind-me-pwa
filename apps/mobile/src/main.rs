//! Mobile app entry point
//! 
//! This is the main entry point for iOS and Android builds.

use remind_me_mobile::App;

fn main() {
    dioxus_mobile::launch(App);
}

