use dioxus::prelude::*;
use crate::i18n::{use_i18n, use_t, Locale};
#[allow(unused_imports)]
use remind_me_shared::router::{update_url, get_initial_route};

#[component]
pub fn LanguageSwitcher(
    class: Option<String>,
) -> Element {
    let i18n = use_i18n();
    let mut open = use_signal(|| false);
    
    // Read current locale reactively from i18n context
    let current_locale = {
        let ctx = i18n.read();
        let locale_str = ctx.current_locale_str();
        Locale::from_str(locale_str)
    };

    let mut classes = String::from("language-switcher");
    if let Some(extra) = class {
        classes.push(' ');
        classes.push_str(&extra);
    }

    // Get current language label
    let current_label = match current_locale {
        Locale::En => use_t("language.en"),
        Locale::ZhHans => use_t("language.zh-Hans"),
        Locale::ZhHant => use_t("language.zh-Hant"),
    };

    // Separate handles for menu actions (avoids FnMut borrow issues across multiple onclick closures)
    let mut i18n_en = i18n;
    let mut i18n_zh_hans = i18n;
    let mut i18n_zh_hant = i18n;
    let mut open_en = open;
    let mut open_zh_hans = open;
    let mut open_zh_hant = open;

    rsx! {
        div { class: "language-switcher-wrap",
            button {
                class: "{classes}",
                aria_label: use_t("language.switch"),
                aria_expanded: if open() { "true" } else { "false" },
                aria_haspopup: "menu",
                onclick: move |_| open.set(!open()),
                onkeydown: move |evt| {
                    // Close on Escape for keyboard users
                    if evt.key() == Key::Escape {
                        open.set(false);
                    }
                },
                span { class: "language-switcher-icon", aria_hidden: "true", "🌐" }
                span { class: "language-switcher-label",
                    {current_label}
                }
            }

            if open() {
                div { class: "language-switcher-menu", role: "menu",
                    // English option
                    button {
                        class: if current_locale == Locale::En { "language-switcher-item active" } else { "language-switcher-item" },
                        role: "menuitemradio",
                        aria_checked: if current_locale == Locale::En { "true" } else { "false" },
                        onclick: move |_| {
                            // Update i18n context
                            let mut ctx = i18n_en.write();
                            ctx.set_locale(Locale::En);

                            // Update URL + <html lang> (WASM only)
                            #[cfg(target_arch = "wasm32")]
                            {
                                let (route, _) = get_initial_route();
                                update_url(&route, "en");

                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(html) = document.document_element() {
                                            let _ = html.set_attribute("lang", "en");
                                        }
                                    }
                                }
                            }

                            open_en.set(false);
                        },
                        {use_t("language.en")}
                    }
                    // Simplified Chinese option
                    button {
                        class: if current_locale == Locale::ZhHans { "language-switcher-item active" } else { "language-switcher-item" },
                        role: "menuitemradio",
                        aria_checked: if current_locale == Locale::ZhHans { "true" } else { "false" },
                        onclick: move |_| {
                            // Update i18n context
                            let mut ctx = i18n_zh_hans.write();
                            ctx.set_locale(Locale::ZhHans);

                            // Update URL + <html lang> (WASM only)
                            #[cfg(target_arch = "wasm32")]
                            {
                                let (route, _) = get_initial_route();
                                update_url(&route, "zh-Hans");

                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(html) = document.document_element() {
                                            let _ = html.set_attribute("lang", "zh-Hans");
                                        }
                                    }
                                }
                            }

                            open_zh_hans.set(false);
                        },
                        {use_t("language.zh-Hans")}
                    }
                    // Traditional Chinese option
                    button {
                        class: if current_locale == Locale::ZhHant { "language-switcher-item active" } else { "language-switcher-item" },
                        role: "menuitemradio",
                        aria_checked: if current_locale == Locale::ZhHant { "true" } else { "false" },
                        onclick: move |_| {
                            // Update i18n context
                            let mut ctx = i18n_zh_hant.write();
                            ctx.set_locale(Locale::ZhHant);

                            // Update URL + <html lang> (WASM only)
                            #[cfg(target_arch = "wasm32")]
                            {
                                let (route, _) = get_initial_route();
                                update_url(&route, "zh-Hant");

                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(html) = document.document_element() {
                                            let _ = html.set_attribute("lang", "zh-Hant");
                                        }
                                    }
                                }
                            }

                            open_zh_hant.set(false);
                        },
                        {use_t("language.zh-Hant")}
                    }
                }
            }
        }
    }
}
