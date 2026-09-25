#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PACKAGE_ROOT="$ROOT_DIR/packaging/deb"
RELEASE_DIR="$ROOT_DIR/target/release"
OUTPUT_DIR="$ROOT_DIR/target/packages"

echo "==> Building Nexus Browser"

cargo build --release --workspace

if [[ ! -x "$RELEASE_DIR/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

mkdir -p "$PACKAGE_ROOT/usr/bin"
mkdir -p "$OUTPUT_DIR"

echo "==> Installing binary into Debian package tree"

cp "$RELEASE_DIR/nexus-browser" \
    "$PACKAGE_ROOT/usr/bin/nexus-browser"

chmod 755 "$PACKAGE_ROOT/usr/bin/nexus-browser"

chmod 755 "$PACKAGE_ROOT/DEBIAN/postinst"
chmod 755 "$PACKAGE_ROOT/DEBIAN/prerm"
chmod 755 "$PACKAGE_ROOT/DEBIAN/postrm"

echo "==> Building .deb package"

dpkg-deb \
    --build \
    "$PACKAGE_ROOT" \
    "$OUTPUT_DIR/nexus-browser_0.1.0_amd64.deb"

echo
echo "==> Debian package created:"
echo "    $OUTPUT_DIR/nexus-browser_0.1.0_amd64.deb"
