---
alwaysApply: true
description: Strict code standards to achieve 100% Lighthouse scores across all categories
---
# Lighthouse 100% Standards

## 🎯 Goal

Achieve **100% scores** across all Lighthouse categories:
- **Performance**: 100%
- **Accessibility**: 100%
- **Best Practices**: 100%
- **SEO**: 100%

## 📋 Mandatory Requirements

### 1. HTML Structure (MANDATORY)

**Every Dioxus component MUST include:**

```rust
// In App component or root component
rsx! {
    // Dioxus 0.6 automatically generates HTML structure
    // But we must ensure proper structure in components
    div {
        // Content
    }
}
```

**For HTML attributes (lang, meta tags):**
- Use `index.html` template if available
- Or configure via Dioxus.toml
- Or use JavaScript to set after mount

### 2. Touch Targets (MANDATORY - Zero Tolerance)

**ALL interactive elements MUST be ≥ 48x48px:**

```css
/* MANDATORY: All buttons, tabs, links, checkboxes */
button, .btn, .tab, a[role="button"], label[for] {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;  /* Fallback */
    min-height: 3rem !important;   /* Fallback */
    padding: 12px 16px;  /* Minimum padding */
    margin: 8px;         /* Minimum spacing */
}

/* MANDATORY: Checkbox touch area */
input[type="checkbox"] {
    width: 24px;
    height: 24px;
}

label.checkbox-label {
    min-width: 48px !important;
    min-height: 48px !important;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 12px;  /* Ensures 48px total */
}
```

**Verification:**
- Use browser DevTools to measure actual rendered size
- Must be ≥ 48x48px in ALL viewport sizes
- Test on mobile devices

### 3. Lang Attribute (MANDATORY)

**MUST be set on `<html>` element:**

**Solution 1: index.html template**
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="A simple and elegant reminder app to help you stay organized">
    <title>Remind Me PWA - Your Personal Reminder Assistant</title>
</head>
<body>
    <div id="main"></div>
</body>
</html>
```

**Solution 2: JavaScript injection (if template not available)**
```rust
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

### 4. Meta Description (MANDATORY for SEO)

**MUST have meta description:**

**Solution 1: index.html template**
```html
<meta name="description" content="A simple and elegant reminder app to help you stay organized">
```

**Solution 2: JavaScript injection**
```rust
use_effect(move || {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                // Check if meta description exists
                let existing = document.query_selector("meta[name='description']");
                if existing.is_ok() && existing.unwrap().is_none() {
                    // Create and append meta description
                    if let Ok(meta) = document.create_element("meta") {
                        let _ = meta.set_attribute("name", "description");
                        let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
                        let _ = head.append_child(&meta);
                    }
                }
            }
        }
    }
});
```

### 5. Source Maps (MANDATORY for Best Practices)

**MUST enable source maps:**

```toml
# Cargo.toml - MANDATORY
[profile.wasm-release]
inherits = "release"
strip = false
debug = true   # MANDATORY: Enable source maps
```

**Verification:**
- Build with `dx build --release --platform web`
- Check for `.wasm.map` files in build output
- Verify source maps are served correctly

### 6. Semantic HTML (MANDATORY)

**MUST use semantic elements:**

```rust
// MANDATORY structure
rsx! {
    header {
        role: "banner",
        // Header content
    }
    main {
        role: "main",
        // Main content
    }
    nav {
        role: "navigation",
        // Navigation content
    }
    article {
        // Article content (e.g., reminder cards)
    }
    section {
        // Section content
    }
    footer {
        role: "contentinfo",
        // Footer content (if any)
    }
}
```

### 7. ARIA Labels (MANDATORY)

**ALL interactive elements MUST have ARIA labels:**

```rust
// MANDATORY: All buttons
button {
    aria_label: "Descriptive action",
    onclick: move |_| { /* ... */ },
    "Button Text"
}

// MANDATORY: All form inputs
input {
    aria_label: "Input purpose",
    aria_required: "true",  // If required
    // ...
}

// MANDATORY: Navigation
nav {
    aria_label: "Navigation purpose",
    // ...
}
```

### 8. Heading Hierarchy (MANDATORY)

**MUST follow proper heading structure:**

```rust
// MANDATORY: One h1 per page
h1 { "Page Title" }

// MANDATORY: Sequential hierarchy
h2 { "Section Title" }
h3 { "Subsection Title" }

// DON'T skip levels
// ❌ h1 → h3 (skips h2)
// ✅ h1 → h2 → h3
```

## 🔍 Pre-Commit Checklist

Before committing ANY code, verify:

- [ ] All touch targets ≥ 48x48px (measure in DevTools)
- [ ] `<html lang="en">` is set (check in DevTools)
- [ ] Meta description exists (check in DevTools)
- [ ] Source maps are generated (check build output)
- [ ] All interactive elements have ARIA labels
- [ ] Semantic HTML structure is used
- [ ] Proper heading hierarchy (h1 → h2 → h3)
- [ ] No console errors
- [ ] Lighthouse audit passes 100% in all categories

## 🧪 Testing Requirements

### Before Every Commit:

1. **Run Lighthouse Audit**:
   ```bash
   # Build and serve
   dx build --release --platform web
   dx serve
   
   # Then run Lighthouse in Chrome DevTools
   ```

2. **Verify Touch Targets**:
   - Open Chrome DevTools
   - Inspect each button/tab
   - Verify computed size ≥ 48x48px
   - Test on mobile viewport

3. **Verify HTML Attributes**:
   - Check `<html lang="en">` in Elements tab
   - Check `<meta name="description">` in Elements tab

4. **Verify Source Maps**:
   - Check Network tab for `.wasm.map` files
   - Verify source maps load without 404

## 🚫 Zero Tolerance Rules

**These will cause immediate PR rejection:**

1. ❌ **Touch targets < 48x48px** - NO EXCEPTIONS
2. ❌ **Missing lang attribute** - NO EXCEPTIONS
3. ❌ **Missing meta description** - NO EXCEPTIONS
4. ❌ **Missing source maps** - NO EXCEPTIONS
5. ❌ **Missing ARIA labels on interactive elements** - NO EXCEPTIONS
6. ❌ **Missing semantic HTML** - NO EXCEPTIONS
7. ❌ **Improper heading hierarchy** - NO EXCEPTIONS
8. ❌ **Console errors** - NO EXCEPTIONS

## 📐 CSS Standards

### Touch Target Enforcement

```css
/* MANDATORY: Enforce minimum touch targets */
* {
    /* Reset to ensure no inheritance issues */
}

/* MANDATORY: All interactive elements */
button,
.btn,
.tab,
a[role="button"],
input[type="button"],
input[type="submit"],
input[type="checkbox"] + label,
label[for] {
    min-width: 48px !important;
    min-height: 48px !important;
    /* Use rem for better scaling */
    min-width: 3rem !important;
    min-height: 3rem !important;
}

/* MANDATORY: Spacing between touch targets */
button + button,
.btn + .btn,
.tab + .tab {
    margin-left: 8px;  /* Minimum spacing */
}
```

## 🔧 Implementation Patterns

### Pattern 1: HTML Attributes Setup

```rust
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    // Set HTML lang attribute on mount
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("lang", "en");
                }
                // Set meta description
                if let Some(head) = document.head() {
                    if let Ok(meta) = document.create_element("meta") {
                        let _ = meta.set_attribute("name", "description");
                        let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
                        let _ = head.append_child(&meta);
                    }
                }
            }
        }
    });

    rsx! {
        div {
            // App content
        }
    }
}
```

### Pattern 2: Touch Target Verification

```rust
// After component mount, verify touch targets
use_effect(move || {
    // In development, log warnings if touch targets are too small
    #[cfg(debug_assertions)]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Check all buttons
                if let Ok(buttons) = document.query_selector_all("button") {
                    for i in 0..buttons.length() {
                        if let Some(button) = buttons.get(i) {
                            if let Ok(rect) = button.get_bounding_client_rect() {
                                if rect.width() < 48.0 || rect.height() < 48.0 {
                                    web_sys::console::warn_1(&format!(
                                        "Touch target too small: {}x{}px (minimum: 48x48px)",
                                        rect.width(),
                                        rect.height()
                                    ).into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
});
```

## 📊 Lighthouse Score Targets

### Current Issues to Fix:

1. **Accessibility (95% → 100%)**:
   - ❌ Missing `<html lang="en">` attribute
   - ❌ Touch targets may still be too small (verify actual rendered size)

2. **Best Practices (100%)**:
   - ⚠️ Missing source maps (verify they're generated and served)

3. **SEO (90% → 100%)**:
   - ❌ Missing meta description (verify it's rendered)

4. **Performance (100%)**:
   - ✅ Already at 100%

## 🎯 Enforcement

### Code Review Checklist

Every PR MUST include:

1. ✅ Lighthouse audit screenshot showing 100% in all categories
2. ✅ DevTools screenshot showing `<html lang="en">`
3. ✅ DevTools screenshot showing meta description
4. ✅ DevTools screenshot showing touch target sizes ≥ 48x48px
5. ✅ Network tab screenshot showing source maps loaded

### Automated Checks (Future)

- Pre-commit hook to run Lighthouse CI
- CI/CD pipeline to verify Lighthouse scores
- Automated touch target size verification

## 📚 Resources

- [Lighthouse 100% Guide](https://web.dev/lighthouse-best-practices/)
- [Touch Target Guidelines](https://web.dev/accessible-tap-targets/)
- [HTML Lang Attribute](https://www.w3.org/International/questions/qa-html-language-declarations)
- [Meta Description Best Practices](https://developers.google.com/search/docs/appearance/snippet)
