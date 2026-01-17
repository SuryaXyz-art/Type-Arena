#!/usr/bin/env bash
set -e

# ============================================================================
# Type Arena - Linera Conway Testnet Deployment Script
# ============================================================================
# This script deploys the Type Arena smart contracts to the Linera Conway
# private testnet and updates the frontend configuration.
# ============================================================================

FAUCET_URL="https://faucet.testnet-conway.linera.net"
WALLET_DIR="/root/.config/linera"
CONFIG_PATH="frontend/client/public/config.json"

echo "=========================================="
echo "Type Arena - Conway Testnet Deployment"
echo "=========================================="
echo ""

# Step 1: Initialize Wallet
echo "[1/7] Initializing Linera Wallet..."
mkdir -p "$WALLET_DIR"

if [ ! -f "$WALLET_DIR/wallet.json" ]; then
    echo "Creating new wallet against Conway testnet..."
    linera wallet init --faucet "$FAUCET_URL"
    echo "✓ Wallet created successfully"
else
    echo "✓ Existing wallet found"
fi

# Step 2: Sync Balance
echo ""
echo "[2/7] Syncing wallet balance..."
linera sync-balance
echo "✓ Balance synced"

# Step 3: Get Chain ID
echo ""
echo "[3/7] Retrieving chain information..."
CHAIN_ID=$(linera wallet show | grep "Chain ID" | awk '{print $3}')

if [ -z "$CHAIN_ID" ]; then
    echo "✗ Error: Failed to retrieve Chain ID"
    exit 1
fi

echo "✓ Chain ID: $CHAIN_ID"

# Step 4: Build Smart Contracts
echo ""
echo "[4/7] Building smart contracts..."
rustup target add wasm32-unknown-unknown

cd contracts/type_arena
echo "Cleaning previous builds..."
cargo clean

echo "Building WASM binaries..."
cargo build --release --target wasm32-unknown-unknown

if [ $? -ne 0 ]; then
    echo "✗ Error: Contract build failed"
    exit 1
fi

cd ../..
echo "✓ Contracts built successfully"

# Step 5: Publish Bytecode
echo ""
echo "[5/7] Publishing bytecode to Conway testnet..."

CONTRACT_WASM="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
SERVICE_WASM="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

# Verify WASM files exist
if [ ! -f "$CONTRACT_WASM" ]; then
    echo "✗ Error: Contract WASM not found at $CONTRACT_WASM"
    exit 1
fi

if [ ! -f "$SERVICE_WASM" ]; then
    echo "✗ Error: Service WASM not found at $SERVICE_WASM"
    exit 1
fi

# Publish bytecode (for Linera SDK 0.15.8)
BYTECODE_OUTPUT=$(linera publish-bytecode "$CONTRACT_WASM" "$SERVICE_WASM" 2>&1)
echo "$BYTECODE_OUTPUT"

# Extract Bytecode ID
BYTECODE_ID=$(echo "$BYTECODE_OUTPUT" | grep -oE "Bytecode ID: [a-f0-9]+" | awk '{print $3}')

if [ -z "$BYTECODE_ID" ]; then
    # Try alternative format
    BYTECODE_ID=$(echo "$BYTECODE_OUTPUT" | grep -oE "Module ID: [a-f0-9]+" | awk '{print $3}')
fi

if [ -z "$BYTECODE_ID" ]; then
    echo "✗ Error: Failed to extract Bytecode ID from output"
    echo "Output was: $BYTECODE_OUTPUT"
    exit 1
fi

echo "✓ Bytecode ID: $BYTECODE_ID"

# Step 6: Create Application
echo ""
echo "[6/7] Creating application instance..."

APP_OUTPUT=$(linera create-application "$BYTECODE_ID" --json-argument "null" 2>&1)
echo "$APP_OUTPUT"

# Extract Application ID
APP_ID=$(echo "$APP_OUTPUT" | grep -oE "Application ID: [a-f0-9]+" | awk '{print $3}')

if [ -z "$APP_ID" ]; then
    echo "✗ Error: Failed to extract Application ID from output"
    echo "Output was: $APP_OUTPUT"
    exit 1
fi

echo "✓ Application ID: $APP_ID"

# Step 7: Update Frontend Configuration
echo ""
echo "[7/7] Updating frontend configuration..."

if [ ! -f "$CONFIG_PATH" ]; then
    echo "Creating new config.json..."
    mkdir -p "$(dirname "$CONFIG_PATH")"
    echo '{"chainId":"","marketAppId":"","tokenAppId":"","oracleAppId":""}' > "$CONFIG_PATH"
fi

# Update config.json using jq
jq --arg chain "$CHAIN_ID" \
   --arg app "$APP_ID" \
   '.chainId = $chain | .marketAppId = $app' \
   "$CONFIG_PATH" > "$CONFIG_PATH.tmp" && mv "$CONFIG_PATH.tmp" "$CONFIG_PATH"

echo "✓ Configuration updated"

# Display deployment summary
echo ""
echo "=========================================="
echo "✓ DEPLOYMENT SUCCESSFUL!"
echo "=========================================="
echo ""
echo "Deployment Details:"
echo "-------------------"
echo "Chain ID:       $CHAIN_ID"
echo "Application ID: $APP_ID"
echo "Bytecode ID:    $BYTECODE_ID"
echo ""
echo "Configuration:"
echo "-------------------"
echo "Config file: $CONFIG_PATH"
cat "$CONFIG_PATH"
echo ""
echo "=========================================="
echo "Next Steps:"
echo "1. Build frontend: cd frontend/client && npm install && npm run build"
echo "2. Serve frontend: npm run preview or deploy dist/ folder"
echo "3. Connect Linera Wallet extension to Conway testnet"
echo "=========================================="
