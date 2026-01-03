---
alwaysApply: false
description: Internationalization (i18n) patterns for multi-language support
---

# Internationalization (i18n) Patterns

## 🎯 Overview

The project supports **3 languages**:
- **English** (`en`) - Default language
- **Simplified Chinese** (`zh-Hans`) - 简体中文
- **Traditional Chinese** (`zh-Hant`) - 繁體中文

## 📁 Translation Files

**Location**: `assets/i18n/`

- `en.json` - English translations
- `zh-Hans.json` - Simplified Chinese (简体中文)
- `zh-Hant.json` - Traditional Chinese (繁體中文)

**Structure**: Nested JSON keys for organization
```json
{
  "app": {
    "title": "Remind Me PWA",
    "tagline": "Reminder Assistant"
  },
  "landing": {
    "nav": {
      "features": "Features",
      "pricing": "Pricing"
    }
  },
  "language": {
    "switch": "Switch language",
    "en": "English",
    "zh-Hans": "简体中文",
    "zh-Hant": "繁體中文"
  }
}
```

## 🔧 Locale System

### Locale Enum

```rust
pub enum Locale {
    En,      // English
    ZhHans,  // Simplified Chinese (简体中文)
    ZhHant,  // Traditional Chinese (繁體中文)
}
```

### Locale Mapping

- `"zh"` → `Locale::ZhHans` (defaults to Simplified Chinese)
- `"zh-Hans"` → `Locale::ZhHans`
- `"zh-CN"` → `Locale::ZhHans`
- `"zh-Hant"` → `Locale::ZhHant`
- `"zh-TW"` → `Locale::ZhHant`
- `"en"` → `Locale::En`

### BCP 47 Language Codes

- `en` → `en`
- `zh-Hans` → `zh-Hans` (Simplified Chinese)
- `zh-Hant` → `zh-Hant` (Traditional Chinese)

## 📝 Usage Patterns

### Getting Translations

```rust
use crate::i18n::use_t;

#[component]
fn MyComponent() -> Element {
    rsx! {
        h1 { {use_t("app.title")} }
        p { {use_t("app.tagline")} }
        button { {use_t("landing.nav.features")} }
    }
}
```

### Language Switcher Component

```rust
use crate::components::LanguageSwitcher;

#[component]
fn App() -> Element {
    rsx! {
        LanguageSwitcher { 
            class: Some("nav-lang-switcher".to_string()) 
        }
    }
}
```

### Setting Locale Programmatically

```rust
use crate::i18n::{use_i18n, Locale};

#[component]
fn MyComponent() -> Element {
    let i18n = use_i18n();
    
    rsx! {
        button {
            onclick: move |_| {
                let mut ctx = i18n.write();
                ctx.set_locale(Locale::ZhHans);
            },
            "Switch to Simplified Chinese"
        }
    }
}
```

## 🌐 URL Integration

### URL Format

- `/en/app` - English app
- `/zh-Hans/app` - Simplified Chinese app
- `/zh-Hant/app` - Traditional Chinese app
- `/en/privacy` - English privacy policy
- `/zh-Hans/terms` - Simplified Chinese terms

### Router Integration

The router automatically:
- Detects locale from URL path
- Maps `"zh"` to `"zh-Hans"` for backward compatibility
- Updates URL when locale changes
- Sets HTML `lang` attribute correctly

## ✅ Best Practices

### DO:
- ✅ Always use `use_t("key.path")` for user-facing text
- ✅ Add translation keys to **all 3 language files** (`en.json`, `zh-Hans.json`, `zh-Hant.json`)
- ✅ Use nested keys for organization (e.g., `"landing.nav.features"`)
- ✅ Keep translation keys consistent across all languages
- ✅ Test language switching functionality
- ✅ Verify HTML `lang` attribute is set correctly
- ✅ Use `LanguageSwitcher` component for language selection

### DON'T:
- ❌ Don't hardcode text strings in components
- ❌ Don't skip adding translations to any language file
- ❌ Don't use `Locale::Zh` (it doesn't exist - use `Locale::ZhHans` or `Locale::ZhHant`)
- ❌ Don't forget to update all 3 translation files when adding new keys
- ❌ Don't use abbreviations in translation keys

## 🔄 Adding New Translations

1. **Add key to English file** (`assets/i18n/en.json`):
```json
{
  "new_section": {
    "title": "New Section Title",
    "description": "New section description"
  }
}
```

2. **Add same key to Simplified Chinese** (`assets/i18n/zh-Hans.json`):
```json
{
  "new_section": {
    "title": "新章节标题",
    "description": "新章节描述"
  }
}
```

3. **Add same key to Traditional Chinese** (`assets/i18n/zh-Hant.json`):
```json
{
  "new_section": {
    "title": "新章節標題",
    "description": "新章節描述"
  }
}
```

4. **Use in component**:
```rust
rsx! {
    h2 { {use_t("new_section.title")} }
    p { {use_t("new_section.description")} }
}
```

## 🧪 Testing

- Test language switching in the UI
- Verify all text is translated correctly
- Check URL updates when switching languages
- Verify HTML `lang` attribute changes
- Test with all 3 languages
- Ensure no hardcoded text remains

## 📚 Related Files

- `src/i18n.rs` - I18n system implementation
- `src/components/language_switcher.rs` - Language switcher component
- `src/router.rs` - Locale-aware routing
- `src/app.rs` - Locale initialization and URL handling
- `assets/i18n/*.json` - Translation files
