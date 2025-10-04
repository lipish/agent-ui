#!/bin/bash
# Web development script

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}"
cat << "EOF"
    ╔═══════════════════════════════════════╗
    ║     Agent UI - Web Version           ║
    ║          (Svelte)                    ║
    ╚═══════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${BLUE}ℹ${NC} Starting development server..."
echo ""

npm run dev

