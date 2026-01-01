---
alwaysApply: true
description: LocalStorage patterns and data persistence for Dioxus PWA
---
# LocalStorage Data Persistence

## 🎯 Storage Strategy

**Storage Type**: Browser LocalStorage  
**Serialization**: JSON via serde  
**Data Format**: Structured Rust types (Vec<T>, structs)

## 📋 Storage Pattern

### Standard Storage Functions

**Pattern**: `load_{entity}()` and `save_{entity}()`

```rust
use web_sys::window;
use serde_json;

// Load function
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
    Vec::new()  // Return empty vec on error
}

// Save function
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

## 🔑 Key Storage Keys

**Naming Convention**: Use plural, lowercase, snake_case

- `reminders` - List of reminder items
- `settings` - User preferences (if needed)
- `cache` - Cached data (if needed)

## 📝 Storage Integration Pattern

### 1. Initialize State from Storage

```rust
#[component]
fn App() -> Element {
    // Load from storage on component mount
    let mut reminders = use_signal(|| load_reminders());
    
    rsx! { /* ... */ }
}
```

### 2. Save on State Changes

```rust
// After adding
on_add: move |reminder: Reminder| {
    let mut new_reminders = reminders();
    new_reminders.push(reminder);
    reminders.set(new_reminders.clone());
    save_reminders(&new_reminders);  // Save immediately
}

// After updating
on_toggle: move |id: String| {
    let mut updated = reminders();
    if let Some(r) = updated.iter_mut().find(|r| r.id == id) {
        r.completed = !r.completed;
    }
    reminders.set(updated.clone());
    save_reminders(&updated);  // Save immediately
}

// After deleting
on_delete: move |id: String| {
    let mut updated = reminders();
    updated.retain(|r| r.id != id);
    reminders.set(updated.clone());
    save_reminders(&updated);  // Save immediately
}
```

## 🛡️ Error Handling

### Graceful Degradation

**Pattern**: Always return a safe default on error

```rust
fn load_reminders() -> Vec<Reminder> {
    // Try to load
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item("reminders") {
                if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
                    return reminders;
                }
            }
        }
    }
    // Fallback: return empty collection
    Vec::new()
}
```

### Save Error Handling

**Pattern**: Silent failure (localStorage errors are non-critical)

```rust
fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(reminders) {
                let _ = storage.set_item("reminders", &json);
                // Ignore set_item errors (quota exceeded, etc.)
            }
        }
    }
}
```

## 🔄 Data Migration Pattern

### Version Management (Future)

If storage format changes, use versioned keys:

```rust
const STORAGE_VERSION: &str = "v1";
const STORAGE_KEY: &str = "reminders_v1";

fn load_reminders() -> Vec<Reminder> {
    // Try current version first
    if let Ok(Some(data)) = get_storage_item(STORAGE_KEY) {
        if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
            return reminders;
        }
    }
    // Try old version and migrate
    if let Ok(Some(data)) = get_storage_item("reminders") {
        if let Ok(old_reminders) = serde_json::from_str::<Vec<OldReminder>>(&data) {
            let migrated = migrate_reminders(old_reminders);
            save_reminders(&migrated);
            return migrated;
        }
    }
    Vec::new()
}
```

## 📊 Storage Limits

**Browser Limits (localStorage)**:
- **Chrome/Edge**: ~10MB per origin
- **Firefox**: ~10MB per origin
- **Safari**: ~5MB per origin

**When to Consider IndexedDB**:
- Data size > 5MB
- Need indexed queries
- Large number of records (1000+)
- Need transactions or complex operations

**Best Practices**:
- Keep stored data minimal
- Don't store large binary data
- Use localStorage for simple key-value storage
- Consider IndexedDB for larger/complex datasets (see `storage-comparison.mdc`)

## 🔄 Alternative: IndexedDB (Future)

For projects requiring larger storage or indexed queries, consider IndexedDB:

**Rust Crates**:
- `idb` - IndexedDB bindings for Rust/WASM
- `gloo-storage` - Higher-level storage abstractions

**Pattern** (similar to Flutter's IndexedDBSer):
- Service manager for all databases
- Specialized DB services per data type
- Async read/write operations
- Reactive state updates

**See Also**: `storage-comparison.mdc` for detailed comparison with Flutter PWA patterns

## 🎯 Best Practices

### DO:
- ✅ Always provide fallback (empty collection) on load error
- ✅ Save immediately after state changes
- ✅ Use descriptive storage keys
- ✅ Handle serialization errors gracefully
- ✅ Use `serde_json` for JSON serialization
- ✅ Keep storage keys consistent

### DON'T:
- ❌ Don't panic on storage errors
- ❌ Don't store sensitive data in localStorage
- ❌ Don't store large binary data
- ❌ Don't forget to save after mutations
- ❌ Don't use `unwrap()` on storage operations
- ❌ Don't mix storage formats (always use JSON)

## 🔗 Related Patterns

- **State Management**: See `skills.md` → State Management
- **Component Integration**: See `skills.md` → Component Development
- **Error Handling**: Always return safe defaults

---

**Storage Type**: Browser LocalStorage  
**Serialization**: serde_json  
**Error Strategy**: Graceful degradation
