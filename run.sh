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
WASM_PATH="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena.wasm"
BYTECODE_ID=$(linera publish-bytecode "$WASM_PATH" "$WASM_PATH" | grep "Bytecode ID:" | awk '{print $3}')
echo "Bytecode ID: $BYTECODE_ID"

echo "Creating Application..."
APP_ID=$(linera create-application "$BYTECODE_ID" --json-argument "{}" | grep "Application ID:" | awk '{print $3}')
echo "Application ID: $APP_ID"

# Update Frontend Environment
echo "VITE_TOKEN_APP_ID=$APP_ID" > frontend/client/.env
echo "VITE_CHAIN_ID=$CHAIN_ID" >> frontend/client/.env

echo "Building Frontend..."
cd frontend/client
npm install
npm run build
cd ../..

echo "Serving Application..."
# We use http-server with specific headers required for SharedArrayBuffer (Linera Wasm requirement)
# -H flags add the Cross-Origin headers.
npx http-server frontend/client/dist -p 8081 --cors -H '{"Cross-Origin-Embedder-Policy": "require-corp", "Cross-Origin-Opener-Policy": "same-origin"}'
