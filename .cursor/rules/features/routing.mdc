---
alwaysApply: false
description: Routing patterns with locale-aware URL handling
---

# Routing Patterns

## 🎯 Overview

The project uses a **locale-aware routing system** that integrates with internationalization. Routes include locale prefixes (e.g., `/en/app`, `/zh-Hans/app`) and support both path-based and hash-based routing for GitHub Pages compatibility.

## 📝 Route Enum

```rust
#[derive(Clone, PartialEq)]
pub enum Route {
    Landing,        // Landing page (/)
    App,            // Reminder app (/app)
    PrivacyPolicy,  // Privacy Policy page (/privacy)
    TermsOfUse,     // Terms of Use page (/terms)
}
```

## 🌐 Locale-Aware URLs

### URL Format

- **With locale**: `/{locale}/{route}`
  - `/en/app` - English app
  - `/zh-Hans/app` - Simplified Chinese app
  - `/zh-Hant/app` - Traditional Chinese app
  - `/en/privacy` - English privacy policy
  - `/zh-Hans/terms` - Simplified Chinese terms

- **Without locale** (defaults to English):
  - `/app` - App (defaults to English)
  - `/` - Landing page (defaults to English)

### Locale Mapping

- `"zh"` → `"zh-Hans"` (defaults to Simplified Chinese)
- `"zh-Hans"` → `"zh-Hans"` (preserved)
- `"zh-Hant"` → `"zh-Hant"` (preserved)
- `"zh-CN"` → `"zh-Hans"` (mapped)
- `"zh-TW"` → `"zh-Hant"` (mapped)
- `"en"` → `"en"` (preserved)

## 🔧 Router Functions

### `get_initial_route()`

Gets the initial route and locale from the current URL.

```rust
let (route, locale) = get_initial_route();
// Returns: (Route::App, "zh-Hans")
```

**Behavior**:
- Checks URL path (path-based routing)
- Falls back to hash (hash-based routing for GitHub Pages)
- Extracts locale from URL
- Maps `"zh"` to `"zh-Hans"` for backward compatibility

### `update_url(route, locale)`

Updates the browser URL to match the route and locale.

```rust
update_url(&Route::App, "zh-Hans");
// Updates URL to: /zh-Hans/app
```

**Behavior**:
- Updates URL path with locale prefix
- Updates browser history
- Works with both path-based and hash-based routing

### `Route::from_path(path)`

Parses a path string into a Route and locale.

```rust
let (route, locale) = Route::from_path("/zh-Hans/app");
// Returns: (Route::App, "zh-Hans")
```

**Behavior**:
- Parses locale from first path segment
- Parses route from second path segment
- Handles base path (for GitHub Pages)
- Falls back to default locale if not found

### `Route::to_path(route, locale)`

Converts a Route and locale to a path string.

```rust
let path = Route::to_path(&Route::App, "zh-Hans");
// Returns: "/zh-Hans/app"
```

### `Route::to_hash(route, locale)`

Converts a Route and locale to a hash string (for hash-based routing).

```rust
let hash = Route::to_hash(&Route::App, "zh-Hans");
// Returns: "#/zh-Hans/app"
```

## 🔄 Routing Flow

### Initial Load

1. **App mounts** → Calls `get_initial_route()`
2. **Router parses URL** → Extracts route and locale
3. **App sets locale** → Updates i18n context
4. **App renders route** → Shows appropriate component

### Navigation

1. **User clicks link** → Calls `update_url(route, locale)`
2. **Router updates URL** → Changes browser URL
3. **App detects change** → Updates route state
4. **App re-renders** → Shows new component

### Language Switch

1. **User switches language** → Updates locale in i18n context
2. **Router updates URL** → Calls `update_url(current_route, new_locale)`
3. **URL updates** → `/en/app` → `/zh-Hans/app`
4. **App re-renders** → Shows same route, new locale

## 📋 Usage Patterns

### In App Component

```rust
use crate::router::{Route, get_initial_route, update_url};

#[component]
fn App() -> Element {
    let (route, locale) = get_initial_route();
    let mut current_route = use_signal(|| route);
    
    // Handle route changes
    use_effect(move || {
        // Listen for URL changes
        // Update current_route when URL changes
    });
    
    rsx! {
        match current_route() {
            Route::Landing => LandingPage {},
            Route::App => ReminderApp {},
            Route::PrivacyPolicy => PrivacyPolicyPage {},
            Route::TermsOfUse => TermsOfUsePage {},
        }
    }
}
```

### Navigation Handler

```rust
let on_navigate = move |route: Route| {
    let locale = i18n.read().current_locale_str();
    update_url(&route, &locale);
    current_route.set(route);
};
```

### Language Switch with Route Preservation

```rust
let on_language_switch = move |new_locale: Locale| {
    let current_route = current_route();
    let locale_str = locale_to_bcp47(&new_locale.to_string());
    update_url(&current_route, &locale_str);
    // Update i18n context...
};
```

## 🌍 GitHub Pages Compatibility

### Hash-Based Routing Fallback

For GitHub Pages (static hosting), the router supports hash-based routing:

- Path-based: `/zh-Hans/app`
- Hash-based: `/#/zh-Hans/app`

The router automatically detects and handles both formats.

### Base Path Handling

The router handles GitHub Pages base paths:

- Full URL: `https://user.github.io/repo/zh-Hans/app`
- Base path: `/repo`
- Route path: `/zh-Hans/app`

The router strips the base path before parsing.

## ✅ Best Practices

### DO:
- ✅ Always use `update_url()` for navigation
- ✅ Preserve locale when navigating between routes
- ✅ Use `get_initial_route()` on app mount
- ✅ Handle both path-based and hash-based routing
- ✅ Update HTML `lang` attribute when locale changes

### DON'T:
- ❌ Don't manually manipulate `window.location` (use router functions)
- ❌ Don't forget to update URL when locale changes
- ❌ Don't hardcode locale in routes (use current locale)
- ❌ Don't skip base path handling (for GitHub Pages)

## 📚 Related Files

- `src/router.rs` - Router implementation
- `src/app.rs` - Route handling in App component
- `src/i18n.rs` - Locale integration
- `src/deployment.rs` - Base path utilities
