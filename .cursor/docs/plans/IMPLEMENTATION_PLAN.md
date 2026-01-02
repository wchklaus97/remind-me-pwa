# UI Component Implementation Plan

## ✅ 已完成

1. ✅ 创建组件库目录结构
2. ✅ 创建基础模块文件
3. ✅ 建立主题系统基础（颜色定义）
4. ✅ 更新主 Cargo.toml 引入 UI crate

## 📋 下一步实施计划

### Phase 1: 核心基础组件（本周）

#### 1. Button 组件
- [ ] 实现基础 Button 组件
- [ ] 支持多种变体（Primary, Secondary, Outline, Ghost, Danger）
- [ ] 支持多种尺寸（Small, Medium, Large）
- [ ] 支持 loading 状态
- [ ] 支持 disabled 状态
- [ ] 添加点击事件处理

**文件**: `crates/ui/src/components/button.rs`

#### 2. Card 组件
- [ ] 实现基础 Card 组件
- [ ] 支持多种变体（Default, Elevated, Outline）
- [ ] 支持 header, footer, content 部分
- [ ] 响应式设计

**文件**: `crates/ui/src/components/card.rs`

#### 3. Input 组件
- [ ] 实现基础 Input 组件
- [ ] 支持多种类型（text, email, password, number）
- [ ] 支持 placeholder, disabled, required
- [ ] 支持错误状态显示
- [ ] 支持标签和帮助文本

**文件**: `crates/ui/src/components/input.rs`

#### 4. Modal 组件
- [ ] 实现基础 Modal 组件
- [ ] 支持打开/关闭状态
- [ ] 支持多种尺寸（Small, Medium, Large, Fullscreen）
- [ ] 支持点击背景关闭
- [ ] 支持标题和关闭按钮
- [ ] 添加动画效果

**文件**: `crates/ui/src/components/modal.rs`

### Phase 2: 布局组件（第二周）

#### 1. AppLayout 组件
- [ ] 实现主应用布局
- [ ] 集成 Navbar 和 Sidebar
- [ ] 响应式设计（移动端/桌面端）
- [ ] 支持侧边栏折叠

**文件**: `crates/ui/src/layout/app_layout.rs`

#### 2. Navbar 组件
- [ ] 实现导航栏
- [ ] 支持 Logo 和导航链接
- [ ] 支持移动端菜单按钮
- [ ] 支持用户菜单

**文件**: `crates/ui/src/layout/navbar.rs`

#### 3. Sidebar 组件
- [ ] 实现侧边栏
- [ ] 支持导航菜单
- [ ] 支持折叠/展开
- [ ] 移动端支持

**文件**: `crates/ui/src/layout/sidebar.rs`

### Phase 3: 表单和数据组件（第三周）

#### 1. FormField 组件
- [ ] 实现表单字段包装器
- [ ] 支持标签、输入、错误消息
- [ ] 支持必填标记

**文件**: `crates/ui/src/components/form.rs`

#### 2. Checkbox 组件
- [ ] 实现复选框
- [ ] 支持选中/未选中状态
- [ ] 支持禁用状态
- [ ] 支持标签

**文件**: `crates/ui/src/components/checkbox.rs`

#### 3. Badge 组件
- [ ] 实现徽章组件
- [ ] 支持多种变体（Primary, Success, Warning, Error）
- [ ] 支持不同尺寸

**文件**: `crates/ui/src/components/badge.rs`

#### 4. EmptyState 组件
- [ ] 实现空状态组件
- [ ] 支持图标、标题、描述
- [ ] 支持操作按钮

**文件**: `crates/ui/src/data/empty_state.rs`

### Phase 4: 业务组件集成（第四周）

#### 1. ReminderCard 组件
- [ ] 使用新的 UI 组件重构
- [ ] 集成 Card, Badge, Button 组件
- [ ] 保持现有功能

**文件**: `src/features/reminders/components/reminder_card.rs`

#### 2. ReminderForm 组件
- [ ] 使用新的 UI 组件重构
- [ ] 集成 Modal, FormField, Input 组件
- [ ] 改进表单验证

**文件**: `src/features/reminders/components/reminder_form.rs`

#### 3. ReminderList 组件
- [ ] 使用新的 UI 组件重构
- [ ] 集成 List, EmptyState 组件
- [ ] 改进加载状态

**文件**: `src/features/reminders/components/reminder_list.rs`

## 🎨 样式方案

### 当前方案：使用 Tailwind CSS 类名

所有组件使用 Tailwind CSS 类名，通过 `class` 属性设置样式。

**优点：**
- 快速开发
- 响应式设计简单
- 易于维护
- 文件大小可控

**示例：**
```rust
rsx! {
    button {
        class: "px-4 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600",
        "Click me"
    }
}
```

### 未来可选：纯 CSS 模块

如果需要更小的文件大小，可以切换到纯 CSS 模块。

## 📚 开发指南

### 组件开发规范

1. **使用 Dioxus 0.6 语法**
   - 使用 `#[component]` 宏
   - 使用 `Element` 返回类型
   - 使用 `rsx!` 宏

2. **Props 定义**
   - 使用 `#[derive(PartialEq, Props)]`
   - 使用 `#[props(default)]` 设置默认值
   - 使用 `EventHandler` 处理事件

3. **样式类名**
   - 使用 Tailwind CSS 类名
   - 通过 `class` 属性设置
   - 支持条件类名

4. **文档注释**
   - 每个组件添加文档注释
   - 说明 Props 和用法
   - 提供示例代码

### 组件示例模板

```rust
use dioxus::prelude::*;

/// Component description
/// 
/// # Example
/// ```rust
/// rsx! {
///     ComponentName {
///         prop1: "value",
///         prop2: true,
///     }
/// }
/// ```
#[derive(PartialEq, Props)]
pub struct ComponentProps {
    /// Prop description
    #[props(default)]
    pub prop1: String,
    
    /// Another prop
    pub prop2: bool,
    
    /// Event handler
    pub onclick: Option<EventHandler<()>>,
    
    /// Children
    pub children: Element,
}

#[component]
pub fn ComponentName(cx: Scope<ComponentProps>) -> Element {
    let base_classes = "base-classes-here";
    
    rsx! {
        div {
            class: "{base_classes}",
            onclick: move |_| {
                if let Some(handler) = props.onclick.as_ref() {
                    handler.call(());
                }
            },
            
            {props.children.as_ref()}
        }
    }
}
```

## 🚀 开始实施

### 第一步：实现 Button 组件

创建 `crates/ui/src/components/button.rs` 并实现基础 Button 组件。

### 第二步：测试组件

在主应用中测试新组件：
```rust
use remind_me_ui::Button;

#[component]
fn TestButton() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            onclick: move |_| {
                log::info!("Button clicked!");
            },
            "Click me"
        }
    }
}
```

### 第三步：逐步替换

逐步将现有组件替换为新的 UI 组件库中的组件。

---

**状态**: 准备开始实施  
**下一步**: 实现 Button 组件

