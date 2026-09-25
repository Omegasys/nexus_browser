#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> Running Nexus Browser test suite"

cargo test --workspace --all-targets --all-features

echo
echo "==> Tests completed successfully"
