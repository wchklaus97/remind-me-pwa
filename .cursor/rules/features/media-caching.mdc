---
alwaysApply: false
description: Media caching and loading patterns for images and videos
---

# Media Caching & Loading Patterns

## 🎯 Overview

The project uses a **shared media cache manager** to efficiently load and cache images and videos with:
- **Deduplication**: Multiple components requesting the same URL only trigger one download
- **Loading States**: Shimmer skeleton placeholders while loading
- **Error Handling**: Fallback text display on load failure
- **Browser Cache Storage**: Persistent caching using Cache Storage API

## 🏗️ Architecture

### Components

1. **MediaCacheProvider** - Context provider that wraps the app
2. **MediaCacheManager** - Shared cache manager (provided via context)
3. **ManagedCachedImage** - Image component with cache integration
4. **ManagedCachedVideo** - Video component with cache integration
5. **CachedImage** - Basic image component (used internally)
6. **CachedVideo** - Basic video component (used internally)

### Service Layer

- **`src/services/media_cache.rs`**: Cache Storage API integration
  - `ensure_cached()`: Ensures a URL is cached
  - `prefetch_assets()`: Prefetches multiple assets

## 📝 Usage Patterns

### Setup MediaCacheProvider

```rust
use crate::components::MediaCacheProvider;

#[component]
fn App() -> Element {
    rsx! {
        MediaCacheProvider {
            // All child components can use ManagedCachedImage/Video
            YourContent {}
        }
    }
}
```

### Using ManagedCachedImage

```rust
use crate::components::ManagedCachedImage;
use dioxus::asset::asset;

#[component]
fn MyComponent() -> Element {
    static LOGO: Asset = asset!("/assets/images/logo.png");
    
    rsx! {
        ManagedCachedImage {
            src: LOGO,
            alt: "App Logo".to_string(),
            class: Some("logo".to_string()),
            width: Some("48".to_string()),
            height: Some("48".to_string()),
        }
    }
}
```

### Using ManagedCachedVideo

```rust
use crate::components::ManagedCachedVideo;
use dioxus::asset::asset;

#[component]
fn MyComponent() -> Element {
    static VIDEO: Asset = asset!("/assets/videos/animation.mp4");
    static POSTER: Asset = asset!("/assets/videos/poster.webp");
    
    rsx! {
        ManagedCachedVideo {
            src: VIDEO,
            poster: POSTER,
            aria_label: Some("Loading animation".to_string()),
            title: Some("Remind Me mascot animation".to_string()),
            fallback_text: Some("Animation failed to load".to_string()),
            class: Some("hero-animation".to_string()),
            width: "120".to_string(),
            height: "120".to_string(),
        }
    }
}
```

## 🔄 How It Works

### Cache Manager Flow

1. **Component mounts** → Calls `manager.ensure(url)`
2. **Manager checks state**:
   - If `Loading` or `Ready` → Do nothing (deduplication)
   - If not cached → Set state to `Loading` and start fetch
3. **Service layer** (`ensure_cached`):
   - Checks browser Cache Storage
   - If not found, fetches and caches
   - Returns `Ok(())` or `Err`
4. **Manager updates state**:
   - `Loading` → `Ready` (on success)
   - `Loading` → `Error` (on failure)
5. **Component reacts**:
   - Shows shimmer while `Loading`
   - Shows media when `Ready`
   - Shows fallback text on `Error`

### Deduplication Example

```rust
// Component A requests "/assets/video.mp4"
ManagedCachedVideo { src: VIDEO, ... }  // Starts loading

// Component B requests same "/assets/video.mp4" (before A finishes)
ManagedCachedVideo { src: VIDEO, ... }  // No new fetch! Waits for A's result
```

## 🎨 CSS Classes

### Media Wrapper
```css
.cached-media-wrap {
    position: relative;
    /* Contains skeleton, fallback, and media */
}
```

### Loading State
```css
.media-skeleton {
    /* Shimmer animation placeholder */
    background: linear-gradient(...);
    animation: shimmer 1.5s infinite;
}
```

### Error State
```css
.media-fallback {
    /* Centered fallback text */
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
}
```

### Media Element
```css
.cached-media {
    /* Actual img/video */
    opacity: 0 → 1 transition when loaded
}
```

## ✅ Best Practices

### DO:
- ✅ Always wrap app with `MediaCacheProvider`
- ✅ Use `ManagedCachedImage`/`ManagedCachedVideo` for cached media
- ✅ Provide `aria_label`, `title`, and `fallback_text` for accessibility
- ✅ Use `Asset` type from Dioxus asset system
- ✅ Provide poster image for videos
- ✅ Test loading and error states

### DON'T:
- ❌ Don't use regular `<img>` or `<video>` tags (use managed components)
- ❌ Don't forget to provide `alt` text for images
- ❌ Don't skip `fallback_text` for videos
- ❌ Don't create multiple `MediaCacheProvider` instances
- ❌ Don't manually manage cache state (let the manager handle it)

## 🔧 Cache Storage

### Cache Name
- Default: `"media-cache-v1"`
- Defined in `MediaCacheProvider` component
- Update version to force cache refresh

### Cache Behavior
- **Persistent**: Survives page reloads
- **Automatic**: Managed by browser Cache Storage API
- **Deduplicated**: Same URL only cached once
- **Async**: Non-blocking cache operations

## 📚 Related Files

- `src/components/media.rs` - Media cache components
- `src/services/media_cache.rs` - Cache Storage API service
- `src/app.rs` - MediaCacheProvider setup
- `assets/css/components.css` - Media skeleton and fallback styles

## 🧪 Testing

- Test with slow network (throttle in DevTools)
- Verify shimmer skeleton appears
- Verify media loads correctly
- Test error state (invalid URL)
- Verify deduplication (multiple components, same URL)
- Check Cache Storage in DevTools → Application → Cache Storage
