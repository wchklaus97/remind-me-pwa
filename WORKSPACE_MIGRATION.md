# Workspace 迁移指南

## 当前状态

已创建基本的 workspace 结构：
- ✅ `Cargo-workspace.toml` - Workspace 根配置
- ✅ `crates/shared/` - 共享代码 crate
- ✅ `crates/web/` - 网页专用 crate
- ✅ `crates/mobile/` - 移动应用专用 crate
- ✅ `apps/web/` - 网页应用入口
- ✅ `apps/mobile/` - 移动应用入口

## 下一步操作

### 选项 1: 完整迁移（推荐，但需要时间）

1. **移动共享代码到 `crates/shared/`**:
   ```bash
   # 复制文件
   cp src/models.rs crates/shared/src/
   cp src/utils.rs crates/shared/src/
   
   # i18n 和 router 需要适配（移除平台特定代码）
   # 将平台特定部分移到 web/mobile crates
   ```

2. **创建平台特定的 storage 实现**:
   - `crates/web/src/storage.rs` - WebStorage 实现
   - `crates/mobile/src/storage.rs` - MobileStorage 实现
   - `crates/shared/src/storage.rs` - Storage trait 定义

3. **移动组件代码**:
   - 大部分组件可以保留在 `crates/ui/`（已存在）
   - 平台特定组件移到各自的 crate

4. **更新所有导入路径**

### 选项 2: 渐进式迁移（更安全）

1. **保持当前结构工作**
2. **逐步迁移**:
   - 先迁移 models 和 utils（最简单）
   - 然后迁移 storage（需要适配）
   - 最后迁移 app 和组件

## 重要注意事项

- `i18n.rs` 和 `router.rs` 包含平台特定代码（web_sys）
- 需要将这些代码拆分到平台特定的实现中
- `components/` 目录很大，需要仔细规划迁移

## 建议

由于这是一个大型重构，建议：
1. 先在分支中完成迁移
2. 确保所有平台都能正常构建
3. 测试所有功能
4. 然后合并到主分支

