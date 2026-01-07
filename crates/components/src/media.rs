use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaState {
    Loading,
    Ready,
    Error,
}

#[derive(Clone)]
pub struct MediaCacheManager {
    cache_name: &'static str,
    states: Signal<HashMap<String, MediaState>>,
}

impl MediaCacheManager {
    pub fn state(&self, key: &str) -> Option<MediaState> {
        (self.states)().get(key).copied()
    }

    pub fn ensure(&self, key: String) {
        // Deduplicate: if already Loading/Ready, do nothing.
        if matches!(
            self.state(&key),
            Some(MediaState::Loading | MediaState::Ready | MediaState::Error)
        ) {
            return;
        }

        let mut states = self.states;
        states.with_mut(|m| {
            m.insert(key.clone(), MediaState::Loading);
        });

        let _cache_name = self.cache_name;
        spawn(async move {
            #[cfg(target_arch = "wasm32")]
            {
                let result = ensure_cached_impl(_cache_name, &key).await;
                states.with_mut(|m| {
                    m.insert(
                        key.clone(),
                        if result.is_ok() { MediaState::Ready } else { MediaState::Error },
                    );
                });
                return;
            }

            // Non-wasm: mark ready (no browser Cache API).
            #[cfg(not(target_arch = "wasm32"))]
            {
                states.with_mut(|m| {
                    m.insert(key.clone(), MediaState::Ready);
                });
            }
        });
    }
}

#[component]
pub fn MediaCacheProvider(children: Element) -> Element {
    let states = use_signal(|| HashMap::<String, MediaState>::new());
    use_context_provider(|| MediaCacheManager {
        cache_name: "media-cache-v1",
        states,
    });

    rsx! { {children} }
}

fn use_media_cache_manager() -> MediaCacheManager {
    use_context::<MediaCacheManager>()
}

/// A lightweight image wrapper that shows a shimmer skeleton until the image is loaded.
///
/// Note: caching is handled by the browser and service worker; this component focuses on UX.
#[component]
pub fn CachedImage(
    src: String,
    alt: String,
    class: Option<String>,
    width: Option<String>,
    height: Option<String>,
) -> Element {
    let mut is_loaded = use_signal(|| false);
    let mut has_error = use_signal(|| false);

    let mut classes = String::from("cached-media");
    if let Some(extra) = class {
        classes.push(' ');
        classes.push_str(&extra);
    }

    rsx! {
        div { class: "cached-media-wrap",
            if !is_loaded() && !has_error() {
                div { class: "media-skeleton", aria_hidden: "true" }
            }
            if has_error() {
                div { class: "media-fallback", role: "status",
                    "{alt}"
                }
            }
            img {
                class: "{classes}",
                src: "{src}",
                alt: "{alt}",
                width: width.unwrap_or_default(),
                height: height.unwrap_or_default(),
                decoding: "async",
                loading: "lazy",
                onload: move |_| is_loaded.set(true),
                onerror: move |_| {
                    has_error.set(true);
                    is_loaded.set(true);
                },
                style: if is_loaded() {
                    "opacity: 1; transition: opacity 160ms ease;"
                } else {
                    "opacity: 0; transition: opacity 160ms ease;"
                },
            }
        }
    }
}

/// A lightweight video wrapper that shows a shimmer skeleton until the video has enough data to render.
#[component]
pub fn CachedVideo(
    src: Asset,
    poster: Asset,
    aria_label: Option<String>,
    title: Option<String>,
    fallback_text: Option<String>,
    class: Option<String>,
    width: String,
    height: String,
) -> Element {
    let mut is_loaded = use_signal(|| false);
    let mut has_error = use_signal(|| false);

    let mut classes = String::from("cached-media");
    if let Some(extra) = class {
        classes.push(' ');
        classes.push_str(&extra);
    }

    rsx! {
        div { class: "cached-media-wrap",
            if !is_loaded() && !has_error() {
                div { class: "media-skeleton", aria_hidden: "true" }
            }
            if has_error() {
                div { class: "media-fallback", role: "status",
                    "{fallback_text.clone().unwrap_or_else(|| \"Media failed to load\".to_string())}"
                }
            }
            video {
                class: "{classes}",
                width: "{width}",
                height: "{height}",
                autoplay: "true",
                loop: "true",
                muted: "true",
                playsinline: "true",
                preload: "metadata",
                poster: poster,
                aria_label: aria_label.unwrap_or_default(),
                title: title.unwrap_or_default(),
                // Use native events to hide skeleton when the first frame is ready.
                onloadeddata: move |_| is_loaded.set(true),
                onerror: move |_| {
                    has_error.set(true);
                    is_loaded.set(true);
                },
                style: if has_error() {
                    "display: none;"
                } else if is_loaded() {
                    "opacity: 1; transition: opacity 160ms ease; width: 100%; height: 100%; object-fit: contain; display: block;"
                } else {
                    "opacity: 0; transition: opacity 160ms ease; width: 100%; height: 100%; object-fit: contain; display: block;"
                },
                source { src: src, r#type: "video/mp4" }
                // Fallback text shows if video can't be rendered.
                if let Some(text) = fallback_text {
                    span { class: "sr-only", "{text}" }
                }
            }
        }
    }
}

/// CachedVideo + shared cache manager integration:
/// - Dedupes downloads across multiple widgets
/// - Warms Cache Storage before the element loads
#[component]
pub fn ManagedCachedVideo(
    src: Asset,
    poster: Asset,
    aria_label: Option<String>,
    title: Option<String>,
    fallback_text: Option<String>,
    class: Option<String>,
    width: String,
    height: String,
) -> Element {
    let manager = use_media_cache_manager();
    let src_key = src.to_string();
    let poster_key = poster.to_string();

    // Notify the manager once; it will dedupe in-flight work.
    use_effect({
        let manager = manager.clone();
        let src_key = src_key.clone();
        let poster_key = poster_key.clone();
        move || {
            manager.ensure(src_key.clone());
            manager.ensure(poster_key.clone());
        }
    });

    rsx! {
        CachedVideo { src, poster, aria_label, title, fallback_text, class, width, height }
    }
}

/// CachedImage + shared cache manager integration.
#[component]
pub fn ManagedCachedImage(
    src: Asset,
    alt: String,
    class: Option<String>,
    width: Option<String>,
    height: Option<String>,
) -> Element {
    let manager = use_media_cache_manager();
    let key = src.to_string();
    use_effect({
        let manager = manager.clone();
        let key = key.clone();
        move || {
            manager.ensure(key.clone());
        }
    });

    rsx! {
        CachedImage {
            src: src.to_string(),
            alt,
            class,
            width,
            height,
        }
    }
}


