use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use remind_me_ui::{Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle};
use crate::i18n::{use_t, Locale, use_i18n};
use remind_me_shared::storage::{load_reminders, save_reminders};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AppSettings {
    language: String,
    notifications_enabled: bool,
}

fn load_settings() -> AppSettings {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(data)) = storage.get_item("app_settings") {
                    if let Ok(settings) = serde_json::from_str::<AppSettings>(&data) {
                        return settings;
                    }
                }
            }
        }
    }
    AppSettings {
        language: "en".to_string(),
        notifications_enabled: true,
    }
}

fn save_settings(settings: &AppSettings) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(json) = serde_json::to_string(settings) {
                    let _ = storage.set_item("app_settings", &json);
                }
            }
        }
    }
}

#[component]
pub fn SettingsView(
    on_close: EventHandler<()>,
) -> Element {
    let mut settings = use_signal(|| load_settings());
    let mut show_export_success = use_signal(|| false);
    let mut show_import_success = use_signal(|| false);
    let i18n = use_i18n();
    
    // Create locale change handlers (clone for each button)
    let mut i18n_en = i18n.clone();
    let mut i18n_zh_hans = i18n.clone();
    let mut i18n_zh_hant = i18n.clone();

    rsx! {
        Card {
            class: "settings-view",
            header: rsx! {
                CardHeader {
                    CardTitle { {use_t("settings.title")} }
                }
            },
            CardContent {
                // Language Selection
                div {
                    class: "settings-section",
                    h3 {
                        class: "settings-section-title",
                        "🌐 Language"
                    }
                    div {
                        class: "settings-options",
                        Button {
                            variant: if settings().language == "en" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            aria_label: Some("English".to_string()),
                            onclick: {
                                let mut i18n_clone = i18n_en.clone();
                                move |_| {
                                    let mut s = settings();
                                    s.language = "en".to_string();
                                    settings.set(s.clone());
                                    save_settings(&s);
                                    i18n_clone.write().set_locale(Locale::En);
                                }
                            },
                            "English"
                        }
                        Button {
                            variant: if settings().language == "zh-Hans" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            aria_label: Some("简体中文".to_string()),
                            onclick: {
                                let mut i18n_clone = i18n_zh_hans.clone();
                                move |_| {
                                    let mut s = settings();
                                    s.language = "zh-Hans".to_string();
                                    settings.set(s.clone());
                                    save_settings(&s);
                                    i18n_clone.write().set_locale(Locale::ZhHans);
                                }
                            },
                            "简体中文"
                        }
                        Button {
                            variant: if settings().language == "zh-Hant" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            aria_label: Some("繁體中文".to_string()),
                            onclick: {
                                let mut i18n_clone = i18n_zh_hant.clone();
                                move |_| {
                                    let mut s = settings();
                                    s.language = "zh-Hant".to_string();
                                    settings.set(s.clone());
                                    save_settings(&s);
                                    i18n_clone.write().set_locale(Locale::ZhHant);
                                }
                            },
                            "繁體中文"
                        }
                    }
                }

                // Notifications
                div {
                    class: "settings-section",
                    h3 {
                        class: "settings-section-title",
                        "🔔 Notifications"
                    }
                    div {
                        class: "settings-row",
                        label {
                            class: "settings-label",
                            "Enable browser notifications"
                        }
                        input {
                            r#type: "checkbox",
                            checked: settings().notifications_enabled,
                            onchange: move |e| {
                                let mut s = settings();
                                s.notifications_enabled = e.checked();
                                settings.set(s.clone());
                                save_settings(&s);
                            },
                        }
                    }
                }

                // Data Management
                div {
                    class: "settings-section",
                    h3 {
                        class: "settings-section-title",
                        "💾 Data Management"
                    }
                    div {
                        class: "settings-actions",
                        Button {
                            variant: ButtonVariant::Outline,
                            aria_label: Some("Export data".to_string()),
                            onclick: move |_| {
                                let reminders = load_reminders();
                                if let Ok(json) = serde_json::to_string(&reminders) {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                                                &wasm_bindgen::JsValue::from(json),
                                                web_sys::BlobPropertyBag::new().type_("application/json"),
                                            ).ok();
                                            if let Some(blob) = blob {
                                                let url = web_sys::Url::create_object_url_with_blob(&blob).ok();
                                                if let Some(url) = url {
                                                    if let Some(document) = window.document() {
                                                        if let Ok(Some(anchor)) = document.create_element("a") {
                                                            let anchor: &web_sys::HtmlAnchorElement = anchor.dyn_ref().unwrap();
                                                            anchor.set_href(&url);
                                                            anchor.set_download("reminders-export.json");
                                                            anchor.click();
                                                            web_sys::Url::revoke_object_url(&url);
                                                            show_export_success.set(true);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "📥 Export Data"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            aria_label: Some("Import data".to_string()),
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                {
                                    // Simplified: Just show a message that import needs to be done manually
                                    // Full file import would require more complex async handling
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.alert_with_message("Please paste JSON data in console or use browser DevTools to import data.");
                                    }
                                }
                            },
                            "📤 Import Data"
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            aria_label: Some("Clear all data".to_string()),
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        if let Some(storage) = window.local_storage().ok().flatten() {
                                            let _ = storage.remove_item("reminders_v2");
                                            let _ = storage.remove_item("reminders");
                                            let _ = storage.remove_item("tags_v1");
                                            // Reload page
                                            if let Some(location) = window.location() {
                                                let _ = location.reload();
                                            }
                                        }
                                    }
                                }
                            },
                            "🗑️ Clear All Data"
                        }
                    }
                    if show_export_success() {
                        div {
                            class: "settings-success",
                            "✅ Data exported successfully!"
                        }
                    }
                    if show_import_success() {
                        div {
                            class: "settings-success",
                            "✅ Data imported successfully!"
                        }
                    }
                }

                // Close Button
                div {
                    class: "settings-actions",
                    Button {
                        variant: ButtonVariant::Primary,
                        aria_label: Some("Close settings".to_string()),
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
