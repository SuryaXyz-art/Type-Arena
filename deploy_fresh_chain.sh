#!/bin/bash
set -e

FAUCET_URL="https://faucet.testnet-conway.linera.net"
PUBLIC_KEY="$1"
CONFIG_PATH="/app/frontend/client/public/config.json"

echo "==========================================="
echo "Type Arena - Conway Testnet Deployment"
echo "==========================================="

# Step 1: Claim a new chain from faucet with our public key
echo "[1/4] Claiming chain from Conway faucet..."
CLAIM_RESULT=$(curl -s -X POST -H "Content-Type: application/json" \
  -d "{\"query\":\"mutation { claim(publicKey: \\\"$PUBLIC_KEY\\\") { messageId chainId } }\"}" \
  "$FAUCET_URL")

echo "Faucet response: $CLAIM_RESULT"

# Extract chain ID from response
CHAIN_ID=$(echo "$CLAIM_RESULT" | jq -r '.data.claim.chainId // empty')
MESSAGE_ID=$(echo "$CLAIM_RESULT" | jq -r '.data.claim.messageId // empty')

if [ -z "$CHAIN_ID" ]; then
    echo "Failed to get chain ID from faucet"
    echo "Response: $CLAIM_RESULT"
    exit 1
fi

echo "✓ Chain ID: $CHAIN_ID"
echo "✓ Message ID: $MESSAGE_ID"

# Step 2: Assign the chain to our wallet
echo ""
echo "[2/4] Assigning chain to wallet..."
linera assign --owner "$PUBLIC_KEY" --chain-id "$CHAIN_ID" --message-id "$MESSAGE_ID"

# Step 3: Publish and create application
echo ""
echo "[3/4] Publishing bytecode and creating application..."
CONTRACT_WASM="/app/contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
SERVICE_WASM="/app/contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

APP_OUTPUT=$(linera publish-and-create "$CONTRACT_WASM" "$SERVICE_WASM" --json-argument null 2>&1)
echo "$APP_OUTPUT"

# Extract Application ID
APP_ID=$(echo "$APP_OUTPUT" | grep -oE '[a-f0-9]{64}' | tail -1)

if [ -z "$APP_ID" ]; then
    echo "Failed to extract Application ID"
    exit 1
fi

echo "✓ Application ID: $APP_ID"

# Step 4: Update config file
echo ""
echo "[4/4] Updating frontend configuration..."
if [ -f "$CONFIG_PATH" ]; then
    jq --arg chain "$CHAIN_ID" --arg app "$APP_ID" \
       '.chainId = $chain | .marketAppId = $app' \
       "$CONFIG_PATH" > "$CONFIG_PATH.tmp" && mv "$CONFIG_PATH.tmp" "$CONFIG_PATH"
    echo "✓ Configuration updated"
    cat "$CONFIG_PATH"
else
    echo "Config file not found, creating..."
    echo "{\"chainId\":\"$CHAIN_ID\",\"marketAppId\":\"$APP_ID\",\"tokenAppId\":\"\",\"oracleAppId\":\"\"}" > "$CONFIG_PATH"
fi

echo ""
echo "==========================================="
echo "✓ DEPLOYMENT SUCCESSFUL!"
echo "==========================================="
