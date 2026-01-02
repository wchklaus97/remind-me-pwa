---
alwaysApply: false
description: Full stack architecture patterns for Dioxus applications
---
# Full Stack Architecture Patterns

## ⚠️ Status: Future Planning - Not Yet Implemented

**Note**: This document describes future architecture patterns. The current project is **client-side only**. These patterns are kept for reference when implementing server-side features.

## 🎯 Overview

This document provides patterns and guidelines for extending the client-side PWA to a full stack application using Dioxus 0.7's full stack capabilities.

## 🏗️ Architecture Options

### Option 1: Dioxus Full Stack (Recommended)

**Benefits:**
- ✅ Shared code between client and server
- ✅ Type-safe server functions
- ✅ SSR support
- ✅ Single codebase

**Structure:**
```
src/
├── lib.rs              # Shared library
├── client.rs           # Client entry (WASM)
├── server.rs           # Server entry
├── components/         # Shared UI components
├── server/            # Server-only code
│   ├── api.rs         # Server functions
│   ├── database.rs    # Database layer
│   └── handlers.rs    # Request handlers
└── shared/            # Shared code
    ├── models.rs      # Data models
    └── utils.rs       # Utilities
```

### Option 2: Separate Backend

**Benefits:**
- ✅ Clear separation of concerns
- ✅ Independent scaling
- ✅ Multiple frontend clients possible

**Structure:**
```
remind-me-pwa/
├── frontend/          # Dioxus PWA (current)
├── backend/          # Separate Rust server
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   ├── database/
│   │   └── models/
└── shared/           # Shared types (if using workspace)
```

## 📋 Dependencies for Full Stack

### Cargo.toml Configuration

```toml
[dependencies]
# Dioxus with full stack features
dioxus = { version = "0.6", features = ["web", "ssr", "fullstack"] }
dioxus-router = "0.6"
dioxus-fullstack = "0.6"

# Server backend
axum = "0.7"  # or warp, actix-web
tokio = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
# OR
# diesel = { version = "2.1", features = ["postgres"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🔧 Server Functions Pattern

### Basic Server Function

```rust
use dioxus::prelude::*;
use crate::shared::models::Reminder;

#[server]
pub async fn get_reminders() -> Result<Vec<Reminder>, ServerFnError> {
    // Server-side code here
    let db = get_database_connection().await?;
    let reminders = db.fetch_all().await?;
    Ok(reminders)
}
```

### Using Server Functions in Components

```rust
use dioxus::prelude::*;
use crate::server::api::get_reminders;

#[component]
fn ReminderList() -> Element {
    let mut reminders = use_signal(|| Vec::<Reminder>::new());
    
    use_effect(move || {
        spawn(async move {
            match get_reminders().await {
                Ok(loaded) => reminders.set(loaded),
                Err(e) => log::error!("Error: {}", e),
            }
        });
    });
    
    rsx! {
        div {
            for reminder in reminders().iter() {
                ReminderCard { reminder: reminder.clone() }
            }
        }
    }
}
```

## 🗄️ Database Patterns

### Connection Management

```rust
use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        Ok(Self { pool })
    }
    
    pub async fn get_reminders(&self) -> Result<Vec<Reminder>, sqlx::Error> {
        sqlx::query_as!(Reminder, "SELECT * FROM reminders")
            .fetch_all(&self.pool)
            .await
    }
}
```

### Migration Pattern

```rust
// migrations/001_create_reminders.sql
CREATE TABLE reminders (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date VARCHAR(255),
    completed BOOLEAN DEFAULT FALSE,
    created_at VARCHAR(255) NOT NULL
);
```

## 🚀 Deployment Patterns

### Full Stack Deployment

**Platform Options:**
- Fly.io (recommended for Rust)
- Railway
- Render
- Self-hosted (VPS)

**Configuration:**
```toml
# Dioxus.toml
[application]
default_platform = "fullstack"

[server]
address = "0.0.0.0"
port = 8080
```

### Hybrid Deployment

**Frontend**: Static hosting (GitHub Pages, Netlify)
**Backend**: Separate API server (Fly.io, Railway)
**Database**: Managed database (Supabase, PlanetScale)

## 🔄 Migration Strategy

### Gradual Migration

1. **Phase 1**: Add server functions alongside localStorage
2. **Phase 2**: Use feature flags to switch between storage methods
3. **Phase 3**: Migrate data from localStorage to database
4. **Phase 4**: Remove localStorage, use server exclusively

### Feature Flag Pattern

```rust
#[cfg(feature = "server")]
use crate::server::api::*;

#[cfg(not(feature = "server"))]
use crate::client::storage::*;

#[component]
fn App() -> Element {
    let reminders = if cfg!(feature = "server") {
        // Use server functions
        load_from_server().await
    } else {
        // Use localStorage
        load_from_storage()
    };
    
    // ...
}
```

## 📊 Best Practices

### DO:
- ✅ Use server functions for data operations
- ✅ Keep shared code in `shared/` module
- ✅ Use type-safe database queries (sqlx, diesel)
- ✅ Handle errors gracefully
- ✅ Use migrations for database schema
- ✅ Separate client and server code clearly

### DON'T:
- ❌ Don't mix client and server code without `#[cfg]`
- ❌ Don't expose sensitive data in server functions
- ❌ Don't forget to handle database connection errors
- ❌ Don't skip migrations
- ❌ Don't hardcode database URLs

## 🔐 Security Considerations

### Authentication

```rust
#[server]
pub async fn get_reminders() -> Result<Vec<Reminder>, ServerFnError> {
    // Verify authentication
    let user = get_current_user()?;
    
    // Return user-specific data
    let db = get_database().await?;
    db.get_reminders_for_user(&user.id).await
}
```

### Input Validation

```rust
#[server]
pub async fn create_reminder(title: String, description: String) -> Result<Reminder, ServerFnError> {
    // Validate input
    if title.is_empty() || title.len() > 255 {
        return Err(ServerFnError::new("Invalid title"));
    }
    
    // Create reminder
    // ...
}
```

## 📚 Resources

- [Dioxus Full Stack Guide](https://dioxuslabs.com/learn/0.6/fullstack)
- [Dioxus Server Functions](https://dioxuslabs.com/learn/0.6/fullstack/server-functions)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Axum Documentation](https://docs.rs/axum/)
