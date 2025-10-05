# Chatbox UI Preview

## 如何查看 UI 效果

### 方法1：ASCII 预览（推荐）
由于磁盘空间限制，最简单的方式是查看 ASCII 艺术预览：

```bash
rustc --edition 2021 crates/chatbox/examples/preview.rs -o /tmp/chatbox_preview && /tmp/chatbox_preview
```

### 方法2：运行示例程序
如果磁盘空间允许，可以运行实际的 GUI 示例：

```bash
cargo run --example simple_demo -p chatbox
```

## UI 设计概览

```
┌─────────────────────────────────────────────────┐
│ Simple Chatbox Demo                             │
│ 💬                                              │
│ This is a simplified chatbox UI component.      │
├─────────────────────────────────────────────────┤
│                                                 │
│ Chat display area                              │
│ ┌─────────────────────────────────────────────┐ │
│ │ Sent: Hello, how are you?                  │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
├─────────────────────────────────────────────────┤
│ Input area                                     │
│ ┌─────────────────────────────────┐ ┌─────────┐ │
│ │ Type your message here...     │ │  Send   │ │
│ └─────────────────────────────────┘ └─────────┘ │
│ ℹ️  Press Enter to send, Escape to cancel      │
└─────────────────────────────────────────────────┘
```

## 组件结构

### ChatView（主容器）
- **Header**: 显示标题和图标
- **Chat Area**: 消息显示区域
- **Input Area**: 输入框和发送按钮
- **Footer**: 操作提示

### 核心特性

1. **简洁的输入界面**
   - 基于 Zed Editor 组件
   - 支持 placeholder 文本
   - 自动高度调整（1-5行）

2. **消息显示区域**
   - 显示发送的消息
   - 支持状态提示
   - 可滚动的历史记录

3. **交互功能**
   - Enter 键发送消息
   - Escape 键取消输入
   - 焦点管理

4. **主题集成**
   - 使用 Zed 主题系统
   - 支持深色/浅色模式
   - 响应式设计

## 代码结构

```rust
Chatbox (主组件)
├── MessageEditor (输入组件)
│   ├── 文本编辑器
│   ├── 事件处理
│   └── 状态管理
└── CopilotChat (显示组件)
    ├── 消息渲染
    └── 状态显示
```

## 使用示例

```rust
use chatbox::Chatbox;

let chatbox = Chatbox::new(
    "Type your message here...",
    editor::EditorMode::AutoHeight {
        min_lines: 1,
        max_lines: Some(5),
    },
    window,
    cx,
);
```

## 视觉特点

- **现代简洁**: 扁平化设计，清晰的边框和分隔
- **响应式**: 自适应窗口大小
- **可访问性**: 清晰的对比度和可读的字体
- **一致性与 Zed 编辑器风格保持一致

## 下一步扩展

这个基础组件可以轻松扩展为：

- 完整的聊天界面
- 项目协作工具
- 代码审查工具
- AI 助手界面
- 文档编辑器

当前的实现专注于提供纯净的输入功能，可以根据具体需求添加更多特性。