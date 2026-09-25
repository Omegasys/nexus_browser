#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> Running Nexus Browser lint checks"

echo "==> Checking formatting"
cargo fmt --all -- --check

echo "==> Running Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo
echo "==> Lint checks completed successfully"
