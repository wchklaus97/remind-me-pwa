# Refactoring Plan: Modular Structure with Multi-Locale Support

## Overview
Splitting `src/main.rs` (1131 lines) into a clean modular structure with routing and internationalization (i18n) support for English (en) and Chinese (zh).

## New Structure

```
src/
├── main.rs              # Entry point only (~25 lines)
├── app.rs               # Main App component with routing
├── models.rs            # Data models (Reminder, Statistics)
├── router.rs            # Routing logic with locale support
├── storage.rs           # localStorage operations
├── utils.rs             # Utility functions
├── i18n.rs              # i18nrs initialization (JSON-based translations)
└── components/
    ├── mod.rs           # Component module declarations
    ├── landing.rs       # LandingPage component
    ├── reminder_app.rs  # ReminderApp main component
    ├── statistics.rs    # StatisticsDisplay component
    ├── forms.rs         # AddReminderForm, EditReminderForm
    ├── cards.rs         # ReminderCard component
    └── modals.rs        # DeleteConfirmModal component

assets/
└── i18n/
    ├── en.json          # English translations (JSON)
    └── zh.json          # Chinese translations (JSON)
```

## Routing Structure

Routes support locale prefixes:
- `/en/` or `/en/app` - English
- `/zh/` or `/zh/app` - Chinese
- `/` - Default (English)

## Implementation Steps

1. ✅ Create module structure (models, router, storage, utils, i18n)
2. ✅ Create component modules (landing, reminder_app, statistics, forms, cards, modals)
3. ✅ Create app.rs with main App component
4. ✅ Update main.rs to use new modules
5. ✅ Test and fix compilation errors

**Status**: ✅ **COMPLETED** (2025-01-03)

## Benefits

- **Maintainability**: Smaller, focused files
- **Reusability**: Components can be easily reused
- **Scalability**: Easy to add new locales or components
- **Testability**: Modules can be tested independently
- **Organization**: Clear separation of concerns

