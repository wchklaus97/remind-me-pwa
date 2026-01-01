---
alwaysApply: false
description: Testing patterns for Dioxus PWA and WASM applications
---
# Testing Patterns for Dioxus PWA

## 🎯 Testing Strategy

### Test Types

1. **Unit Tests**: Test individual functions and utilities
2. **Component Tests**: Test Dioxus components in isolation
3. **Integration Tests**: Test storage and state management
4. **E2E Tests**: Test complete user workflows (if needed)

## Unit Testing

### Testing Storage Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_load_empty_reminders() {
        // Clear localStorage before test
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.remove_item("reminders");
            }
        }
        
        let reminders = load_reminders();
        assert_eq!(reminders.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_save_and_load_reminders() {
        let test_reminders = vec![
            Reminder {
                id: "test-1".to_string(),
                title: "Test Reminder".to_string(),
                description: String::new(),
                due_date: String::new(),
                completed: false,
                created_at: chrono::Utc::now().to_rfc3339(),
            }
        ];
        
        save_reminders(&test_reminders);
        let loaded = load_reminders();
        
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "test-1");
    }
}
```

### Testing Utility Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let rfc3339 = "2025-01-15T10:30:00Z";
        let formatted = format_date(rfc3339);
        assert_eq!(formatted, "2025-01-15 10:30");
    }

    #[test]
    fn test_format_date_local() {
        let local = "2025-01-15T10:30";
        let formatted = format_date(local);
        assert_eq!(formatted, "2025-01-15 10:30");
    }
}
```

## Component Testing

### Testing Component Logic

```rust
#[cfg(test)]
mod component_tests {
    use super::*;

    #[test]
    fn test_reminder_filtering() {
        let reminders = vec![
            Reminder { id: "1".to_string(), completed: false, .. },
            Reminder { id: "2".to_string(), completed: true, .. },
        ];
        
        let active: Vec<_> = reminders
            .iter()
            .filter(|r| !r.completed)
            .collect();
        
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "1");
    }
}
```

## Integration Testing

### Testing State Management

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_reminder_lifecycle() {
        // Create
        let reminder = Reminder {
            id: "test-1".to_string(),
            title: "Test".to_string(),
            description: String::new(),
            due_date: String::new(),
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        // Save
        let mut reminders = vec![reminder.clone()];
        save_reminders(&reminders);
        
        // Load
        let loaded = load_reminders();
        assert_eq!(loaded.len(), 1);
        
        // Update
        reminders[0].completed = true;
        save_reminders(&reminders);
        
        // Verify
        let updated = load_reminders();
        assert!(updated[0].completed);
        
        // Delete
        reminders.clear();
        save_reminders(&reminders);
        
        let empty = load_reminders();
        assert_eq!(empty.len(), 0);
    }
}
```

## Test Configuration

### Cargo.toml

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
wasm-bindgen = "0.2"

[profile.test]
opt-level = 0  # Faster test compilation
```

### Test Organization

```rust
// src/main.rs

#[cfg(test)]
mod storage_tests {
    // Storage-related tests
}

#[cfg(test)]
mod component_tests {
    // Component-related tests
}

#[cfg(test)]
mod integration_tests {
    // Integration tests
}
```

## Running Tests

### Native Tests (Non-WASM)

```bash
# Run all tests
cargo test

# Run specific test module
cargo test storage_tests

# Run with output
cargo test -- --nocapture
```

### WASM Tests

```bash
# Install wasm-bindgen-test
cargo install wasm-bindgen-cli

# Run WASM tests in browser
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
```

## Mocking Patterns

### Mock Storage for Tests

```rust
#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn setup_test_storage() {
        // Clear storage before test
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.clear();
            }
        }
    }

    pub fn create_test_reminder(id: &str) -> Reminder {
        Reminder {
            id: id.to_string(),
            title: format!("Test Reminder {}", id),
            description: String::new(),
            due_date: String::new(),
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```

## Best Practices

### DO:
- ✅ Test storage functions with real localStorage
- ✅ Test utility functions in isolation
- ✅ Clean up test data after each test
- ✅ Use descriptive test names
- ✅ Test error cases (empty storage, invalid data)
- ✅ Group related tests in modules

### DON'T:
- ❌ Don't test Dioxus internals (test your code)
- ❌ Don't leave test data in localStorage
- ❌ Don't test implementation details
- ❌ Don't skip error handling tests
- ❌ Don't create tests that depend on each other

## Test Coverage Goals

- **Storage Functions**: 100% coverage
- **Utility Functions**: 100% coverage
- **Component Logic**: Critical paths covered
- **Error Handling**: All error paths tested

## Resources

- [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
