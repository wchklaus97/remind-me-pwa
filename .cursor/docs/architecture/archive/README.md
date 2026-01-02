# Archived Architecture Documents

This directory contains architecture documents that describe patterns **not currently implemented** in the project.

## 📁 Contents

### BLoC Architecture Documents

These documents describe a BLoC (Business Logic Component) + Repository Pattern architecture that was **planned but not implemented**.

**Current Reality**: 
- Project uses simple Dioxus Signals for state management
- Project uses direct localStorage functions (no Repository pattern)
- No BLoC layer exists

**Files**:
- `ARCHITECTURE_DECISION.md` - Decision to use BLoC pattern
- `ARCHITECTURE_OVERVIEW.md` - Overview of BLoC architecture
- `BLOC_ARCHITECTURE_PLAN.md` - Detailed implementation plan

**Why Archived**: 
These documents describe a different architecture than what's actually implemented. They're kept here for reference if BLoC pattern is ever considered in the future.

## 🔄 Current Architecture

The project currently uses:
- **State Management**: Dioxus Signals (`use_signal`)
- **Storage**: Direct localStorage functions (`src/storage.rs`)
- **Components**: Simple Dioxus components (`src/components/`)
- **i18n**: i18nrs library with JSON translations (`src/i18n.rs` + `assets/i18n/*.json`)

See `.cursor/docs/DEAD_CODE_ANALYSIS.md` for more details.

---

**Archived Date**: 2025-01-03

