const CACHE_NAME = 'remind-me-pwa-v0.0.3';

// Derive base path from service worker scope so this works on:
// - GitHub Pages: /<repo>/
// - Local dev: /
const SCOPE_URL = new URL(self.registration.scope);
const BASE_PATH = SCOPE_URL.pathname.replace(/\/$/, '');

const urlsToCache = [
  `${BASE_PATH}/`,
  `${BASE_PATH}/index.html`,
  // GitHub Pages SPA fallback
  `${BASE_PATH}/404.html`,
  `${BASE_PATH}/assets/manifest.json`,
  `${BASE_PATH}/assets/sw.js`,
  // CSS files - cache all stylesheets
  `${BASE_PATH}/assets/css/base.css`,
  `${BASE_PATH}/assets/css/components.css`,
  `${BASE_PATH}/assets/css/app.css`,
  `${BASE_PATH}/assets/css/landing.css`,
  `${BASE_PATH}/assets/css/layout.css`,
  `${BASE_PATH}/assets/css/utilities.css`,
  `${BASE_PATH}/assets/css/responsive.css`,
];

const ONE_YEAR_SECONDS = 60 * 60 * 24 * 365;

function is_same_origin(request_url) {
  try {
    const url = new URL(request_url);
    return url.origin === self.location.origin;
  } catch (_) {
    return false;
  }
}

function is_static_asset(request_url) {
  try {
    const url = new URL(request_url);
    if (url.origin !== self.location.origin) return false;

    // Cache hashed Dioxus assets aggressively (safe because filenames include a content hash).
    if (url.pathname.startsWith(`${BASE_PATH}/assets/`)) return true;

    // Some builds place wasm/js under other directories; keep these cached too.
    if (url.pathname.endsWith('.wasm')) return true;
    if (url.pathname.endsWith('.js')) return true;
    if (url.pathname.endsWith('.css')) return true;
    if (url.pathname.endsWith('.mp4')) return true;
    if (url.pathname.endsWith('.webm')) return true;
    if (url.pathname.endsWith('.webp')) return true;
    if (url.pathname.endsWith('.avif')) return true;
    if (url.pathname.endsWith('.png')) return true;
    if (url.pathname.endsWith('.ico')) return true;

    return false;
  } catch (_) {
    return false;
  }
}

async function with_long_cache_headers(response) {
  // We only do this for cached static assets. This helps "cache lifetime" audits for repeat visits,
  // but note: GitHub Pages' *network* Cache-Control headers cannot be changed.
  const headers = new Headers(response.headers);
  headers.set('Cache-Control', `public, max-age=${ONE_YEAR_SECONDS}, immutable`);

  // Rebuild the response so the cached version carries our updated headers.
  const blob = await response.blob();
  return new Response(blob, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

function parse_range_header(range_header) {
  // "bytes=START-END" (END optional)
  const match = /^bytes=(\d+)-(\d+)?$/i.exec(range_header || '');
  if (!match) return null;
  const start = Number(match[1]);
  const end = match[2] ? Number(match[2]) : null;
  if (!Number.isFinite(start) || start < 0) return null;
  if (end != null && (!Number.isFinite(end) || end < start)) return null;
  return { start, end };
}

async function respond_with_range(full_response, range_header) {
  const buf = await full_response.arrayBuffer();
  const total = buf.byteLength;
  const parsed = parse_range_header(range_header);
  if (!parsed) return full_response;

  const start = parsed.start;
  const end = parsed.end != null ? Math.min(parsed.end, total - 1) : total - 1;
  if (start >= total) {
    return new Response(null, {
      status: 416,
      headers: { 'Content-Range': `bytes */${total}` },
    });
  }

  const sliced = buf.slice(start, end + 1);
  const headers = new Headers(full_response.headers);
  headers.set('Content-Range', `bytes ${start}-${end}/${total}`);
  headers.set('Accept-Ranges', 'bytes');
  headers.set('Content-Length', String(sliced.byteLength));

  // Range responses must be 206.
  return new Response(sliced, {
    status: 206,
    statusText: 'Partial Content',
    headers,
  });
}

// Install event - cache resources
self.addEventListener('install', (event) => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(urlsToCache))
  );
});

// Fetch event - serve from cache, fallback to network
self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET') return;

  // SPA navigation fallback (important for multi-locale path routing on static hosts)
  if (event.request.mode === 'navigate') {
    event.respondWith(
      fetch(event.request).catch(() => caches.match(`${BASE_PATH}/index.html`))
    );
    return;
  }

  // Cache-first for static assets (best for repeat visits + LCP).
  if (is_static_asset(event.request.url)) {
    event.respondWith((async () => {
      const cache = await caches.open(CACHE_NAME);
      // For Range requests (video), CacheStorage won't match/serve partial content automatically.
      // We cache the *full* response once, then serve byte ranges from it.
      const range = event.request.headers.get('range');
      if (range) {
        const url = event.request.url;
        const cached_full = await cache.match(url);
        if (cached_full) {
          return respond_with_range(cached_full, range);
        }

        // First time: fetch full file (no Range), cache it, then serve the requested range.
        const full = await fetch(new Request(url, { method: 'GET' }));
        if (full && full.ok) {
          const cacheable = await with_long_cache_headers(full.clone());
          await cache.put(url, cacheable);
          return respond_with_range(full, range);
        }
        return full;
      }

      const cached = await cache.match(event.request);
      if (cached) {
        // Update in the background.
        event.waitUntil((async () => {
          try {
            const network = await fetch(event.request);
            if (network && network.ok) {
              const cacheable = await with_long_cache_headers(network.clone());
              await cache.put(event.request, cacheable);
            }
          } catch (_) {
            // ignore
          }
        })());
        return cached;
      }

      // First time: fetch, cache, return.
      const network = await fetch(event.request);
      if (network && network.ok) {
        const cacheable = await with_long_cache_headers(network.clone());
        await cache.put(event.request, cacheable);
      }
      return network;
    })());
    return;
  }

  event.respondWith(
    caches.match(event.request)
      .then((response) => {
        // Return cached version or fetch from network and cache it (runtime caching)
        if (response) return response;

        return fetch(event.request).then((networkResponse) => {
          // Cache successful same-origin responses to speed up repeat visits.
          try {
            if (is_same_origin(event.request.url) && networkResponse && networkResponse.ok) {
              const copy = networkResponse.clone();
              caches.open(CACHE_NAME).then((cache) => cache.put(event.request, copy));
            }
          } catch (_) {
            // ignore
          }

          return networkResponse;
        });
      })
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME) {
            return caches.delete(cacheName);
          }
        })
      );
    }).then(() => self.clients.claim())
  );
});


