# Cursor AI Configuration

This directory contains Cursor AI-specific configuration, rules, and skills for the Remind Me PWA project.

## 📁 Structure

### `.cursor/rules/`
**Purpose**: Always-applied rules and patterns for Cursor AI

- **`core/`**: Foundational rules
  - Project structure, code formatting, Rust best practices
  - Lighthouse 100% standards (mandatory requirements)
  - Code standards enforcement
  
- **`features/`**: Feature-specific patterns
  - i18n (internationalization), routing, storage
  - Media caching, page templates, PWA development
  - Error handling, testing, deployment
  
- **`skills.md`**: Master reference for Dioxus PWA development
  - Comprehensive guide to all development patterns
  - Component patterns, state management, storage integration

**How it works**: Rules with `alwaysApply: true` are automatically loaded by Cursor. Other rules are contextually applied.

### `.cursor/skills/`
**Purpose**: Specialized on-demand expertise (Anthropic Skills format)

- Activated automatically when Cursor detects relevant context
- Provides deep domain knowledge without loading everything upfront
- Follows [Anthropic Skills Specification](https://github.com/anthropics/skills)

**Available Skills**:
- `dioxus-pwa-development/` - Build Progressive Web Apps with Dioxus
- `dioxus-component-patterns/` - Master Dioxus component patterns
- `pwa-storage-patterns/` - Implement data persistence patterns
- `lighthouse-optimization/` - Optimize for 95%+ Lighthouse scores
- `rust-wasm-optimization/` - Optimize Rust for WebAssembly
- `changelog-management/` - Manage project changelogs

## 🔗 Relationship to Other Documentation

### AGENTS.md (Project Root)
**Universal instructions for ALL AI agents** (OpenCode, Cursor, Aider, Gemini CLI, etc.)

- **Focus**: Setup, build, test, deploy commands
- **Format**: AGENTS.md standard (https://agents.md/)
- **Audience**: All AI coding agents

### .cursor/rules/ (This Directory)
**Cursor-specific detailed patterns and rules**

- **Focus**: How to code, what patterns to use, architectural decisions
- **Format**: Markdown with frontmatter metadata
- **Audience**: Cursor AI specifically

### .cursor/skills/ (This Directory)
**On-demand specialized expertise**

- **Focus**: Deep domain knowledge activated by context
- **Format**: Anthropic Skills Specification
- **Audience**: Cursor AI (auto-activated)

## 🎯 How They Work Together

1. **AGENTS.md** provides universal setup and workflow instructions
2. **.cursor/rules/** provides detailed coding patterns and standards
3. **.cursor/skills/** provides specialized expertise when needed

**Example Flow**:
1. Agent reads `AGENTS.md` → learns how to build and test
2. Agent reads `.cursor/rules/core/code-standards-100.mdc` → learns Lighthouse requirements
3. Agent activates `lighthouse-optimization/` skill → gets deep optimization knowledge

## 📚 Quick Reference

### For New Components
- Read: `.cursor/rules/skills.md` → Component Development section
- Check: `.cursor/rules/core/project-structure.mdc` → File organization

### For State Management
- Read: `.cursor/rules/skills.md` → State Management section
- Check: `.cursor/rules/features/storage.mdc` → Storage patterns

### For Internationalization
- Read: `.cursor/rules/features/i18n.mdc` → i18n patterns
- Use: `use_t("key.path")` hook

### For Lighthouse Optimization
- Read: `.cursor/rules/core/lighthouse-100-standards.mdc` → Requirements
- Check: `.cursor/rules/features/lighthouse-optimization.mdc` → Optimization guide
- Skill: `.cursor/skills/lighthouse-optimization/` → Deep expertise

## 🚨 Critical Requirements

All code must maintain:
- ✅ **100% Lighthouse scores** (Performance, Accessibility, Best Practices, SEO)
- ✅ **Touch targets ≥ 48x48px** (mandatory)
- ✅ **ARIA labels** on all interactive elements
- ✅ **Semantic HTML** structure
- ✅ **Dioxus 0.7** patterns (not 0.6)

See `.cursor/rules/core/code-standards-100.mdc` for complete requirements.

## 📖 Documentation Files

- **`rules/README.md`**: Detailed rules documentation
- **`skills/README.md`**: Skills index and usage guide

## 🔄 Maintenance

When updating documentation:
1. **Version consistency**: Ensure Dioxus version matches across all files (currently 0.7)
2. **Cross-references**: Update links when files move or change
3. **Rule priority**: Keep `alwaysApply: true` only on essential rules
4. **Skills**: Keep skills focused and non-overlapping with rules

---

**Last Updated**: 2026-01-14  
**Dioxus Version**: 0.7  
**Project Version**: 0.0.1
