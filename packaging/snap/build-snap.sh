#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "==> Building Nexus Browser"

cargo build --release --workspace

if [[ ! -x "$ROOT_DIR/target/release/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

if ! command -v snapcraft >/dev/null 2>&1; then
    echo "ERROR: snapcraft is not installed."
    exit 1
fi

echo "==> Building Snap package"

cd "$ROOT_DIR/packaging/snap"

snapcraft pack \
    --destructive-mode

echo
echo "==> Snap packaging completed"
