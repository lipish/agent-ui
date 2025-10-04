# 开发工作流 - 自动重新编译

## 🚀 推荐方案：cargo-watch

### 安装
```bash
cargo install cargo-watch
```

### 基本使用
```bash
# 进入项目目录
cd /Users/mac-m4/github/agent-ui

# 启动自动监听（推荐）
cargo watch -c -x run

# 或者只在检查通过后运行
cargo watch -c -x check -x run

# 或者使用 release 模式（更快但编译慢）
cargo watch -c -x 'run --release'
```

### 参数说明
- `-c` 或 `--clear`: 每次运行前清屏
- `-x` 或 `--exec`: 执行命令
- `-w` 或 `--watch`: 监听特定目录（默认监听 src/）
- `-i` 或 `--ignore`: 忽略特定文件

### 高级用法
```bash
# 监听多个目录
cargo watch -w src -w assets -x run

# 忽略某些文件
cargo watch -i '*.md' -i 'target/*' -x run

# 执行多个命令
cargo watch -x check -x test -x run

# 只在测试通过后运行
cargo watch -x test -x run
```

---

## 📝 开发流程

### 1. 启动 cargo-watch
```bash
cargo watch -c -x run
```

### 2. 修改代码
在你的编辑器中修改代码，保存文件

### 3. 自动重新编译
cargo-watch 会自动检测到文件变化，重新编译并运行

### 4. 查看结果
程序会自动重启，你可以立即看到修改效果

---

## ⚡ 加速编译技巧

### 1. 使用增量编译（默认开启）
```toml
# .cargo/config.toml
[build]
incremental = true
```

### 2. 使用更快的链接器
```toml
# .cargo/config.toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### 3. 减少优化级别（开发时）
```toml
# Cargo.toml
[profile.dev]
opt-level = 0  # 不优化，编译最快
```

### 4. 并行编译
```bash
# 使用所有 CPU 核心
cargo build -j $(sysctl -n hw.ncpu)
```

### 5. 使用 sccache 缓存
```bash
# 安装
cargo install sccache

# 配置
export RUSTC_WRAPPER=sccache

# 查看缓存统计
sccache --show-stats
```

---

## 🔧 配置文件

### .cargo/config.toml
创建 `.cargo/config.toml` 文件：

```toml
[build]
# 增量编译
incremental = true
# 并行任务数
jobs = 8

[target.x86_64-apple-darwin]
# 使用更快的链接器（需要安装 llvm）
# rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[profile.dev]
# 开发模式：快速编译
opt-level = 0
debug = true
incremental = true

[profile.release]
# 发布模式：最大优化
opt-level = 3
lto = true
codegen-units = 1
```

---

## 📊 编译时间对比

### 完整重新编译
```bash
cargo clean && cargo build
# 时间: ~1-2 分钟
```

### 增量编译（修改一个文件）
```bash
cargo build
# 时间: ~5-10 秒
```

### 使用 cargo-watch（增量编译）
```bash
cargo watch -x run
# 时间: ~5-10 秒（自动）
```

---

## 🎯 实际使用示例

### 场景 1: 开发 UI
```bash
# 启动 cargo-watch
cargo watch -c -x run

# 修改 src/components/assistant_panel.rs
# 保存文件
# cargo-watch 自动重新编译和运行
# 窗口自动重启，显示新的 UI
```

### 场景 2: 修复 Bug
```bash
# 启动 cargo-watch 并运行测试
cargo watch -x test -x run

# 修改代码
# 保存文件
# 自动运行测试
# 测试通过后自动运行程序
```

### 场景 3: 调试
```bash
# 启动 cargo-watch，显示详细错误
RUST_BACKTRACE=1 cargo watch -c -x run

# 修改代码
# 保存文件
# 自动重新编译，显示详细错误信息
```

---

## 🛠️ 其他工具

### 1. bacon - 更好的 cargo-watch
```bash
# 安装
cargo install bacon

# 使用
bacon

# 配置 bacon.toml
```

### 2. cargo-expand - 查看宏展开
```bash
cargo install cargo-expand
cargo expand
```

### 3. cargo-bloat - 分析二进制大小
```bash
cargo install cargo-bloat
cargo bloat --release
```

### 4. cargo-tree - 查看依赖树
```bash
cargo tree
```

---

## 💡 最佳实践

### 1. 开发时使用 cargo-watch
```bash
cargo watch -c -x run
```

### 2. 提交前完整测试
```bash
cargo clean
cargo test
cargo build --release
```

### 3. 使用 .gitignore
```gitignore
/target/
**/*.rs.bk
.cargo/
```

### 4. 定期清理
```bash
# 清理旧的编译产物
cargo clean

# 清理所有项目的编译产物
cargo cache --autoclean
```

---

## 🚀 快速开始

### 一键安装所有工具
```bash
# 安装 cargo-watch
cargo install cargo-watch

# 安装 sccache（可选）
cargo install sccache

# 配置环境变量
echo 'export RUSTC_WRAPPER=sccache' >> ~/.zshrc
source ~/.zshrc
```

### 启动开发模式
```bash
cd /Users/mac-m4/github/agent-ui
cargo watch -c -x run
```

### 现在你可以：
1. ✅ 修改代码
2. ✅ 保存文件（Cmd+S）
3. ✅ 自动重新编译
4. ✅ 自动重新运行
5. ✅ 立即看到效果

---

## ✨ 总结

### 推荐工作流
```bash
# 1. 安装 cargo-watch
cargo install cargo-watch

# 2. 启动自动监听
cargo watch -c -x run

# 3. 修改代码，保存
# 4. 自动重新编译和运行
# 5. 立即看到效果
```

### 优点
- ✅ 自动化：不需要手动运行命令
- ✅ 快速：只重新编译修改的部分
- ✅ 高效：提高开发效率 10 倍
- ✅ 简单：一个命令搞定

### 编译时间
- 完整编译: ~1-2 分钟
- 增量编译: ~5-10 秒
- 使用 cargo-watch: 自动，无需等待

**现在你可以像开发 JavaScript/Python 一样快速迭代了！** 🎉

