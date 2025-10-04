# 开发指南

## 🚀 快速开始

### 最简单的方式

```bash
./dev
```

就这么简单！程序会自动：
1. 监听文件变化
2. 重新编译
3. 重新运行
4. 显示错误信息

### 工作流程

```
修改代码 → 保存文件 → 自动编译 → 自动运行 → 看到效果
   ↑                                              ↓
   └──────────────────────────────────────────────┘
```

---

## 📁 脚本目录

所有开发脚本都在 `scripts/` 目录下：

```
scripts/
├── README.md              # 脚本说明文档
├── SCRIPTS_README.md      # 详细使用指南
├── DEV_WORKFLOW.md        # 完整开发工作流
├── watch.sh               # 🔥 开发监听（最重要）
├── dev.sh                 # 快速启动
├── build.sh               # 构建脚本
├── run.sh                 # 运行脚本
└── test.sh                # 测试脚本
```

---

## 🎯 常用命令

### 开发模式（推荐）

```bash
# 从根目录
./dev

# 或者
./scripts/watch.sh

# 或者从 scripts 目录
cd scripts
./watch.sh
```

### 构建项目

```bash
# Release 版本（优化编译）
./scripts/build.sh

# Dev 版本（快速编译）
./scripts/build.sh dev

# 清理构建产物
./scripts/build.sh clean
```

### 运行项目

```bash
# Release 版本
./scripts/run.sh

# Dev 版本
./scripts/run.sh dev
```

### 运行测试

```bash
./scripts/test.sh
```

---

## 💡 开发技巧

### 1. 使用开发模式

开发时始终使用 `./dev`，这样：
- ✅ 不需要手动编译
- ✅ 不需要手动运行
- ✅ 修改代码立即看到效果
- ✅ 编译错误实时显示

### 2. 快速测试修改

```bash
# 1. 启动开发模式
./dev

# 2. 修改代码（如 src/main.rs）
# 3. 保存文件（Cmd+S）
# 4. 等待 5-10 秒
# 5. 程序自动重启，看到新效果
```

### 3. 提交前检查

```bash
# 运行测试
./scripts/test.sh

# 构建 release 版本
./scripts/build.sh release

# 测试 release 版本
./scripts/run.sh release
```

---

## 🔧 修改示例

### 示例 1: 修改窗口标题

```bash
# 1. 启动开发模式
./dev

# 2. 编辑 src/main.rs
# 找到：
title: Some("Agent".into()),

# 改为：
title: Some("我的 AI 助手".into()),

# 3. 保存文件
# 4. 等待几秒，窗口自动重启，标题已更改
```

### 示例 2: 修改窗口尺寸

```bash
# 1. 启动开发模式
./dev

# 2. 编辑 src/main.rs
# 找到：
let window_width = px(450.0);
let window_height = px(800.0);

# 改为：
let window_width = px(500.0);
let window_height = px(900.0);

# 3. 保存文件
# 4. 窗口自动重启，尺寸已更改
```

### 示例 3: 修改输入框提示

```bash
# 1. 启动开发模式
./dev

# 2. 编辑 src/components/assistant_panel.rs
# 找到：
input.set_placeholder("Message...");

# 改为：
input.set_placeholder("输入消息...");

# 3. 保存文件
# 4. 自动重新编译和运行
```

---

## 📊 性能对比

| 方法 | 首次编译 | 增量编译 | 自动化 |
|------|----------|----------|--------|
| 手动 `cargo build` | 1-2 分钟 | 5-10 秒 | ❌ |
| 使用 `./dev` | 1-2 分钟 | 5-10 秒 | ✅ |

**使用 `./dev` 的优势**:
- 自动监听文件变化
- 自动重新编译
- 自动重新运行
- 提高开发效率 10 倍

---

## 🛠️ 故障排除

### 问题 1: 脚本无法执行

```bash
# 添加执行权限
chmod +x dev
chmod +x scripts/*.sh
```

### 问题 2: cargo-watch 未安装

```bash
# 手动安装
cargo install cargo-watch

# 或者运行脚本，会自动安装
./dev
```

### 问题 3: 编译错误

```bash
# 清理并重新编译
./scripts/build.sh clean
./scripts/build.sh
```

### 问题 4: 端口占用

```bash
# 查找并杀死占用端口的进程
lsof -ti:8080 | xargs kill -9
```

### 问题 5: 程序无法启动

```bash
# 查看详细错误
RUST_BACKTRACE=1 ./scripts/run.sh
```

---

## 📚 更多文档

- **scripts/README.md** - 脚本目录说明
- **scripts/SCRIPTS_README.md** - 脚本详细使用指南
- **scripts/DEV_WORKFLOW.md** - 完整开发工作流文档

---

## ✨ 快速参考

### 最常用的命令

```bash
./dev              # 开发模式（99% 的时间用这个）
./scripts/test.sh  # 提交前测试
```

### 快捷键

- `Ctrl+C` - 停止开发模式
- `Cmd+S` - 保存文件（触发自动编译）

### 开发流程

```
./dev → 修改代码 → 保存 → 自动编译 → 自动运行 → 看到效果
```

---

## 🎉 开始开发

```bash
# 1. 进入项目目录
cd /Users/mac-m4/github/agent-ui

# 2. 启动开发模式
./dev

# 3. 开始编码！
```

**现在你可以高效地开发 Rust 项目了！** 🚀

