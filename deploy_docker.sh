#!/bin/bash
set -e

FAUCET_URL="https://faucet.testnet-conway.linera.net"
WALLET_DIR="/root/.config/linera"
mkdir -p "$WALLET_DIR"

echo "Initializing Wallet..."
if [ ! -f "$WALLET_DIR/wallet.json" ]; then
    linera wallet init --faucet "$FAUCET_URL"
else
    echo "Wallet found."
fi

echo "Syncing Balance..."
linera sync-balance

CHAIN_ID=$(linera wallet show | grep "Chain ID" | awk '{print $3}')
echo "Chain ID: $CHAIN_ID"

echo "Building Contracts..."
rustup target add wasm32-unknown-unknown
cd contracts/type_arena
cargo clean
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "Publishing Bytecode..."
CONTRACT="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
SERVICE="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

# For Linera 0.15.8, command is publish-bytecode
BYTECODE_OUT=$(linera publish-bytecode "$CONTRACT" "$SERVICE")
echo "$BYTECODE_OUT"
BYTECODE_ID=$(echo "$BYTECODE_OUT" | grep "Bytecode ID" | awk '{print $3}')

if [ -z "$BYTECODE_ID" ]; then
    echo "Failed to extract Bytecode ID"
    exit 1
fi

echo "Bytecode ID: $BYTECODE_ID"

echo "Creating Application..."
APP_OUT=$(linera create-application "$BYTECODE_ID" --json-argument "null")
echo "$APP_OUT"
APP_ID=$(echo "$APP_OUT" | grep "Application ID" | awk '{print $3}')

if [ -z "$APP_ID" ]; then
    echo "Failed to extract Application ID"
    exit 1
fi

echo "App ID: $APP_ID"

# Update config.json
CONFIG_PATH="frontend/client/public/config.json"
echo "Updating $CONFIG_PATH..."

if [ ! -f "$CONFIG_PATH" ]; then
    echo '{"chainId":"","marketAppId":"","tokenAppId":"","oracleAppId":""}' > "$CONFIG_PATH"
fi

# Use jq to update
jq --arg app "$APP_ID" --arg chain "$CHAIN_ID" '.marketAppId = $app | .chainId = $chain' "$CONFIG_PATH" > "$CONFIG_PATH.tmp" && mv "$CONFIG_PATH.tmp" "$CONFIG_PATH"

echo "Deployment Complete!"