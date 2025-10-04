# Scripts 目录

这个目录包含了所有开发和构建相关的脚本。

## 📁 目录结构

```
scripts/
├── README.md              # 本文件
├── SCRIPTS_README.md      # 脚本详细使用指南
├── DEV_WORKFLOW.md        # 完整开发工作流文档
├── watch.sh               # 🔥 开发监听脚本（最重要）
├── dev.sh                 # 快速启动脚本
├── build.sh               # 构建脚本
├── run.sh                 # 运行脚本
└── test.sh                # 测试脚本
```

## 🚀 快速开始

### 从项目根目录
```bash
# 最简单的方式
./dev

# 或者
./scripts/watch.sh
```

### 从 scripts 目录
```bash
cd scripts
./watch.sh
```

## 📖 脚本说明

### watch.sh - 开发监听（推荐）
自动监听文件变化，重新编译和运行。

```bash
./scripts/watch.sh              # 开发模式
./scripts/watch.sh --release    # Release 模式
./scripts/watch.sh --check      # 只检查编译
./scripts/watch.sh --test       # 先测试后运行
./scripts/watch.sh --help       # 显示帮助
```

### dev.sh - 快速启动
`watch.sh` 的快捷方式。

```bash
./scripts/dev.sh
```

### build.sh - 构建项目
编译项目，不运行。

```bash
./scripts/build.sh              # 构建 release 版本
./scripts/build.sh dev          # 构建 dev 版本
./scripts/build.sh clean        # 清理构建产物
```

### run.sh - 运行项目
运行已编译的项目。

```bash
./scripts/run.sh                # 运行 release 版本
./scripts/run.sh dev            # 运行 dev 版本
```

### test.sh - 运行测试
运行所有测试。

```bash
./scripts/test.sh
```

## 💡 推荐工作流

### 日常开发
```bash
# 从项目根目录
./dev

# 或者
cd scripts
./watch.sh
```

### 提交代码前
```bash
./scripts/test.sh
./scripts/build.sh release
./scripts/run.sh release
```

## 📚 详细文档

- **SCRIPTS_README.md** - 脚本详细使用指南
- **DEV_WORKFLOW.md** - 完整开发工作流文档

## ✨ 特点

- ✅ 自动监听文件变化
- ✅ 自动重新编译
- ✅ 自动重新运行
- ✅ 彩色输出，美观易读
- ✅ 错误提示清晰
- ✅ 支持多种模式

## 🎯 使用建议

1. **开发时**: 使用 `./dev` 或 `./scripts/watch.sh`
2. **测试时**: 使用 `./scripts/test.sh`
3. **构建时**: 使用 `./scripts/build.sh`
4. **运行时**: 使用 `./scripts/run.sh`

**现在开始高效开发吧！** 🚀

