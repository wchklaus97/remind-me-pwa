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
echo ""
echo "🔨 Building project first (to ensure WASM files are generated)..."
dx build > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully"
else
    echo "⚠️  Build had warnings, but continuing..."
fi
echo ""
echo "🚀 Starting development server..."
echo "   The app will open in your browser automatically"
echo "   Press Ctrl+C to stop the server"
echo ""

# Start the development server
dx serve

