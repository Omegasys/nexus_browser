#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

BUILD_DIR="$ROOT_DIR/target/flatpak"
RELEASE_DIR="$ROOT_DIR/target/release"

echo "==> Building Nexus Browser"

cargo build --release --workspace

if [[ ! -x "$RELEASE_DIR/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

if ! command -v flatpak-builder >/dev/null 2>&1; then
    echo "ERROR: flatpak-builder is not installed."
    exit 1
fi

rm -rf "$BUILD_DIR"

mkdir -p "$BUILD_DIR"

echo "==> Building Flatpak"

flatpak-builder \
    --force-clean \
    --repo="$BUILD_DIR/repo" \
    "$BUILD_DIR/build" \
    "$ROOT_DIR/packaging/flatpak/com.nexusbrowser.NexusBrowser.yml"

echo
echo "==> Flatpak bundle can be generated with:"

echo "flatpak build-bundle \\"
echo "    \"$BUILD_DIR/repo\" \\"
echo "    \"$ROOT_DIR/target/packages/NexusBrowser.flatpak\" \\"
echo "    com.nexusbrowser.NexusBrowser"

echo
echo "==> Flatpak build completed"
