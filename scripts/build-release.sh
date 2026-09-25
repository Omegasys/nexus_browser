#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

RELEASE_DIR="$ROOT_DIR/target/release"

echo "========================================"
echo " Nexus Browser Release Build"
echo "========================================"
echo

echo "==> Repository: $ROOT_DIR"
echo "==> Release directory: $RELEASE_DIR"

echo
echo "==> Checking formatting"
cargo fmt --all -- --check

echo
echo "==> Running Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo
echo "==> Running tests"
cargo test --workspace --all-targets --all-features

echo
echo "==> Building rendering engines"
"$ROOT_DIR/scripts/build-engines.sh"

echo
echo "==> Building optimized release binaries"
cargo build --workspace --release --all-features

echo
echo "==> Verifying release artifacts"

if [[ ! -d "$RELEASE_DIR" ]]; then
    echo "ERROR: Release directory was not created."
    exit 1
fi

echo
echo "Release artifacts:"
find "$RELEASE_DIR" -maxdepth 2 -type f \
    \( -name "nexus*" -o -name "*.so" -o -name "*.dll" -o -name "*.dylib" -o -name "*.wasm" \) \
    -print 2>/dev/null || true

echo
echo "========================================"
echo " Release build completed successfully"
echo "========================================"
