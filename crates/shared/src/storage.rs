// Storage trait definition (platform-agnostic)
use serde::{Deserialize, Serialize};

/// Storage error type
#[derive(Debug, Clone)]
pub enum StorageError {
    Unavailable,
    SerializationFailed,
    SaveFailed,
}

/// Platform storage trait for key-value storage
pub trait PlatformStorage {
    /// Get a value from storage by key
    fn get(key: &str) -> Option<String>;
    
    /// Set a value in storage by key
    fn set(key: &str, value: &str) -> Result<(), StorageError>;
}

// Storage keys
pub const REMINDERS_V2_KEY: &str = "reminders_v2";
pub const REMINDERS_V1_KEY: &str = "reminders";
pub const TAGS_V1_KEY: &str = "tags_v1";

