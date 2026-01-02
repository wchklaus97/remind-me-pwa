# 架构文档总览

## 📚 文档索引

### 1. **BLOC_ARCHITECTURE_PLAN.md** ⭐ 核心架构
**内容**：
- BLoC 模式完整实现方案
- Repository Pattern 设计
- 渐进式迁移计划
- 代码示例和目录结构

**适用场景**：主要参考文档，包含完整的架构设计

### 2. **ARCHITECTURE_DECISION.md** ⭐ 决策记录
**内容**：
- 架构选择理由
- 技术栈说明
- 实施计划概览
- 优势分析

**适用场景**：了解为什么选择这个架构

### 3. **FULL_STACK_MIGRATION_GUIDE.md** ⭐ 全栈迁移
**内容**：
- 从客户端到全栈的迁移步骤
- 服务器函数实现
- 数据库集成（PostgreSQL）
- 部署选项

**适用场景**：需要添加服务器功能时参考

### 4. **UI_COMPONENT_ARCHITECTURE.md** ⭐ UI 组件
**内容**：
- UI 组件库架构
- 组件开发计划
- 样式方案选择
- 实施优先级

**适用场景**：开发 UI 组件时参考

## 🎯 架构核心概念

### BLoC 模式（业务逻辑组件）

```
┌─────────────┐
│   UI 组件    │
└──────┬──────┘
       │ dispatch(event)
       ↓
┌─────────────┐
│    BLoC     │ ← 业务逻辑处理
│  (State)    │
└──────┬──────┘
       │ state()
       ↓
┌─────────────┐
│   Repository│ ← 数据访问
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  DataSource │ ← 数据源（IndexedDB/PostgreSQL）
└─────────────┘
```

### Repository Pattern（数据访问抽象）

```rust
// 接口定义
trait ReminderRepository {
    async fn get_all() -> Result<Vec<Reminder>>;
    async fn save(reminder: &Reminder) -> Result<()>;
}

// 本地实现（当前）
struct LocalRepository {
    store: IndexedDBStore,
}

// 远程实现（未来）
struct RemoteRepository {
    pool: PgPool,
}

// 同步实现（智能切换）
struct SyncRepository {
    local: LocalRepository,
    remote: Option<RemoteRepository>,
    strategy: StorageStrategy,
}
```

## 📋 实施阶段

### Phase 1: BLoC 重构（当前阶段）
**目标**：重构现有代码使用 BLoC 模式

**任务**：
- [ ] 实现 Base BLoC trait
- [ ] 实现 Reminder BLoC
- [ ] 重构现有组件
- [ ] 保持 IndexedDB 存储

**时间**：1-2 周

**风险**：低（不改变存储方式）

### Phase 2: 服务器支持（可选）
**目标**：添加 PostgreSQL 和 API 服务器

**任务**：
- [ ] 实现 PostgreSQL Repository
- [ ] 创建 API 服务器
- [ ] 配置特性开关
- [ ] 测试服务器功能

**时间**：2-3 周

**风险**：中（需要部署基础设施）

### Phase 3: 智能同步（未来）
**目标**：实现本地和远程数据同步

**任务**：
- [ ] 实现 SyncRepository
- [ ] 添加冲突解决
- [ ] 实现后台同步
- [ ] 数据迁移工具

**时间**：1-2 周

**风险**：中（需要处理数据一致性）

### Phase 4: UI 组件库（并行）
**目标**：建立可复用的 UI 组件系统

**任务**：
- [ ] 实现基础组件（Button, Card, Modal）
- [ ] 实现布局组件（Navbar, Sidebar）
- [ ] 集成到现有应用
- [ ] 文档和示例

**时间**：持续进行

**风险**：低（不影响核心功能）

## 🔧 技术栈

### 当前（客户端）
- **框架**：Dioxus 0.6 (WASM)
- **状态管理**：Dioxus Signals + BLoC Pattern
- **存储**：IndexedDB (via wasm-bindgen)
- **部署**：GitHub Pages (静态)

### 未来（全栈）
- **框架**：Dioxus 0.6 (Full Stack)
- **服务器**：Axum + Tokio
- **数据库**：PostgreSQL (via SQLx)
- **部署**：Fly.io / Railway / 自托管

## 📊 架构优势

### 1. Flutter 经验复用 ✅
- BLoC 模式与 Flutter 一致
- 学习曲线低
- 代码结构熟悉

### 2. 渐进式迁移 ✅
- 不破坏现有功能
- 可以逐步添加服务器
- 支持特性开关

### 3. 可测试性 ✅
- 每层独立测试
- BLoC 易于单元测试
- Repository 可以 mock

### 4. 可扩展性 ✅
- 易于添加新功能
- 支持多数据源
- 模块化设计

### 5. 离线优先 ✅
- IndexedDB 保证离线可用
- 智能同步策略
- 自动冲突解决

## 🎯 关键决策点

### 1. 存储策略选择

**选项 A：LocalFirstThenSync（推荐）**
- ✅ 快速响应（先存本地）
- ✅ 后台同步（不阻塞 UI）
- ✅ 离线可用
- ⚠️ 需要处理冲突

**选项 B：RemoteOnly**
- ✅ 数据集中管理
- ✅ 多设备同步
- ❌ 需要网络连接
- ❌ 响应较慢

**选项 C：Auto**
- ✅ 自动选择最优策略
- ⚠️ 逻辑复杂
- ⚠️ 可能不一致

### 2. 同步时机

**选项 A：立即同步**
- ✅ 数据实时
- ❌ 可能阻塞 UI
- ❌ 网络消耗大

**选项 B：后台同步（推荐）**
- ✅ 不阻塞 UI
- ✅ 批量同步
- ⚠️ 可能延迟

**选项 C：定时同步**
- ✅ 可控的网络使用
- ⚠️ 数据可能过时

### 3. 冲突解决策略

**选项 A：最后写入获胜（LWW）**
- ✅ 简单
- ❌ 可能丢失数据

**选项 B：版本向量（推荐）**
- ✅ 保留所有更改
- ⚠️ 需要合并逻辑

**选项 C：用户选择**
- ✅ 用户控制
- ❌ 需要 UI 支持

## 📝 下一步建议

### 立即开始（推荐）

1. **实现 BLoC 基础结构**
   - 创建 `core/bloc/base_bloc.rs`
   - 创建 `core/bloc/bloc_provider.rs`
   - 创建 `core/bloc/use_bloc.rs`

2. **实现 Reminder BLoC**
   - 创建 `features/reminders/bloc/reminder_bloc.rs`
   - 创建事件和状态定义
   - 测试 BLoC 功能

3. **重构现有组件**
   - 更新 `ReminderCard` 使用 BLoC
   - 更新 `ReminderList` 使用 BLoC
   - 保持现有功能

### 可选（并行进行）

1. **UI 组件库**
   - 实现 Button 组件
   - 实现 Card 组件
   - 逐步替换现有组件

2. **文档完善**
   - 添加代码注释
   - 创建使用示例
   - 编写测试用例

## 🔍 需要确认的问题

在开始实施前，请确认：

1. **存储策略**：选择 LocalFirstThenSync 还是其他？
2. **服务器优先级**：是否需要立即添加服务器支持？
3. **UI 组件**：是否先完成 UI 组件库再重构？
4. **测试策略**：是否需要先编写测试？

## 📚 相关资源

- [Dioxus BLoC 示例](https://dioxuslabs.com/learn/0.6/)
- [Flutter BLoC 文档](https://bloclibrary.dev/)
- [Repository Pattern](https://martinfowler.com/eaaCatalog/repository.html)
- [IndexedDB API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)

---

**状态**：准备实施  
**下一步**：等待确认后开始实现

