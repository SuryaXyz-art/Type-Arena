#!/bin/bash

set -e

echo "🚀 Flash Markets - Build Script"
echo "================================"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Rust version
echo -e "${YELLOW}Checking Rust version...${NC}"
if ! rustc --version | grep -q "1.86.0"; then
    echo "⚠️  Warning: Expected Rust 1.86.0"
    echo "Current version: $(rustc --version)"
fi

# Install wasm target
echo -e "${YELLOW}Installing wasm32-unknown-unknown target...${NC}"
rustup target add wasm32-unknown-unknown

# Build all applications
echo -e "${YELLOW}Building Token app...${NC}"
cargo build -p token --release --target wasm32-unknown-unknown

echo -e "${YELLOW}Building Market app...${NC}"
cargo build -p market --release --target wasm32-unknown-unknown

echo -e "${YELLOW}Building Oracle app...${NC}"
cargo build -p oracle --release --target wasm32-unknown-unknown

# Verify builds
echo -e "${YELLOW}Verifying builds...${NC}"
WASM_DIR="target/wasm32-unknown-unknown/release"

if [ ! -f "$WASM_DIR/token_contract.wasm" ]; then
    echo "❌ Token contract not built!"
    exit 1
fi

if [ ! -f "$WASM_DIR/market_contract.wasm" ]; then
    echo "❌ Market contract not built!"
    exit 1
fi

if [ ! -f "$WASM_DIR/oracle_contract.wasm" ]; then
    echo "❌ Oracle contract not built!"
    exit 1
fi

echo -e "${GREEN}✅ All applications built successfully!${NC}"
echo ""
echo "📦 WASM binaries location: $WASM_DIR/"
echo ""
echo "Next steps:"
echo "  1. Run 'linera net up' to start a local network"
echo "  2. Deploy applications with 'linera project publish-and-create'"
echo "  3. Start GraphQL service with 'linera service --port 8080'"
echo ""
echo "Or use Docker:"
echo "  docker compose up -d --build"
