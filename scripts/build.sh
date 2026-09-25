#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> Building Nexus Browser"
echo "    Repository: $ROOT_DIR"

cargo build --workspace

echo
echo "==> Build completed successfully"
