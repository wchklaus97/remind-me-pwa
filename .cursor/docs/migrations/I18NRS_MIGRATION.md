# Migration to i18nrs

## Overview

We're migrating from a custom i18n implementation to the [i18nrs](https://docs.rs/i18nrs/latest/i18nrs/) library, which provides better features and maintenance.

## Benefits of i18nrs

- ✅ JSON-based translations (easier to manage)
- ✅ Dynamic language switching
- ✅ LocalStorage persistence
- ✅ Nested key support (e.g., `app.header.title`)
- ✅ Auto RTL/LTR switching
- ✅ Better maintained and tested

## Migration Steps

1. ✅ Add i18nrs dependency to Cargo.toml
2. ✅ Create JSON translation files in `assets/i18n/`
3. ✅ Initialize i18nrs in `src/i18n.rs`
4. ✅ Update router to use string locales ("en", "zh")
5. ✅ Update all components to use i18nrs API
6. ✅ Remove custom locale system files

## Current Status

**Status**: ✅ **COMPLETED** (2025-01-03)

- ✅ JSON files created: `assets/i18n/en.json`, `assets/i18n/zh.json`
- ✅ Dependency added to Cargo.toml
- ✅ i18nrs initialized in `src/i18n.rs`
- ✅ All components using i18nrs API
- ✅ Router using string locales

## Notes

- i18nrs uses string locales ("en", "zh") instead of enums
- Translations are loaded from JSON files
- The library handles language persistence automatically

