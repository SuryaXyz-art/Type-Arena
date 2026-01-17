#!/bin/bash
set -e

# Install netcat for port checking if not present (handled in Dockerfile ideally)

echo "Starting Local Linera Network..."
# Start network with faucet on 8080. 
# Note: In docker, we bind to 0.0.0.0 via the binary or it defaults to localhost? 
# linera net up defaults to localhost usually. We might need a proxy or rely on docker network mode host, 
# but host mode doesn't work well on Windows.
# We will use 'linera net up --faucet-port 8080' and assume it binds to 0.0.0.0 or we can access it via localhost inside container.
# External access requires mapping.

linera net up --with-faucet --faucet-port 8080 &
NET_PID=$!

echo "Waiting for faucet..."
# Simple wait loop
for i in {1..30}; do
    if curl -s http://localhost:8080 > /dev/null; then
        echo "Faucet is up!"
        break
    fi
    sleep 1
    echo "Waiting..."
done

echo "Initializing Wallet..."
linera wallet init --faucet http://localhost:8080

echo "Syncing..."
linera sync-balance

CHAIN_ID=
echo "Chain ID: "

echo "Building Contracts..."
rustup target add wasm32-unknown-unknown
cd contracts/type_arena
cargo build --release --target wasm32-unknown-unknown
cd ../..

echo "Publishing Module..."
CONTRACT="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
SERVICE="contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

# Using publish-module for 0.16
BYTECODE_OUT=
echo ""
BYTECODE_ID=

if [ -z "" ]; then
    # Fallback search
    BYTECODE_ID=
fi

echo "Module ID: "

echo "Creating App..."
APP_OUT=
echo ""
APP_ID=

echo "App ID: "

# Update config.json (mounted volume)
echo "Updating config..."
echo "{\"chainId\":\"\",\"marketAppId\":\"\",\"tokenAppId\":\"\",\"oracleAppId\":\"\"}" > /app/frontend/client/public/config.json

echo "Starting Node Service on port 8081..."
# Validating: --port binds to 127.0.0.1 by default? Linera service usually binds to localhost.
# To make it accessible outside Docker, we might need a workaround or use 'linera service --port 8081' and hope.
# Actually, standard Linera service binds to 127.0.0.1. 
# We need to use socat to forward 0.0.0.0:8081 -> 127.0.0.1:8081 if Linera doesn't support binding host.
# But let's try running it. 
linera service --port 8081 &
SERVICE_PID=$!

# Also forward Faucet if needed? Faucet is part of net up.
# We will use socat for robust forwarding if installed.

echo "------------------------------------------------"
echo "LOCAL NETWORK READY"
echo "------------------------------------------------"
echo "Faucet: http://localhost:8080"
echo "Service: http://localhost:8081"
echo "Chain ID: "
echo "App ID: "
echo "------------------------------------------------"
echo "Press Ctrl+C to stop"

# Keep running
wait 