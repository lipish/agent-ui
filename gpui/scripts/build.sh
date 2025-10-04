#!/bin/bash
# 构建脚本

set -e

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

clear
echo -e "${GREEN}"
cat << "EOF"
    ╔═══════════════════════════════════════╗
    ║        Agent UI - 构建脚本           ║
    ╚═══════════════════════════════════════╝
EOF
echo -e "${NC}"

# 解析参数
MODE="${1:-release}"

case "$MODE" in
    dev|debug)
        print_info "构建开发版本（快速编译，未优化）"
        cargo build
        print_success "构建完成: target/debug/agent-ui"
        ;;
    release|prod)
        print_info "构建发布版本（优化编译）"
        cargo build --release
        print_success "构建完成: target/release/agent-ui"
        ;;
    clean)
        print_info "清理构建产物..."
        cargo clean
        print_success "清理完成"
        ;;
    *)
        echo -e "${YELLOW}用法:${NC}"
        echo "  ./build.sh [dev|release|clean]"
        echo ""
        echo "  dev      - 开发版本（快速）"
        echo "  release  - 发布版本（优化，默认）"
        echo "  clean    - 清理构建产物"
        exit 1
        ;;
esac

