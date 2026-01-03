---
alwaysApply: false
description: Page template patterns for reusable layouts with navbar and footer
---

# Page Templates

## 🎯 Overview

The project uses **page templates** to create consistent layouts across public pages. The `PublicPageTemplate` component wraps content with the landing page navbar and footer, providing a reusable structure for pages like Privacy Policy and Terms of Use.

## 📝 PublicPageTemplate

### Purpose

- **Consistent Layout**: Same navbar and footer across all public pages
- **Reusability**: Avoid duplicating navbar/footer code
- **Maintainability**: Update navbar/footer in one place
- **Navigation**: Integrated navigation handling

### Component Structure

```rust
#[component]
pub fn PublicPageTemplate(
    title: String,                    // Page title
    active_section: String,           // Active section for navbar highlighting
    on_enter_app: EventHandler<()>,   // Handler for "Try it" button
    on_jump: EventHandler<&'static str>, // Handler for section navigation
    on_navigate: EventHandler<Route>, // Handler for route navigation
    children: Element,                 // Page content
) -> Element
```

### Layout Structure

```
landing-page
└── landing-container
    └── landing-shell
        ├── LandingNavbar
        ├── main (public-page-main)
        │   └── section (public-page-card)
        │       ├── h2 (public-page-title)
        │       └── div (public-page-content)
        │           └── {children}
        └── LandingFooter
```

## 🔧 Usage Pattern

### Basic Example

```rust
use crate::components::page_template::PublicPageTemplate;
use crate::router::Route;
use crate::i18n::use_t;

#[component]
fn PrivacyPolicyPage() -> Element {
    let mut active_section = use_signal(|| "".to_string());
    
    rsx! {
        PublicPageTemplate {
            title: use_t("legal.privacy.title"),
            active_section: active_section(),
            on_enter_app: move |_| {
                // Navigate to app
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/app");
                }
            },
            on_jump: move |_section: &'static str| {
                // Handle section navigation (if needed)
            },
            on_navigate: move |route: Route| {
                // Handle route navigation
            },
            // Page content
            div {
                h3 { {use_t("legal.privacy.section1.title")} }
                p { {use_t("legal.privacy.section1.content")} }
                // ... more content
            }
        }
    }
}
```

### With Internationalization

```rust
use crate::components::page_template::PublicPageTemplate;
use crate::i18n::use_t;

#[component]
fn TermsOfUsePage() -> Element {
    let mut active_section = use_signal(|| "".to_string());
    
    rsx! {
        PublicPageTemplate {
            title: use_t("legal.terms.title"),
            active_section: active_section(),
            on_enter_app: move |_| {
                // Navigate to app
            },
            on_jump: move |_| {},
            on_navigate: move |_| {},
            // Internationalized content
            div {
                {use_t("legal.terms.intro")}
                // ... more content
            }
        }
    }
}
```

## 🎨 CSS Classes

### Template Classes

```css
.public-page-main {
    /* Main content area */
    padding: 2rem 0;
}

.public-page-card {
    /* Card container */
    background: white;
    border-radius: 12px;
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
}

.public-page-title {
    /* Page title */
    font-size: 2rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
}

.public-page-content {
    /* Content area */
    /* Add your content styles here */
}
```

## ✅ Best Practices

### DO:
- ✅ Use `PublicPageTemplate` for all public pages (Privacy Policy, Terms of Use)
- ✅ Provide internationalized titles using `use_t()`
- ✅ Handle navigation events properly
- ✅ Keep content focused and scannable
- ✅ Use semantic HTML in content (`h3`, `p`, `ul`, etc.)
- ✅ Test with all supported languages

### DON'T:
- ❌ Don't create custom navbar/footer for public pages (use template)
- ❌ Don't forget to handle navigation events
- ❌ Don't hardcode text (use `use_t()` for i18n)
- ❌ Don't use template for landing page (uses `LandingPage`)
- ❌ Don't use template for app pages (uses `ReminderApp`)

## 📚 Related Files

- `src/components/page_template.rs` - Template component definition
- `src/components/legal.rs` - Example usage (Privacy Policy, Terms of Use)
- `src/components/landing_layout.rs` - Navbar and footer components
- `assets/css/layout.css` - Template styling

## 🔄 When to Use vs. Not Use

### Use PublicPageTemplate For:
- ✅ Privacy Policy pages
- ✅ Terms of Use pages
- ✅ Any public informational page
- ✅ Pages that need navbar + footer
- ✅ Pages that should match landing page style

### Don't Use PublicPageTemplate For:
- ❌ Landing page (uses `LandingPage` component)
- ❌ App pages (uses `ReminderApp` component)
- ❌ Pages with different layout needs
- ❌ Pages that don't need navbar/footer

## 🧪 Testing

- Test page renders correctly
- Verify navbar and footer appear
- Test navigation handlers work
- Verify internationalization
- Test responsive design
- Check accessibility (semantic HTML)
