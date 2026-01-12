#!/usr/bin/env bash
set -eu

# ------------------------------------------------------------------
# Type Arena - Local Development Runner
# ------------------------------------------------------------------

echo "Starting Local Linera Network..."
# Start the Linera network in the background
linera net up &
NET_PID=$!

# Ensure we kill the network when this script exits
trap "kill $NET_PID" EXIT

# Wait for the network to be ready
sleep 5

echo "Initializing Wallet..."
linera wallet init --faucet http://localhost:8080
CHAIN_ID=$(linera wallet show | grep "Chain ID" | awk '{print $3}')
echo "Using Chain ID: $CHAIN_ID"

echo "Building Rust Contracts..."
# Ensure target exists
rustup target add wasm32-unknown-unknown
cd contracts/type_arena
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "Publishing Bytecode..."
CONTRACT_WASM="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
SERVICE_WASM="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"
BYTECODE_ID=$(linera publish-bytecode "$CONTRACT_WASM" "$SERVICE_WASM" | grep "Bytecode ID:" | awk '{print $3}')
echo "Bytecode ID: $BYTECODE_ID"

echo "Creating Application..."
# Using "null" for instantiation argument
APP_ID=$(linera create-application "$BYTECODE_ID" --json-argument "null" | grep "Application ID:" | awk '{print $3}')
echo "Application ID: $APP_ID"

# Update Frontend Configuration
CONFIG_PATH="frontend/client/public/config.json"
echo "{" > $CONFIG_PATH
echo "  \"chainId\": \"$CHAIN_ID\"," >> $CONFIG_PATH
echo "  \"marketAppId\": \"$APP_ID\"," >> $CONFIG_PATH
echo "  \"tokenAppId\": \"\"," >> $CONFIG_PATH
echo "  \"oracleAppId\": \"\"" >> $CONFIG_PATH
echo "}" >> $CONFIG_PATH
echo "Updated $CONFIG_PATH"

echo "Building Frontend..."
cd frontend/client
npm install
npm run build
cd ../..

echo "Serving Application..."
# We use http-server with specific headers required for SharedArrayBuffer (Linera Wasm requirement)
# -H flags add the Cross-Origin headers.
npx http-server frontend/client/dist -p 8081 --cors -H '{"Cross-Origin-Embedder-Policy": "require-corp", "Cross-Origin-Opener-Policy": "same-origin"}'
