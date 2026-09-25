#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

APPDIR="$ROOT_DIR/target/appimage/NexusBrowser.AppDir"
RELEASE_DIR="$ROOT_DIR/target/release"
OUTPUT_DIR="$ROOT_DIR/target/packages"

mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/metainfo"

echo "==> Preparing AppImage directory"

cargo build --release --workspace

if [[ ! -x "$RELEASE_DIR/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

cp "$RELEASE_DIR/nexus-browser" \
    "$APPDIR/usr/bin/nexus-browser"

cp "$ROOT_DIR/packaging/appimage/AppRun" \
    "$APPDIR/AppRun"

cp "$ROOT_DIR/packaging/appimage/nexus-browser.desktop" \
    "$APPDIR/usr/share/applications/nexus-browser.desktop"

cp "$ROOT_DIR/packaging/appimage/nexus-browser.desktop" \
    "$APPDIR/nexus-browser.desktop"

cp "$ROOT_DIR/packaging/appimage/nexus-browser.appdata.xml" \
    "$APPDIR/usr/share/metainfo/com.nexusbrowser.NexusBrowser.appdata.xml"

chmod +x "$APPDIR/AppRun"

mkdir -p "$OUTPUT_DIR"

if command -v appimagetool >/dev/null 2>&1; then
    echo "==> Creating AppImage"

    appimagetool \
        "$APPDIR" \
        "$OUTPUT_DIR/NexusBrowser-x86_64.AppImage"
else
    echo "WARNING: appimagetool is not installed."
    echo "The AppDir was created successfully:"
    echo "    $APPDIR"
fi

echo "==> AppImage packaging completed"
