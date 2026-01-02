/// Deployment environment detection and utilities
/// 
/// This module provides functions to detect the deployment environment
/// (GitHub Pages vs normal production) and handle environment-specific
/// configurations like base paths and URLs.
/// 
/// GitHub Pages is a static hosting service that only serves static files,
/// so it requires hash-based routing. Normal production deployments
/// (Netlify, Vercel, etc.) support server-side routing and can use
/// path-based routing for better SEO.

/// Detect if we're running on GitHub Pages (static hosting)
/// 
/// GitHub Pages only supports static files, so we must use hash-based routing.
/// Normal production deployments (Netlify, Vercel, etc.) support server-side routing
/// and can use path-based routing for better SEO.
/// 
/// # Returns
/// 
/// - `true` if running on GitHub Pages (detected by `github.io` domain or base_path)
/// - `false` for normal production deployments or local development
/// 
/// # Examples
/// 
/// ```rust
/// if is_github_pages() {
///     // Use hash-based routing
///     update_url_with_hash(route, locale);
/// } else {
///     // Use path-based routing
///     update_url_with_path(route, locale);
/// }
/// ```
#[cfg(target_arch = "wasm32")]
pub fn is_github_pages() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(hostname) = window.location().hostname() {
            // GitHub Pages uses github.io domain
            if hostname.contains("github.io") {
                return true;
            }
        }
    }
    false
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_github_pages() -> bool {
    false
}

/// Get the base path for the application
/// 
/// For GitHub Pages subdirectory deployments, this returns the repository name
/// (e.g., "/remind-me-pwa"). For root deployments or other hosting, returns empty string.
/// 
/// # Returns
/// 
/// - `"/remind-me-pwa"` for GitHub Pages subdirectory deployment
/// - `""` for root deployments or other hosting
/// 
/// # Examples
/// 
/// ```rust
/// let base_path = get_base_path();
/// let asset_url = format!("{}/assets/image.png", base_path);
/// // GitHub Pages: "/remind-me-pwa/assets/image.png"
/// // Production: "/assets/image.png"
/// ```
#[cfg(target_arch = "wasm32")]
pub fn get_base_path() -> String {
    if let Some(window) = web_sys::window() {
        // On GitHub Pages the app is served from a subdirectory:
        //   https://<user>.github.io/<repo>/
        // So the base path is the first non-empty segment of pathname.
        // This avoids hardcoding the repository name.
        if let Ok(hostname) = window.location().hostname() {
            if hostname.contains("github.io") {
                if let Ok(pathname) = window.location().pathname() {
                    if let Some(first) = pathname
                        .trim_start_matches('/')
                        .split('/')
                        .find(|p| !p.is_empty())
                    {
                        return format!("/{}", first);
                    }
                }
            }
        }
    }
    String::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_base_path() -> String {
    String::new()
}

/// Get the base URL for absolute asset URLs
/// 
/// Returns the origin URL (protocol + hostname + port) for constructing
/// absolute URLs needed for meta tags (Open Graph, Twitter Cards, etc.).
/// 
/// # Returns
/// 
/// - Origin URL (e.g., `"https://wchklaus97.github.io"`)
/// - Falls back to GitHub Pages URL if origin can't be detected
/// 
/// # Examples
/// 
/// ```rust
/// let base_url = get_base_url();
/// let base_path = get_base_path();
/// let image_url = format!("{}{}/assets/image.png", base_url, base_path);
/// // Result: "https://wchklaus97.github.io/remind-me-pwa/assets/image.png"
/// ```
#[cfg(target_arch = "wasm32")]
pub fn get_base_url() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(origin) = window.location().origin() {
            return origin;
        }
    }
    // Fallback to GitHub Pages URL if we can't detect origin
    if is_github_pages() {
        "https://wchklaus97.github.io".to_string()
    } else {
        String::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_base_url() -> String {
    String::new()
}

/// Get the full base URL with path for constructing absolute asset URLs
/// 
/// Combines `get_base_url()` and `get_base_path()` for convenience.
/// 
/// # Returns
/// 
/// - Full base URL with path (e.g., `"https://wchklaus97.github.io/remind-me-pwa"`)
/// - Or just base URL if no base path (e.g., `"https://example.com"`)
/// 
/// # Examples
/// 
/// ```rust
/// let full_base = get_full_base_url();
/// let image_url = format!("{}/assets/image.png", full_base);
/// // GitHub Pages: "https://wchklaus97.github.io/remind-me-pwa/assets/image.png"
/// // Production: "https://example.com/assets/image.png"
/// ```
pub fn get_full_base_url() -> String {
    let base_url = get_base_url();
    let base_path = get_base_path();
    
    if base_path.is_empty() {
        base_url
    } else {
        format!("{}{}", base_url, base_path)
    }
}

/// Check if we're in a production environment
/// 
/// Returns `true` if not running on localhost (development).
/// 
/// # Returns
/// 
/// - `true` for production deployments (GitHub Pages, Netlify, Vercel, etc.)
/// - `false` for local development (localhost)
pub fn is_production() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(hostname) = window.location().hostname() {
                return hostname != "localhost" && hostname != "127.0.0.1";
            }
        }
    }
    false
}

