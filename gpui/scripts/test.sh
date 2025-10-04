#!/bin/bash
# 测试脚本

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
    ║        Agent UI - 测试脚本           ║
    ╚═══════════════════════════════════════╝
EOF
echo -e "${NC}"

print_info "运行测试..."
echo ""

cargo test

echo ""
print_success "所有测试通过！"

