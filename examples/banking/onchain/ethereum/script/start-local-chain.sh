#!/usr/bin/env bash
set -euo pipefail

exec anvil \
  --host "${ETHEREUM_RPC_HOST:-127.0.0.1}" \
  --port "${ETHEREUM_RPC_PORT:-8545}" \
  --chain-id "${ETHEREUM_CHAIN_ID:-31337}"
