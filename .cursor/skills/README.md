# Agent Skills Index

This directory contains specialized Agent Skills that extend Cursor AI's capabilities with domain-specific knowledge for the Dioxus PWA project. These skills follow Anthropic's [Agent Skills Specification](https://github.com/anthropics/skills/blob/main/agent_skills_spec.md).

## 📁 Skills List

| Skill Name | Description | Use When |
|------------|-------------|----------|
| **dioxus-pwa-development** | Build Progressive Web Apps with Dioxus framework | Developing Dioxus web applications, creating PWA features, or working with WASM targets |
| **rust-wasm-optimization** | Optimize Rust code for WebAssembly compilation and runtime performance | Building WASM applications, optimizing binary size, or improving WASM runtime performance |
| **pwa-storage-patterns** | Implement data persistence patterns for Progressive Web Apps | Working with browser storage APIs, implementing localStorage or IndexedDB, or managing client-side data persistence |
| **dioxus-component-patterns** | Master Dioxus component patterns and reactive state management | Building Dioxus components, managing component state, or implementing component interactions |
| **changelog-management** | Manage project changelogs, track changes, and prepare version releases | Updating CHANGELOG.md, tracking project evolution, or preparing for a new release |
| **lighthouse-optimization** | Optimize web applications to achieve 95%+ Lighthouse scores | Optimizing performance, accessibility, best practices, or SEO for web applications |

## 💡 How Skills Work

Skills are automatically activated when Cursor AI detects matching patterns in your request. They provide deep expertise in specific domains without loading everything into context upfront, ensuring token efficiency and specialized guidance.

## 🔗 Related Documentation

- [Agent Skills Specification](https://github.com/anthropics/skills/blob/main/agent_skills_spec.md)
- [.cursor/rules/README.md](./../rules/README.md) - Project-specific Cursor Rules

