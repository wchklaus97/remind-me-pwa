# Architecture Decision Record

## 决策：采用 BLoC + Repository Pattern 架构

### 背景

项目需要从纯客户端 PWA 迁移到全栈应用，同时：
1. 保持现有功能可用
2. 支持渐进式迁移
3. 利用 Flutter BLoC 经验
4. 准备 PostgreSQL 集成

### 决策

采用 **BLoC + Repository Pattern** 架构，结合：
- **BLoC 模式**：业务逻辑与 UI 分离（类似 Flutter）
- **Repository Pattern**：数据访问抽象
- **渐进迁移**：IndexedDB → PostgreSQL
- **智能同步**：本地优先，后台同步

### 架构层次

```
UI Layer (Components)
    ↓
BLoC Layer (Business Logic)
    ↓
Repository Layer (Data Access Interface)
    ↓
Data Source Layer (IndexedDB / PostgreSQL)
```

### 优势

1. ✅ **Flutter 经验复用**：BLoC 模式熟悉
2. ✅ **渐进迁移**：不破坏现有功能
3. ✅ **可测试性**：每层独立测试
4. ✅ **可扩展性**：易于添加新功能
5. ✅ **离线优先**：IndexedDB 保证可用
6. ✅ **智能同步**：自动处理冲突

### 实施计划

#### Phase 1: BLoC 重构（当前）
- 创建 BLoC 基础结构
- 重构现有组件
- 保持 IndexedDB

#### Phase 2: 服务器支持（未来）
- 添加 PostgreSQL Repository
- 实现 API 服务器
- 配置特性开关

#### Phase 3: 智能同步（未来）
- 实现 SyncRepository
- 添加冲突解决
- 数据迁移工具

### 技术栈

- **前端**：Dioxus 0.6 + WASM
- **状态管理**：Dioxus Signals + BLoC Pattern
- **本地存储**：IndexedDB（当前）
- **远程存储**：PostgreSQL（未来）
- **同步**：自定义同步管理器
- **服务器**：Axum + SQLx（可选）

### 文件结构

```
src/
├── core/              # 核心架构
│   ├── bloc/         # BLoC 实现
│   ├── models/       # 领域模型
│   ├── repositories/  # 仓库接口
│   └── errors/       # 错误类型
├── data/             # 数据层
│   ├── repositories/ # 仓库实现
│   ├── datasources/  # 数据源
│   └── sync/         # 同步逻辑
├── features/         # 功能模块
│   └── reminders/    # Reminder 功能
├── client/           # 客户端代码
├── server/           # 服务端代码（可选）
└── shared/           # 共享代码
```

### 决策日期

2025-01-02

### 状态

✅ 已决定  
🚧 实施中

