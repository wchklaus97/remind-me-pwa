#!/bin/bash

# Remind Me PWA - Local Development Server Script

echo "🔔 Starting Remind Me PWA development server..."
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

# Dioxus workspace: explicitly select the binary package to build/serve.
# Override if needed, e.g.:
#   DX_PACKAGE=remind-me-mobile-app ./run-dev.sh
DX_PACKAGE="${DX_PACKAGE:-remind-me-web-app}"

# For dev builds, keep debug symbols enabled by default (better error messages/source maps).
# Override if needed:
#   DX_DEBUG_SYMBOLS=false ./run-dev.sh
DX_DEBUG_SYMBOLS="${DX_DEBUG_SYMBOLS:-true}"

# Check for version mismatch
DX_VERSION=$(dx --version 2>&1 | grep -oE '[0-9]+\.[0-9]+' | head -1)
DIOXUS_VERSION=$(grep -E '^dioxus = ' Cargo.toml | grep -oE '[0-9]+\.[0-9]+' | head -1)

if [ ! -z "$DX_VERSION" ] && [ ! -z "$DIOXUS_VERSION" ]; then
    DX_MAJOR=$(echo $DX_VERSION | cut -d. -f1)
    DIOXUS_MAJOR=$(echo $DIOXUS_VERSION | cut -d. -f1)
    if [ "$DX_MAJOR" != "$DIOXUS_MAJOR" ]; then
        echo "⚠️  Warning: Dioxus CLI version ($DX_VERSION) doesn't match dioxus library version ($DIOXUS_VERSION)"
        echo "   Consider updating: cargo install dioxus-cli --version $DIOXUS_VERSION --locked"
    fi
fi

echo ""
echo "🔨 Building project first (to ensure WASM files are generated)..."
# Clean build directory if it's corrupted
if [ -d "target/dx" ] && [ ! -f "target/dx/remind-me-pwa/debug/web/public/wasm-bindgen/remind-me-pwa.js" ]; then
    echo "🧹 Cleaning corrupted build directory..."
    rm -rf target/dx/remind-me-pwa/debug/web/public/wasm-bindgen 2>/dev/null || true
fi

dx build --platform web --package "$DX_PACKAGE" --debug-symbols "$DX_DEBUG_SYMBOLS" > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully"
else
    echo "⚠️  Build had warnings, but continuing..."
    echo "   If you see persistent errors, try: cargo clean && dx build"
fi
echo ""
echo "🚀 Starting development server..."
echo "   The app will open in your browser automatically"
echo "   Press Ctrl+C to stop the server"
echo ""

# Start the development server
dx serve --package "$DX_PACKAGE" --debug-symbols "$DX_DEBUG_SYMBOLS"

