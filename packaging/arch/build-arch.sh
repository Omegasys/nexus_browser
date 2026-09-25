#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

RELEASE_DIR="$ROOT_DIR/target/release"
OUTPUT_DIR="$ROOT_DIR/target/packages"

echo "==> Building Nexus Browser"

cargo build --release --workspace

if [[ ! -x "$RELEASE_DIR/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

if ! command -v makepkg >/dev/null 2>&1; then
    echo "ERROR: makepkg is not installed."
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

echo "==> Building Arch Linux package"

cd "$ROOT_DIR/packaging/arch"

makepkg \
    --cleanbuild \
    --clean \
    --force

echo "==> Moving package artifacts"

find "$ROOT_DIR/packaging/arch" \
    -maxdepth 1 \
    -type f \
    \( -name "*.pkg.tar.zst" -o -name "*.pkg.tar.xz" \) \
    -exec cp {} "$OUTPUT_DIR/" \;

echo
echo "==> Arch Linux packaging completed"
