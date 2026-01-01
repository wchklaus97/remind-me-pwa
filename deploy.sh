#!/bin/bash

# Deployment script for GitHub Pages

echo "🚀 Building Remind Me PWA for GitHub Pages..."

# Build the project
dx bundle --release --out-dir docs

# Move files from docs/public to docs if needed
if [ -d "docs/public" ]; then
    echo "📦 Moving files from docs/public to docs..."
    mv docs/public/* docs/
    rmdir docs/public
fi

# LCP Optimization: Optimize WASM bundle with wasm-opt (if available)
# This reduces WASM file size and improves loading performance
if command -v wasm-opt &> /dev/null; then
    echo "🔧 Optimizing WASM bundle with wasm-opt for better LCP..."
    # Find WASM files and optimize them
    find docs -name "*.wasm" -type f | while read wasm_file; do
        echo "  Optimizing: $wasm_file"
        wasm-opt "$wasm_file" -o "$wasm_file.tmp" -Oz --strip-debug
        mv "$wasm_file.tmp" "$wasm_file"
    done
    echo "✅ WASM optimization complete"
else
    echo "ℹ️  wasm-opt not found (optional: install with 'brew install binaryen' or 'npm install -g wasm-opt')"
    echo "   Skipping WASM optimization. Install wasm-opt for better LCP performance."
fi

# Create 404.html for client-side routing
if [ -f "docs/index.html" ]; then
    echo "📄 Creating 404.html for client-side routing..."
    cp docs/index.html docs/404.html
fi

# Copy robots.txt and sitemap.xml to root (SEO: must be at site root, not in assets/)
if [ -f "assets/robots.txt" ]; then
    echo "🤖 Copying robots.txt to root..."
    cp assets/robots.txt docs/robots.txt
fi
if [ -f "assets/sitemap.xml" ]; then
    echo "🗺️  Copying sitemap.xml to root..."
    cp assets/sitemap.xml docs/sitemap.xml
fi

echo "✅ Build complete! Files are in the docs/ directory."
echo ""
echo "Next steps:"
echo "1. Review the docs/ directory"
echo "2. Commit and push: git add docs/ && git commit -m 'Deploy' && git push"
echo "3. Enable GitHub Pages in repository settings (docs folder)"


