#!/bin/bash

# Remind Me PWA - iOS Mobile App Build Script

echo "🍎 Building Remind Me PWA for iOS..."
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

# Check for iOS build tools
if ! command -v xcodebuild &> /dev/null; then
    echo "❌ Xcode is not installed or not in PATH!"
    echo ""
    echo "iOS development requires Xcode. Install it from the App Store."
    exit 1
fi

echo "✅ Dioxus CLI found"
echo "✅ Xcode found"
echo "✅ Project files found"
echo ""

# Build for iOS
echo "🔨 Building iOS app..."
dx build --platform ios --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ iOS build completed successfully!"
    echo ""
    echo "Next steps:"
    echo "1. Open the Xcode project in mobile/ios/"
    echo "2. Configure signing in Xcode"
    echo "3. Build and run on simulator or device"
else
    echo ""
    echo "❌ iOS build failed"
    echo "   Check the error messages above for details"
    exit 1
fi

