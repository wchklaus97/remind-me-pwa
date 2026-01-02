# Full Stack Migration Guide

## ⚠️ Status: Future Planning - Not Yet Implemented

**Note**: This document describes a future architecture. The current project is **client-side only** (localStorage, static hosting). This guide is kept for reference when implementing server-side features.

## 🎯 Current Architecture (Client-Side Only)

**Current State:**
- ✅ Client-side PWA (WASM)
- ✅ LocalStorage persistence
- ✅ Static hosting (GitHub Pages)
- ❌ No backend/server
- ❌ No API endpoints
- ❌ No database
- ❌ No server-side rendering

## 🏗️ Full Stack Architecture Options

### Option 1: Dioxus Full Stack (Recommended)

Dioxus 0.6 supports full stack development with:
- **SSR (Server-Side Rendering)**
- **API Routes** (Server Functions)
- **Shared code** between client and server
- **Multiple server backends** (Axum, Warp, etc.)

### Option 2: Separate Backend

- **Frontend**: Current Dioxus PWA
- **Backend**: Separate Rust server (Axum, Actix-web, etc.)
- **API**: REST or GraphQL
- **Database**: PostgreSQL, SQLite, MongoDB, etc.

---

## 📋 Migration Plan: Dioxus Full Stack

### Phase 1: Project Structure Restructuring

#### New Directory Structure

```
remind-me-pwa/
├── src/
│   ├── lib.rs              # Shared code (models, utilities)
│   ├── client.rs           # Client-side entry point
│   ├── server.rs           # Server-side entry point
│   ├── components/         # UI components (shared)
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── reminder_card.rs
│   │   └── add_form.rs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── api.rs          # API routes
│   │   ├── database.rs     # Database connection
│   │   └── handlers.rs     # Request handlers
│   └── shared/
│       ├── mod.rs
│       ├── models.rs       # Shared data models
│       └── storage.rs      # Storage utilities
├── server/
│   └── main.rs            # Server entry point (if separate)
├── Cargo.toml              # Updated dependencies
├── Dioxus.toml             # Updated configuration
└── ...
```

### Phase 2: Update Cargo.toml

```toml
[package]
name = "remind-me-pwa"
version = "0.0.1"
edition = "2021"

[dependencies]
# Dioxus with full stack features
dioxus = { version = "0.6", features = ["web", "ssr", "fullstack"] }
dioxus-router = "0.6"

# Server backend (choose one)
dioxus-fullstack = "0.6"  # For Axum backend
# OR
# dioxus-fullstack = { version = "0.6", features = ["warp"] }  # For Warp

# Database (choose one)
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "sqlite"] }
# OR
# diesel = { version = "2.1", features = ["postgres", "sqlite"] }
# OR
# surrealdb = "1.0"

# Server runtime
tokio = { version = "1", features = ["full"] }
axum = "0.7"  # If using Axum backend

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/time
chrono = { version = "0.4", features = ["serde", "clock"] }

# Client-side (WASM)
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Storage"] }

# Server-side
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
# Server-only dependencies here
```

### Phase 3: Create Shared Code Structure

#### `src/lib.rs` - Shared Library

```rust
pub mod components;
pub mod shared;
pub mod server;

// Re-export commonly used items
pub use shared::models::Reminder;
pub use shared::models::ReminderFilter;
```

#### `src/shared/models.rs` - Shared Data Models

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
    pub created_at: String,
    pub user_id: Option<String>,  // For multi-user support
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReminderFilter {
    All,
    Active,
    Completed,
}

// API Request/Response types
#[derive(Deserialize)]
pub struct CreateReminderRequest {
    pub title: String,
    pub description: String,
    pub due_date: String,
}

#[derive(Serialize)]
pub struct ReminderResponse {
    pub reminder: Reminder,
}

#[derive(Serialize)]
pub struct RemindersResponse {
    pub reminders: Vec<Reminder>,
}
```

### Phase 4: Server-Side Code

#### `src/server/mod.rs`

```rust
pub mod api;
pub mod database;
pub mod handlers;

#[cfg(not(target_arch = "wasm32"))]
pub use api::*;
#[cfg(not(target_arch = "wasm32"))]
pub use database::*;
```

#### `src/server/database.rs` - Database Connection

```rust
use sqlx::{PgPool, SqlitePool};
use crate::shared::models::Reminder;

pub struct Database {
    pool: PgPool,  // or SqlitePool
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_reminders(&self, user_id: Option<&str>) -> Result<Vec<Reminder>, sqlx::Error> {
        let reminders = sqlx::query_as!(
            Reminder,
            "SELECT id, title, description, due_date, completed, created_at, user_id 
             FROM reminders 
             WHERE user_id = $1 OR user_id IS NULL
             ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(reminders)
    }

    pub async fn create_reminder(&self, reminder: &Reminder) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO reminders (id, title, description, due_date, completed, created_at, user_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            reminder.id,
            reminder.title,
            reminder.description,
            reminder.due_date,
            reminder.completed,
            reminder.created_at,
            reminder.user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn update_reminder(&self, reminder: &Reminder) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE reminders 
             SET title = $1, description = $2, due_date = $3, completed = $4
             WHERE id = $5",
            reminder.title,
            reminder.description,
            reminder.due_date,
            reminder.completed,
            reminder.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn delete_reminder(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM reminders WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}
```

#### `src/server/api.rs` - Server Functions (Dioxus Server Functions)

```rust
use dioxus::prelude::*;
use crate::shared::models::{Reminder, CreateReminderRequest};

// Server Functions - automatically generate API endpoints
#[server]
pub async fn get_reminders() -> Result<Vec<Reminder>, ServerFnError> {
    // Get database connection from context
    let db = use_server_context::<Database>()?;
    let reminders = db.get_reminders(None).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    Ok(reminders)
}

#[server]
pub async fn create_reminder(req: CreateReminderRequest) -> Result<Reminder, ServerFnError> {
    let db = use_server_context::<Database>()?;
    
    let reminder = Reminder {
        id: format!("reminder_{}", chrono::Utc::now().timestamp_millis()),
        title: req.title,
        description: req.description,
        due_date: req.due_date,
        completed: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        user_id: None,
    };
    
    db.create_reminder(&reminder).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    
    Ok(reminder)
}

#[server]
pub async fn update_reminder(reminder: Reminder) -> Result<(), ServerFnError> {
    let db = use_server_context::<Database>()?;
    db.update_reminder(&reminder).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    Ok(())
}

#[server]
pub async fn delete_reminder(id: String) -> Result<(), ServerFnError> {
    let db = use_server_context::<Database>()?;
    db.delete_reminder(&id).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    Ok(())
}
```

### Phase 5: Update Client Code

#### `src/client.rs` - Client Entry Point

```rust
use dioxus::prelude::*;
use dioxus_web::Config;

fn main() {
    dioxus_web::launch_with_props(
        App,
        Config::default(),
        (),
    );
}
```

#### Update Components to Use Server Functions

```rust
use dioxus::prelude::*;
use crate::server::api::*;
use crate::shared::models::Reminder;

#[component]
fn App() -> Element {
    let mut reminders = use_signal(|| Vec::<Reminder>::new());
    
    // Load reminders from server on mount
    use_effect(move || {
        spawn(async move {
            match get_reminders().await {
                Ok(loaded) => reminders.set(loaded),
                Err(e) => log::error!("Failed to load reminders: {}", e),
            }
        });
    });
    
    rsx! {
        div {
            // ... existing UI
        }
    }
}
```

### Phase 6: Server Entry Point

#### `src/server/main.rs` or `server/main.rs`

```rust
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use remind_me_pwa::server::Database;

#[tokio::main]
async fn main() {
    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = Database::new(&database_url).await
        .expect("Failed to connect to database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await
        .expect("Failed to run migrations");
    
    // Launch server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    
    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(
                axum::Router::new()
                    .serve_dioxus_application(ServeConfig::default(), App)
                    .with_state(db)
                    .into_make_service()
            )
            .await
            .unwrap();
    });
    
    println!("Server running on http://localhost:8080");
}
```

### Phase 7: Database Migrations

#### `migrations/001_create_reminders.sql`

```sql
CREATE TABLE reminders (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date VARCHAR(255),
    completed BOOLEAN DEFAULT FALSE,
    created_at VARCHAR(255) NOT NULL,
    user_id VARCHAR(255),
    INDEX idx_user_id (user_id),
    INDEX idx_created_at (created_at)
);
```

### Phase 8: Update Dioxus.toml

```toml
[application]
name = "Remind Me PWA"
default_platform = "fullstack"  # Changed from "web"

[web.app]
base_path = ""
out_dir = "docs"

[server]
address = "0.0.0.0"
port = 8080

[web.pwa]
enabled = true
manifest = "assets/manifest.json"
service_worker = "assets/sw.js"
```

---

## 🚀 Deployment Options for Full Stack

### Option 1: Self-Hosted Server

- **Platform**: VPS (DigitalOcean, Linode, AWS EC2)
- **Requirements**: 
  - Rust runtime
  - Database (PostgreSQL/SQLite)
  - Reverse proxy (Nginx)
- **Deployment**: 
  - Build binary: `cargo build --release`
  - Run as systemd service
  - Use Nginx as reverse proxy

### Option 2: Cloud Platforms

- **Fly.io**: Native Rust support, easy deployment
- **Railway**: Simple deployment, database included
- **Render**: Easy setup, PostgreSQL support
- **Heroku**: Traditional PaaS (if still available)

### Option 3: Hybrid Approach

- **Frontend**: Static files (GitHub Pages, Netlify, Vercel)
- **Backend**: Separate API server (Fly.io, Railway, etc.)
- **Database**: Managed database (Supabase, PlanetScale, etc.)

---

## 📊 Migration Checklist

### Phase 1: Structure Setup
- [ ] Create new directory structure
- [ ] Split code into `lib.rs`, `client.rs`, `server.rs`
- [ ] Move components to `components/` directory
- [ ] Create `shared/` module for shared code

### Phase 2: Dependencies
- [ ] Update `Cargo.toml` with full stack features
- [ ] Add database dependencies
- [ ] Add server runtime dependencies
- [ ] Update `Dioxus.toml` configuration

### Phase 3: Server Implementation
- [ ] Create database connection module
- [ ] Implement server functions (API routes)
- [ ] Create database migrations
- [ ] Set up server entry point

### Phase 4: Client Updates
- [ ] Update components to use server functions
- [ ] Remove localStorage persistence (or keep as fallback)
- [ ] Add error handling for API calls
- [ ] Add loading states

### Phase 5: Testing
- [ ] Test server functions locally
- [ ] Test database operations
- [ ] Test client-server communication
- [ ] Test error handling

### Phase 6: Deployment
- [ ] Set up production database
- [ ] Configure environment variables
- [ ] Deploy server
- [ ] Update frontend to point to production API
- [ ] Test end-to-end

---

## 🔄 Migration Strategy: Gradual Migration

### Step 1: Keep Current Structure Working
- Maintain current client-side code
- Keep localStorage as fallback

### Step 2: Add Server Functions Alongside
- Implement server functions
- Use feature flags to switch between localStorage and server
- Test both paths

### Step 3: Migrate Data
- Create migration script
- Move data from localStorage to database
- Verify data integrity

### Step 4: Remove Client-Side Storage
- Remove localStorage code
- Use server functions exclusively
- Update error handling

---

## 📚 Resources

- [Dioxus Full Stack Guide](https://dioxuslabs.com/learn/0.6/fullstack)
- [Dioxus Server Functions](https://dioxuslabs.com/learn/0.6/fullstack/server-functions)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Axum Documentation](https://docs.rs/axum/)

---

## ⚠️ Important Considerations

1. **Data Migration**: Plan how to migrate existing localStorage data
2. **Authentication**: Add user authentication for multi-user support
3. **Offline Support**: Consider keeping service worker for offline mode
4. **Performance**: Server-side rendering vs client-side rendering trade-offs
5. **Cost**: Server hosting costs vs free static hosting
6. **Complexity**: Full stack adds complexity - ensure it's necessary

---

**Next Steps**: Choose your migration path and start with Phase 1!

