# Assets Directory

This directory contains all static assets for the Remind Me PWA application.

## Directory Structure

```
assets/
├── images/          # Image files (PNG, JPG, SVG, WebP)
│   ├── backgrounds/ # Background images
│   ├── logos/       # Logo images
│   └── misc/        # Other images
├── icons/           # Icon files (SVG, PNG, ICO)
│   ├── app/         # Application icons (for PWA)
│   └── ui/          # UI icons (buttons, actions, etc.)
├── videos/          # Video files (MP4, WebM)
├── fonts/           # Custom font files (if needed)
├── css/             # Stylesheets (split for better maintainability)
│   ├── base.css     # Reset, variables, base HTML/body styles
│   ├── components.css # Reusable components (buttons, forms, tabs)
│   ├── app.css      # App-specific styles (reminder app)
│   ├── landing.css  # Landing page styles
│   ├── layout.css   # Navbar, footer, menu styles
│   ├── utilities.css # Utility classes
│   └── responsive.css # Media queries and responsive design
├── style.css        # Legacy stylesheet (kept for reference, not used)
├── manifest.json    # PWA manifest
├── sw.js            # Service worker
├── robots.txt       # SEO robots file
└── sitemap.xml      # SEO sitemap
```

## Usage

### In Dioxus Components

All assets are served from the root `/assets/` path:

```rust
// Images
img { src: "/assets/images/logos/app-logo.png" }

// Icons
img { src: "/assets/icons/ui/check.svg" }

// Videos
video {
    src: "/assets/videos/intro.mp4",
    // ...
}
```

### In CSS

```css
/* Background image */
.hero {
    background-image: url('/assets/images/backgrounds/hero-bg.jpg');
}

/* Icon in CSS */
.icon-check {
    background-image: url('/assets/icons/ui/check.svg');
}
```

## File Organization Guidelines

### Images (`images/`)
- **backgrounds/**: Background images, gradients, patterns
- **logos/**: Brand logos, app icons (larger sizes)
- **misc/**: Other images that don't fit categories

### Icons (`icons/`)
- **app/**: PWA app icons (required sizes: 192x192, 512x512)
  - `icon-192x192.png`
  - `icon-512x512.png`
- **ui/**: UI element icons (buttons, status indicators, etc.)
  - Use SVG when possible for scalability
  - Use PNG for complex icons

### Videos (`videos/`)
- Keep file sizes small for web
- Use WebM format when possible for better compression
- Provide MP4 fallback for compatibility

## Best Practices

1. **Naming Convention**: Use kebab-case for file names (`app-logo.png`, `check-icon.svg`)
2. **Optimization**: 
   - Compress images before adding
   - Use appropriate formats (SVG for icons, WebP for photos)
   - Keep file sizes small for faster loading
3. **Responsive Images**: Consider multiple sizes for different viewports
4. **Accessibility**: Always include alt text for images in components
5. **PWA Icons**: Ensure app icons are in `icons/app/` with proper sizes

## Service Worker Caching

All assets in this directory are automatically cached by the service worker (`sw.js`). Make sure to update the service worker version when adding new assets to force cache refresh.

