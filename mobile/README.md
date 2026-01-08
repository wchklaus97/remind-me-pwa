# Mobile Project Structure

This directory contains platform-specific files for iOS and Android mobile apps.

## iOS

The iOS project files will be generated when building for iOS. The project uses the bundle ID configured in `Dioxus.toml`:
- Bundle ID: `com.remindme.app`
- Display Name: `Remind Me`

## Android

The Android project files will be generated when building for Android. The project uses the package name configured in `Dioxus.toml`:
- Package Name: `com.remindme.app`
- App Name: `Remind Me`

## Building

See the build scripts in the project root:
- `build-mobile-ios.sh` - Build iOS app
- `build-mobile-android.sh` - Build Android app

## Notes

The mobile projects are managed by Dioxus Mobile. The actual project files (Xcode project, Android Gradle project) are typically generated during the build process or may require additional setup depending on your Dioxus version.

