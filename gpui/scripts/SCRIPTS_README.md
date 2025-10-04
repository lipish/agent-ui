# 开发脚本使用指南

## 📁 脚本列表

项目根目录下提供了以下便捷脚本：

| 脚本 | 用途 | 说明 |
|------|------|------|
| `watch.sh` | 🔥 **开发监听** | 自动监听文件变化，重新编译和运行（推荐） |
| `dev.sh` | 🚀 **快速开发** | `watch.sh` 的快捷方式 |
| `build.sh` | 🔨 **构建项目** | 编译项目（dev/release） |
| `run.sh` | ▶️ **运行项目** | 运行已编译的项目 |
| `test.sh` | ✅ **运行测试** | 运行所有测试 |

---

## 🔥 watch.sh - 开发监听（推荐）

**最常用的脚本**，自动监听文件变化，重新编译和运行。

### 基本用法
```bash
./watch.sh
```

### 高级用法
```bash
# 开发模式（默认，快速编译）
./watch.sh

# Release 模式（优化编译）
./watch.sh --release

# 只检查编译，不运行
./watch.sh --check

# 先运行测试，再运行程序
./watch.sh --test

# 显示帮助
./watch.sh --help
```

### 工作流程
1. 运行 `./watch.sh`
2. 修改代码（如 `src/components/assistant_panel.rs`）
3. 保存文件（Cmd+S）
4. 自动重新编译（5-10秒）
5. 自动重新运行程序
6. 立即看到效果

### 停止监听
按 `Ctrl+C`

---

## 🚀 dev.sh - 快速开发

最简单的启动方式，等同于 `./watch.sh`

```bash
./dev.sh
```

---

## 🔨 build.sh - 构建项目

编译项目，不运行。

### 用法
```bash
# 构建 release 版本（默认，优化编译）
./build.sh

# 构建 dev 版本（快速编译）
./build.sh dev

# 清理构建产物
./build.sh clean
```

### 输出位置
- Dev 版本: `target/debug/agent-ui`
- Release 版本: `target/release/agent-ui`

---

## ▶️ run.sh - 运行项目

运行已编译的项目。

### 用法
```bash
# 运行 release 版本（默认）
./run.sh

# 运行 dev 版本
./run.sh dev
```

**注意**: 如果没有编译过，会先自动编译。

---

## ✅ test.sh - 运行测试

运行所有测试。

### 用法
```bash
./test.sh
```

---

## 🎯 推荐工作流

### 日常开发
```bash
# 1. 启动开发监听
./watch.sh

# 2. 修改代码，保存
# 3. 自动重新编译和运行
# 4. 立即看到效果
```

### 提交代码前
```bash
# 1. 运行测试
./test.sh

# 2. 构建 release 版本
./build.sh release

# 3. 运行 release 版本测试
./run.sh release
```

### 快速测试
```bash
# 修改代码后快速测试
./watch.sh --test
```

---

## 📊 脚本对比

| 场景 | 推荐脚本 | 说明 |
|------|----------|------|
| 日常开发 | `./watch.sh` | 自动监听，最高效 |
| 快速启动 | `./dev.sh` | 最简单 |
| 只编译 | `./build.sh` | 不运行 |
| 只运行 | `./run.sh` | 不重新编译 |
| 运行测试 | `./test.sh` | 测试所有功能 |

---

## ⚡ 性能对比

| 方法 | 首次编译 | 增量编译 | 自动化 |
|------|----------|----------|--------|
| `cargo build` | 1-2 分钟 | 5-10 秒 | ❌ 手动 |
| `./build.sh` | 1-2 分钟 | 5-10 秒 | ❌ 手动 |
| `./watch.sh` | 1-2 分钟 | 5-10 秒 | ✅ **自动** |

---

## 🛠️ 自定义配置

### 修改监听目录
编辑 `watch.sh`，修改这一行：
```bash
cargo watch -c -w src -w assets -x run
#                 ^^^    ^^^^^^^ 添加更多目录
```

### 修改编译选项
编辑 `.cargo/config.toml`：
```toml
[profile.dev]
opt-level = 0  # 0=不优化, 1=基本优化, 2=更多优化
```

---

## 💡 提示和技巧

### 1. 使用别名
在 `~/.zshrc` 或 `~/.bashrc` 中添加：
```bash
alias dev='cd /Users/mac-m4/github/agent-ui && ./watch.sh'
alias build='cd /Users/mac-m4/github/agent-ui && ./build.sh'
alias test='cd /Users/mac-m4/github/agent-ui && ./test.sh'
```

然后在任何目录都可以运行：
```bash
dev      # 启动开发监听
build    # 构建项目
test     # 运行测试
```

### 2. 查看详细错误
```bash
RUST_BACKTRACE=1 ./watch.sh
```

### 3. 并行编译
```bash
# 使用所有 CPU 核心
cargo build -j $(sysctl -n hw.ncpu)
```

### 4. 清理缓存
```bash
# 清理当前项目
./build.sh clean

# 清理所有 Rust 项目缓存
cargo cache --autoclean
```

---

## 🐛 故障排除

### 问题 1: cargo-watch 未安装
```bash
# 脚本会自动安装，或手动安装：
cargo install cargo-watch
```

### 问题 2: 权限错误
```bash
# 给脚本添加执行权限
chmod +x *.sh
```

### 问题 3: 编译错误
```bash
# 清理并重新编译
./build.sh clean
./build.sh
```

### 问题 4: 端口占用
```bash
# 查找并杀死占用端口的进程
lsof -ti:8080 | xargs kill -9
```

---

## 📝 示例场景

### 场景 1: 修改 UI
```bash
# 1. 启动监听
./watch.sh

# 2. 修改 src/components/assistant_panel.rs
# 3. 保存文件
# 4. 自动重新编译和运行
# 5. 窗口自动重启，显示新 UI
```

### 场景 2: 修复 Bug
```bash
# 1. 启动测试监听
./watch.sh --test

# 2. 修改代码
# 3. 保存文件
# 4. 自动运行测试
# 5. 测试通过后自动运行程序
```

### 场景 3: 性能优化
```bash
# 1. 构建 release 版本
./build.sh release

# 2. 运行并测试性能
./run.sh release

# 3. 修改代码
# 4. 重新构建和测试
```

---

## ✨ 总结

### 最常用的命令
```bash
./watch.sh    # 开发时 99% 的时间用这个
./test.sh     # 提交前运行测试
./build.sh    # 需要构建时
```

### 优势
- ✅ 自动化：不需要手动运行命令
- ✅ 快速：增量编译，5-10 秒
- ✅ 高效：提高开发效率 10 倍
- ✅ 简单：一个命令搞定

### 开发体验
- 像 JavaScript/Python 一样快速迭代
- 修改代码，保存，立即看到效果
- 不需要手动编译和运行

**现在你可以高效地开发 Rust 项目了！** 🎉

