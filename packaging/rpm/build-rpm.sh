#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

RELEASE_DIR="$ROOT_DIR/target/release"
RPM_ROOT="$ROOT_DIR/target/rpm"
OUTPUT_DIR="$ROOT_DIR/target/packages"

echo "==> Building Nexus Browser"

cargo build --release --workspace

if [[ ! -x "$RELEASE_DIR/nexus-browser" ]]; then
    echo "ERROR: nexus-browser binary was not found."
    exit 1
fi

if ! command -v rpmbuild >/dev/null 2>&1; then
    echo "ERROR: rpmbuild is not installed."
    exit 1
fi

rm -rf "$RPM_ROOT"

mkdir -p \
    "$RPM_ROOT/BUILD" \
    "$RPM_ROOT/RPMS" \
    "$RPM_ROOT/SOURCES" \
    "$RPM_ROOT/SPECS" \
    "$RPM_ROOT/SRPMS" \
    "$OUTPUT_DIR"

cp "$RELEASE_DIR/nexus-browser" \
    "$RPM_ROOT/BUILD/nexus-browser"

cp "$ROOT_DIR/packaging/rpm/nexus-browser.spec" \
    "$RPM_ROOT/SPECS/nexus-browser.spec"

echo "==> Building RPM"

rpmbuild \
    --define "_topdir $RPM_ROOT" \
    -bb \
    "$RPM_ROOT/SPECS/nexus-browser.spec"

find "$RPM_ROOT/RPMS" \
    -type f \
    -name "*.rpm" \
    -exec cp {} "$OUTPUT_DIR/" \;

echo
echo "==> RPM packaging completed"
