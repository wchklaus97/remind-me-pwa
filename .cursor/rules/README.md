# Cursor Rules Index

This directory contains Cursor AI rules that help maintain code quality and consistency across the Dioxus PWA project.

---

## 📁 Directory Structure

```
.cursor/rules/
├── core/                           # Core rules (Project structure, formatting, etc.)
│   ├── project-structure.mdc
│   ├── code-formatting.mdc
│   └── cursor-rules-summary.mdc
├── features/                       # Feature-specific rules (if needed)
├── templates/                      # Code templates (if needed)
├── skills.md                       # Dioxus PWA development skills reference
└── README.md                       # This file
```

---

## 🎯 Core Rules

### 0. `skills.md` ⭐⭐⭐⭐⭐ **Master Reference**

**Scope**: Manual reference

**Purpose**: Complete Dioxus PWA development skills and patterns

**Use**: `@skills.md` when implementing new features or components

---

### 1. `core/project-structure.mdc` ⭐⭐⭐

**Scope**: Always applied

**Purpose**: Directory layout, file organization, naming conventions

**Use**: Automatically applied to maintain project structure

---

### 2. `core/code-formatting.mdc` ⭐⭐⭐

**Scope**: Always applied

**Purpose**: Code style, formatting standards, naming conventions

**Use**: Automatically applied to maintain code style

---

### 3. `core/cursor-rules-summary.mdc` ⭐⭐⭐

**Scope**: Always applied

**Purpose**: High-level overview of rules and project structure

**Use**: Automatically applied as entry point

---

## 📋 All Rules List

| # | Rule File | Auto Apply | Priority | Purpose |
|---|---------|-----------|----------|---------|
| 0 | `skills.md` | ❌ Manual | ⭐⭐⭐⭐⭐ | Dioxus PWA development skills |
| 1 | `core/project-structure.mdc` | ✅ Always | ⭐⭐⭐ | Project structure & organization |
| 2 | `core/code-formatting.mdc` | ✅ Always | ⭐⭐⭐ | Code style & formatting |
| 3 | `core/rust-best-practices.mdc` | ✅ Always | ⭐⭐⭐⭐ | Rust code best practices & patterns |
| 4 | `core/cursor-rules-summary.mdc` | ✅ Always | ⭐⭐⭐ | Rules overview |
| 5 | `features/storage.mdc` | ✅ Always | ⭐⭐⭐⭐ | LocalStorage patterns & persistence |
| 6 | `features/storage-comparison.mdc` | ❌ Manual | ⭐⭐ | Storage approach comparison (Flutter vs Dioxus) |
| 7 | `features/pwa-development.mdc` | ❌ Manual | ⭐⭐⭐ | PWA development patterns |
| 8 | `features/testing.mdc` | ❌ Manual | ⭐⭐⭐ | Testing patterns for Dioxus PWA |
| 9 | `features/deployment.mdc` | ❌ Manual | ⭐⭐⭐ | Deployment patterns for GitHub Pages |
| 10 | `features/error-handling.mdc` | ✅ Always | ⭐⭐⭐⭐ | Error handling patterns |
| 11 | `features/changelog-management.mdc` | ❌ Manual | ⭐⭐ | Changelog and development plan management |

---

## 🛠️ Usage Guide

### For New Components
1. Read `skills.md` → Component Development section
2. Follow patterns in `core/project-structure.mdc`
3. Use formatting from `core/code-formatting.mdc`
4. Apply Rust best practices from `core/rust-best-practices.mdc`

### For State Management
1. Read `skills.md` → State Management section
2. Use `use_signal()` pattern
3. Persist to localStorage when needed
4. Follow storage patterns in `features/storage.mdc`

### For PWA Features
1. Read `skills.md` → PWA Configuration section
2. Update `assets/manifest.json` if needed
3. Update `assets/sw.js` for caching

---

## 📚 Related Documentation

- **Skills Reference**: [skills.md](./skills.md)
- **Project Structure**: [core/project-structure.mdc](./core/project-structure.mdc)
- **Code Formatting**: [core/code-formatting.mdc](./core/code-formatting.mdc)

---

**Updated**: 2025-01-15  
**Version**: 1.0  
**Dioxus Version**: 0.6

