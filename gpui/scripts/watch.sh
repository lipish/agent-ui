#!/bin/bash
# cargo-watch 开发脚本
# 自动监听文件变化，重新编译和运行

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# 检查 cargo-watch 是否安装
check_cargo_watch() {
    if ! command -v cargo-watch &> /dev/null; then
        print_warning "cargo-watch 未安装"
        print_info "正在安装 cargo-watch..."
        cargo install cargo-watch
        print_success "cargo-watch 安装成功"
    else
        print_success "cargo-watch 已安装"
    fi
}

# 显示帮助信息
show_help() {
    cat << EOF
${GREEN}Agent UI - 开发监听脚本${NC}

${YELLOW}用法:${NC}
  ./watch.sh [选项]

${YELLOW}选项:${NC}
  (无参数)    启动开发模式（自动重新编译和运行）
  -r, --release   使用 release 模式（优化编译）
  -c, --check     只检查编译，不运行
  -t, --test      运行测试后再运行程序
  -h, --help      显示此帮助信息

${YELLOW}示例:${NC}
  ./watch.sh              # 开发模式
  ./watch.sh --release    # release 模式
  ./watch.sh --check      # 只检查编译
  ./watch.sh --test       # 先测试后运行

${YELLOW}快捷键:${NC}
  Ctrl+C    停止监听

${YELLOW}说明:${NC}
  - 修改代码后保存，会自动重新编译和运行
  - 编译错误会显示在终端
  - 程序会自动重启

EOF
}

# 主函数
main() {
    clear
    echo -e "${GREEN}"
    cat << "EOF"
    ╔═══════════════════════════════════════╗
    ║     Agent UI - 开发监听模式          ║
    ╚═══════════════════════════════════════╝
EOF
    echo -e "${NC}"

    # 检查 cargo-watch
    check_cargo_watch
    echo ""

    # 解析参数
    MODE="dev"
    case "${1:-}" in
        -r|--release)
            MODE="release"
            print_info "使用 release 模式（优化编译）"
            ;;
        -c|--check)
            MODE="check"
            print_info "只检查编译，不运行"
            ;;
        -t|--test)
            MODE="test"
            print_info "运行测试后再运行程序"
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        "")
            print_info "使用开发模式（快速编译）"
            ;;
        *)
            print_error "未知选项: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac

    echo ""
    print_info "正在启动 cargo-watch..."
    print_info "监听目录: src/, assets/"
    print_info "按 Ctrl+C 停止"
    echo ""
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    # 根据模式启动 cargo-watch
    case "$MODE" in
        dev)
            cargo watch -c -w src -w assets -x run
            ;;
        release)
            cargo watch -c -w src -w assets -x 'run --release'
            ;;
        check)
            cargo watch -c -w src -w assets -x check
            ;;
        test)
            cargo watch -c -w src -w assets -x test -x run
            ;;
    esac
}

# 捕获 Ctrl+C
trap 'echo ""; print_info "停止监听..."; exit 0' INT

# 运行主函数
main "$@"

