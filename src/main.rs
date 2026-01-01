use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::TimeZone;

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Reminder {
    id: String,
    title: String,
    description: String,
    due_date: String,
    completed: bool,
    created_at: String,
}

// Route enum for navigation
#[derive(Clone, PartialEq)]
enum Route {
    Landing,
    App,
}

#[component]
fn App() -> Element {
    // LCP Optimization: Default to Landing immediately (no blocking)
    // Route detection happens asynchronously in use_effect to avoid blocking initial render
    let mut current_route = use_signal(|| Route::Landing);

    // Lighthouse 100%: Set HTML lang attribute, meta tags, and optimize performance
    // Note: We use web_sys here ONLY for DOM manipulation that Dioxus doesn't provide APIs for
    // (removing Google Fonts links from <head> for performance optimization)
    // Priority: Use Dioxus APIs (use_signal, use_effect, etc.) whenever possible
    use_effect(move || {
        // LCP Optimization: Detect route asynchronously after initial render
        // This prevents blocking the initial render with web_sys calls
        let detected_route = get_initial_route();
        if detected_route != Route::Landing {
            current_route.set(detected_route);
        }
        
        // Use web_sys ONLY for removing render-blocking Google Fonts links
        // Dioxus doesn't provide APIs for manipulating <head> elements or removing external links
        // This is the minimal web-sys usage necessary for performance optimization
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;
            
                    if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // LCP Optimization: Critical operations only (lang attribute, font removal)
                    // Non-critical meta tags are deferred to reduce render delay
                    
                    // Set lang attribute on <html> (critical for accessibility)
                    if let Some(html) = document.document_element() {
                        let _ = html.set_attribute("lang", "en");
                    }
                    
                    // Get head element
                    if let Ok(Some(head)) = document.query_selector("head") {
                        if let Some(head_element) = head.dyn_ref::<web_sys::HtmlElement>() {
                        // Performance: Aggressively remove Google Fonts links (render-blocking)
                        // This must be done immediately and repeatedly to prevent font loading
                        let remove_all_font_links = || {
                            // Remove font stylesheet links
                            if let Ok(font_links) = document.query_selector_all("link[href*='fonts.googleapis.com']") {
                                for i in 0..font_links.length() {
                                    if let Some(link) = font_links.get(i) {
                                        if let Some(parent) = link.parent_node() {
                                            let _ = parent.remove_child(&link);
                                        }
                                    }
                                }
                            }
                            // Remove preconnect to fonts.googleapis.com
                            if let Ok(preconnects) = document.query_selector_all("link[rel='preconnect'][href*='fonts.googleapis.com']") {
                                for i in 0..preconnects.length() {
                                    if let Some(preconnect) = preconnects.get(i) {
                                        if let Some(parent) = preconnect.parent_node() {
                                            let _ = parent.remove_child(&preconnect);
                                        }
                                    }
                                }
                            }
                            // Remove dns-prefetch to fonts.googleapis.com
                            if let Ok(dns_prefetch) = document.query_selector_all("link[rel='dns-prefetch'][href*='fonts.googleapis.com']") {
                                for i in 0..dns_prefetch.length() {
                                    if let Some(dns) = dns_prefetch.get(i) {
                                        if let Some(parent) = dns.parent_node() {
                                            let _ = parent.remove_child(&dns);
                                        }
                                    }
                                }
                            }
                            // Also remove any links to fonts.gstatic.com
                            if let Ok(gstatic_links) = document.query_selector_all("link[href*='fonts.gstatic.com']") {
                                for i in 0..gstatic_links.length() {
                                    if let Some(link) = gstatic_links.get(i) {
                                        if let Some(parent) = link.parent_node() {
                                            let _ = parent.remove_child(&link);
                                        }
                                    }
                                }
                            }
                        };
                        
                        // Remove immediately (multiple times to catch any that appear)
                        remove_all_font_links();
                        
                        // LCP Optimization: Non-critical meta tags (these don't block rendering)
                        // Set meta description if not exists
                        let existing_meta = document.query_selector("meta[name='description']");
                        if let Ok(None) = existing_meta {
                            if let Ok(meta) = document.create_element("meta") {
                                let _ = meta.set_attribute("name", "description");
                                let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
                                let _ = head_element.append_child(&meta);
                            }
                        }
                        
                        // Add viewport meta tag if not exists (for SEO and mobile)
                        let existing_viewport = document.query_selector("meta[name='viewport']");
                        if let Ok(None) = existing_viewport {
                            if let Ok(viewport) = document.create_element("meta") {
                                let _ = viewport.set_attribute("name", "viewport");
                                let _ = viewport.set_attribute("content", "width=device-width, initial-scale=1.0");
                                let _ = head_element.append_child(&viewport);
                            }
                        }
                        
                        // Add charset meta tag if not exists
                        let existing_charset = document.query_selector("meta[charset]");
                        if let Ok(None) = existing_charset {
                            if let Ok(charset) = document.create_element("meta") {
                                let _ = charset.set_attribute("charset", "utf-8");
                                let _ = head_element.insert_before(&charset, head_element.first_child().as_ref());
                            }
                        }
                        }
                        
                        // Set title if not exists
                        if document.title().is_empty() {
                            document.set_title("Remind Me PWA - Your Personal Reminder Assistant");
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div {
            if current_route() == Route::Landing {
                LandingPage {
                    on_enter_app: move |_| {
                        current_route.set(Route::App);
                        update_url("/app");
                    }
                }
            } else {
                ReminderApp {}
            }
        }
    }
}

fn get_initial_route() -> Route {
    // Use web_sys only when necessary - Dioxus doesn't provide URL routing APIs
    if let Some(window) = web_sys::window() {
        let location = window.location();
        if let Ok(href) = location.href() {
            if href.contains("/app") || href.contains("#app") {
                return Route::App;
            }
        }
    }
    Route::Landing
}

fn update_url(path: &str) {
    // Use web_sys only when necessary - Dioxus doesn't provide URL manipulation APIs
    if let Some(window) = web_sys::window() {
        let location = window.location();
        let _ = location.set_hash(path);
    }
}

#[component]
fn LandingPage(on_enter_app: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "landing-page",
            header {
                role: "banner",
                class: "landing-header",
                h1 { "🔔 Remind Me" }
                p { class: "tagline", "Your Personal Reminder Assistant" }
            }
            main {
                role: "main",
                class: "landing-content",
                section {
                    class: "hero",
                    h2 { "Stay Organized, Never Forget" }
                    p {
                        "A beautiful and functional Progressive Web App to help you manage your reminders. "
                        "Works offline, installs on your device, and keeps your data private."
                    }
                    div {
                        class: "features",
                        div {
                            class: "feature",
                            "📱", br {}
                            "Installable PWA"
                        }
                        div {
                            class: "feature",
                            "💾", br {}
                            "Offline Support"
                        }
                        div {
                            class: "feature",
                            "🔒", br {}
                            "Private & Secure"
                        }
                    }
                }
                section {
                    class: "cta",
                    button {
                        class: "btn btn-primary btn-large",
                        aria_label: "Enter the reminder application",
                        onclick: move |_| on_enter_app.call(()),
                        "Get Started"
                    }
                }
            }
        }
    }
}

#[component]
fn ReminderApp() -> Element {
    let mut reminders = use_signal(|| load_reminders());
    let mut show_add_form = use_signal(|| false);
    let mut filter = use_signal(|| "all".to_string());

    rsx! {
        div {
            class: "app-container",
            header {
                role: "banner",
                class: "app-header",
                h1 { "🔔 Remind Me" }
                button {
                    class: "btn btn-primary",
                    aria_label: if show_add_form() { "Cancel adding reminder" } else { "Add new reminder" },
                    onclick: move |_| show_add_form.set(!show_add_form()),
                    if show_add_form() { "Cancel" } else { "+ New Reminder" }
                }
            }

            main {
                role: "main",
                if show_add_form() {
                    AddReminderForm {
                        on_add: move |reminder: Reminder| {
                            let mut new_reminders = reminders();
                            new_reminders.push(reminder);
                            reminders.set(new_reminders);
                            save_reminders(&reminders());
                            show_add_form.set(false);
                        }
                    }
                }

                nav {
                    role: "navigation",
                    aria_label: "Filter reminders",
                    class: "filter-tabs",
                    button {
                        class: if filter() == "all" { "tab active" } else { "tab" },
                        aria_label: "Show all reminders",
                        onclick: move |_| filter.set("all".to_string()),
                        "All"
                    }
                    button {
                        class: if filter() == "active" { "tab active" } else { "tab" },
                        aria_label: "Show active reminders",
                        onclick: move |_| filter.set("active".to_string()),
                        "Active"
                    }
                    button {
                        class: if filter() == "completed" { "tab active" } else { "tab" },
                        aria_label: "Show completed reminders",
                        onclick: move |_| filter.set("completed".to_string()),
                        "Completed"
                    }
                }

                section {
                    class: "reminders-list",
                    aria_label: "List of reminders",
                    for reminder in reminders().iter().filter(|r| {
                        match filter().as_str() {
                            "active" => !r.completed,
                            "completed" => r.completed,
                            _ => true,
                        }
                    }) {
                        ReminderCard {
                            reminder: reminder.clone(),
                            on_toggle: move |id: String| {
                                let mut updated = reminders();
                                if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
                                    r.completed = !r.completed;
                                }
                                reminders.set(updated);
                                save_reminders(&reminders());
                            },
                            on_delete: move |id: String| {
                                let mut updated = reminders();
                                updated.retain(|r| r.id != id);
                                reminders.set(updated);
                                save_reminders(&reminders());
                            }
                        }
                    }
                }

                if reminders().is_empty() {
                    div {
                        class: "empty-state",
                        role: "status",
                        p { "No reminders yet. Add one to get started!" }
                    }
                }
            }
        }
    }
}

#[component]
fn AddReminderForm(on_add: EventHandler<Reminder>) -> Element {
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut due_date = use_signal(|| String::new());

    rsx! {
        div {
            class: "add-form",
            h2 { "New Reminder" }
            label {
                r#for: "reminder-title",
                "Title"
            }
            input {
                id: "reminder-title",
                class: "form-input",
                r#type: "text",
                placeholder: "Enter reminder title",
                aria_label: "Reminder title",
                required: true,
                value: "{title()}",
                oninput: move |e| title.set(e.value())
            }
            label {
                r#for: "reminder-description",
                "Description (optional)"
            }
            textarea {
                id: "reminder-description",
                class: "form-input",
                placeholder: "Enter reminder description",
                aria_label: "Reminder description",
                value: "{description()}",
                oninput: move |e| description.set(e.value())
            }
            label {
                r#for: "reminder-due-date",
                "Due Date (optional)"
            }
            input {
                id: "reminder-due-date",
                class: "form-input",
                r#type: "datetime-local",
                aria_label: "Reminder due date",
                value: "{due_date()}",
                oninput: move |e| due_date.set(e.value())
            }
            button {
                class: "btn btn-primary",
                aria_label: "Add reminder",
                disabled: title().is_empty(),
                onclick: move |_| {
                    if !title().is_empty() {
                        let reminder = Reminder {
                            id: format!("reminder_{}", chrono::Utc::now().timestamp_millis()),
                            title: title(),
                            description: description(),
                            due_date: due_date(),
                            completed: false,
                            created_at: chrono::Utc::now().to_rfc3339(),
                        };
                        on_add.call(reminder);
                        title.set(String::new());
                        description.set(String::new());
                        due_date.set(String::new());
                    }
                },
                "Add Reminder"
            }
        }
    }
}

#[component]
fn ReminderCard(reminder: Reminder, on_toggle: EventHandler<String>, on_delete: EventHandler<String>) -> Element {
    let is_overdue = if !reminder.completed && !reminder.due_date.is_empty() {
        if let Ok(due) = chrono::DateTime::parse_from_rfc3339(&reminder.due_date) {
            due < chrono::Utc::now()
        } else if let Ok(due) = chrono::NaiveDateTime::parse_from_str(&reminder.due_date, "%Y-%m-%dT%H:%M") {
            if let Some(local_dt) = chrono::Local.from_local_datetime(&due).single() {
                local_dt < chrono::Local::now()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    let card_class = if reminder.completed {
        "reminder-card completed"
    } else if is_overdue {
        "reminder-card overdue"
    } else {
        "reminder-card"
    };

    let reminder_id_toggle = reminder.id.clone();
    let reminder_id_delete = reminder.id.clone();
    let toggle_label = if reminder.completed {
        "Mark as incomplete"
    } else {
        "Mark as complete"
    };

    rsx! {
        article {
            class: "{card_class}",
            div {
                class: "reminder-header",
                label {
                    class: "checkbox-label",
                    input {
                        class: "checkbox",
                        r#type: "checkbox",
                        aria_label: "{toggle_label}",
                        checked: reminder.completed,
                        onclick: move |_| {
                            on_toggle.call(reminder_id_toggle.clone());
                        }
                    }
                }
                div {
                    class: "reminder-content",
                    h3 { class: if reminder.completed { "strikethrough" } else { "" }, "{reminder.title}" }
                    if !reminder.description.is_empty() {
                        p { "{reminder.description}" }
                    }
                    if !reminder.due_date.is_empty() {
                        div {
                            class: "due-date",
                            if is_overdue {
                                span { class: "badge overdue", "⚠️ Overdue" }
                            }
                            span { "Due: {format_date(&reminder.due_date)}" }
                        }
                    }
                }
                button {
                    class: "btn btn-danger btn-small",
                    aria_label: "Delete reminder",
                    onclick: move |_| {
                        on_delete.call(reminder_id_delete.clone());
                    },
                    "🗑️"
                }
            }
        }
    }
}

fn format_date(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M") {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else {
        date_str.to_string()
    }
}

fn load_reminders() -> Vec<Reminder> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item("reminders") {
                if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
                    return reminders;
                }
            }
        }
    }
    Vec::new()
}

fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(reminders) {
                let _ = storage.set_item("reminders", &json);
            }
        }
    }
}
