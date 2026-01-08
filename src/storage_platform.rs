// Platform-agnostic storage abstraction
// This module provides a unified storage interface for both web (localStorage) and mobile (file system)

/// Platform storage trait for key-value storage
pub trait PlatformStorage {
    /// Get a value from storage by key
    fn get(key: &str) -> Option<String>;
    
    /// Set a value in storage by key
    fn set(key: &str, value: &str) -> Result<(), StorageError>;
}

/// Storage error type
#[derive(Debug, Clone)]
pub enum StorageError {
    Unavailable,
    SerializationFailed,
    SaveFailed,
}

// Web platform implementation using localStorage
#[cfg(target_arch = "wasm32")]
pub struct WebStorage;

#[cfg(target_arch = "wasm32")]
impl PlatformStorage for WebStorage {
    fn get(key: &str) -> Option<String> {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(data)) = storage.get_item(key) {
                    return Some(data);
                }
            }
        }
        None
    }
    
    fn set(key: &str, value: &str) -> Result<(), StorageError> {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Err(_) = storage.set_item(key, value) {
                    return Err(StorageError::SaveFailed);
                }
                return Ok(());
            }
        }
        Err(StorageError::Unavailable)
    }
}

// Mobile platform implementation using file system
#[cfg(any(target_os = "ios", target_os = "android"))]
pub struct MobileStorage;

#[cfg(any(target_os = "ios", target_os = "android"))]
impl PlatformStorage for MobileStorage {
    fn get(key: &str) -> Option<String> {
        use std::path::PathBuf;
        use std::fs;
        
        // Get app data directory
        if let Some(data_dir) = get_app_data_dir() {
            let file_path = data_dir.join(format!("{}.json", key));
            if let Ok(content) = fs::read_to_string(&file_path) {
                return Some(content);
            }
        }
        None
    }
    
    fn set(key: &str, value: &str) -> Result<(), StorageError> {
        use std::path::PathBuf;
        use std::fs;
        
        // Get app data directory
        if let Some(data_dir) = get_app_data_dir() {
            // Ensure directory exists
            if let Err(_) = fs::create_dir_all(&data_dir) {
                return Err(StorageError::Unavailable);
            }
            
            let file_path = data_dir.join(format!("{}.json", key));
            if let Err(_) = fs::write(&file_path, value) {
                return Err(StorageError::SaveFailed);
            }
            return Ok(());
        }
        Err(StorageError::Unavailable)
    }
}

#[cfg(any(target_os = "ios", target_os = "android"))]
fn get_app_data_dir() -> Option<std::path::PathBuf> {
    // For mobile, use the app's document directory
    // Dioxus mobile provides access to app directories via platform-specific APIs
    // 
    // Note: This implementation uses a simple file-based approach.
    // For production, consider using dioxus-mobile's native storage APIs
    // or platform-specific storage solutions (UserDefaults on iOS, SharedPreferences on Android)
    //
    // Current approach: Use a data directory relative to the app's executable location
    // This works for both iOS and Android in most cases
    
    // Try to get the executable directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let mut path = parent.to_path_buf();
            path.push("data");
            return Some(path);
        }
    }
    
    // Fallback: Use a standard location
    // On iOS: This would typically be in the app's Documents directory
    // On Android: This would typically be in the app's files directory
    // For now, use a relative path that should work in most cases
    Some(std::path::PathBuf::from("data"))
}

// Export the appropriate storage implementation based on platform
#[cfg(target_arch = "wasm32")]
pub use WebStorage as PlatformStorageImpl;

#[cfg(any(target_os = "ios", target_os = "android"))]
pub use MobileStorage as PlatformStorageImpl;

