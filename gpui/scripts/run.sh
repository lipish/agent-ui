#!/bin/bash
# 运行脚本

set -e

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

MODE="${1:-release}"

case "$MODE" in
    dev|debug)
        print_info "运行开发版本..."
        cargo run
        ;;
    release|prod)
        print_info "运行发布版本..."
        cargo run --release
        ;;
    *)
        echo "用法: ./run.sh [dev|release]"
        exit 1
        ;;
esac

