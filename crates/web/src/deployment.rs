//! Deployment environment detection and utilities for web platform
//! 
//! This module provides functions to detect the deployment environment
//! (GitHub Pages vs normal production) and handle environment-specific
//! configurations like base paths and URLs.

/// Detect if we're running on GitHub Pages (static hosting)
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

/// Get the base path for the application
/// 
/// For GitHub Pages subdirectory deployments, this returns the repository name
/// (e.g., "/remind-me-pwa"). For root deployments or other hosting, returns empty string.
pub fn get_base_path() -> String {
    if let Some(window) = web_sys::window() {
        // On GitHub Pages the app is served from a subdirectory:
        //   https://<user>.github.io/<repo>/
        // So the base path is the first non-empty segment of pathname.
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

/// Get the base URL for absolute asset URLs
/// 
/// Returns the origin URL (protocol + hostname + port) for constructing
/// absolute URLs needed for meta tags (Open Graph, Twitter Cards, etc.).
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

