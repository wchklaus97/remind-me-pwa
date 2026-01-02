# Fixing Component String Literal Issues

## Problem
The translation calls `t.t("key")` inside rsx! format strings `"{t.t("key")}"` are causing Rust parsing errors because of nested quotes.

## Solution
In Dioxus rsx!, expressions go directly in `{}`, not inside quoted strings. We need to:
1. Call `t.t("key")` directly: `{t.t("key")}` instead of `"{t.t("key")}"`
2. Or store translations in variables first

## Files to Fix
- src/components/landing.rs - Multiple translation calls
- src/components/statistics.rs - Translation calls  
- src/components/forms.rs - Translation calls
- src/components/cards.rs - Translation calls + missing chrono::TimeZone import
- src/components/modals.rs - Translation calls
- src/components/reminder_app.rs - Translation calls

## Pattern
```rust
// WRONG:
"{t.t("key")}"

// CORRECT:
{t.t("key")}
```

