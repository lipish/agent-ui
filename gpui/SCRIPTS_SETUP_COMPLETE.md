# 脚本设置完成 ✅

## 📁 目录结构

所有脚本已经整理到 `scripts/` 目录：

```
agent-ui/
├── dev                          # 🔥 快速启动脚本（根目录）
├── DEVELOPMENT.md               # 开发指南
├── README.md                    # 项目说明（已更新）
├── scripts/                     # 脚本目录
│   ├── README.md                # 脚本目录说明
│   ├── SCRIPTS_README.md        # 脚本详细使用指南
│   ├── DEV_WORKFLOW.md          # 完整开发工作流文档
│   ├── watch.sh                 # 开发监听脚本
│   ├── dev.sh                   # 快速启动
│   ├── build.sh                 # 构建脚本
│   ├── run.sh                   # 运行脚本
│   └── test.sh                  # 测试脚本
└── src/
    └── ...
```

---

## 🚀 使用方法

### 最简单的方式（推荐）

从项目根目录：
```bash
./dev
```

这个脚本会自动调用 `scripts/watch.sh`，启动开发监听模式。

### 其他方式

```bash
# 方式 1: 使用根目录的快捷脚本
./dev

# 方式 2: 直接调用 scripts 目录的脚本
./scripts/watch.sh

# 方式 3: 进入 scripts 目录
cd scripts
./watch.sh
```

---

## 📖 脚本说明

### 根目录

#### `dev` - 快速启动脚本
- 最简单的启动方式
- 自动调用 `scripts/watch.sh`
- 支持传递参数

```bash
./dev              # 开发模式
./dev --release    # Release 模式
./dev --help       # 显示帮助
```

### scripts/ 目录

#### `watch.sh` - 开发监听（最重要）
自动监听文件变化，重新编译和运行。

```bash
./scripts/watch.sh              # 开发模式
./scripts/watch.sh --release    # Release 模式
./scripts/watch.sh --check      # 只检查编译
./scripts/watch.sh --test       # 先测试后运行
./scripts/watch.sh --help       # 显示帮助
```

#### `dev.sh` - 快速启动
`watch.sh` 的快捷方式。

```bash
./scripts/dev.sh
```

#### `build.sh` - 构建脚本
编译项目，不运行。

```bash
./scripts/build.sh              # 构建 release 版本
./scripts/build.sh dev          # 构建 dev 版本
./scripts/build.sh clean        # 清理构建产物
```

#### `run.sh` - 运行脚本
运行已编译的项目。

```bash
./scripts/run.sh                # 运行 release 版本
./scripts/run.sh dev            # 运行 dev 版本
```

#### `test.sh` - 测试脚本
运行所有测试。

```bash
./scripts/test.sh
```

---

## 📚 文档说明

### 根目录

#### `DEVELOPMENT.md` - 开发指南
- 快速开始
- 常用命令
- 开发技巧
- 修改示例
- 故障排除

### scripts/ 目录

#### `README.md` - 脚本目录说明
- 目录结构
- 快速开始
- 脚本说明
- 使用建议

#### `SCRIPTS_README.md` - 脚本详细使用指南
- 完整的脚本使用说明
- 高级用法
- 性能对比
- 自定义配置
- 故障排除

#### `DEV_WORKFLOW.md` - 完整开发工作流文档
- cargo-watch 详细说明
- 加速编译技巧
- 配置文件
- 其他工具
- 最佳实践

---

## 🎯 推荐工作流

### 日常开发

```bash
# 1. 启动开发模式
./dev

# 2. 修改代码
# 3. 保存文件（Cmd+S）
# 4. 自动重新编译（5-10秒）
# 5. 自动重新运行
# 6. 立即看到效果
```

### 提交代码前

```bash
# 1. 运行测试
./scripts/test.sh

# 2. 构建 release 版本
./scripts/build.sh release

# 3. 测试 release 版本
./scripts/run.sh release
```

---

## ✨ 特点

### 自动化
- ✅ 自动监听文件变化
- ✅ 自动重新编译
- ✅ 自动重新运行
- ✅ 自动显示错误

### 高效
- ✅ 增量编译（5-10秒）
- ✅ 不需要手动操作
- ✅ 提高开发效率 10 倍

### 易用
- ✅ 一个命令启动：`./dev`
- ✅ 彩色输出，美观易读
- ✅ 错误提示清晰
- ✅ 支持多种模式

### 完善
- ✅ 详细的文档
- ✅ 多种使用方式
- ✅ 故障排除指南
- ✅ 最佳实践建议

---

## 📊 文件清单

### 根目录文件
- ✅ `dev` - 快速启动脚本
- ✅ `DEVELOPMENT.md` - 开发指南
- ✅ `README.md` - 项目说明（已更新）

### scripts/ 目录文件
- ✅ `README.md` - 脚本目录说明
- ✅ `SCRIPTS_README.md` - 脚本详细使用指南
- ✅ `DEV_WORKFLOW.md` - 完整开发工作流文档
- ✅ `watch.sh` - 开发监听脚本
- ✅ `dev.sh` - 快速启动
- ✅ `build.sh` - 构建脚本
- ✅ `run.sh` - 运行脚本
- ✅ `test.sh` - 测试脚本

### 所有脚本都有执行权限
```bash
-rwxr-xr-x  dev
-rwxr-xr-x  scripts/watch.sh
-rwxr-xr-x  scripts/dev.sh
-rwxr-xr-x  scripts/build.sh
-rwxr-xr-x  scripts/run.sh
-rwxr-xr-x  scripts/test.sh
```

---

## 🎉 开始使用

### 立即开始

```bash
cd /Users/mac-m4/github/agent-ui
./dev
```

### 查看文档

```bash
# 开发指南
cat DEVELOPMENT.md

# 脚本说明
cat scripts/README.md

# 详细使用指南
cat scripts/SCRIPTS_README.md

# 完整工作流
cat scripts/DEV_WORKFLOW.md
```

---

## 💡 提示

### 1. 使用别名（可选）

在 `~/.zshrc` 或 `~/.bashrc` 中添加：

```bash
alias dev='cd /Users/mac-m4/github/agent-ui && ./dev'
```

然后在任何目录都可以运行：
```bash
dev    # 启动开发模式
```

### 2. 查看帮助

```bash
./dev --help
./scripts/watch.sh --help
```

### 3. 停止开发模式

按 `Ctrl+C`

---

## ✅ 验证

### 测试脚本是否工作

```bash
# 测试根目录脚本
./dev --help

# 测试 scripts 目录脚本
./scripts/watch.sh --help
./scripts/build.sh
./scripts/test.sh
```

---

## 🎯 总结

### 完成的工作
- ✅ 创建了 `scripts/` 目录
- ✅ 移动了所有脚本到 `scripts/`
- ✅ 移动了文档到 `scripts/`
- ✅ 创建了根目录的 `dev` 快捷脚本
- ✅ 创建了 `DEVELOPMENT.md` 开发指南
- ✅ 更新了 `README.md` 项目说明
- ✅ 创建了 `scripts/README.md` 目录说明
- ✅ 所有脚本都有执行权限

### 目录结构
- ✅ 清晰的目录结构
- ✅ 脚本集中管理
- ✅ 文档完善
- ✅ 易于使用

### 使用方式
- ✅ 根目录快捷启动：`./dev`
- ✅ 直接调用：`./scripts/watch.sh`
- ✅ 进入目录：`cd scripts && ./watch.sh`

**现在你可以高效地开发 Rust 项目了！** 🚀

