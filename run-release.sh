#!/bin/bash

# Remind Me PWA - Production Build Server Script (for performance testing)

echo "🚀 Starting Remind Me PWA production build server..."
echo "⚠️  This builds in RELEASE mode (optimized, smaller files)"
echo ""

# Check if Dioxus CLI is installed
if ! command -v dx &> /dev/null; then
    echo "❌ Dioxus CLI (dx) is not installed!"
    echo ""
    echo "Install it with:"
    echo "  cargo install dioxus-cli --locked"
    echo ""
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "Dioxus.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

echo "✅ Dioxus CLI found"
echo "✅ Project files found"
echo ""

echo "🔨 Building project in RELEASE mode (this may take a few minutes)..."
echo "   This will create optimized, smaller WASM files (~1-2 MB instead of ~25 MB)"
echo ""

# Build in release mode
dx build --release --platform web

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Release build completed successfully"
    echo ""
    
    # Find WASM file
    WASM_FILE=$(find target/dx -name "*.wasm" -path "*/release/*" | head -1)
    
    if [ -n "$WASM_FILE" ]; then
        ORIGINAL_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
        echo "📦 Original WASM file size: $ORIGINAL_SIZE"
        
        # LCP Optimization: Optimize WASM bundle with wasm-opt (if available)
        # This can reduce WASM file size by 10-30% and improve LCP significantly
        # Note: If wasm-opt fails, we continue with unoptimized WASM (non-critical optimization)
        # WARNING: wasm-opt can cause runtime "unreachable" errors with DWARF debug info
        # Disabled by default - enable with ENABLE_WASM_OPT=true if needed
        ENABLE_WASM_OPT="${ENABLE_WASM_OPT:-false}"
        if [ "$ENABLE_WASM_OPT" = "true" ] && command -v wasm-opt &> /dev/null; then
            echo ""
            echo "🔧 Optimizing WASM bundle with wasm-opt for better LCP..."
            # Try wasm-opt with different strategies to handle DWARF debug info issues
            # Strategy 1: Try with --strip-debug (may fail with DWARF issues)
            if wasm-opt "$WASM_FILE" -o "$WASM_FILE.tmp" -Oz --strip-debug 2>/dev/null; then
                mv "$WASM_FILE.tmp" "$WASM_FILE"
                OPTIMIZED_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
                echo "✅ WASM optimization complete!"
                echo "📦 Optimized WASM file size: $OPTIMIZED_SIZE (reduced from $ORIGINAL_SIZE)"
            # Strategy 2: Try without --strip-debug (keeps debug info but still optimizes)
            elif wasm-opt "$WASM_FILE" -o "$WASM_FILE.tmp" -Oz 2>/dev/null; then
                mv "$WASM_FILE.tmp" "$WASM_FILE"
                OPTIMIZED_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
                echo "✅ WASM optimization complete! (debug info preserved)"
                echo "📦 Optimized WASM file size: $OPTIMIZED_SIZE (reduced from $ORIGINAL_SIZE)"
            else
                echo "⚠️  wasm-opt optimization failed (DWARF debug info incompatibility)"
                echo "   This is non-critical - the app will work without optimization"
                echo "   The build already includes size optimizations from Rust compiler"
                rm -f "$WASM_FILE.tmp"
            fi
        else
            if [ "$ENABLE_WASM_OPT" != "true" ]; then
                echo ""
                echo "ℹ️  wasm-opt optimization is disabled by default (can cause runtime errors)"
                echo "   To enable: ENABLE_WASM_OPT=true ./run-release.sh"
                echo "   The build already includes size optimizations from Rust compiler"
            else
                echo ""
                echo "💡 TIP: Install wasm-opt for even better performance (10-30% smaller WASM):"
                echo "   macOS: brew install binaryen"
                echo "   npm:   npm install -g wasm-opt"
                echo "   This can help you achieve 100% performance score!"
            fi
        fi
    fi
    
    echo ""
    echo "🚀 Starting production server..."
    echo "   The app will open in your browser automatically"
    echo "   Press Ctrl+C to stop the server"
    echo ""
    echo "💡 Use this for Lighthouse performance testing!"
    echo ""
    
    # Start the server with release build
    dx serve --release
else
    echo ""
    echo "❌ Build failed. Check the errors above."
    exit 1
fi

