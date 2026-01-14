# Dioxus PWA Development Skills & Capabilities

**Last Updated**: 2026-01-14  
**Project Type**: Frontend PWA (Progressive Web App)  
**Framework**: Dioxus 0.7 (Rust Web Framework)  
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

### 6. Internationalization (i18n)

**Purpose**: Support multiple languages in the application.

**Key Capabilities**:
- ✅ **3 Languages**: English, Simplified Chinese (简体中文), Traditional Chinese (繁體中文)
- ✅ **Locale Enum**: `Locale::En`, `Locale::ZhHans`, `Locale::ZhHant`
- ✅ **Translation Files**: JSON files in `assets/i18n/`
- ✅ **Translation Hook**: `use_t("key.path")` for getting translated strings
- ✅ **Language Switcher**: `LanguageSwitcher` component for switching languages
- ✅ **URL Integration**: Locale in URL path (e.g., `/en/app`, `/zh-Hans/app`)
- ✅ **HTML Lang Attribute**: Automatically set based on locale

**Translation File Structure**:
```json
{
  "app": {
    "title": "Remind Me PWA",
    "tagline": "Reminder Assistant"
  },
  "landing": {
    "nav": {
      "features": "Features",
      "pricing": "Pricing"
    }
  },
  "language": {
    "switch": "Switch language",
    "en": "English",
    "zh-Hans": "简体中文",
    "zh-Hant": "繁體中文"
  }
}
```

**Usage Pattern**:
```rust
use crate::i18n::use_t;

#[component]
fn MyComponent() -> Element {
    rsx! {
        h1 { {use_t("app.title")} }
        p { {use_t("app.tagline")} }
        button { {use_t("landing.nav.features")} }
    }
}
```

**Language Switcher Component**:
```rust
use crate::components::LanguageSwitcher;

#[component]
fn App() -> Element {
    rsx! {
        LanguageSwitcher { class: Some("nav-lang-switcher".to_string()) }
    }
}
```

**Locale System**:
- Default `"zh"` maps to `zh-Hans` (Simplified Chinese)
- BCP 47 codes: `en`, `zh-Hans`, `zh-Hant`
- Locale persisted in localStorage
- URL format: `/{locale}/app`, `/{locale}/privacy`, etc.

**Translation File Locations**:
- `assets/i18n/en.json` - English
- `assets/i18n/zh-Hans.json` - Simplified Chinese (简体中文)
- `assets/i18n/zh-Hant.json` - Traditional Chinese (繁體中文)

---

### 7. Media Caching & Loading

**Purpose**: Efficiently cache and load images and videos with loading states and error handling.

**Key Capabilities**:
- ✅ **MediaCacheProvider**: Context provider for shared cache manager
- ✅ **MediaCacheManager**: Deduplicates in-flight downloads across components
- ✅ **ManagedCachedImage**: Image component with cache integration and shimmer loading
- ✅ **ManagedCachedVideo**: Video component with cache integration and shimmer loading
- ✅ **CachedImage**: Basic image component with loading states
- ✅ **CachedVideo**: Basic video component with loading states
- ✅ **Browser Cache Storage**: Uses Cache Storage API for persistent caching
- ✅ **Shimmer Loading**: Shows skeleton placeholder while loading
- ✅ **Error Handling**: Displays fallback text on load failure

**MediaCacheProvider Setup**:
```rust
use crate::components::MediaCacheProvider;

#[component]
fn App() -> Element {
    rsx! {
        MediaCacheProvider {
            // All child components can use ManagedCachedImage/Video
            LandingPage {}
        }
    }
}
```

**ManagedCachedImage Usage**:
```rust
use crate::components::ManagedCachedImage;
use dioxus::asset::asset;

#[component]
fn MyComponent() -> Element {
    static MY_IMAGE: Asset = asset!("/assets/images/logo.png");
    
    rsx! {
        ManagedCachedImage {
            src: MY_IMAGE,
            alt: "Logo".to_string(),
            class: Some("logo".to_string()),
            width: Some("48".to_string()),
            height: Some("48".to_string()),
        }
    }
}
```

**ManagedCachedVideo Usage**:
```rust
use crate::components::ManagedCachedVideo;
use dioxus::asset::asset;

#[component]
fn MyComponent() -> Element {
    static VIDEO: Asset = asset!("/assets/videos/animation.mp4");
    static POSTER: Asset = asset!("/assets/videos/poster.webp");
    
    rsx! {
        ManagedCachedVideo {
            src: VIDEO,
            poster: POSTER,
            aria_label: Some("Animation".to_string()),
            title: Some("Loading animation".to_string()),
            fallback_text: Some("Animation failed to load".to_string()),
            class: Some("hero-animation".to_string()),
            width: "120".to_string(),
            height: "120".to_string(),
        }
    }
}
```

**Media Cache Manager**:
- **Deduplication**: Multiple components requesting the same URL only trigger one download
- **State Management**: Tracks `Loading`, `Ready`, `Error` states per URL
- **Cache Storage**: Uses browser Cache Storage API (`media-cache-v1`)
- **Automatic**: Components automatically call `manager.ensure()` on mount

**Service Layer** (`src/services/media_cache.rs`):
- `ensure_cached()`: Ensures a URL is cached (checks cache, fetches if needed)
- `prefetch_assets()`: Prefetches multiple assets (fire-and-forget)

**CSS Classes**:
- `.cached-media-wrap`: Wrapper for media element
- `.media-skeleton`: Shimmer loading placeholder
- `.media-fallback`: Error fallback text display
- `.cached-media`: The actual media element

---

### 8. Page Templates

**Purpose**: Create reusable page layouts with consistent navbar and footer.

**Key Capabilities**:
- ✅ **PublicPageTemplate**: Wraps content with navbar and footer
- ✅ **Consistent Layout**: Same structure across public pages
- ✅ **Navigation Integration**: Handles navigation events
- ✅ **Section Highlighting**: Supports active section highlighting

**PublicPageTemplate Usage**:
```rust
use crate::components::page_template::PublicPageTemplate;
use crate::router::Route;

#[component]
fn PrivacyPolicyPage() -> Element {
    let mut active_section = use_signal(|| "".to_string());
    
    rsx! {
        PublicPageTemplate {
            title: "Privacy Policy".to_string(),
            active_section: active_section(),
            on_enter_app: move |_| {
                // Navigate to app
            },
            on_jump: move |section: &'static str| {
                // Handle section navigation
            },
            on_navigate: move |route: Route| {
                // Handle route navigation
            },
            // Page content here
            div {
                h3 { "Privacy Policy Content" }
                p { "..." }
            }
        }
    }
}
```

**Template Structure**:
- Wraps content in `landing-page` → `landing-container` → `landing-shell`
- Includes `LandingNavbar` at top
- Main content in `<main>` with `public-page-main` class
- Content wrapped in `public-page-card` with title
- Includes `LandingFooter` at bottom

**CSS Classes**:
- `.public-page-main`: Main content area
- `.public-page-card`: Card container for content
- `.public-page-title`: Page title styling
- `.public-page-content`: Content area

**When to Use**:
- ✅ Privacy Policy pages
- ✅ Terms of Use pages
- ✅ Any public page that needs navbar + footer
- ✅ Pages that should match landing page layout

**When NOT to Use**:
- ❌ Landing page (uses `LandingPage` component)
- ❌ App pages (uses `ReminderApp` component)
- ❌ Pages that need different layout

---

### 9. Routing & Navigation

**Purpose**: Handle client-side routing with locale-aware URLs.

**Key Capabilities**:
- ✅ **Route Enum**: `Route::Landing`, `Route::App`, `Route::PrivacyPolicy`, `Route::TermsOfUse`
- ✅ **Locale-Aware URLs**: Routes include locale prefix (e.g., `/en/app`, `/zh-Hans/app`)
- ✅ **URL Parsing**: `get_initial_route()` extracts route and locale from URL
- ✅ **URL Updates**: `update_url()` updates browser URL and history
- ✅ **GitHub Pages Support**: Handles both path-based and hash-based routing
- ✅ **Base Path Handling**: Strips base path for GitHub Pages deployment

**Route Enum**:
```rust
#[derive(Clone, PartialEq)]
pub enum Route {
    Landing,        // Landing page (/)
    App,            // Reminder app (/app)
    PrivacyPolicy,  // Privacy Policy (/privacy)
    TermsOfUse,     // Terms of Use (/terms)
}
```

**Getting Initial Route**:
```rust
use crate::router::get_initial_route;

let (route, locale) = get_initial_route();
// Returns: (Route::App, "zh-Hans")
```

**Updating URL**:
```rust
use crate::router::{Route, update_url};

update_url(&Route::App, "zh-Hans");
// Updates URL to: /zh-Hans/app
```

**URL Format**:
- With locale: `/{locale}/{route}` (e.g., `/en/app`, `/zh-Hans/app`)
- Without locale: `/{route}` (defaults to English, e.g., `/app`)

**Locale Mapping**:
- `"zh"` → `"zh-Hans"` (defaults to Simplified Chinese)
- `"zh-Hans"` → `"zh-Hans"` (preserved)
- `"zh-Hant"` → `"zh-Hant"` (preserved)

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
- [ ] Organize styles in modular CSS files (`assets/css/`)
- [ ] Use responsive design patterns
- [ ] Style components conditionally
- [ ] Follow CSS file organization (base, components, app, landing, layout, utilities, responsive)

#### 6. **Internationalization (i18n)**
- [ ] Use `use_t("key.path")` hook for translations
- [ ] Add translation keys to all 3 language files (`en.json`, `zh-Hans.json`, `zh-Hant.json`)
- [ ] Use `LanguageSwitcher` component for language switching
- [ ] Ensure all user-facing text uses translation keys
- [ ] Test language switching functionality
- [ ] Verify URL updates with locale changes
- [ ] Check HTML `lang` attribute is set correctly

#### 7. **Media Caching & Loading**
- [ ] Wrap app with `MediaCacheProvider`
- [ ] Use `ManagedCachedImage` for images with caching
- [ ] Use `ManagedCachedVideo` for videos with caching
- [ ] Provide `aria_label`, `title`, and `fallback_text` for accessibility
- [ ] Understand cache deduplication across components
- [ ] Test loading states (shimmer skeleton)
- [ ] Test error states (fallback text)

#### 8. **Page Templates**
- [ ] Use `PublicPageTemplate` for public pages
- [ ] Provide required props (title, active_section, handlers)
- [ ] Understand template structure (navbar + content + footer)
- [ ] Use appropriate CSS classes for styling

#### 9. **Routing & Navigation**
- [ ] Use `get_initial_route()` to get route and locale from URL
- [ ] Use `update_url()` for navigation
- [ ] Understand locale-aware URL format (`/{locale}/{route}`)
- [ ] Handle route changes in App component
- [ ] Preserve locale when navigating between routes
- [ ] Test with both path-based and hash-based routing

#### 10. **Date/Time Handling**
- [ ] Use chrono for date operations
- [ ] Parse date strings
- [ ] Format dates for display
- [ ] Handle timezones correctly
- [ ] Compare dates (overdue detection)

#### 11. **Build & Deployment**
- [ ] Build for production (`dx build --release`)
- [ ] Configure base_path for GitHub Pages
- [ ] Optimize WASM binary size
- [ ] Test production builds locally
- [ ] Deploy to GitHub Pages

#### 12. **Error Handling**
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

**Last Updated**: 2026-01-14  
**Dioxus Version**: 0.6  
**Rust Edition**: 2021

