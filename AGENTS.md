# AGENTS.md

This file provides context and instructions for AI coding agents working on this Dioxus PWA project.

## Project Overview

**Remind Me PWA** is a Progressive Web App built with **Dioxus 0.7** (Rust) for managing reminders. It compiles to WebAssembly (WASM) and supports both web (PWA) and native mobile (iOS/Android) platforms.

**Key Technologies:**
- **Framework**: Dioxus 0.7 (React-like UI framework for Rust)
- **Target**: Web (WASM) via `wasm32-unknown-unknown`
- **Storage**: Browser localStorage (web), file system (mobile)
- **Deployment**: GitHub Pages (static hosting)
- **Architecture**: Client-side only PWA with localStorage persistence

**Features:**
- Create, edit, delete reminders with due dates
- Tag management with color coding
- Multiple view modes (List, Card, Folder, Calendar)
- Multi-language support (English, 简体中文, 繁體中文)
- PWA features (offline support, installable)
- Full accessibility (ARIA labels, keyboard navigation)
- Media caching with shimmer loading

---

## Setup Commands

### Prerequisites

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli --locked

# Verify installation
dx --version
rustc --version
```

### Initial Setup

```bash
# Clone repository (if needed)
git clone <repo-url>
cd remind-me-pwa

# Install dependencies (Rust automatically handles this via Cargo.toml)
# No separate install step needed - Cargo handles dependencies

# Verify workspace structure
cargo check
```

---

## Build Commands

### Development

```bash
# Start development server (hot reload enabled)
dx serve

# Or use the provided script
./run-dev.sh

# Development server runs on: http://localhost:8080
```

### Production Build

```bash
# Build for web (WASM)
dx build --release --platform web

# Build output: target/dx/remind-me-pwa/release/web/

# For GitHub Pages deployment
dx bundle --release --out-dir docs
```

### Mobile Builds

```bash
# iOS (macOS only)
./build-mobile-ios.sh
# Or: dx build --platform ios --release

# Android (requires ANDROID_HOME)
./build-mobile-android.sh
# Or: dx build --platform android --release
```

---

## Testing Instructions

### Run Checks

```bash
# Check code compiles
cargo check

# Check with clippy (linter)
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt --check

# Run all checks before committing
cargo check && cargo clippy --all-targets -- -D warnings && cargo fmt --check
```

### Lighthouse Audit

```bash
# Build and serve
dx build --release --platform web
dx serve

# Then run Lighthouse in Chrome DevTools
# Target: 100% in all categories (Performance, Accessibility, Best Practices, SEO)

# Or use the automated script
./lighthouse-audit.sh
```

### Manual Testing

```bash
# Test on local server
dx serve

# Test on mobile viewports:
# - 375px (iPhone SE)
# - 414px (iPhone Pro Max)
# - 768px (iPad)

# Test keyboard navigation
# Test with screen reader (VoiceOver/NVDA)
```

---

## Code Style

### Rust Code Style

- **Formatting**: Use `cargo fmt` - always format before committing
- **Linting**: Use `cargo clippy` - fix all warnings
- **Edition**: Rust 2021 edition
- **Naming**: Follow Rust conventions (snake_case for functions/variables, PascalCase for types)

### Dioxus Component Patterns

```rust
// Component structure
#[component]
fn ComponentName(
    prop1: String,
    on_action: EventHandler<String>,
) -> Element {
    let mut state = use_signal(|| initial_value);
    
    rsx! {
        div {
            class: "component-class",
            // JSX-like syntax
        }
    }
}
```

### Key Patterns

- **State Management**: Use `use_signal()` for reactive state
- **Event Handlers**: Use `move` closures: `onclick: move |_| { ... }`
- **Props**: Use `EventHandler<T>` for callbacks
- **Conditional Rendering**: `if condition() { rsx! { ... } }`
- **List Rendering**: `for item in items().iter() { ItemComponent { item } }`

### CSS Organization

- **Modular CSS**: Split into 7 files in `assets/css/`
  - `base.css`: Reset, variables, base styles
  - `components.css`: Reusable UI components
  - `app.css`: App-specific styles
  - `landing.css`: Landing page styles
  - `layout.css`: Navbar, footer, menu
  - `utilities.css`: Utility classes
  - `responsive.css`: Media queries

- **CSS Variables**: Defined in `base.css`, use `var(--variable-name)`
- **Touch Targets**: **MANDATORY** - All interactive elements ≥ 48x48px
- **Mobile First**: Design for mobile, enhance for desktop

---

## Architecture Patterns

### Component Organization

Components are organized by purpose in `src/components/`:
- **Pages**: `landing.rs`, `reminder_app.rs`, `legal.rs`
- **Landing Sections**: `hero_section.rs`, `features_section.rs`, etc.
- **Forms**: `forms.rs` (AddReminderForm, EditReminderForm)
- **Cards**: `cards.rs` (ReminderCard)
- **Modals**: `modals.rs` (DeleteConfirmModal)
- **Layouts**: `landing_layout.rs`, `page_template.rs`

### State Management

```rust
// Load from localStorage on mount
let mut reminders = use_signal(|| load_reminders());

// Update state
reminders.set(new_reminders);

// Save to localStorage after changes
save_reminders(&reminders());
```

### Storage Pattern

```rust
// Load function (returns safe default on error)
fn load_reminders() -> Vec<Reminder> {
    // Try localStorage, return Vec::new() on error
}

// Save function (silent failure)
fn save_reminders(reminders: &[Reminder]) {
    // Save to localStorage, ignore errors
}
```

### Internationalization (i18n)

```rust
// Use translation hook
let t = use_t();

// In components
{use_t("app.header.title")}

// Translation files: assets/i18n/en.json, zh-Hans.json, zh-Hant.json
// Always add translations to all 3 language files
```

### Routing

- **Locale-aware URLs**: Routes include locale prefix (e.g., `/en/app`, `/zh-Hans/app`)
- **GitHub Pages Compatible**: Supports both path-based and hash-based routing
- **Automatic Locale Detection**: From URL or browser settings

---

## Design System

### Bell Crab Theme

The application uses a warm, friendly design system with soft rounded corners and a carefully selected color palette.

### Color Palette

- **Primary Color**: `#FA8A59` (Crab Coral) - Primary actions and CTAs
- **Secondary Color**: `#F4D273` (Bell Gold) - Success states and highlights
- **Brand Blue**: `#6A7CED` (Clay Blue) - Links and accents
- **Brand Purple**: `#9E75E9` (Soft Amethyst) - Hover states and gradients
- **Danger Color**: `#FF6B6B` - Errors and destructive actions
- **Text Primary**: `#33334D` (Deep Slate) - Main text
- **Text Secondary**: `#5F5F7A` (Medium Slate) - Secondary text
- **Background**: `#F9F9FC` (Off-White) - Main background
- **Card Background**: `#FFFFFF` - Card surfaces

### Spacing System

Uses an 8px base scale:
- `xs`: 4px
- `sm`: 8px
- `md`: 16px
- `lg`: 24px
- `xl`: 32px
- `2xl`: 48px

### Border Radius Tokens

Soft, rounded corners for a friendly aesthetic:
- `sm`: 12px - Small elements
- `md`: 16px - Default radius
- `lg`: 24px - Large elements
- `xl`: 32px - Extra large elements
- `pill`: 999px - Pill-shaped elements

### Shadow Hierarchy

Three levels of elevation:
- `shadow`: Base shadow for cards
- `shadow-lg`: Elevated shadow for hover states
- `shadow-soft`: Soft shadow with coral tint

### Typography

System font stack for optimal performance:
```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
```

---

## Critical Requirements

### Lighthouse 100% Scores (MANDATORY)

**Zero tolerance rules - any violation causes PR rejection:**

1. **Touch Targets**: ALL interactive elements MUST be ≥ 48x48px (NOT 24px!)
   ```css
   button, .btn, .tab {
       min-width: 48px !important;  /* Lighthouse requirement */
       min-height: 48px !important; /* MANDATORY - not negotiable */
   }
   ```

2. **HTML Lang Attribute**: MUST be set on `<html>` element
   ```rust
   use_effect(move || {
       // Set lang="en" on html element
   });
   ```

3. **Meta Description**: MUST exist for SEO
   ```rust
   // Set meta description in App component
   ```

4. **Source Maps**: MUST be enabled
   ```toml
   [profile.wasm-release]
   debug = true  # Enable source maps
   ```

5. **ARIA Labels**: ALL interactive elements MUST have ARIA labels
   ```rust
   button {
       aria_label: "Descriptive action",
   }
   ```

6. **Semantic HTML**: MUST use semantic elements (`<main>`, `<nav>`, `<header>`, etc.)

7. **Heading Hierarchy**: MUST follow proper structure (h1 → h2 → h3)

### Pre-Commit Checklist

Before EVERY commit:
- [ ] Run `cargo check && cargo clippy --all-targets -- -D warnings && cargo fmt --check`
- [ ] Run Lighthouse audit (must be 100% in all categories)
- [ ] Verify touch targets ≥ 48x48px in DevTools
- [ ] Check `<html lang="en">` in DevTools
- [ ] Check meta description in DevTools
- [ ] Test on mobile viewport (375px)
- [ ] Test keyboard navigation
- [ ] No console errors

---

## File Structure

### Key Files

- `src/app.rs`: Main App component (routing, i18n, providers)
- `src/components/reminder_app.rs`: Main reminder app component
- `src/components/landing.rs`: Landing page orchestrator
- `src/storage.rs`: LocalStorage utilities
- `src/i18n.rs`: Internationalization system
- `src/router.rs`: Routing logic
- `assets/css/app.css`: App-specific styles
- `assets/i18n/*.json`: Translation files (en, zh-Hans, zh-Hant)
- `.cursor/rules/`: Development rules and patterns

### Workspace Structure

This is a Rust workspace with multiple crates:
- `crates/shared`: Shared code (models, utils)
- `crates/ui`: UI component library
- `crates/components`: App-specific components
- `crates/web`: Web-specific code
- `crates/mobile`: Mobile-specific code
- `apps/web`: Web application entry point
- `apps/mobile`: Mobile application entry point

---

## Development Workflow

### Making Changes

1. **Create feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make changes** following code style and patterns

3. **Run checks**:
   ```bash
   cargo check && cargo clippy --all-targets -- -D warnings && cargo fmt
   ```

4. **Test locally**:
   ```bash
   dx serve
   # Test in browser, test on mobile viewport
   ```

5. **Run Lighthouse audit**:
   ```bash
   ./lighthouse-audit.sh
   # Must achieve 100% in all categories
   ```

6. **Commit with clear message**:
   ```bash
   git commit -m "feat: Add dark mode toggle"
   ```

7. **Push and create PR**

### Adding New Features

1. **Follow component patterns** in `.cursor/rules/skills.md`
2. **Add translations** to all 3 language files (`assets/i18n/`)
3. **Update CSS** in appropriate file (`app.css`, `components.css`, etc.)
4. **Ensure accessibility** (ARIA labels, keyboard navigation)
5. **Verify touch targets** ≥ 48x48px
6. **Test on mobile viewport**
7. **Run Lighthouse audit**

### Adding New Components

1. Create component file in `src/components/`
2. Export in `src/components/mod.rs`
3. Follow Dioxus component patterns
4. Add styles to appropriate CSS file
5. Ensure accessibility compliance
6. Add to component documentation

---

## Common Tasks

### Add New Translation Key

1. Add key to `assets/i18n/en.json`
2. Add same key to `assets/i18n/zh-Hans.json`
3. Add same key to `assets/i18n/zh-Hant.json`
4. Use in component: `{use_t("key.path")}`

### Add New View Mode

1. Create view component in `src/components/`
2. Add view state in `reminder_app.rs`
3. Add view switcher button
4. Add styles to `app.css`
5. Ensure mobile responsiveness

### Fix Lighthouse Issues

1. Run Lighthouse audit: `./lighthouse-audit.sh`
2. Identify issues from report
3. Fix according to `.cursor/rules/core/lighthouse-100-standards.mdc`
4. Re-run audit until 100% in all categories

### Optimize Performance

1. Check animation performance (use `transform` and `opacity`)
2. Verify no layout shifts
3. Optimize re-renders (use signals correctly)
4. Consider virtualization for large lists (100+ items)

---

## Deployment

### GitHub Pages (Automatic)

The project uses GitHub Actions for automatic deployment:
- Workflow: `.github/workflows/github-pages-deploy.yml`
- Triggers on push to `main` branch
- Builds and deploys to `gh-pages` branch

### Manual Deployment

```bash
# Build for production
dx bundle --release --out-dir docs

# Move files to docs root (if needed)
if [ -d "docs/public" ]; then
    mv docs/public/* docs/
    rmdir docs/public
fi

# Create 404.html for client-side routing
cp docs/index.html docs/404.html

# Commit and push
git add docs/
git commit -m "Deploy to GitHub Pages"
git push origin main
```

---

## Troubleshooting

### Build Issues

```bash
# Clean build
cargo clean
dx clean

# Rebuild
cargo build --release
dx build --release --platform web
```

### WASM Size Issues

```bash
# Optimize WASM
wasm-opt -Oz target/dx/remind-me-pwa/release/web/*.wasm -o optimized.wasm
```

### LocalStorage Issues

- Check browser console for errors
- Verify storage quota not exceeded
- Test in incognito mode (fresh state)

### CSS Not Loading

- Verify CSS paths match `base_path` in `Dioxus.toml`
- Check service worker cache URLs
- Clear browser cache

---

## Known Issues

### Build Issues with Dioxus 0.7.0-alpha.3

**Issue**: Compilation errors with `Context` and `SuspenseExtension`
```
error[E0432]: unresolved imports `dioxus_core::Context`, `dioxus_core::SuspenseExtension`
```

**Solution**: 
```bash
# Clean and rebuild
cargo clean
cargo update
cargo build
```

### Current Project State (as of 2026-01-15)

- **Design System**: Fully implemented Bell Crab theme
- **Touch Targets**: Updated to 48px minimum (was 24px)
- **Accessibility**: 
  - ✅ ARIA labels on all interactive elements
  - ✅ Skip navigation link implemented
  - ✅ HTML lang attribute automatically set
  - ✅ Meta descriptions dynamically updated
- **Performance**: 
  - ✅ Animations use transform/opacity only
  - ✅ will-change properties added
  - ✅ 60fps animations with cubic-bezier easing
- **Responsive**: Comprehensive mobile styles for all breakpoints

---

## Related Documentation

This project uses a multi-layered documentation approach:

1. **AGENTS.md** (this file): Universal instructions for all AI agents
   - Setup, build, test, deploy commands
   - Quick reference for common tasks
   - Project overview

2. **.cursor/rules/**: Detailed Cursor-specific rules and patterns
   - **Core rules**: Project structure, code style, best practices, Lighthouse standards
   - **Feature rules**: i18n, routing, storage, media caching, page templates, etc.
   - **Always applied** when using Cursor AI

3. **.cursor/skills/**: Specialized on-demand expertise
   - Activated automatically based on context
   - Deep domain knowledge (Dioxus, PWA, Lighthouse, WASM optimization)

**For detailed patterns and implementation guides**, see:
- `.cursor/rules/core/cursor-rules-summary.mdc` - Quick reference and rule index
- `.cursor/rules/skills.md` - Master skills document (Dioxus PWA development)
- `.cursor/rules/features/` - Feature-specific patterns
- `.cursor/skills/` - Specialized agent skills (Anthropic format)

## Resources

### Documentation

- **Project Rules**: `.cursor/rules/` - Comprehensive development rules
- **Skills Reference**: `.cursor/rules/skills.md` - Dioxus PWA development guide
- **Lighthouse Standards**: `.cursor/rules/core/lighthouse-100-standards.mdc`
- **Code Standards**: `.cursor/rules/core/code-standards-100.mdc`
- **Structure Analysis**: `.cursor/STRUCTURE_ANALYSIS.md` - Documentation structure overview

### External Resources

- **Dioxus Docs**: https://dioxuslabs.com/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Lighthouse**: https://developer.chrome.com/docs/lighthouse/
- **AGENTS.md Format**: https://agents.md/

---

## Important Notes

1. **Always maintain 100% Lighthouse scores** - this is non-negotiable
2. **All touch targets must be ≥ 48x48px** - verify in DevTools
3. **Add translations to all 3 language files** when adding new text
4. **Test on mobile viewport** before committing
5. **Follow Rust best practices** - use `Result` for errors, avoid `unwrap()` in production
6. **Use signals correctly** - initialize from storage, save after changes
7. **Maintain accessibility** - ARIA labels, keyboard navigation, screen reader support

---

## PR Instructions

### Title Format

```
feat: Add dark mode toggle
fix: Fix touch target size on mobile
refactor: Improve component structure
docs: Update README
```

### PR Checklist

- [ ] Code compiles (`cargo check`)
- [ ] No clippy warnings (`cargo clippy --all-targets -- -D warnings`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Lighthouse audit passes (100% in all categories)
- [ ] Touch targets verified (≥ 48x48px)
- [ ] Tested on mobile viewport (375px)
- [ ] Keyboard navigation works
- [ ] Translations added to all 3 language files (if applicable)
- [ ] No console errors
- [ ] CHANGELOG.md updated (if applicable)

---

**Last Updated**: 2026-01-14  
**Project Version**: 0.0.1  
**Dioxus Version**: 0.7
