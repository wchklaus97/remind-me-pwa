# SEO Configuration for Remind Me PWA

## Overview

This document describes the SEO configuration and settings for the background images and overall site SEO.

## Background Images

The app uses two optimized AVIF background images:

- **Desktop**: `assets/images/backgrounds/desktop.avif` (1584x672px)
- **Mobile**: `assets/images/backgrounds/mobile.avif` (768x1376px)

These images are used responsively:
- Mobile viewports (< 768px): `mobile.avif`
- Desktop viewports (≥ 768px): `desktop.avif`

## SEO Meta Tags

The app includes comprehensive SEO meta tags added via JavaScript in `src/main.rs`:

### Basic SEO Tags
- `description`: "A simple and elegant reminder app to help you stay organized"
- `keywords`: "reminder app, PWA, task manager, productivity, offline app, progressive web app"
- `author`: "Remind Me PWA"
- `robots`: "index, follow"
- `theme-color`: "#5e5eb4" (matches background gradient)

### Open Graph Tags (for Facebook, LinkedIn, etc.)
- `og:type`: "website"
- `og:title`: "Remind Me PWA - Your Personal Reminder Assistant"
- `og:description`: "A simple and elegant reminder app to help you stay organized"
- `og:url`: Auto-detected based on current location
- `og:image`: `/assets/images/backgrounds/desktop.avif` (desktop version for social sharing)
- `og:image:type`: "image/avif"
- `og:image:width`: "1584"
- `og:image:height`: "672"
- `og:image:alt`: "Remind Me PWA - Beautiful gradient background"
- `og:site_name`: "Remind Me PWA"
- `og:locale`: "en_US"

### Twitter Card Tags
- `twitter:card`: "summary_large_image"
- `twitter:title`: "Remind Me PWA - Your Personal Reminder Assistant"
- `twitter:description`: "A simple and elegant reminder app to help you stay organized"
- `twitter:image`: `/assets/images/backgrounds/desktop.avif`
- `twitter:image:alt`: "Remind Me PWA - Beautiful gradient background"

## Manifest.json Configuration

The PWA manifest includes:
- `background_color`: "#5e5eb4" (matches gradient start color)
- `theme_color`: "#5e5eb4" (matches gradient start color)

## robots.txt

Located at `assets/robots.txt`:
- Allows all crawlers
- Disallows `/wasm/` directory
- Points to sitemap location

## sitemap.xml

Located at `assets/sitemap.xml`:
- Includes main landing page
- Includes app page (#app route)
- Updated lastmod dates

## Testing SEO

### Validate Meta Tags
1. Use [Facebook Sharing Debugger](https://developers.facebook.com/tools/debug/) to test Open Graph tags
2. Use [Twitter Card Validator](https://cards-dev.twitter.com/validator) to test Twitter Cards
3. Use [LinkedIn Post Inspector](https://www.linkedin.com/post-inspector/) for LinkedIn sharing

### Validate Structured Data
- Use [Google Rich Results Test](https://search.google.com/test/rich-results)
- Use [Schema.org Validator](https://validator.schema.org/)

### Check Image Optimization
- Images are AVIF format for optimal compression
- Desktop image: 8.3KB (original PNG was 641KB)
- Mobile image: 6.9KB (original PNG was 546KB)
- Both images use `background-size: cover` for responsive display

## Best Practices Applied

1. ✅ **Optimized Images**: Using AVIF format for 98%+ size reduction
2. ✅ **Responsive Images**: Different images for mobile/desktop viewports
3. ✅ **Proper Alt Text**: All images have descriptive alt attributes
4. ✅ **Open Graph**: Complete OG tags for social sharing
5. ✅ **Twitter Cards**: Large image card for better visibility
6. ✅ **Theme Colors**: Matching theme-color for browser UI
7. ✅ **Sitemap**: Proper sitemap for search engine indexing
8. ✅ **robots.txt**: Proper crawler directives

## Future Improvements

- [ ] Add JSON-LD structured data for better search results
- [ ] Add image preloading for faster LCP
- [ ] Consider adding WebP fallback for older browsers
- [ ] Add language alternates if supporting multiple languages

