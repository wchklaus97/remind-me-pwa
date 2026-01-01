# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## 📖 Changelog Guide

### Purpose

The CHANGELOG.md file serves as a historical record of all changes made to the project. It helps:

- **Users**: Understand what changed in each version
- **Developers**: Track project evolution and decisions
- **Contributors**: See what's been done and what's planned

### Structure

Each version entry follows this structure:

```markdown
## [Version] - YYYY-MM-DD

### Added
- New features and functionality

### Changed
- Changes to existing features

### Fixed
- Bug fixes

### Documentation
- Documentation updates
```

### Version Format

- **Format**: `[MAJOR.MINOR.PATCH]` following [Semantic Versioning](https://semver.org/)
- **Examples**: `[0.1.0]`, `[0.2.0]`, `[1.0.0]`
- **Breaking Changes**: Increment MAJOR version (e.g., `1.0.0` → `2.0.0`)
- **New Features**: Increment MINOR version (e.g., `0.1.0` → `0.2.0`)
- **Bug Fixes**: Increment PATCH version (e.g., `0.1.0` → `0.1.1`)

### Date Format

- **Format**: ISO 8601 date format `YYYY-MM-DD`
- **Example**: `2025-01-15`
- **Consistency**: Always use the same date format throughout

### Change Categories

#### Added

Use for new features, functionality, or capabilities:

```markdown
### Added
- **Feature Name**
  - Description of what was added
  - Any important details or context
```

#### Changed

Use for changes to existing functionality:

```markdown
### Changed
- **Feature Name** (YYYY-MM-DD)
  - What changed and why
  - Impact on users (if any)
```

#### Fixed

Use for bug fixes:

```markdown
### Fixed
- **Issue Description** (YYYY-MM-DD)
  - What was fixed
  - How it was fixed (if relevant)
```

#### Documentation

Use for documentation updates:

```markdown
### Documentation
- **Document Name**
  - What was updated
  - Why it was updated
```

### Writing Guidelines

#### DO

- ✅ Use clear, descriptive language
- ✅ Group related changes together
- ✅ Include dates for significant changes
- ✅ Explain the "why" for major changes
- ✅ Use bullet points for readability
- ✅ Mark breaking changes clearly
- ✅ Include migration notes if needed

#### DON'T

- ❌ Don't use vague descriptions
- ❌ Don't mix different change types in one entry
- ❌ Don't forget to include dates
- ❌ Don't skip version numbers
- ❌ Don't use technical jargon without explanation
- ❌ Don't include internal-only changes (unless relevant)

### Display Format

When displaying the changelog:

1. **Latest Version First**: Most recent changes appear at the top
2. **Unreleased Section**: Planned features appear in `[Unreleased]`
3. **Version Links**: Link to GitHub releases (if applicable)
4. **Grouping**: Changes grouped by type (Added, Changed, Fixed)
5. **Dates**: Visible and consistent format

### Example Entry

```markdown
## [0.2.0] - 2025-02-01

### Added
- **Search Functionality**
  - Real-time search by title and description
  - Search history with recent searches
  - Keyboard shortcut (Ctrl/Cmd + K)
  - Estimated: 1-2 days development

### Changed
- **Storage Implementation** (2025-02-01)
  - Migrated from localStorage to IndexedDB
  - Improved performance for large datasets (> 1000 reminders)
  - Added automatic data migration script

### Fixed
- **Search Performance** (2025-02-01)
  - Fixed slow search with large reminder lists
  - Optimized search algorithm
  - Reduced search latency by 80%

### Documentation
- Updated DEVELOPMENT_PLAN.md with completed features
- Added search functionality to user guide
```

### Unreleased Section

The `[Unreleased]` section tracks planned features:

```markdown
## [Unreleased]

### Planned
- [ ] Feature name
- [ ] Another feature
- [ ] Bug fix to implement
```

### Breaking Changes

Mark breaking changes clearly:

```markdown
## [2.0.0] - 2025-03-01

### Changed (Breaking)
- **Storage API** (2025-03-01)
  - Changed storage key format from `reminders` to `reminders_v2`
  - **Migration Required**: Run migration script before upgrading
  - Old data format no longer supported
```

### Links and References

Link to related resources:

```markdown
### Added
- **New Feature**
  - Description
  - See [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for details
  - Related issue: #123
```

### Maintenance

- **Update Frequency**: Update with every release
- **Review**: Review before each version release
- **Accuracy**: Ensure all changes are documented
- **Completeness**: Don't skip minor changes

### Resources

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) - Standard format
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html) - Version numbering
- [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) - Date format standard
- [.cursor/rules/features/changelog-management.mdc](./.cursor/rules/features/changelog-management.mdc) - Management rules

---

## [Unreleased]

### Planned

- [ ] Add reminder categories/tags
- [ ] Add reminder search functionality
- [ ] Add reminder sorting options
- [ ] Add reminder export/import
- [ ] Add dark mode support
- [ ] Add reminder notifications
- [ ] Migrate to IndexedDB for larger datasets

---

## [0.1.0] - 2025-01-15

### Added

- **Initial PWA Setup**
  - Dioxus 0.6 framework integration
  - WASM compilation target (`wasm32-unknown-unknown`)
  - PWA manifest configuration
  - Service worker for offline support
  - GitHub Pages deployment setup

- **Core Features**
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

- **Storage Implementation**
  - LocalStorage integration
  - JSON serialization with serde
  - Error handling with graceful fallbacks
  - Safe default values on load errors

- **Build Configuration**
  - Optimized Cargo.toml profiles (dev, wasm-dev, wasm-release)
  - Thin LTO for faster builds
  - Size optimization (opt-level = "z")
  - Incremental compilation for dev builds

- **CI/CD Pipeline**
  - GitHub Actions workflow for automated deployment
  - Rust toolchain caching
  - Cargo registry caching
  - Dioxus CLI installation with cargo-binstall
  - Automated GitHub Pages deployment

- **Documentation**
  - Comprehensive Cursor Rules structure
  - Agent Skills following Agent Skills Specification
  - Rust best practices documentation
  - Storage patterns and comparison
  - Testing patterns
  - Deployment guide
  - Error handling patterns

### Changed

- **Build Optimization** (2025-01-15)
  - Changed LTO from `true` to `"thin"` for WASM compatibility
  - Disabled `strip` for WASM builds
  - Added `wasm-release` profile inheriting from `release`
  - Optimized `wasm-dev` profile with `codegen-units = 16`

- **CI/CD Improvements** (2025-01-15)
  - Unified caching strategy
  - Conditional tool installation
  - Robust build output detection
  - Improved error handling in build steps

- **Storage Patterns** (2025-01-15)
  - Documented localStorage patterns
  - Added IndexedDB comparison from Flutter PWA project
  - Created storage service patterns for future migration

### Fixed

- **GitHub Pages Deployment** (2025-01-15)
  - Fixed `base_path` configuration (added leading slash)
  - Fixed 404.html for client-side routing
  - Fixed file copying to include all subdirectories
  - Fixed build output directory detection

- **Build Issues** (2025-01-15)
  - Fixed `dx build` exit code handling in CI
  - Fixed Dioxus CLI version mismatch (pinned to 0.6.3)
  - Fixed WASM strip configuration
  - Fixed invalid GitHub Actions expressions

### Documentation

- **Cursor Rules** (2025-01-15)
  - Created comprehensive rule structure
  - Added project structure guidelines
  - Added code formatting standards
  - Added Rust best practices
  - Added storage patterns
  - Added PWA development patterns
  - Added testing patterns
  - Added deployment patterns
  - Added error handling patterns

- **Agent Skills** (2025-01-15)
  - Created `dioxus-pwa-development` skill
  - Created `rust-wasm-optimization` skill
  - Created `pwa-storage-patterns` skill
  - Created `dioxus-component-patterns` skill
  - All skills follow Agent Skills Specification

- **Learning Documentation** (2025-01-15)
  - Added storage comparison with Flutter PWA project
  - Documented IndexedDB patterns for future use
  - Added migration strategies

---

## Development Timeline

### 2025-01-15

- ✅ Initial project setup
- ✅ Core reminder functionality
- ✅ LocalStorage integration
- ✅ PWA configuration
- ✅ GitHub Pages deployment
- ✅ Cursor Rules structure
- ✅ Agent Skills creation
- ✅ Build optimization
- ✅ CI/CD pipeline setup
- ✅ Documentation completion

---

## Version History

- **0.1.0** (2025-01-15): Initial release with core features

## Notes

- All dates are in ISO 8601 format (YYYY-MM-DD)
- Changes are grouped by type: Added, Changed, Fixed, Documentation
- Planned features are listed under [Unreleased] section
- Breaking changes will be clearly marked
