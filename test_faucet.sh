#!/bin/bash
set -e

FAUCET_URL="https://faucet.testnet-conway.linera.net"
PUBLIC_KEY="$1"

echo "Claiming chain from faucet with owner: $PUBLIC_KEY"

# Using different format - the faucet expects a simpler request
CLAIM_RESULT=$(curl -s \
  "$FAUCET_URL" \
  -X POST \
  -H "Content-Type: application/json" \
  --data '{"query":"mutation { claim(owner: \"'"$PUBLIC_KEY"'\") { messageId outcome { User { chainId epoch } } } }"}')

echo "Result: $CLAIM_RESULT"
