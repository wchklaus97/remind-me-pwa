# Changelog Guide

## Purpose

The CHANGELOG.md file serves as a historical record of all changes made to the project. It helps:

- **Users**: Understand what changed in each version
- **Developers**: Track project evolution and decisions
- **Contributors**: See what's been done and what's planned

## Structure

Each version entry follows this nested structure:

```markdown
## [Version] - YYYY-MM-DD

<details>
<summary>Added</summary>

#### Category Name
- **Feature Name**
  - Description of what was added
  - Any important details or context

</details>

<details>
<summary>Changed</summary>

#### Category Name
- **Feature Name** (YYYY-MM-DD)
  - What changed and why
  - Impact on users (if any)

</details>

<details>
<summary>Fixed</summary>

#### Category Name
- **Issue Description** (YYYY-MM-DD)
  - What was fixed
  - How it was fixed (if relevant)

</details>

<details>
<summary>Documentation</summary>

#### Category Name
- **Document Name**
  - What was updated
  - Why it was updated

</details>
```

## Version Format

- **Format**: `[MAJOR.MINOR.PATCH]` following [Semantic Versioning](https://semver.org/)
- **Examples**: `[0.0.1]`, `[0.1.0]`, `[1.0.0]`
- **Breaking Changes**: Increment MAJOR version (e.g., `1.0.0` → `2.0.0`)
- **New Features**: Increment MINOR version (e.g., `0.0.1` → `0.1.0`)
- **Bug Fixes**: Increment PATCH version (e.g., `0.0.1` → `0.0.2`)

## Date Format

- **Format**: ISO 8601 date format `YYYY-MM-DD`
- **Example**: `2025-01-15`
- **Consistency**: Always use the same date format throughout

## Change Categories

### Added

Use for new features, functionality, or capabilities with nested categories:

```markdown
<details>
<summary>Added</summary>

#### Core Features
- **Feature Name**
  - Description of what was added
  - Any important details or context

</details>
```

### Changed

Use for changes to existing functionality:

```markdown
<details>
<summary>Changed</summary>

#### Category Name
- **Feature Name** (YYYY-MM-DD)
  - What changed and why
  - Impact on users (if any)

</details>
```

### Fixed

Use for bug fixes:

```markdown
<details>
<summary>Fixed</summary>

#### Category Name
- **Issue Description** (YYYY-MM-DD)
  - What was fixed
  - How it was fixed (if relevant)

</details>
```

### Documentation

Use for documentation updates:

```markdown
<details>
<summary>Documentation</summary>

#### Category Name
- **Document Name**
  - What was updated
  - Why it was updated

</details>
```

## Writing Guidelines

### DO

- ✅ Use clear, descriptive language
- ✅ Group related changes together using nested categories
- ✅ Include dates for significant changes
- ✅ Explain the "why" for major changes
- ✅ Use bullet points for readability
- ✅ Mark breaking changes clearly
- ✅ Include migration notes if needed
- ✅ Use `<details>` and `<summary>` tags for expandable sections

### DON'T

- ❌ Don't use vague descriptions
- ❌ Don't mix different change types in one entry
- ❌ Don't forget to include dates
- ❌ Don't skip version numbers
- ❌ Don't use technical jargon without explanation
- ❌ Don't include internal-only changes (unless relevant)

## Display Format

When displaying the changelog:

1. **Latest Version First**: Most recent changes appear at the top
2. **Unreleased Section**: Planned features appear in `[Unreleased]`
3. **Version Links**: Link to GitHub releases (if applicable)
4. **Nested Grouping**: Changes grouped by type (Added, Changed, Fixed) and then by category
5. **Expandable Sections**: Use HTML `<details>` tags for collapsible content
6. **Dates**: Visible and consistent format

## Resources

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) - Standard format
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html) - Version numbering
- [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) - Date format standard
- [MDN: HTML Details Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details) - Expandable sections

