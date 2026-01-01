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

# Create 404.html for client-side routing
if [ -f "docs/index.html" ]; then
    echo "📄 Creating 404.html for client-side routing..."
    cp docs/index.html docs/404.html
fi

echo "✅ Build complete! Files are in the docs/ directory."
echo ""
echo "Next steps:"
echo "1. Review the docs/ directory"
echo "2. Commit and push: git add docs/ && git commit -m 'Deploy' && git push"
echo "3. Enable GitHub Pages in repository settings (docs folder)"


