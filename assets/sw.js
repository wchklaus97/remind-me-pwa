const CACHE_NAME = 'remind-me-pwa-v0.0.1';

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
];

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

  event.respondWith(
    caches.match(event.request)
      .then((response) => {
        // Return cached version or fetch from network and cache it (runtime caching)
        if (response) return response;

        return fetch(event.request).then((networkResponse) => {
          // Cache successful same-origin responses to speed up repeat visits.
          try {
            const url = new URL(event.request.url);
            if (url.origin === self.location.origin && networkResponse && networkResponse.ok) {
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


