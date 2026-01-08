#!/bin/bash

# Remind Me PWA - Android Mobile App Build Script

echo "🤖 Building Remind Me PWA for Android..."
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

# Check for Android build tools
if [ -z "$ANDROID_HOME" ] && [ -z "$ANDROID_SDK_ROOT" ]; then
    echo "⚠️  Warning: ANDROID_HOME or ANDROID_SDK_ROOT not set"
    echo "   Android development requires Android SDK."
    echo "   Set ANDROID_HOME to your Android SDK path."
    echo ""
fi

echo "✅ Dioxus CLI found"
echo "✅ Project files found"
echo ""

# Build for Android
echo "🔨 Building Android app..."
dx build --platform android --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Android build completed successfully!"
    echo ""
    echo "Next steps:"
    echo "1. The APK should be in mobile/android/app/build/outputs/apk/"
    echo "2. Install on device: adb install <path-to-apk>"
    echo "3. Or open the Android project in Android Studio"
else
    echo ""
    echo "❌ Android build failed"
    echo "   Check the error messages above for details"
    exit 1
fi

