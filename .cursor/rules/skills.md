# Dioxus PWA Development Skills & Capabilities

**Last Updated**: 2025-01-15  
**Project Type**: Frontend PWA (Progressive Web App)  
**Framework**: Dioxus 0.6 (Rust Web Framework)  
**Target Platform**: Web (WASM)

---

## 🎯 Overview

This document defines the skills, capabilities, and architectural patterns for developing a Dioxus-based Progressive Web App. The project follows a **client-side only architecture** with:

1. **Dioxus Framework**: React-like UI framework for Rust
2. **WASM Target**: Compiles to WebAssembly for web deployment
3. **PWA Features**: Service worker, manifest, offline support
4. **Local Storage**: Browser localStorage for data persistence

---

## 🏗️ Core Architecture Components

### 1. Dioxus Components

**Purpose**: Build reactive UI components using Dioxus's component system.

**Key Capabilities**:
- ✅ **Component-based architecture**: Reusable, composable components
- ✅ **Reactive signals**: `use_signal()` for state management
- ✅ **Event handlers**: `onclick`, `oninput` for user interactions
- ✅ **Conditional rendering**: `if` expressions in `rsx!` macros
- ✅ **List rendering**: `for` loops in `rsx!` macros

**Usage Pattern**:
```rust
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            h1 { "Count: {count()}" }
            button {
                onclick: move |_| count.set(count() + 1),
                "Increment"
            }
        }
    }
}
```

**Component Props**:
```rust
#[component]
fn ReminderCard(
    reminder: Reminder,
    on_toggle: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "reminder-card",
            h3 { "{reminder.title}" }
            button {
                onclick: move |_| on_toggle.call(reminder.id.clone()),
                "Toggle"
            }
        }
    }
}
```

---

### 2. State Management

**Purpose**: Manage application state using Dioxus signals.

**Key Capabilities**:
- ✅ **Local component state**: `use_signal()` for component-local state
- ✅ **Shared state**: Signals can be passed between components
- ✅ **Derived state**: Computed values from signals
- ✅ **State persistence**: Save to localStorage

**Usage Pattern**:
```rust
// Local state
let mut reminders = use_signal(|| load_reminders());
let mut show_form = use_signal(|| false);

// Update state
reminders.set(new_reminders);
show_form.set(!show_form());

// Read state
let current_reminders = reminders();
let is_form_visible = show_form();
```

---

### 3. Local Storage Integration

**Purpose**: Persist data in browser's localStorage.

**Key Capabilities**:
- ✅ **Save data**: Serialize and store in localStorage
- ✅ **Load data**: Deserialize from localStorage on app start
- ✅ **Error handling**: Graceful fallback if storage unavailable
- ✅ **Automatic persistence**: Save on state changes
- ✅ **JSON serialization**: Use serde_json for data format

**Storage Pattern**:
```rust
use web_sys::window;
use serde_json;

// Load function - returns safe default on error
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
    Vec::new()  // Safe fallback
}

// Save function - silent failure on error
fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(reminders) {
                let _ = storage.set_item("reminders", &json);
            }
        }
    }
}
```

**Integration with State**:
```rust
// Initialize from storage
let mut reminders = use_signal(|| load_reminders());

// Save after mutations
on_add: move |reminder: Reminder| {
    let mut new_reminders = reminders();
    new_reminders.push(reminder);
    reminders.set(new_reminders.clone());
    save_reminders(&new_reminders);  // Persist immediately
}
```

**Storage Keys Convention**:
- Use plural, lowercase, snake_case: `reminders`, `settings`, `cache`
- Keep keys consistent across the app
- Document storage keys in code comments

**Error Handling Strategy**:
- **Load errors**: Return empty collection (Vec::new())
- **Save errors**: Silent failure (localStorage errors are non-critical)
- **Serialization errors**: Graceful fallback to default state

**See Also**: 
- `.cursor/rules/features/storage.mdc` for detailed storage patterns
- `.cursor/rules/features/storage-comparison.mdc` for localStorage vs IndexedDB comparison (learned from Flutter PWA patterns)

---

### 4. PWA Configuration

**Purpose**: Enable Progressive Web App features.

**Key Capabilities**:
- ✅ **Manifest**: App metadata, icons, display mode
- ✅ **Service Worker**: Offline support, caching
- ✅ **Install prompt**: Users can install as native app

**Configuration Files**:
- `assets/manifest.json`: PWA manifest
- `assets/sw.js`: Service worker script
- `Dioxus.toml`: Dioxus configuration with PWA settings

**Dioxus.toml Configuration**:
```toml
[web.pwa]
enabled = true
manifest = "assets/manifest.json"
service_worker = "assets/sw.js"
```

---

### 5. Date/Time Handling

**Purpose**: Handle dates and times in the application.

**Key Capabilities**:
- ✅ **Chrono library**: Rust's date/time library
- ✅ **UTC timestamps**: Store dates as RFC3339 strings
- ✅ **Local display**: Format dates for user's timezone
- ✅ **Date parsing**: Parse various date formats

**Usage Pattern**:
```rust
use chrono::{Utc, Local, DateTime, NaiveDateTime};

// Create timestamp
let now = Utc::now();
let timestamp = now.to_rfc3339();

// Parse date
let dt = DateTime::parse_from_rfc3339(&date_string)?;
let is_overdue = dt < Utc::now();

// Format for display
let formatted = dt.format("%Y-%m-%d %H:%M").to_string();
```

---

## 📋 Development Skills Checklist

### Required Skills for Dioxus PWA Development

#### 1. **Component Development**
- [ ] Create reusable components with `#[component]` attribute
- [ ] Use `use_signal()` for component state
- [ ] Pass props between components
- [ ] Handle events with `onclick`, `oninput`, etc.
- [ ] Use conditional rendering (`if` in `rsx!`)
- [ ] Render lists with `for` loops in `rsx!`

#### 2. **State Management**
- [ ] Manage local component state
- [ ] Share state between components via props
- [ ] Update state reactively
- [ ] Persist state to localStorage

#### 3. **Data Persistence**
- [ ] Save data to localStorage using `save_{entity}()` pattern
- [ ] Load data from localStorage on startup using `load_{entity}()` pattern
- [ ] Handle serialization/deserialization with serde_json
- [ ] Handle storage errors gracefully (return safe defaults)
- [ ] Save immediately after state mutations
- [ ] Use consistent storage key naming (plural, lowercase, snake_case)
- [ ] Follow storage patterns in `.cursor/rules/features/storage.mdc`

#### 4. **PWA Features**
- [ ] Configure manifest.json
- [ ] Implement service worker
- [ ] Cache static assets
- [ ] Handle offline scenarios
- [ ] Test installability

#### 5. **Styling**
- [ ] Use CSS classes in `rsx!` (`class: "..."`)
- [ ] Organize styles in external CSS file
- [ ] Use responsive design patterns
- [ ] Style components conditionally

#### 6. **Date/Time Handling**
- [ ] Use chrono for date operations
- [ ] Parse date strings
- [ ] Format dates for display
- [ ] Handle timezones correctly
- [ ] Compare dates (overdue detection)

#### 7. **Build & Deployment**
- [ ] Build for production (`dx build --release`)
- [ ] Configure base_path for GitHub Pages
- [ ] Optimize WASM binary size
- [ ] Test production builds locally
- [ ] Deploy to GitHub Pages

#### 8. **Error Handling**
- [ ] Handle Option/Result types
- [ ] Provide fallback values
- [ ] Log errors appropriately
- [ ] Handle storage failures

---

## 🔄 Common Patterns

### Pattern 1: Form Input with Validation

```rust
#[component]
fn AddReminderForm(on_add: EventHandler<Reminder>) -> Element {
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut due_date = use_signal(|| String::new());

    rsx! {
        div {
            class: "add-form",
            input {
                class: "form-input",
                r#type: "text",
                placeholder: "Title",
                value: "{title()}",
                oninput: move |e| title.set(e.value())
            }
            button {
                class: "btn btn-primary",
                disabled: title().is_empty(),
                onclick: move |_| {
                    if !title().is_empty() {
                        let reminder = Reminder {
                            id: format!("reminder_{}", Utc::now().timestamp_millis()),
                            title: title(),
                            description: description(),
                            due_date: due_date(),
                            completed: false,
                            created_at: Utc::now().to_rfc3339(),
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
```

### Pattern 2: List Rendering with Filtering

```rust
rsx! {
    div {
        class: "reminders-list",
        for reminder in reminders().iter().filter(|r| {
            match filter().as_str() {
                "active" => !r.completed,
                "completed" => r.completed,
                _ => true,
            }
        }) {
            ReminderCard {
                reminder: reminder.clone(),
                on_toggle: move |id| { /* ... */ },
                on_delete: move |id| { /* ... */ },
            }
        }
    }
}
```

### Pattern 3: Conditional Rendering

```rust
rsx! {
    if show_add_form() {
        AddReminderForm {
            on_add: move |reminder| { /* ... */ }
        }
    }
    
    if reminders().is_empty() {
        div {
            class: "empty-state",
            p { "No reminders yet. Add one to get started!" }
        }
    }
}
```

---

## 🎯 Best Practices

### DO:
- ✅ Use `use_signal()` for reactive state
- ✅ Keep components small and focused
- ✅ Use `EventHandler` for parent-child communication
- ✅ Persist important state to localStorage
- ✅ Use serde for serialization
- ✅ Handle Option/Result types properly
- ✅ Use conditional rendering for UI states
- ✅ Follow Rust naming conventions (snake_case)

### DON'T:
- ❌ Don't mutate state directly (use `.set()`)
- ❌ Don't create signals inside render loops
- ❌ Don't forget to handle storage errors
- ❌ Don't use `unwrap()` without error handling
- ❌ Don't create unnecessary clones
- ❌ Don't skip PWA configuration
- ❌ Don't hardcode paths (use base_path)

---

## 📚 Project Structure

```
remind-me-pwa/
├── src/
│   └── main.rs              # Main application code
├── assets/
│   ├── style.css            # Styles
│   ├── manifest.json        # PWA manifest
│   └── sw.js                # Service worker
├── .cursor/
│   └── rules/               # Cursor AI rules
├── .github/
│   └── workflows/
│       └── github-pages-deploy.yml  # CI/CD workflow (branch-based deployment)
├── Cargo.toml               # Rust dependencies
├── Dioxus.toml              # Dioxus configuration
└── README.md                # Project documentation
```

---

## 🔗 Related Documentation

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.6/)
- [Dioxus Web Guide](https://dioxuslabs.com/learn/0.6/getting_started/web)
- [PWA Best Practices](https://web.dev/progressive-web-apps/)
- [Rust WebAssembly](https://rustwasm.github.io/docs/book/)

---

**Last Updated**: 2025-01-15  
**Dioxus Version**: 0.6  
**Rust Edition**: 2021

