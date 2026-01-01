# Development vs Production Build Sizes

## The Problem: Why Your WASM File is 25 MB

You're seeing **25,014 KiB (24.4 MB)** WASM file because you're testing with a **development build**.

### Development Build Characteristics

When you run `dx serve` or `dx build` (without `--release`):

- **File Size**: 24.9 MB WASM (as shown in your Lighthouse report)
- **Optimization**: `opt-level = 0` (no optimization)
- **Debug Info**: `debug = true` (includes full debug symbols)
- **Codegen Units**: 16 (faster compilation, larger binary)
- **Purpose**: Fast compilation for development, not optimized for size

### Production Build Characteristics

When you run `dx build --release`:

- **File Size**: ~1-2 MB WASM (with wasm-opt: ~800 KB - 1.5 MB)
- **Optimization**: `opt-level = "z"` (maximum size optimization)
- **Debug Info**: Minimal (only for source maps)
- **Codegen Units**: 1 (better optimization, smaller binary)
- **LTO**: Thin LTO enabled (additional size reduction)
- **Purpose**: Optimized for production deployment

## Size Comparison

| Build Type | WASM Size | Optimization | Use Case |
|------------|-----------|--------------|----------|
| **Development** (`dx serve`) | ~25 MB | None (`opt-level = 0`) | Local development |
| **Production** (`dx build --release`) | ~1-2 MB | Maximum (`opt-level = "z"`) | Deployment |
| **Production + wasm-opt** | ~800 KB - 1.5 MB | Maximum + wasm-opt | Optimal deployment |

## The Lighthouse Issues Explained

### 1. ✅ Enormous Network Payload (25 MB) - EXPECTED IN DEV

**Why it happens**:
- Development build has no optimization
- Includes debug symbols and metadata
- Multiple codegen units = less efficient code

**Solution**:
- **Always test Lighthouse with release builds**: `dx build --release`
- Production build will be 10-20x smaller
- With wasm-opt, it can be 15-30x smaller

### 2. ✅ Reduce Unused JavaScript (40 KiB) - EXPECTED IN DEV

**Why it happens**:
- Development code includes debug helpers
- Hot-reload code and development utilities
- These are removed in production builds

**Solution**:
- Production builds automatically remove unused code
- Tree shaking eliminates dead code
- This will be much smaller in release builds

### 3. ⚠️ Page Prevented Back/Forward Cache Restoration

**Why it happens**:
- Service worker may prevent bfcache
- Some browser APIs prevent bfcache
- Development mode may have additional code preventing it

**Solution**:
- This is less critical but can be optimized
- Service worker configuration may need adjustment
- Most PWA apps have this limitation

## How to Test Correctly

### ❌ DON'T Test Performance with Dev Builds

```bash
# This creates 25 MB WASM files - NOT for performance testing
dx serve
dx build
```

### ✅ DO Test Performance with Release Builds

```bash
# Build for production
dx build --release --platform web

# Or use the deploy script (includes wasm-opt)
./deploy.sh

# Then serve the production build
dx serve --release
```

### Verify Production Build Size

```bash
# Check WASM file size in production build
find target/dx -name "*.wasm" -path "*/release/*" -exec ls -lh {} \;

# Expected: ~1-2 MB (or ~800 KB - 1.5 MB with wasm-opt)
```

## Current Configuration

Your `Cargo.toml` already has optimal production settings:

```toml
[profile.release]
opt-level = "z"        # Maximum size optimization ✅
lto = "thin"           # Thin LTO ✅
codegen-units = 1      # Better optimization ✅
panic = "abort"        # Smaller binary ✅

[profile.wasm-release]
inherits = "release"
strip = false
debug = true           # Source maps (minimal size impact)
```

Your `deploy.sh` script also includes wasm-opt optimization:

```bash
wasm-opt "$wasm_file" -o "$wasm_file.tmp" -Oz --strip-debug
```

## Expected Production Results

With release build + wasm-opt:

- **WASM File Size**: ~800 KB - 1.5 MB (vs 25 MB in dev)
- **Total Payload**: ~2-3 MB (vs 25 MB in dev)
- **LCP**: Should be < 2.5s (vs much higher in dev)
- **Performance Score**: Should be 90-100 (vs lower in dev)

## Summary

**The issues you're seeing are NORMAL for development builds.**

✅ **Development builds are intentionally large** (fast compilation > small size)  
✅ **Production builds are optimized** (small size > fast compilation)  
✅ **Always test Lighthouse with release builds**  
✅ **Your production configuration is already optimal**

## Next Steps

1. **Test with release build**:
   ```bash
   dx build --release --platform web
   dx serve --release
   ```

2. **Or use deploy script** (includes wasm-opt):
   ```bash
   ./deploy.sh
   # Then test the docs/ directory
   ```

3. **Run Lighthouse on the release build** to see realistic performance metrics

The 25 MB WASM file is expected in development - production will be much smaller!



