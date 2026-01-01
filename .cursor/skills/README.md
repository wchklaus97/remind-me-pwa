# Agent Skills for Dioxus PWA Project

This directory contains Agent Skills following the [Agent Skills Specification](https://github.com/anthropics/skills/blob/main/agent_skills_spec.md). These skills provide specialized knowledge for Dioxus PWA development.

## Skills Overview

| Skill | Description | Use When |
|-------|-------------|----------|
| **dioxus-pwa-development** | Build Progressive Web Apps with Dioxus framework | Developing Dioxus web applications, creating PWA features, or working with WASM targets |
| **rust-wasm-optimization** | Optimize Rust code for WebAssembly compilation and runtime performance | Building WASM applications, optimizing binary size, or improving WASM runtime performance |
| **pwa-storage-patterns** | Implement data persistence patterns for Progressive Web Apps | Working with browser storage APIs, implementing localStorage or IndexedDB |
| **dioxus-component-patterns** | Master Dioxus component patterns and reactive state management | Building Dioxus components, managing component state, or implementing component interactions |
| **changelog-management** | Manage project changelogs with commit tracking and structured documentation | Updating CHANGELOG.md, tracking project changes, or documenting version releases |

## Skill Structure

Each skill follows the Agent Skills Specification:

- **YAML Frontmatter**: `name` and `description` with "Use when" clause
- **Progressive Disclosure**: Metadata → Instructions → Resources
- **Activation Triggers**: Clear "Use when" clauses for automatic invocation

## Usage

These skills are automatically activated when Claude detects matching patterns:

```
User: "Create a Dioxus component for reminders"
→ Activates: dioxus-component-patterns, dioxus-pwa-development

User: "Optimize WASM binary size"
→ Activates: rust-wasm-optimization

User: "Implement localStorage for reminders"
→ Activates: pwa-storage-patterns

User: "Update CHANGELOG with new feature"
→ Activates: changelog-management
```

## Skill Files

Each skill is in its own directory:

```
.cursor/skills/
├── dioxus-pwa-development/
│   └── SKILL.md
├── rust-wasm-optimization/
│   └── SKILL.md
├── pwa-storage-patterns/
│   └── SKILL.md
├── dioxus-component-patterns/
│   └── SKILL.md
├── changelog-management/
│   └── SKILL.md
└── README.md
```

## Integration with Cursor Rules

These Agent Skills complement the Cursor Rules:

- **Cursor Rules** (`.cursor/rules/`): Project-specific patterns and conventions
- **Agent Skills** (`.cursor/skills/`): Reusable domain knowledge and best practices

## Creating New Skills

To add a new skill:

1. Create directory: `.cursor/skills/{skill-name}/`
2. Create `SKILL.md` with YAML frontmatter:
   ```yaml
   ---
   name: skill-name
   description: What the skill does. Use when [activation trigger].
   ---
   ```
3. Write comprehensive skill content
4. Update this README.md

## Resources

- [Agent Skills Specification](https://github.com/anthropics/skills/blob/main/agent_skills_spec.md)
- [Anthropic Skills Repository](https://github.com/anthropics/skills)
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.6/)

