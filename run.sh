#!/usr/bin/env bash
set -eu

# Start the Linera network
linera net up &

# Wait for the network to be ready
sleep 5

# Create a new wallet and chain
linera wallet init --faucet http://localhost:8080
CHAIN_ID=$(linera wallet show | grep "Chain ID" | awk '{print $3}')

# Build the Linera applications
cargo build --release --target wasm32-unknown-unknown

# Publish and create the token application
TOKEN_APP_ID=$(linera project publish-and-create token)

# Publish and create the market application
MARKET_APP_ID=$(linera project publish-and-create market --required-application-ids $TOKEN_APP_ID)

# Publish and create the oracle application
ORACLE_APP_ID=$(linera project publish-and-create oracle --required-application-ids $MARKET_APP_ID)

# Build the frontend
cd frontend/client
npm run build
cd ../..

# Create a dist directory
mkdir -p dist

# Copy the built frontend to the dist directory
cp -r frontend/client/dist/* dist

# Generate the config.json file
jq -n \
  --arg chainId "$CHAIN_ID" \
  --arg tokenAppId "$TOKEN_APP_ID" \
  --arg marketAppId "$MARKET_APP_ID" \
  --arg oracleAppId "$ORACLE_APP_ID" \
  '{
    chainId: $chainId,
    tokenAppId: $tokenAppId,
    marketAppId: $marketAppId,
    oracleAppId: $oracleAppId
  }' > dist/config.json

# Start the web server
npx http-server dist
