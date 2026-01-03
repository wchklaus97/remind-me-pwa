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
- 💾 Local storage persistence
- 📱 PWA support - install on your device
- ⚠️ Overdue reminders highlighting
- 🎨 Modern, responsive UI
- 🌐 Multi-language support (English, 简体中文, 繁體中文)
- 🎬 Media caching with shimmer loading
- 🧩 Modular component architecture
- 🧭 Locale-aware routing
- 📄 Reusable page templates

## Prerequisites

- Rust (latest stable version)
- Dioxus CLI: `cargo install dioxus-cli --locked`

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
│   │   ├── landing.rs    # Landing page
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

