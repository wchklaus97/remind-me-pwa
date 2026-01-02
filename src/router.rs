#[derive(Clone, PartialEq)]
pub enum Route {
    Landing,
    App,
}

impl Route {
    pub fn from_path(path: &str) -> (Route, String) {
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        
        // Check for locale prefix: /en/app, /zh/app, etc.
        if parts.len() >= 2 && !parts[0].is_empty() {
            let locale = parts[0].to_string();
            let route_str = parts[1];
            
            match route_str {
                "app" => return (Route::App, locale),
                _ => return (Route::Landing, locale),
            }
        }
        
        // Check for simple paths: /app, /#app
        if path.contains("/app") || path.contains("#app") {
            return (Route::App, "en".to_string());
        }
        
        // Default to landing page with English
        (Route::Landing, "en".to_string())
    }
    
    #[allow(dead_code)] // Reserved for future use (e.g., server-side routing)
    pub fn to_path(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("/{}/", locale),
            Route::App => format!("/{}/app", locale),
        }
    }
    
    pub fn to_hash(&self, locale: &str) -> String {
        match self {
            Route::Landing => format!("#/{}/", locale),
            Route::App => format!("#/{}/app", locale),
        }
    }
}

pub fn get_initial_route() -> (Route, String) {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        // Extract path from URL
        if let Ok(pathname) = location.pathname() {
            return Route::from_path(&pathname);
        }
        // Fallback to hash
        if let Ok(hash) = location.hash() {
            let path = hash.trim_start_matches('#');
            return Route::from_path(path);
        }
        // Try to get locale from localStorage (i18nrs storage)
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(saved_locale)) = storage.get_item("remind-me-locale") {
                return (Route::Landing, saved_locale);
            }
        }
    }
    (Route::Landing, "en".to_string())
}

pub fn update_url(route: &Route, locale: &str) {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        let hash = route.to_hash(locale);
        let _ = location.set_hash(&hash);
    }
}
