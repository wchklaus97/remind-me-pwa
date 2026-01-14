---
alwaysApply: true
description: Strict code standards to achieve and maintain 100% Lighthouse scores
---
# Code Standards for 100% Lighthouse Scores

## 🎯 Mandatory Requirements

### Zero Tolerance Rules

These rules are **MANDATORY** and **NON-NEGOTIABLE**. Any violation will cause PR rejection.

## 1. Touch Targets (MANDATORY - 48px NOT 24px!)

**ALL interactive elements MUST be ≥ 48x48px (Lighthouse requirement):**

⚠️ **CRITICAL**: The touch target MUST be 48px, NOT 24px! This is a Lighthouse 100% requirement.

```css
/* MANDATORY: Use !important to enforce */
:root {
    --touch-target-min: 48px;  /* Lighthouse requirement - NOT 24px! */
}

button, .btn, .tab, a[role="button"], label[for] {
    min-width: var(--touch-target-min) !important;
    min-height: var(--touch-target-min) !important;
    min-width: 48px !important;  /* Fallback */
    min-height: 48px !important;
    box-sizing: border-box;  /* Include padding in size */
}

/* MANDATORY: Spacing between touch targets */
button + button, .btn + .btn {
    margin-left: 8px !important;
    margin-right: 8px !important;
}
```

**Verification Required:**
- Measure in browser DevTools
- Must be ≥ 48x48px in ALL viewport sizes
- Test on mobile devices

## 2. HTML Lang Attribute (MANDATORY)

**MUST be set on `<html>` element:**

```rust
// MANDATORY: Set in App component
use_effect(move || {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("lang", "en");
            }
        }
    }
});
```

**Verification:**
- Check in DevTools Elements tab
- Must be `<html lang="en">`

## 3. Meta Description (MANDATORY)

**MUST have meta description for SEO:**

```rust
// MANDATORY: Set in App component
use_effect(move || {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(Some(head)) = document.query_selector("head") {
                let existing_meta = document.query_selector("meta[name='description']");
                if let Ok(Some(_)) = existing_meta {
                    // Already exists
                } else {
                    if let Ok(meta) = document.create_element("meta") {
                        let _ = meta.set_attribute("name", "description");
                        let _ = meta.set_attribute("content", "Your app description here");
                        if let Some(head_element) = head.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = head_element.append_child(&meta);
                        }
                    }
                }
            }
        }
    }
});
```

**Verification:**
- Check in DevTools Elements tab
- Must have `<meta name="description" content="...">`

## 4. Source Maps (MANDATORY)

**MUST enable source maps:**

```toml
# Cargo.toml - MANDATORY
[profile.wasm-release]
inherits = "release"
strip = false
debug = true   # MANDATORY
```

**Verification:**
- Check build output for `.wasm.map` files
- Check Network tab for source maps loading

## 5. Semantic HTML (MANDATORY)

**MUST use semantic elements:**

```rust
// MANDATORY structure
rsx! {
    header { role: "banner", /* ... */ }
    main { role: "main", /* ... */ }
    nav { role: "navigation", /* ... */ }
    article { /* ... */ }
    section { /* ... */ }
}
```

## 6. ARIA Labels (MANDATORY)

**ALL interactive elements MUST have ARIA labels:**

```rust
// MANDATORY: All buttons
button {
    aria_label: "Descriptive action",
    // ...
}

// MANDATORY: All form inputs
input {
    aria_label: "Input purpose",
    // ...
}
```

## 7. Heading Hierarchy (MANDATORY)

**MUST follow proper structure:**
- One `<h1>` per page
- Sequential hierarchy (h1 → h2 → h3)
- No skipping levels

## 📋 Pre-Commit Checklist

Before EVERY commit:

- [ ] Run Lighthouse audit
- [ ] Verify all scores are 100%
- [ ] Check `<html lang="en">` in DevTools
- [ ] Check meta description in DevTools
- [ ] Measure touch targets in DevTools (≥ 48x48px)
- [ ] Verify source maps are generated
- [ ] Check for console errors
- [ ] Test on mobile viewport

## 🚫 Zero Tolerance Violations

These will cause **immediate PR rejection**:

1. ❌ Touch targets < 48x48px
2. ❌ Missing lang attribute
3. ❌ Missing meta description
4. ❌ Missing source maps
5. ❌ Missing ARIA labels
6. ❌ Missing semantic HTML
7. ❌ Console errors
8. ❌ Lighthouse score < 100% in any category

## 📐 CSS Enforcement

```css
/* MANDATORY: Enforce touch targets with !important */
* {
    box-sizing: border-box;  /* Ensure padding included */
}

button, .btn, .tab {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;
    min-height: 3rem !important;
}
```

## 🔧 Implementation Pattern

```rust
// MANDATORY: Set HTML attributes in App component
#[component]
fn App() -> Element {
    use_effect(move || {
        // Set lang and meta description
        set_html_attributes();
    });
    
    rsx! { /* ... */ }
}

fn set_html_attributes() {
    // Implementation from lighthouse-100-standards.mdc
}
```
