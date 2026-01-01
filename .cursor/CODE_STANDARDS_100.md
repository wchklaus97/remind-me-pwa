# Code Standards for 100% Lighthouse Scores

## 🎯 Goal

Achieve and maintain **100% scores** across ALL Lighthouse categories:
- **Performance**: 100%
- **Accessibility**: 100%
- **Best Practices**: 100%
- **SEO**: 100%

## 🚫 Zero Tolerance Rules

These rules are **MANDATORY** and **NON-NEGOTIABLE**. Any violation will cause PR rejection.

### 1. Touch Targets (MANDATORY)

**ALL interactive elements MUST be ≥ 48x48px:**

```css
/* MANDATORY: Use !important to enforce */
button, .btn, .tab, a[role="button"], label[for] {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;  /* Fallback in rem */
    min-height: 3rem !important;
    box-sizing: border-box;  /* Include padding in size */
    padding: 12px 16px;  /* Minimum padding */
    margin: 8px;  /* Minimum spacing */
}

/* MANDATORY: Spacing between touch targets */
button + button, .btn + .btn, .tab + .tab {
    margin-left: 8px !important;
    margin-right: 8px !important;
}
```

**Verification Required:**
- ✅ Measure in browser DevTools (Computed tab)
- ✅ Must be ≥ 48x48px in ALL viewport sizes
- ✅ Test on mobile devices (375px, 414px viewports)
- ✅ Use browser's responsive design mode

### 2. HTML Lang Attribute (MANDATORY)

**MUST be set on `<html>` element:**

```rust
// MANDATORY: Set in App component using use_effect
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[component]
fn App() -> Element {
    // Set HTML lang attribute on mount
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("lang", "en");
                }
            }
        }
    });
    
    rsx! { /* ... */ }
}
```

**Verification:**
- ✅ Check in DevTools Elements tab
- ✅ Must see `<html lang="en">`
- ✅ Test in incognito mode (fresh page load)

### 3. Meta Description (MANDATORY)

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
                        let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
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
- ✅ Check in DevTools Elements tab
- ✅ Must see `<meta name="description" content="...">`
- ✅ Content should be 120-160 characters
- ✅ Test in incognito mode

### 4. Source Maps (MANDATORY)

**MUST enable source maps:**

```toml
# Cargo.toml - MANDATORY
[profile.wasm-release]
inherits = "release"
strip = false
debug = true   # MANDATORY: Enable source maps
```

**Verification:**
- ✅ Check build output for `.wasm.map` files
- ✅ Check Network tab for source maps loading (no 404)
- ✅ Verify source maps are accessible

### 5. Semantic HTML (MANDATORY)

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
}
```

**Verification:**
- ✅ Check in DevTools Elements tab
- ✅ Must have `<main role="main">`
- ✅ Must have `<header role="banner">` (if header exists)
- ✅ Must have `<nav role="navigation">` (if nav exists)

### 6. ARIA Labels (MANDATORY)

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

**Verification:**
- ✅ Check in DevTools Elements tab
- ✅ All buttons must have `aria-label` attribute
- ✅ All form inputs must have `aria-label` attribute
- ✅ Test with screen reader

### 7. Heading Hierarchy (MANDATORY)

**MUST follow proper structure:**

```rust
// MANDATORY: One h1 per page
h1 { "Page Title" }

// MANDATORY: Sequential hierarchy
h2 { "Section Title" }
h3 { "Subsection Title" }

// ❌ DON'T skip levels
// ❌ h1 → h3 (skips h2)
// ✅ h1 → h2 → h3
```

**Verification:**
- ✅ Check heading structure in DevTools
- ✅ Must have one `<h1>` per page
- ✅ No skipping levels

## 📋 Pre-Commit Checklist

Before **EVERY** commit, verify:

### Lighthouse Audit
- [ ] Run Lighthouse audit in Chrome DevTools
- [ ] **Performance**: 100%
- [ ] **Accessibility**: 100%
- [ ] **Best Practices**: 100%
- [ ] **SEO**: 100%
- [ ] Screenshot of scores for PR

### HTML Structure
- [ ] `<html lang="en">` exists (check in DevTools)
- [ ] `<meta name="description">` exists (check in DevTools)
- [ ] `<main role="main">` exists
- [ ] Proper heading hierarchy (h1 → h2 → h3)

### Touch Targets
- [ ] All buttons ≥ 48x48px (measure in DevTools)
- [ ] All tabs ≥ 48x48px (measure in DevTools)
- [ ] All checkboxes have 48x48px touch area
- [ ] Minimum 8px spacing between touch targets
- [ ] Test on mobile viewport (375px, 414px)

### ARIA Labels
- [ ] All buttons have `aria-label`
- [ ] All form inputs have `aria-label`
- [ ] Navigation has `aria-label`
- [ ] Test with screen reader

### Source Maps
- [ ] Source maps are generated (check build output)
- [ ] Source maps load without 404 (check Network tab)

### Console
- [ ] No console errors
- [ ] No console warnings (unless intentional)

## 🧪 Testing Requirements

### Before Every PR:

1. **Lighthouse Audit**:
   ```bash
   # Build and serve
   dx build --release --platform web
   dx serve
   
   # Then run Lighthouse in Chrome DevTools
   # Must achieve 100% in all categories
   ```

2. **Touch Target Verification**:
   - Open Chrome DevTools
   - Go to Elements tab
   - Inspect each button/tab
   - Check Computed tab
   - Verify width and height ≥ 48px
   - Test on mobile viewport

3. **HTML Attributes Verification**:
   - Check `<html lang="en">` in Elements tab
   - Check `<meta name="description">` in Elements tab
   - Test in incognito mode (fresh page load)

4. **Source Maps Verification**:
   - Check Network tab for `.wasm.map` files
   - Verify no 404 errors
   - Verify source maps load successfully

## 🚫 Zero Tolerance Violations

These will cause **immediate PR rejection**:

1. ❌ **Touch targets < 48x48px** - NO EXCEPTIONS
2. ❌ **Missing lang attribute** - NO EXCEPTIONS
3. ❌ **Missing meta description** - NO EXCEPTIONS
4. ❌ **Missing source maps** - NO EXCEPTIONS
5. ❌ **Missing ARIA labels** - NO EXCEPTIONS
6. ❌ **Missing semantic HTML** - NO EXCEPTIONS
7. ❌ **Improper heading hierarchy** - NO EXCEPTIONS
8. ❌ **Console errors** - NO EXCEPTIONS
9. ❌ **Lighthouse score < 100%** - NO EXCEPTIONS

## 📐 CSS Enforcement Standards

```css
/* MANDATORY: Enforce touch targets with !important */
:root {
    --touch-target-min: 48px;
    --touch-target-spacing: 8px;
}

/* MANDATORY: All interactive elements */
button,
.btn,
.tab,
a[role="button"],
input[type="button"],
input[type="submit"],
label[for] {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;  /* Fallback */
    min-height: 3rem !important;  /* Fallback */
    box-sizing: border-box !important;  /* Include padding */
    padding: 12px 16px;  /* Minimum padding */
    margin: 8px;  /* Minimum spacing */
}

/* MANDATORY: Spacing between touch targets */
button + button,
.btn + .btn,
.tab + .tab {
    margin-left: 8px !important;
    margin-right: 8px !important;
}

/* MANDATORY: Checkbox touch area */
label.checkbox-label {
    min-width: 48px !important;
    min-height: 48px !important;
    padding: 12px;  /* Ensures total size is 48px */
    display: inline-flex;
    align-items: center;
    justify-content: center;
}
```

## 🔧 Implementation Patterns

### Pattern 1: HTML Attributes Setup (MANDATORY)

```rust
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[component]
fn App() -> Element {
    // MANDATORY: Set HTML lang and meta description
    use_effect(move || {
        set_html_attributes();
    });
    
    rsx! { /* ... */ }
}

fn set_html_attributes() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Set lang attribute
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("lang", "en");
            }
            
            // Set meta description
            if let Ok(Some(head)) = document.query_selector("head") {
                let existing_meta = document.query_selector("meta[name='description']");
                if let Ok(Some(_)) = existing_meta {
                    // Already exists
                } else {
                    if let Ok(meta) = document.create_element("meta") {
                        let _ = meta.set_attribute("name", "description");
                        let _ = meta.set_attribute("content", "A simple and elegant reminder app to help you stay organized");
                        if let Some(head_element) = head.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = head_element.append_child(&meta);
                        }
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
```

### Pattern 2: Touch Target CSS (MANDATORY)

```css
/* MANDATORY: All buttons */
.btn {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;
    min-height: 3rem !important;
    box-sizing: border-box !important;
    padding: 12px 20px;
    margin: 8px;
}

/* MANDATORY: All tabs */
.tab {
    min-width: 48px !important;
    min-height: 48px !important;
    min-width: 3rem !important;
    min-height: 3rem !important;
    box-sizing: border-box !important;
    padding: 12px 20px;
    margin: 0 8px;
}
```

## 📊 Current Issues to Fix

Based on Lighthouse reports:

1. **Accessibility (95% → 100%)**:
   - ❌ Missing `<html lang="en">` - **FIXED** (use_effect)
   - ⚠️ Touch targets may still be too small - **FIXED** (CSS with !important)

2. **Best Practices (100%)**:
   - ⚠️ Missing source maps - **FIXED** (debug = true in wasm-release)

3. **SEO (90% → 100%)**:
   - ❌ Missing meta description - **FIXED** (use_effect)

4. **Performance (100%)**:
   - ✅ Already at 100%

## 🎯 Enforcement

### Code Review Requirements

Every PR MUST include:

1. ✅ Lighthouse audit screenshot showing **100% in all categories**
2. ✅ DevTools screenshot showing `<html lang="en">`
3. ✅ DevTools screenshot showing `<meta name="description">`
4. ✅ DevTools screenshot showing touch target sizes ≥ 48x48px
5. ✅ Network tab screenshot showing source maps loaded

### Automated Checks (Future)

- Pre-commit hook to run Lighthouse CI
- CI/CD pipeline to verify Lighthouse scores
- Automated touch target size verification
- Automated HTML attribute verification

## 📚 Resources

- [Lighthouse 100% Guide](https://web.dev/lighthouse-best-practices/)
- [Touch Target Guidelines](https://web.dev/accessible-tap-targets/)
- [HTML Lang Attribute](https://www.w3.org/International/questions/qa-html-language-declarations)
- [Meta Description Best Practices](https://developers.google.com/search/docs/appearance/snippet)
- [Source Maps Guide](https://developer.mozilla.org/en-US/docs/Tools/Debugger/How_to/Use_a_source_map)

