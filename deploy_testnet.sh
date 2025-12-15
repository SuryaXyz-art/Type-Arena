#!/usr/bin/env bash
set -e

# ------------------------------------------------------------------
# Type Arena - Testnet Conway Deployer
# Use this script to deploy your game to the public Linera Testnet.
# ------------------------------------------------------------------

FAUCET_URL="https://faucet.testnet-conway.linera.net"

echo "Initializing Wallet against Testnet Conway..."
# Check if wallet already exists to avoid overwriting
if [ ! -f "$HOME/.config/linera/wallet.json" ]; then
    linera wallet init --faucet "$FAUCET_URL"
else
    echo "Wallet found. Using existing wallet."
fi

# Sync balance to ensure we are connected
linera sync-balance

CHAIN_ID=$(linera wallet show | grep "Chain ID" | awk '{print $3}')
echo "Deploying from Chain: $CHAIN_ID"

echo "Building Rust Contracts..."
rustup target add wasm32-unknown-unknown
cd contracts/type_arena
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "Publishing Bytecode to Testnet..."
WASM_PATH="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena.wasm"
BYTECODE_ID=$(linera publish-bytecode "$WASM_PATH" "$WASM_PATH" | grep "Bytecode ID:" | awk '{print $3}')
echo "Bytecode ID: $BYTECODE_ID"

echo "Creating Application on Testnet..."
APP_ID=$(linera create-application "$BYTECODE_ID" --json-argument "{}" | grep "Application ID:" | awk '{print $3}')
echo "--------------------------------------------------------"
echo "DEPLOYMENT SUCCESSFUL!"
echo "--------------------------------------------------------"
echo "Application ID: $APP_ID"
echo "Chain ID:       $CHAIN_ID"
echo "--------------------------------------------------------"
echo "Next Steps:"
echo "1. Update your 'frontend/client/.env' file with these values."
echo "2. Run 'cd frontend/client && npm run dev' to test locally against the testnet."
echo "3. Or deploy your 'frontend/client/dist' folder to Vercel/Netlify."
