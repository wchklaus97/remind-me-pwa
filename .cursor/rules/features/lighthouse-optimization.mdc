---
alwaysApply: true
description: Lighthouse optimization patterns to achieve 95%+ scores across all categories
---
# Lighthouse Optimization Patterns

## 🎯 Target Scores

- **Performance**: ≥ 95%
- **Accessibility**: ≥ 95%
- **Best Practices**: ≥ 95%
- **SEO**: ≥ 95%

## 🔍 Common Issues and Solutions

### 1. Accessibility Issues

#### Touch Targets (Score Impact: High)

**Problem**: Touch targets are too small (< 48x48px) or lack spacing

**Solution**:
```css
/* Minimum touch target size: 48x48px */
.btn, .tab, button {
    min-width: 48px;
    min-height: 48px;
    padding: 12px 16px; /* Ensures sufficient touch area */
    margin: 4px; /* Spacing between targets */
}

/* For small buttons, use padding to increase touch area */
.btn-small {
    padding: 12px; /* Even small buttons have large touch area */
}
```

**Implementation**:
- All interactive elements must be ≥ 48x48px
- Minimum 8px spacing between touch targets
- Use padding, not just width/height

#### Main Landmark (Score Impact: Medium)

**Problem**: Document missing `<main>` landmark

**Solution**:
```rust
rsx! {
    html {
        lang: "en",
        head {
            // ...
        }
        body {
            main {
                role: "main",
                // Main content here
            }
        }
    }
}
```

#### Lang Attribute (Score Impact: Medium)

**Problem**: `<html>` element missing `lang` attribute

**Solution**:
```rust
rsx! {
    html {
        lang: "en",  // Add lang attribute
        // ...
    }
}
```

### 2. Performance Issues

#### Source Maps (Score Impact: Low, but important for debugging)

**Problem**: Missing source maps for WASM files

**Solution**:
```toml
# Cargo.toml
[profile.release]
debug = true  # Include debug info for source maps

[profile.wasm-release]
inherits = "release"
debug = true  # Enable source maps
```

**Build Configuration**:
```bash
# Build with source maps
dx build --release --platform web
# Source maps will be generated automatically
```

#### CSS Loading (Score Impact: High)

**Problem**: CSS file 404 errors

**Solution**:
- Ensure CSS path matches `base_path`
- Verify file exists in build output
- Check service worker cache URLs

### 3. Best Practices Issues

#### Console Errors (Score Impact: Medium)

**Problem**: Browser errors logged to console

**Solution**:
- Fix all JavaScript/WASM errors
- Handle all error cases gracefully
- Remove debug `console.log` in production

#### Security Headers (Score Impact: Low, but important)

**Problem**: Missing security headers

**Solution**:
- Add CSP (Content Security Policy)
- Add HSTS headers (if using HTTPS)
- Add COOP headers for origin isolation

## 📋 Implementation Checklist

### Accessibility (Target: 95%+)

- [ ] All touch targets ≥ 48x48px
- [ ] Minimum 8px spacing between touch targets
- [ ] Add `<main>` landmark to main content
- [ ] Add `lang` attribute to `<html>` element
- [ ] Add ARIA labels where needed
- [ ] Ensure proper heading hierarchy (h1 → h2 → h3)
- [ ] Add alt text to images (if any)
- [ ] Ensure color contrast ratio ≥ 4.5:1

### Performance (Target: 95%+)

- [ ] Enable source maps for WASM
- [ ] Optimize asset loading
- [ ] Fix CSS 404 errors
- [ ] Minimize main thread blocking
- [ ] Optimize images (if any)
- [ ] Use efficient cache lifetimes
- [ ] Minimize JavaScript execution time

### Best Practices (Target: 95%+)

- [ ] Fix all console errors
- [ ] Add source maps
- [ ] Implement security headers (CSP, HSTS, COOP)
- [ ] Use HTTPS (GitHub Pages provides this)
- [ ] Avoid deprecated APIs
- [ ] Ensure proper error handling

### SEO (Target: 95%+)

- [ ] Add `<title>` tag
- [ ] Add meta description
- [ ] Add Open Graph tags
- [ ] Ensure proper heading structure
- [ ] Add `lang` attribute
- [ ] Ensure mobile-friendly design

## 🎨 CSS Fixes for Touch Targets

```css
/* Touch Target Standards */
:root {
    --touch-target-min: 48px;
    --touch-target-spacing: 8px;
}

/* Buttons */
.btn {
    min-width: var(--touch-target-min);
    min-height: var(--touch-target-min);
    padding: 12px 16px;
    margin: var(--touch-target-spacing);
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

/* Tabs */
.tab {
    min-width: var(--touch-target-min);
    min-height: var(--touch-target-min);
    padding: 12px 20px;
    margin: 0 var(--touch-target-spacing);
}

/* Small buttons still need large touch area */
.btn-small {
    min-width: var(--touch-target-min);
    min-height: var(--touch-target-min);
    padding: 12px;
}
```

## 🔧 HTML Structure Fixes

```rust
// Proper HTML structure with landmarks
rsx! {
    html {
        lang: "en",
        head {
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            meta { name: "description", content: "A simple and elegant reminder app" }
            title { "Remind Me PWA" }
            link { rel: "stylesheet", href: "assets/style.css" }
        }
        body {
            header {
                role: "banner",
                // Header content
            }
            main {
                role: "main",
                // Main content
            }
            footer {
                role: "contentinfo",
                // Footer content (if any)
            }
        }
    }
}
```

## 📊 Performance Optimization

### Asset Loading

```rust
// Preload critical resources
rsx! {
    head {
        link { rel: "preload", href: "assets/style.css", r#as: "style" }
        link { rel: "preload", href: "assets/manifest.json", r#as: "manifest" }
    }
}
```

### Service Worker Caching

```javascript
// Update service worker to cache all assets
const CACHE_NAME = 'remind-me-pwa-v2';
const urlsToCache = [
  '/remind-me-pwa/',
  '/remind-me-pwa/index.html',
  '/remind-me-pwa/assets/style.css',
  '/remind-me-pwa/assets/manifest.json',
  '/remind-me-pwa/wasm/remind-me-pwa.wasm',
  '/remind-me-pwa/wasm/remind-me-pwa.js'
];
```

## 🛡️ Security Headers

### Content Security Policy

Add to `index.html` or via meta tag:

```html
<meta http-equiv="Content-Security-Policy" 
      content="default-src 'self'; 
               script-src 'self' 'wasm-unsafe-eval'; 
               style-src 'self' 'unsafe-inline';">
```

## 🎯 Best Practices

### DO:
- ✅ Always use semantic HTML elements
- ✅ Add `lang` attribute to `<html>`
- ✅ Use `<main>` landmark for main content
- ✅ Ensure touch targets ≥ 48x48px
- ✅ Add spacing between interactive elements
- ✅ Fix all console errors
- ✅ Enable source maps in development
- ✅ Use proper heading hierarchy
- ✅ Add ARIA labels where needed

### DON'T:
- ❌ Don't use small touch targets (< 48px)
- ❌ Don't skip semantic HTML
- ❌ Don't ignore console errors
- ❌ Don't forget `lang` attribute
- ❌ Don't skip landmarks
- ❌ Don't use deprecated APIs

## 📈 Monitoring

### Regular Lighthouse Audits

Run Lighthouse audits regularly:
- After each major feature
- Before releases
- Monthly maintenance checks

### Target Metrics

- **FCP (First Contentful Paint)**: < 1.8s
- **LCP (Largest Contentful Paint)**: < 2.5s
- **TBT (Total Blocking Time)**: < 200ms
- **CLS (Cumulative Layout Shift)**: < 0.1
- **Speed Index**: < 3.4s

## Resources

- [Lighthouse Scoring Guide](https://developer.chrome.com/docs/lighthouse/performance/performance-scoring/)
- [Web.dev Accessibility](https://web.dev/accessible/)
- [Touch Target Guidelines](https://web.dev/accessible-tap-targets/)
- [Semantic HTML](https://developer.mozilla.org/en-US/docs/Glossary/Semantics#semantic_elements)
