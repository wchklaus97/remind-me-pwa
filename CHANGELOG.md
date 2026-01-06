# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

For detailed guidelines on writing changelog entries, see [.cursor/changelog-guide.md](.cursor/changelog-guide.md).

---

## [Unreleased]

### Added

- **App Page Improvements - Stage 1: Mobile Responsive Fixes** ([ea66ac1](https://github.com/wchklaus97/remind-me-pwa/commit/ea66ac1))
  - Comprehensive mobile responsive fixes for app page
  - Fixed width overflow issues on mobile devices
  - Improved touch target sizes (≥48x48px) for accessibility
  - Responsive layouts for app-container, app-header, stats-grid, controls, filter-tabs, reminder-card
  - Mobile-friendly view switcher, card-grid, folder-group, and calendar-grid
  - Enhanced form layouts for mobile screens

- **App Page Improvements - Stage 2: UI/UX Visual Enhancements** ([533de4b](https://github.com/wchklaus97/remind-me-pwa/commit/533de4b))
  - Modern visual design improvements across app page
  - Enhanced app-header with glow effects and improved typography
  - Improved statistics cards with hover gradients and shadows
  - Pill-shaped filter tabs and view switcher with hover effects
  - Gradient borders on reminder cards with enhanced hover states
  - Improved form styling with gradient borders and focus states
  - Better visual consistency with landing page design

- **App Page Improvements - Stage 3: Code Organization and Refactoring** ([0fb6176](https://github.com/wchklaus97/remind-me-pwa/commit/0fb6176), [e0981d9](https://github.com/wchklaus97/remind-me-pwa/commit/e0981d9), [e84e4e1](https://github.com/wchklaus97/remind-me-pwa/commit/e84e4e1))
  - Type safety improvements with ReminderFilter and ReminderSort enums
  - Replaced string-based filter/sort with strongly-typed enums
  - Added Display and FromStr trait implementations for enums
  - Improved compile-time type checking and code clarity
  - Better error handling with Default trait implementations

- **App Page Improvements - Stage 4: Accessibility and Performance** ([5ea98b0](https://github.com/wchklaus97/remind-me-pwa/commit/5ea98b0), [5360be1](https://github.com/wchklaus97/remind-me-pwa/commit/5360be1))
  - Comprehensive ARIA labels for all interactive elements
  - Focus indicators (focus-visible) for keyboard navigation
  - Enhanced error handling with non-blocking log::error! calls
  - Extended Button, Input, Select, Checkbox, Textarea components with aria_label support
  - Performance documentation and assessment
  - Improved accessibility for screen readers

- **Tag Management System** ([5a78407](https://github.com/wchklaus97/remind-me-pwa/commit/5a78407))
  - TagForm component for creating/editing tags with name and color picker
  - TagManager component with modal interface for tag management
  - Tag selection UI (multi-select checkboxes) in AddReminderForm and EditReminderForm
  - 12 preset colors for tag color picker
  - Tag display on reminder cards with color coding
  - Tag management button in ReminderApp header
  - Full i18n support (en/zh-Hans/zh-Hant) for tag management
  - Responsive design for tag management interface

- **Multi-View Switching** ([f3471fd](https://github.com/wchklaus97/remind-me-pwa/commit/f3471fd))
  - ListView, CardView, and FolderView components
  - View switcher UI in ReminderApp
  - Folder view groups reminders by tags
  - Untagged reminders section in folder view

- **Calendar View** ([f6da14f](https://github.com/wchklaus97/remind-me-pwa/commit/f6da14f))
  - CalendarView component with month grid display
  - Calendar navigation (prev/next month, today button)
  - Reminders grouped by date
  - Unscheduled reminders section
  - Date selection and reminder display

- **Tags System** ([2bc34d0](https://github.com/wchklaus97/remind-me-pwa/commit/2bc34d0))
  - Tag data model with id, name, and color
  - Reminder model extended with tag_ids field
  - Tag storage in localStorage
  - Data migration from v1 to v2 for backward compatibility

### Planned

- [ ] Add reminder search functionality
- [ ] Add reminder sorting options
- [ ] Add reminder export/import
- [ ] Add dark mode support
- [ ] Add reminder notifications
- [ ] Migrate to IndexedDB for larger datasets

---

## [0.0.1] - 2025-01-02

<details>
<summary>Added</summary>

#### Core Features
- **Initial PWA Setup**
  - Dioxus 0.6 framework integration
  - WASM compilation target (`wasm32-unknown-unknown`)
  - PWA manifest configuration
  - Service worker for offline support
  - GitHub Pages deployment setup

- **Reminder Management**
  - Reminder CRUD operations (Create, Read, Update, Delete)
  - Reminder completion toggle
  - Reminder filtering (All, Active, Completed)
  - Due date support with overdue detection
  - LocalStorage persistence

- **UI Components**
  - Main App component
  - AddReminderForm component
  - ReminderCard component
  - Filter tabs
  - Empty state display
  - Responsive design

#### Storage Implementation
- **LocalStorage Integration**
  - JSON serialization with serde
  - Error handling with graceful fallbacks
  - Safe default values on load errors

#### Build Configuration
- **Optimized Profiles**
  - Optimized Cargo.toml profiles (dev, wasm-dev, wasm-release)
  - Thin LTO for faster builds
  - Size optimization (opt-level = "z")
  - Incremental compilation for dev builds

#### CI/CD Pipeline
- **GitHub Actions Workflow**
  - Automated deployment workflow
  - Rust toolchain caching
  - Cargo registry caching
  - Dioxus CLI installation with cargo-binstall
  - Branch-based deployment (gh-pages and release branches)
  - WASM optimization with wasm-opt

#### Documentation
- **Project Documentation**
  - Comprehensive Cursor Rules structure
  - Agent Skills following Agent Skills Specification
  - Rust best practices documentation
  - Storage patterns and comparison
  - Testing patterns
  - Deployment guide
  - Error handling patterns

</details>

<details>
<summary>Changed</summary>

#### Build Optimization
- **Build Configuration** (2025-01-02)
  - Changed LTO from `true` to `"thin"` for WASM compatibility
  - Disabled `strip` for WASM builds
  - Added `wasm-release` profile inheriting from `release`
  - Optimized `wasm-dev` profile with `codegen-units = 16`

#### CI/CD Improvements
- **Deployment Workflow** (2025-01-02)
  - Unified caching strategy
  - Conditional tool installation
  - Robust build output detection
  - Improved error handling in build steps
  - Switched to branch-based deployment (gh-pages and release branches)

#### Storage Patterns
- **Documentation Updates** (2025-01-02)
  - Documented localStorage patterns
  - Added IndexedDB comparison from Flutter PWA project
  - Created storage service patterns for future migration

</details>

<details>
<summary>Fixed</summary>

#### GitHub Pages Deployment
- **Deployment Issues** (2025-01-02)
  - Fixed `base_path` configuration (added leading slash)
  - Fixed 404.html for client-side routing
  - Fixed file copying to include all subdirectories
  - Fixed build output directory detection

#### Build **Issues**
- **Build Configuration** (2025-01-02)
  - Fixed `dx build` exit code handling in CI
  - Fixed Dioxus CLI version mismatch (pinned to 0.6.3)
  - Fixed WASM strip configuration
  - Fixed invalid GitHub Actions expressions

</details>

<details>
<summary>Documentation</summary>

#### Cursor Rules
- **Rule Structure** (2025-01-02)
  - Created comprehensive rule structure
  - Added project structure guidelines
  - Added code formatting standards
  - Added Rust best practices
  - Added storage patterns
  - Added PWA development patterns
  - Added testing patterns
  - Added deployment patterns
  - Added error handling patterns

#### Agent Skills
- **Skill Documentation** (2025-01-02)
  - Created `dioxus-pwa-development` skill
  - Created `rust-wasm-optimization` skill
  - Created `pwa-storage-patterns` skill
  - Created `dioxus-component-patterns` skill
  - All skills follow Agent Skills Specification

#### Learning Documentation
- **Reference Materials** (2025-01-02)
  - Added storage comparison with Flutter PWA project
  - Documented IndexedDB patterns for future use
  - Added migration strategies

</details>

---

## Version History

- **0.0.1** (2025-01-02): Initial release with core features

---

## Notes

- All dates are in ISO 8601 format (YYYY-MM-DD)
- Changes are grouped by type: Added, Changed, Fixed, Documentation
- Nested categories help organize related changes
- Planned features are listed under [Unreleased] section
- Breaking changes will be clearly marked
- For detailed changelog guidelines, see [.cursor/changelog-guide.md](.cursor/changelog-guide.md)
