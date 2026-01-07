# Remind Me PWA 🔔

![GitHub Actions Workflow](https://github.com/wchklaus97/remind-me-pwa/actions/workflows/github-pages-deploy.yml/badge.svg?branch=main)
![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust)
![Dioxus](https://img.shields.io/badge/dioxus-0.7-purple.svg)

A beautiful and functional Progressive Web App (PWA) built with Dioxus for managing reminders. Deploy it to GitHub Pages with ease!

## Features

- ✅ Create, edit, and delete reminders
- 📅 Set due dates and times
- 🎯 Filter reminders (All, Active, Completed)
- 🔍 Search and sort reminders
- 🏷️ Tag management system with color coding
- 📊 Multiple view modes (List, Card, Folder, Calendar)
- 📆 Calendar view with date grouping
- 💾 Local storage persistence
- 📱 PWA support - install on your device
- ⚠️ Overdue reminders highlighting
- 🎨 Modern, responsive UI with enhanced visual design
- ♿ Full accessibility support (ARIA labels, keyboard navigation)
- 🌐 Multi-language support (English, 简体中文, 繁體中文)
- 🎬 Media caching with shimmer loading
- 🧩 Modular component architecture
- 🧭 Locale-aware routing
- 📄 Reusable page templates

## Prerequisites

- Rust (latest stable version)
- Dioxus CLI: `cargo install dioxus-cli --locked`

### For Mobile Development

- **iOS**: Xcode (macOS only)
- **Android**: Android SDK (set `ANDROID_HOME` environment variable)

## Getting Started

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd remind-me-pwa
   ```

2. **Run the development server**
   ```bash
   dx serve
   ```

3. **Build for production**
   ```bash
   dx build --release
   ```

## Mobile App Development

The project supports building native iOS and Android apps using Dioxus Mobile while keeping the PWA build intact.

### Building for iOS

1. **Prerequisites**: Install Xcode from the App Store
2. **Build iOS app**:
   ```bash
   ./build-mobile-ios.sh
   ```
   Or manually:
   ```bash
   dx build --platform ios --release
   ```
3. **Open in Xcode**: The project will be in `mobile/ios/`
4. **Configure signing** in Xcode
5. **Run on simulator or device**

### Building for Android

1. **Prerequisites**: 
   - Install Android Studio
   - Set `ANDROID_HOME` environment variable
2. **Build Android app**:
   ```bash
   ./build-mobile-android.sh
   ```
   Or manually:
   ```bash
   dx build --platform android --release
   ```
3. **Install APK**: The APK will be in `mobile/android/app/build/outputs/apk/`
4. **Or open in Android Studio**: Open `mobile/android/` in Android Studio

### Mobile vs PWA

- **PWA**: Web build (WASM) - works in browsers, deployable to GitHub Pages
- **Mobile**: Native iOS/Android apps - uses the same codebase with platform-specific storage
- **Code Sharing**: ~95% of code is shared between web and mobile platforms
- **Storage**: Web uses localStorage, mobile uses file system storage

## SSR (Landing/Legal) + SPA (/app)

This repo supports **Option A**:
- **SSR** for public pages (Landing + Legal) to improve SEO
- **SPA** for `/app` (the reminder app) because it relies on browser APIs like localStorage

### Build + run SSR server (local)

1. **Build web assets** (required so the server can serve hashed CSS/JS/WASM files):

```bash
dx build --release --platform web
```

2. **Run the SSR server**:

```bash
cargo run --features server --bin server
```

Defaults:
- **URL**: `http://127.0.0.1:8080`
- **PUBLIC_DIR**: `target/dx/remind-me-pwa/release/web/public`

### Environment variables

- **PUBLIC_DIR**: path to the Dioxus web build output (must contain `index.html` + `assets/`)
- **HOST**: bind host (default `127.0.0.1`)
- **PORT**: bind port (default `8080`)
- **BASE_PATH**: optional subdirectory mount (example: `/remind-me-pwa`)

Example:

```bash
PUBLIC_DIR=target/dx/remind-me-pwa/release/web/public PORT=8080 cargo run --features server --bin server
```

### Hosting note

- **GitHub Pages**: static-only (no SSR). Use the default SPA build (`dx build`) + service worker.
- **SSR server**: requires a host that can run a Rust server binary (e.g. Fly.io / Render / Railway / VPS).

## Deployment to GitHub Pages

### Manual Deployment

1. **Build the project**
   ```bash
   dx bundle --out-dir docs
   ```

2. **Move files to docs directory**
   ```bash
   mv docs/public/* docs/
   rm -rf docs/public
   ```

3. **Create 404.html for client-side routing**
   ```bash
   cp docs/index.html docs/404.html
   ```

4. **Commit and push**
   ```bash
   git add docs/
   git commit -m "Deploy to GitHub Pages"
   git push origin main
   ```

5. **Enable GitHub Pages**
   - Go to your repository Settings → Pages
   - Set source to `docs` directory on `main` branch
   - Your app will be available at `https://yourusername.github.io/remind-me-pwa/`

**Note**: Manual deployment uses the `docs` folder on `main` branch. For automatic deployment, see the Automatic Deployment section below.

### Automatic Deployment (GitHub Actions)

The included `.github/workflows/github-pages-deploy.yml` will automatically build and deploy your app whenever you push to the `main` branch. The workflow:
- Builds the project with Dioxus CLI
- Optimizes WASM bundle with wasm-opt
- Deploys to `gh-pages` branch for GitHub Pages
- Creates/updates `release` branch for backup and reference

## Project Structure

```
remind-me-pwa/
├── src/
│   ├── main.rs          # Main application entry point
│   ├── app.rs            # Main App component and routing
│   ├── router.rs         # Routing logic with locale support
│   ├── i18n.rs           # Internationalization system
│   ├── components/       # Reusable components (organized by purpose)
│   │   ├── landing.rs    # Landing page (orchestrator)
│   │   ├── hero_section.rs # Hero section
│   │   ├── features_section.rs # Features grid
│   │   ├── workflow_section.rs # "How it works" section
│   │   ├── testimonials_section.rs # Testimonials carousel
│   │   ├── pricing_section.rs # Pricing section
│   │   ├── faq_section.rs # FAQ section
│   │   ├── final_cta_section.rs # Final CTA section
│   │   ├── landing_layout.rs # Navbar and footer
│   │   ├── reminder_app.rs # Main reminder app
│   │   ├── page_template.rs # Public page template
│   │   ├── legal.rs      # Legal pages (Privacy, Terms)
│   │   ├── language_switcher.rs # Language switcher
│   │   ├── media.rs      # Media cache components
│   │   ├── forms.rs      # Form components
│   │   ├── cards.rs      # Card components
│   │   ├── modals.rs     # Modal components
│   │   └── statistics.rs # Statistics display
│   └── services/         # Service modules
│       └── media_cache.rs # Media caching service
├── assets/
│   ├── css/             # Stylesheets (split for maintainability)
│   │   ├── base.css     # Reset, variables, base styles
│   │   ├── components.css # Reusable components
│   │   ├── app.css      # App-specific styles
│   │   ├── landing.css  # Landing page styles
│   │   ├── layout.css   # Navbar, footer, menu
│   │   ├── utilities.css # Utility classes
│   │   └── responsive.css # Media queries
│   ├── i18n/            # Internationalization translations
│   │   ├── en.json      # English translations
│   │   ├── zh-Hans.json # Simplified Chinese (简体中文)
│   │   └── zh-Hant.json # Traditional Chinese (繁體中文)
│   ├── manifest.json    # PWA manifest
│   └── sw.js            # Service worker
├── .cursor/
│   └── rules/           # Cursor AI rules and documentation
│       ├── core/        # Core development rules
│       ├── features/    # Feature-specific rules
│       └── skills.md    # Development skills reference
├── mobile/              # Mobile project files
│   ├── ios/            # iOS project (generated)
│   ├── android/        # Android project (generated)
│   └── README.md       # Mobile setup guide
├── build-mobile-ios.sh # iOS build script
├── build-mobile-android.sh # Android build script
├── Cargo.toml           # Rust dependencies
├── Dioxus.toml          # Dioxus configuration
└── README.md            # This file
```

## Configuration

Edit `Dioxus.toml` to change:
- App name
- Base path (for GitHub Pages)
- Output directory
- PWA settings

## Architecture Highlights

### Component Organization
- **Modular Architecture**: Components organized by purpose (pages, layouts, forms, cards, modals)
- **Reusable Templates**: `PublicPageTemplate` for consistent page layouts
- **Media Caching**: Shared cache manager with shimmer loading states
- **Internationalization**: Full i18n support with 3 languages

### Routing System
- **Locale-Aware URLs**: Routes include locale prefix (e.g., `/en/app`, `/zh-Hans/app`)
- **GitHub Pages Compatible**: Supports both path-based and hash-based routing
- **Automatic Locale Detection**: Detects locale from URL or browser settings

### CSS Organization
- **7 Modular Files**: Split for maintainability (base, components, app, landing, layout, utilities, responsive)
- **Mobile-First**: Responsive design with mobile-first approach
- **Consistent Styling**: Shared variables and utility classes

## PWA Features

- **Offline Support**: Works offline after first visit
- **Installable**: Can be installed on mobile and desktop
- **App-like Experience**: Standalone display mode
- **Service Worker**: Automatic caching of assets
- **Media Caching**: Efficient image/video loading with Cache Storage API

### PWA Icons

Icons are located in `assets/icons/app/`:
- `icon-192x192.avif` (192x192 pixels)
- `icon-512x512.avif` (512x512 pixels)
- `favicon-32x32.avif` and `favicon-16x16.avif`

## Browser Support

- Chrome/Edge (recommended)
- Firefox
- Safari (iOS 11.3+)
- Opera

## License

MIT License - feel free to use this project for your own purposes!

## Documentation

- **[QUICK_START.md](./QUICK_START.md)**: Quick-start guide for new developers
- **[CHANGELOG.md](./CHANGELOG.md)**: Detailed change history with dates
- **[DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md)**: Roadmap and planned features
- **[DEPLOYMENT.md](./DEPLOYMENT.md)**: Deployment guide
- **[.cursor/rules/](./.cursor/rules/)**: Development rules and patterns
  - **Core Rules**: Project structure, code formatting, best practices
  - **Feature Rules**: i18n, routing, media caching, page templates, storage, etc.
  - **Skills Reference**: Complete Dioxus PWA development guide

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before contributing, please:
1. Check [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for planned features
2. Follow the patterns in [.cursor/rules/](./.cursor/rules/)
3. Update [CHANGELOG.md](./CHANGELOG.md) with your changes

