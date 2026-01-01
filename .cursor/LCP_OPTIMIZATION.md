# LCP (Largest Contentful Paint) Optimization Guide

Based on [Chrome Developer Documentation](https://developer.chrome.com/docs/lighthouse/performance/lighthouse-largest-contentful-paint)

## LCP Target Thresholds

According to Chrome's LCP documentation:
- **Mobile**: 0-2.5s (green/good), 2.5-4s (orange/moderate), >4s (red/poor)
- **Desktop**: 0-1.2s (green/good), 1.2-2.4s (orange/moderate), >2.4s (red/poor)

## LCP Breakdown - 4 Subparts

LCP time is divided into 4 subparts, each with specific optimization strategies:

### 1. TTFB (Time to First Byte) ✅ OPTIMIZED

**Status**: Handled by GitHub Pages CDN

**What it measures**: Time from navigation start to first byte of HTML response

**Optimizations applied**:
- ✅ Static hosting via GitHub Pages (fast CDN)
- ✅ No server-side processing (pure static files)
- ✅ Compression enabled automatically (gzip/brotli)

**Target**: < 600ms

### 2. Load Delay ⚠️ MINIMIZED

**Status**: Minimized through build optimizations

**What it measures**: Gap between TTFB and when browser starts loading LCP resource

**Optimizations applied**:
- ✅ WASM bundle optimization with `wasm-opt` (reduces file size → faster discovery)
- ✅ CSS in assets/ directory (loaded immediately)
- ✅ No render-blocking external resources (removed Google Fonts)
- ✅ System fonts (instant rendering, no network requests)

**Remaining considerations**:
- Dioxus doesn't easily support resource hints in initial HTML
- WASM file discovery happens automatically (no preload needed)

**Target**: Minimize the gap between TTFB and resource load start

### 3. Load Time ✅ OPTIMIZED

**Status**: Optimized through WASM optimization and compression

**What it measures**: Time to actually load the LCP resource (WASM file)

**Optimizations applied**:
- ✅ **WASM optimization**: `wasm-opt -Oz --strip-debug` in deploy script
  - Reduces WASM file size by 10-30%
  - Faster download time
  - Faster parsing/instantiation
- ✅ **Compression**: Automatic gzip/brotli on GitHub Pages
- ✅ **Build optimization**: `opt-level = "z"`, `thin LTO`, `codegen-units = 1`
  - Maximum size optimization
  - Smaller binary = faster load
- ✅ **Minimal dependencies**: Only essential web-sys features

**Target**: Minimize WASM file size and load time

### 4. Render Delay ✅ OPTIMIZED

**Status**: Optimized through non-blocking initialization

**What it measures**: Gap between resource load completion and LCP element fully rendering

**Optimizations applied**:
- ✅ **Non-blocking route detection**: Default to Landing route immediately
  - Changed from `use_signal(|| get_initial_route())` to `use_signal(|| Route::Landing)`
  - Route detection happens asynchronously in `use_effect`
- ✅ **Minimized blocking operations**: Critical operations only in initial render
  - Lang attribute set immediately
  - Font removal happens quickly
  - Non-critical meta tags don't block rendering
- ✅ **System fonts**: No external font loading = instant text rendering
- ✅ **Optimized CSS**: Minimal, efficient stylesheets
- ✅ **LCP element optimization**: Landing page hero section is simple and renders quickly

**Target**: < 100ms render delay

## Current Implementation

### Build Configuration (`Cargo.toml`)

```toml
[profile.release]
opt-level = "z"           # Maximum size optimization
lto = "thin"              # Thin LTO for faster builds
codegen-units = 1         # Better optimization
panic = "abort"           # Smaller binary

[profile.wasm-release]
inherits = "release"
strip = false
debug = true              # Source maps (doesn't affect production size significantly)
```

### Deployment Script (`deploy.sh`)

Includes WASM optimization with `wasm-opt`:
```bash
wasm-opt "$wasm_file" -o "$wasm_file.tmp" -Oz --strip-debug
```

### Code Optimizations (`src/main.rs`)

1. **Non-blocking initialization**:
   - Default route: `use_signal(|| Route::Landing)`
   - Async route detection in `use_effect`

2. **Minimal blocking operations**:
   - Only critical operations block initial render
   - System fonts (no external loading)
   - Aggressive font link removal (prevents render-blocking)

### CSS Optimizations (`assets/style.css`)

1. **System fonts**: No external font requests
2. **Efficient selectors**: Optimized CSS structure
3. **Minimal styles**: Only necessary styles for LCP element

## Measurement

Lighthouse will show LCP breakdown in the "LCP element" diagnostic:
- TTFB: Server response time
- Load Delay: Gap before resource loading
- Load Time: Resource download time
- Render Delay: Time to render after load

## Expected Results

With these optimizations:
- **TTFB**: < 200ms (GitHub Pages CDN)
- **Load Delay**: < 100ms (optimized resource discovery)
- **Load Time**: < 1.5s (optimized WASM bundle)
- **Render Delay**: < 100ms (non-blocking initialization)

**Total LCP**: Target < 2.0s (mobile), < 1.2s (desktop)

## Monitoring

1. Run Lighthouse audit after each deployment
2. Check LCP breakdown in Lighthouse diagnostics
3. Monitor each subpart to identify regression
4. Test in release mode (`dx build --release`)

## Further Optimization Opportunities

If LCP is still high:

1. **Code splitting**: Split WASM bundle (if Dioxus supports it)
2. **Lazy loading**: Load non-critical components on demand
3. **Resource hints**: Add preload hints if Dioxus template system supports it
4. **Service Worker optimization**: Ensure SW doesn't block initial load
5. **Critical CSS inlining**: Inline critical above-the-fold CSS (complex for Dioxus)

## References

- [Chrome Developer: LCP Documentation](https://developer.chrome.com/docs/lighthouse/performance/lighthouse-largest-contentful-paint)
- [Web.dev: LCP Optimization](https://web.dev/lcp/)
- [LCP API](https://web.dev/lcp/#measure-lcp-in-javascript)



