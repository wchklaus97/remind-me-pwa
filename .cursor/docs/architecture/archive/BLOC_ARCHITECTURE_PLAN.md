# BLoC Architecture + Full Stack Migration Plan

## 🎯 核心设计理念

结合 Flutter BLoC 模式和渐进式全栈迁移，创建可扩展的架构：

1. **BLoC 模式**：业务逻辑与 UI 分离
2. **渐进迁移**：客户端优先，服务器可选
3. **存储抽象**：IndexedDB → PostgreSQL 平滑过渡
4. **状态管理**：使用 Dioxus Signals + BLoC 模式

## 🏗️ 推荐架构：BLoC + Repository Pattern

### 架构图

```
┌─────────────────────────────────────────┐
│           Presentation Layer           │ ← UI 组件层
│  ┌──────────┐      ┌──────────┐       │
│  │Components│      │  Pages   │       │
│  └────┬─────┘      └────┬─────┘       │
│       │                 │             │
│       └────────┬────────┘             │
│                ↓                       │
│  ┌──────────────────────────┐         │
│  │    BLoC Provider         │         │
│  │  (use_bloc hook)         │         │
│  └──────────────────────────┘         │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│            BLoC Layer                   │ ← 业务逻辑层
│  ┌──────────┐      ┌──────────┐       │
│  │ TodoBloc │      │FilterBloc│       │
│  └────┬─────┘      └────┬─────┘       │
│       │                 │             │
│       └────────┬────────┘             │
│                ↓                       │
│  ┌──────────────────────────┐         │
│  │    Event Handler         │         │
│  │  (Event → State)         │         │
│  └──────────────────────────┘         │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│         Repository Layer                │ ← 数据访问层
│  ┌──────────┐      ┌──────────┐       │
│  │TodoRepo  │      │SyncRepo  │       │
│  └────┬─────┘      └────┬─────┘       │
│       │                 │             │
│       └────────┬────────┘             │
│                ↓                       │
│  ┌──────────────────────────┐         │
│  │  Repository Interface    │         │
│  └──────────────────────────┘         │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│         Data Source Layer               │ ← 数据源层
│  ┌──────────┐      ┌──────────┐       │
│  │IndexedDB │      │PostgreSQL │       │
│  │ (Local)  │      │ (Remote)  │       │
│  └──────────┘      └──────────┘       │
└─────────────────────────────────────────┘
```

## 📁 目录结构

```
remind-me-pwa/
├── src/
│   ├── main.rs                    # 应用入口（平台分发）
│   ├── lib.rs                     # 库入口
│   │
│   ├── core/                      # 核心架构
│   │   ├── bloc/                  # BLoC 实现
│   │   │   ├── mod.rs
│   │   │   ├── base_bloc.rs       # 基础 BLoC trait
│   │   │   ├── bloc_provider.rs   # BLoC Provider
│   │   │   └── use_bloc.rs        # use_bloc hook
│   │   │
│   │   ├── models/                 # 领域模型
│   │   │   ├── mod.rs
│   │   │   ├── reminder.rs        # Reminder 实体
│   │   │   ├── filter.rs          # Filter 枚举
│   │   │   └── app_state.rs       # 应用状态
│   │   │
│   │   ├── repositories/           # 仓库接口
│   │   │   ├── mod.rs
│   │   │   └── reminder_repository.rs  # 仓库 trait
│   │   │
│   │   └── errors/                 # 错误类型
│   │       ├── mod.rs
│   │       └── app_error.rs
│   │
│   ├── data/                      # 数据层实现
│   │   ├── repositories/           # 仓库实现
│   │   │   ├── mod.rs
│   │   │   ├── local_repository.rs     # IndexedDB 实现
│   │   │   ├── remote_repository.rs    # PostgreSQL 实现
│   │   │   └── sync_repository.rs      # 同步仓库
│   │   │
│   │   ├── datasources/            # 数据源
│   │   │   ├── mod.rs
│   │   │   ├── indexeddb/          # IndexedDB 数据源
│   │   │   │   ├── mod.rs
│   │   │   │   ├── indexeddb_client.rs
│   │   │   │   └── reminder_store.rs
│   │   │   │
│   │   │   └── postgres/           # PostgreSQL 数据源（未来）
│   │   │       ├── mod.rs
│   │   │       ├── postgres_client.rs
│   │   │       └── migrations/
│   │   │
│   │   └── sync/                   # 同步逻辑
│   │       ├── mod.rs
│   │       ├── sync_manager.rs
│   │       └── conflict_resolver.rs
│   │
│   ├── features/                   # 功能模块
│   │   └── reminders/              # Reminder 功能
│   │       ├── mod.rs
│   │       ├── bloc/                # 功能 BLoC
│   │       │   ├── mod.rs
│   │       │   ├── reminder_bloc.rs
│   │       │   └── reminder_state.rs
│   │       │
│   │       ├── events/              # BLoC 事件
│   │       │   ├── mod.rs
│   │       │   └── reminder_event.rs
│   │       │
│   │       ├── components/          # UI 组件
│   │       │   ├── mod.rs
│   │       │   ├── reminder_card.rs
│   │       │   ├── reminder_form.rs
│   │       │   └── reminder_list.rs
│   │       │
│   │       └── pages/               # 页面
│   │           ├── mod.rs
│   │           └── reminder_page.rs
│   │
│   ├── client/                     # 客户端代码
│   │   ├── main.rs                 # WASM 入口
│   │   └── app.rs                  # 客户端应用
│   │
│   ├── server/                     # 服务端代码（可选）
│   │   ├── main.rs                 # 服务器入口
│   │   ├── api/                    # API 路由
│   │   │   ├── mod.rs
│   │   │   └── reminder_api.rs
│   │   │
│   │   └── database/               # 数据库配置
│   │       ├── mod.rs
│   │       ├── connection.rs
│   │       └── migrations/
│   │
│   └── shared/                     # 共享代码
│       ├── mod.rs
│       ├── config.rs               # 配置管理
│       └── utils.rs                # 工具函数
│
├── crates/
│   └── ui/                         # UI 组件库（之前创建）
│
├── migrations/                     # 数据库迁移
│   ├── 001_create_reminders.sql
│   └── 002_add_sync_fields.sql
│
└── Cargo.toml
```

## 🔧 Cargo.toml 配置

```toml
[package]
name = "remind-me-pwa"
version = "0.0.1"
edition = "2021"
resolver = "2"

[features]
default = ["client"]  # 默认构建客户端
client = ["dioxus/web", "dioxus/router", "indexeddb"]
server = ["dioxus/ssr", "dioxus/fullstack", "axum", "sqlx/postgres"]
sync = ["server", "serde", "chrono"]
offline = ["indexeddb", "wasm-bindgen"]

[dependencies]
# Dioxus
dioxus = { version = "0.6", default-features = false }
dioxus-router = "0.6"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/Time
chrono = { version = "0.4", features = ["serde", "clock"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Async
futures = "0.3"
async-channel = "2.0"
tokio = { version = "1.0", features = ["full"] }

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

# Logging
tracing = "0.1"
tracing-wasm = "0.1"

# Client-side dependencies (WASM)
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
gloo-storage = "0.3"
indexed-db = "0.4"
web-sys = { version = "0.3", features = ["Window", "Storage", "Location", "Document"] }
dioxus-web = { version = "0.6", features = ["hydrate"] }

# Server-side dependencies
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
dioxus-fullstack = { version = "0.6", features = ["axum"] }
axum = "0.7"
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "sqlite", "uuid", "chrono", "offline"] }
tower-http = { version = "0.5", features = ["cors", "compression"] }

[dev-dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "sqlite", "migrate", "offline"] }
```

## 📋 核心实现

### 1. Base BLoC Trait

```rust
// core/bloc/base_bloc.rs
use dioxus::prelude::*;
use std::sync::Arc;

pub trait Bloc<T, E>: Send + Sync + 'static
where
    T: Clone + PartialEq + 'static,
    E: Clone + Send + 'static,
{
    /// 获取当前状态
    fn state(&self) -> ReadOnlySignal<T>;
    
    /// 分发事件
    fn dispatch(&self, event: E);
    
    /// 添加副作用（异步操作）
    fn add_effect<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static;
    
    /// 订阅状态变化
    fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&T) + Send + Sync + 'static;
}
```

### 2. BLoC Provider

```rust
// core/bloc/bloc_provider.rs
use dioxus::prelude::*;
use std::sync::Arc;
use crate::core::bloc::base_bloc::Bloc;

pub struct BlocProvider<T, E> {
    bloc: Arc<dyn Bloc<T, E>>,
}

impl<T, E> BlocProvider<T, E>
where
    T: Clone + PartialEq + 'static,
    E: Clone + Send + 'static,
{
    pub fn new(bloc: Arc<dyn Bloc<T, E>>) -> Self {
        Self { bloc }
    }
    
    pub fn bloc(&self) -> &Arc<dyn Bloc<T, E>> {
        &self.bloc
    }
}

/// use_bloc hook
pub fn use_bloc<T, E>(cx: Scope) -> Arc<dyn Bloc<T, E>>
where
    T: Clone + PartialEq + 'static,
    E: Clone + Send + 'static,
{
    use_context::<BlocProvider<T, E>>(cx)
        .expect("BlocProvider not found")
        .bloc()
        .clone()
}
```

### 3. Reminder BLoC 实现

```rust
// features/reminders/bloc/reminder_bloc.rs
use dioxus::prelude::*;
use std::sync::Arc;
use async_channel::{Receiver, Sender};
use futures::StreamExt;
use crate::core::bloc::base_bloc::Bloc;
use crate::core::repositories::reminder_repository::ReminderRepository;
use crate::features::reminders::events::reminder_event::ReminderEvent;
use crate::features::reminders::bloc::reminder_state::ReminderState;

pub struct ReminderBloc {
    state: Signal<ReminderState>,
    event_tx: Sender<ReminderEvent>,
    repository: Arc<dyn ReminderRepository>,
}

impl ReminderBloc {
    pub fn new(
        cx: Scope,
        repository: Arc<dyn ReminderRepository>,
    ) -> Arc<Self> {
        let state = use_signal(cx, || ReminderState::initial());
        let (event_tx, event_rx) = async_channel::unbounded();
        
        let bloc = Arc::new(Self {
            state,
            event_tx,
            repository,
        });
        
        // 启动事件处理循环
        let bloc_clone = bloc.clone();
        spawn(async move {
            bloc_clone.handle_events(event_rx).await;
        });
        
        bloc
    }
    
    async fn handle_events(&self, mut event_rx: Receiver<ReminderEvent>) {
        while let Ok(event) = event_rx.next().await {
            match event {
                ReminderEvent::Load => {
                    self.state.write().loading = true;
                    match self.repository.get_all().await {
                        Ok(reminders) => {
                            self.state.write().reminders = reminders;
                            self.state.write().loading = false;
                        }
                        Err(e) => {
                            self.state.write().error = Some(e.to_string());
                            self.state.write().loading = false;
                        }
                    }
                }
                ReminderEvent::Create { reminder } => {
                    match self.repository.save(&reminder).await {
                        Ok(_) => {
                            let mut state = self.state.write();
                            state.reminders.push(reminder);
                        }
                        Err(e) => {
                            self.state.write().error = Some(e.to_string());
                        }
                    }
                }
                ReminderEvent::Update { reminder } => {
                    match self.repository.update(&reminder).await {
                        Ok(_) => {
                            let mut state = self.state.write();
                            if let Some(pos) = state.reminders.iter()
                                .position(|r| r.id == reminder.id) {
                                state.reminders[pos] = reminder;
                            }
                        }
                        Err(e) => {
                            self.state.write().error = Some(e.to_string());
                        }
                    }
                }
                ReminderEvent::Delete { id } => {
                    match self.repository.delete(&id).await {
                        Ok(_) => {
                            self.state.write().reminders.retain(|r| r.id != id);
                        }
                        Err(e) => {
                            self.state.write().error = Some(e.to_string());
                        }
                    }
                }
                ReminderEvent::Toggle { id } => {
                    let mut state = self.state.write();
                    if let Some(reminder) = state.reminders.iter_mut()
                        .find(|r| r.id == id) {
                        reminder.completed = !reminder.completed;
                        if let Err(e) = self.repository.update(reminder).await {
                            state.error = Some(e.to_string());
                        }
                    }
                }
            }
        }
    }
}

impl Bloc<ReminderState, ReminderEvent> for ReminderBloc {
    fn state(&self) -> ReadOnlySignal<ReminderState> {
        self.state.read_only()
    }
    
    fn dispatch(&self, event: ReminderEvent) {
        let _ = self.event_tx.try_send(event);
    }
    
    fn add_effect<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        spawn(future);
    }
    
    fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&ReminderState) + Send + Sync + 'static,
    {
        // 使用 Dioxus 的 use_effect 实现订阅
    }
}
```

### 4. Repository 接口

```rust
// core/repositories/reminder_repository.rs
use async_trait::async_trait;
use crate::core::models::reminder::Reminder;
use crate::core::errors::app_error::AppError;

#[async_trait]
pub trait ReminderRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Reminder>, AppError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Reminder>, AppError>;
    async fn save(&self, reminder: &Reminder) -> Result<(), AppError>;
    async fn update(&self, reminder: &Reminder) -> Result<(), AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;
}
```

### 5. IndexedDB 实现（当前）

```rust
// data/repositories/local_repository.rs
use async_trait::async_trait;
use crate::core::repositories::reminder_repository::ReminderRepository;
use crate::core::models::reminder::Reminder;
use crate::core::errors::app_error::AppError;
use crate::data::datasources::indexeddb::reminder_store::ReminderStore;

pub struct LocalRepository {
    store: ReminderStore,
}

impl LocalRepository {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            store: ReminderStore::new()?,
        })
    }
}

#[async_trait]
impl ReminderRepository for LocalRepository {
    async fn get_all(&self) -> Result<Vec<Reminder>, AppError> {
        self.store.get_all().await
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Reminder>, AppError> {
        self.store.get_by_id(id).await
    }
    
    async fn save(&self, reminder: &Reminder) -> Result<(), AppError> {
        self.store.save(reminder).await
    }
    
    async fn update(&self, reminder: &Reminder) -> Result<(), AppError> {
        self.store.update(reminder).await
    }
    
    async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.store.delete(id).await
    }
}
```

### 6. PostgreSQL 实现（未来）

```rust
// data/repositories/remote_repository.rs
use async_trait::async_trait;
use sqlx::PgPool;
use crate::core::repositories::reminder_repository::ReminderRepository;
use crate::core::models::reminder::Reminder;
use crate::core::errors::app_error::AppError;

pub struct RemoteRepository {
    pool: PgPool,
}

impl RemoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReminderRepository for RemoteRepository {
    async fn get_all(&self) -> Result<Vec<Reminder>, AppError> {
        let reminders = sqlx::query_as!(
            Reminder,
            r#"
            SELECT id, title, description, due_date, completed, 
                   created_at, updated_at, version, sync_status
            FROM reminders
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        
        Ok(reminders)
    }
    
    // ... 其他方法实现
}
```

### 7. 同步仓库（智能切换）

```rust
// data/repositories/sync_repository.rs
use async_trait::async_trait;
use crate::core::repositories::reminder_repository::ReminderRepository;
use crate::core::models::reminder::Reminder;
use crate::core::errors::app_error::AppError;
use crate::data::repositories::local_repository::LocalRepository;
use crate::data::repositories::remote_repository::RemoteRepository;
use crate::data::sync::sync_manager::SyncManager;

pub struct SyncRepository {
    local: Arc<LocalRepository>,
    remote: Option<Arc<RemoteRepository>>,
    sync_manager: Arc<SyncManager>,
    strategy: StorageStrategy,
}

#[derive(Clone, Copy)]
pub enum StorageStrategy {
    LocalOnly,
    RemoteOnly,
    LocalFirstThenSync,  // 推荐
    Auto,
}

impl SyncRepository {
    pub fn new(
        local: Arc<LocalRepository>,
        remote: Option<Arc<RemoteRepository>>,
        strategy: StorageStrategy,
    ) -> Self {
        let sync_manager = Arc::new(SyncManager::new(
            local.clone(),
            remote.clone(),
        ));
        
        Self {
            local,
            remote,
            sync_manager,
            strategy,
        }
    }
}

#[async_trait]
impl ReminderRepository for SyncRepository {
    async fn get_all(&self) -> Result<Vec<Reminder>, AppError> {
        match self.strategy {
            StorageStrategy::LocalOnly => self.local.get_all().await,
            StorageStrategy::RemoteOnly => {
                self.remote.as_ref()
                    .ok_or_else(|| AppError::NoRemote)?
                    .get_all()
                    .await
            }
            StorageStrategy::LocalFirstThenSync => {
                // 先返回本地数据（快速响应）
                let local_data = self.local.get_all().await?;
                
                // 后台同步
                if let Some(remote) = &self.remote {
                    let remote_clone = remote.clone();
                    let local_clone = self.local.clone();
                    spawn(async move {
                        if let Ok(remote_data) = remote_clone.get_all().await {
                            // 合并数据
                            let merged = merge_reminders(local_data, remote_data);
                            // 保存合并后的数据
                            for reminder in merged {
                                let _ = local_clone.save(&reminder).await;
                            }
                        }
                    });
                }
                
                Ok(local_data)
            }
            StorageStrategy::Auto => {
                if self.is_online() && self.remote.is_some() {
                    self.remote.as_ref().unwrap().get_all().await
                } else {
                    self.local.get_all().await
                }
            }
        }
    }
    
    async fn save(&self, reminder: &Reminder) -> Result<(), AppError> {
        match self.strategy {
            StorageStrategy::LocalFirstThenSync => {
                // 1. 立即保存到本地
                self.local.save(reminder).await?;
                
                // 2. 后台同步到服务器
                if let Some(remote) = &self.remote {
                    let reminder_clone = reminder.clone();
                    let remote_clone = remote.clone();
                    spawn(async move {
                        if let Err(e) = remote_clone.save(&reminder_clone).await {
                            log::warn!("Sync failed: {}", e);
                            // 加入重试队列
                        }
                    });
                }
                
                Ok(())
            }
            // ... 其他策略
            _ => self.local.save(reminder).await,
        }
    }
    
    // ... 其他方法
}
```

## 🚀 渐进迁移计划

### Phase 1: BLoC 架构重构（1-2周）
- [x] 创建 BLoC 基础结构
- [ ] 实现 ReminderBloc
- [ ] 重构现有组件使用 BLoC
- [ ] 保持 IndexedDB 存储

### Phase 2: 添加服务器支持（2-3周）
- [ ] 实现 PostgreSQL Repository
- [ ] 添加服务器 API
- [ ] 配置特性开关
- [ ] 测试服务器功能

### Phase 3: 智能同步（1-2周）
- [ ] 实现 SyncRepository
- [ ] 添加冲突解决
- [ ] 实现后台同步
- [ ] 数据迁移工具

### Phase 4: 生产部署（1周）
- [ ] 部署 PostgreSQL 数据库
- [ ] 部署 API 服务器
- [ ] 配置环境变量
- [ ] 监控和日志

## 📊 使用示例

### 在组件中使用 BLoC

```rust
// features/reminders/components/reminder_list.rs
use dioxus::prelude::*;
use crate::core::bloc::bloc_provider::use_bloc;
use crate::features::reminders::bloc::reminder_bloc::ReminderBloc;
use crate::features::reminders::events::reminder_event::ReminderEvent;
use crate::features::reminders::bloc::reminder_state::ReminderState;

#[component]
pub fn ReminderList(cx: Scope) -> Element {
    let bloc = use_bloc::<ReminderState, ReminderEvent>(cx);
    let state = bloc.state();
    
    // 加载数据
    use_effect(cx, move || {
        bloc.dispatch(ReminderEvent::Load);
    });
    
    rsx! {
        div {
            if state().loading {
                "Loading..."
            } else if let Some(error) = &state().error {
                div { "Error: {error}" }
            } else {
                for reminder in state().reminders.iter() {
                    ReminderCard {
                        reminder: reminder.clone(),
                        on_toggle: move |id| {
                            bloc.dispatch(ReminderEvent::Toggle { id });
                        },
                        on_delete: move |id| {
                            bloc.dispatch(ReminderEvent::Delete { id });
                        },
                    }
                }
            }
        }
    }
}
```

## 🎯 优势

1. **Flutter 经验复用**：BLoC 模式与 Flutter 一致
2. **渐进迁移**：不破坏现有功能
3. **可测试性**：BLoC 易于单元测试
4. **可扩展性**：易于添加新功能
5. **离线优先**：IndexedDB 保证离线可用
6. **智能同步**：自动处理在线/离线切换

---

**下一步**：开始实现 BLoC 基础结构

