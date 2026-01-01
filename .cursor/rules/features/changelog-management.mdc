---
alwaysApply: false
description: Changelog and development plan management patterns
---
# Changelog and Development Plan Management

## 📋 Overview

This project maintains two key documents for tracking changes and planning:

1. **CHANGELOG.md**: Detailed history of all changes with dates
2. **DEVELOPMENT_PLAN.md**: Roadmap, planned features, and timeline

## Changelog Format

### Structure

Follow [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:

```markdown
## [Version] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing features

### Fixed
- Bug fixes

### Documentation
- Documentation updates
```

### Date Format

- Use ISO 8601 format: `YYYY-MM-DD`
- Example: `2025-01-15`

### Version Format

- Follow [Semantic Versioning](https://semver.org/)
- Format: `MAJOR.MINOR.PATCH`
- Example: `0.1.0`, `0.2.0`, `1.0.0`

## Development Plan Format

### Structure

```markdown
## Phase X: Name (Status: Planned/In Progress/Completed)

### Features
- [ ] Feature name (Priority: High/Medium/Low)
  - Description
  - Estimated: X days
```

### Status Indicators

- `✅ Complete` - Feature is done
- `🔄 In Progress` - Currently being developed
- `📋 Planned` - Planned for future
- `❌ Cancelled` - No longer planned

### Priority Levels

- **High**: Next sprint, critical features
- **Medium**: Next quarter, important features
- **Low**: Future, nice-to-have features

## Update Workflow

### When Adding a Feature

1. **Update DEVELOPMENT_PLAN.md**
   ```markdown
   - [ ] Feature name (Priority: Medium)
     - Description
     - Estimated: 2 days
   ```

2. **Develop the feature**
   - Follow Cursor Rules
   - Write tests
   - Update documentation
   - Commit with descriptive message

3. **Get commit hash**
   ```bash
   git log --oneline -1
   # Output: abc1234 Add feature name
   ```

4. **Update CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2025-02-01
   
   ### Added
   - **Feature name** ([abc1234](https://github.com/user/repo/commit/abc1234))
     - Description of what was added
   ```

5. **Update DEVELOPMENT_PLAN.md**
   ```markdown
   - [x] Feature name (Priority: Medium) ✅
   ```

### When Fixing a Bug

1. **Fix the bug**
   - Commit with descriptive message
   - Get commit hash: `git log --oneline -1`

2. **Update CHANGELOG.md**
   ```markdown
   ### Fixed
   - **Bug description** ([abc1234](https://github.com/user/repo/commit/abc1234))
     - What was fixed
   ```

### When Changing Existing Features

1. **Make the change**
2. **Update CHANGELOG.md**
   ```markdown
   ### Changed
   - Feature name
     - What changed and why
   ```

## Best Practices

### DO:
- ✅ Update CHANGELOG.md for every release
- ✅ Use clear, descriptive change descriptions
- ✅ Group changes by type (Added, Changed, Fixed)
- ✅ Include dates for all changes
- ✅ Update DEVELOPMENT_PLAN.md when planning features
- ✅ Mark completed features in DEVELOPMENT_PLAN.md
- ✅ Estimate effort for planned features
- ✅ Update timeline when priorities change

### DON'T:
- ❌ Don't forget to update CHANGELOG.md
- ❌ Don't use vague descriptions
- ❌ Don't skip dates
- ❌ Don't leave DEVELOPMENT_PLAN.md outdated
- ❌ Don't forget to mark completed features
- ❌ Don't mix different change types in one entry

## Changelog Entry Examples

### Added Feature
```markdown
### Added
- **Search Functionality**
  - Real-time search by title and description
  - Search history with recent searches
  - Keyboard shortcut (Ctrl/Cmd + K)
```

### Changed Feature
```markdown
### Changed
- **Storage Implementation** (2025-01-15)
  - Migrated from localStorage to IndexedDB
  - Improved performance for large datasets
  - Added data migration script
```

### Fixed Bug
```markdown
### Fixed
- **GitHub Pages Deployment** (2025-01-15)
  - Fixed `base_path` configuration (added leading slash)
  - Fixed 404.html for client-side routing
  - Fixed file copying to include all subdirectories
```

## Development Plan Entry Examples

### Planned Feature
```markdown
- [ ] **Reminder Categories** (Priority: Medium)
  - Add category field to Reminder struct
  - Category filtering UI
  - Color coding for categories
  - Estimated: 2-3 days
```

### In Progress Feature
```markdown
- [🔄] **Search Functionality** (Priority: High)
  - Search by title, description
  - Real-time search filtering
  - Estimated: 1-2 days (1 day remaining)
```

### Completed Feature
```markdown
- [x] **LocalStorage Integration** (Priority: High) ✅
  - JSON serialization with serde
  - Error handling with graceful fallbacks
  - Completed: 2025-01-15
```

## Version Release Process

### Pre-Release

1. Update CHANGELOG.md with all changes
2. Update version in Cargo.toml
3. Create git tag: `git tag -a v0.2.0 -m "Release v0.2.0"`
4. Push tag: `git push origin v0.2.0`

### Post-Release

1. Update DEVELOPMENT_PLAN.md
2. Mark completed features
3. Update timeline if needed
4. Create GitHub release with CHANGELOG notes

## Resources

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- [ISO 8601 Date Format](https://en.wikipedia.org/wiki/ISO_8601)
