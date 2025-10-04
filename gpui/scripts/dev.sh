#!/bin/bash
# 快速开发脚本 - 最简单的启动方式

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"$SCRIPT_DIR/watch.sh" "$@"

