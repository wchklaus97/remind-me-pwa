use dioxus::prelude::*;

/// Best-effort prefetching into the browser Cache Storage (WASM only).
///
/// This is intentionally "fire-and-forget":
/// - No panics
/// - No console errors (Lighthouse)
/// - Falls back to normal browser caching if Cache API isn't available
#[allow(dead_code)]
pub fn prefetch_assets(_cache_name: &str, _assets: &[Asset]) {
    #[cfg(target_arch = "wasm32")]
    {
        prefetch_urls(_cache_name, &_assets.iter().map(|a| a.to_string()).collect::<Vec<_>>());
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn prefetch_urls(cache_name: &str, urls: &[String]) {
    use js_sys::{Array, Function, Reflect};
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};

    let Some(window) = web_sys::window() else {
        return;
    };

    let caches = Reflect::get(&window, &JsValue::from_str("caches")).ok();
    let Some(caches) = caches else {
        return;
    };

    let open = Reflect::get(&caches, &JsValue::from_str("open")).ok();
    let Some(open) = open.and_then(|v| v.dyn_into::<Function>().ok()) else {
        return;
    };

    let promise = open.call1(&caches, &JsValue::from_str(cache_name)).ok();
    let Some(promise) = promise else {
        return;
    };

    let urls_js = Array::new();
    for u in urls {
        urls_js.push(&JsValue::from_str(u));
    }

    // then(cache => cache.addAll(urls))
    let then = Reflect::get(&promise, &JsValue::from_str("then")).ok();
    let Some(then) = then.and_then(|v| v.dyn_into::<Function>().ok()) else {
        return;
    };

    let on_ok = Closure::wrap(Box::new(move |cache: JsValue| {
        let add_all = Reflect::get(&cache, &JsValue::from_str("addAll")).ok();
        let Some(add_all) = add_all.and_then(|v| v.dyn_into::<Function>().ok()) else {
            return;
        };

        let _ = add_all.call1(&cache, &urls_js);
    }) as Box<dyn FnMut(JsValue)>);

    // Ignore failures silently.
    let on_err = Closure::wrap(Box::new(move |_err: JsValue| {}) as Box<dyn FnMut(JsValue)>);

    let _ = then.call2(&promise, on_ok.as_ref().unchecked_ref(), on_err.as_ref().unchecked_ref());

    // Fire-and-forget; keep closures alive.
    on_ok.forget();
    on_err.forget();
}

/// Ensure a single URL is stored in Cache Storage (WASM only).
///
/// This is used by the shared cache manager to:
/// - Deduplicate in-flight downloads (only one fetch per URL)
/// - Notify all subscribers when ready
#[cfg(target_arch = "wasm32")]
pub async fn ensure_cached(cache_name: &str, url: &str) -> Result<(), wasm_bindgen::JsValue> {
    use js_sys::{Function, Reflect};
    use js_sys::Promise;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;

    let Some(window) = web_sys::window() else {
        return Ok(());
    };

    let caches = Reflect::get(&window, &JsValue::from_str("caches"))?;
    let open = Reflect::get(&caches, &JsValue::from_str("open"))?
        .dyn_into::<Function>()?;

    let open_promise: Promise =
        open.call1(&caches, &JsValue::from_str(cache_name))?.dyn_into()?;
    let cache = JsFuture::from(open_promise).await?;

    // cache.match(url)
    let match_fn = Reflect::get(&cache, &JsValue::from_str("match"))?
        .dyn_into::<Function>()?;
    let match_promise: Promise = match_fn.call1(&cache, &JsValue::from_str(url))?.dyn_into()?;
    let matched = JsFuture::from(match_promise).await?;

    if !matched.is_null() && !matched.is_undefined() {
        return Ok(());
    }

    // cache.add(url)
    let add_fn = Reflect::get(&cache, &JsValue::from_str("add"))?
        .dyn_into::<Function>()?;
    let add_promise: Promise = add_fn.call1(&cache, &JsValue::from_str(url))?.dyn_into()?;
    let _ = JsFuture::from(add_promise).await?;
    Ok(())
}


