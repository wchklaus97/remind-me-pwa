# Remind Me PWA 🔔

![GitHub Actions Workflow](https://img.shields.io/github/actions/workflow/status/wchklaus97/remind-me-pwa/github-pages-deploy.yml?branch=main&label=build&logo=github)
![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust)
![Dioxus](https://img.shields.io/badge/dioxus-0.6-purple.svg)

A beautiful and functional Progressive Web App (PWA) built with Dioxus for managing reminders. Deploy it to GitHub Pages with ease!

## Features

- ✅ Create, edit, and delete reminders
- 📅 Set due dates and times
- 🎯 Filter reminders (All, Active, Completed)
- 💾 Local storage persistence
- 📱 PWA support - install on your device
- ⚠️ Overdue reminders highlighting
- 🎨 Modern, responsive UI

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
│   └── main.rs          # Main application code
├── assets/
│   ├── style.css        # Styles
│   ├── manifest.json    # PWA manifest
│   └── sw.js           # Service worker
├── Cargo.toml          # Rust dependencies
├── Dioxus.toml         # Dioxus configuration
└── README.md           # This file
```

## Configuration

Edit `Dioxus.toml` to change:
- App name
- Base path (for GitHub Pages)
- Output directory
- PWA settings

## PWA Features

- **Offline Support**: Works offline after first visit
- **Installable**: Can be installed on mobile and desktop
- **App-like Experience**: Standalone display mode

### PWA Icons

To complete the PWA setup, you'll need to add icon files:
- `assets/icon-192.png` (192x192 pixels)
- `assets/icon-512.png` (512x512 pixels)

You can create these using any image editor or online tool. The icons should represent your reminder app (e.g., a bell or calendar icon).

## Browser Support

- Chrome/Edge (recommended)
- Firefox
- Safari (iOS 11.3+)
- Opera

## License

MIT License - feel free to use this project for your own purposes!

## Documentation

- **[CHANGELOG.md](./CHANGELOG.md)**: Detailed change history with dates
- **[DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md)**: Roadmap and planned features
- **[DEPLOYMENT.md](./DEPLOYMENT.md)**: Deployment guide
- **[.cursor/rules/](./.cursor/rules/)**: Development rules and patterns

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before contributing, please:
1. Check [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for planned features
2. Follow the patterns in [.cursor/rules/](./.cursor/rules/)
3. Update [CHANGELOG.md](./CHANGELOG.md) with your changes

