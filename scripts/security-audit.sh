#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> Running Nexus Browser security audit"

if ! command -v cargo-audit >/dev/null 2>&1; then
    echo "ERROR: cargo-audit is not installed."
    echo
    echo "Install it with:"
    echo "    cargo install cargo-audit"
    exit 1
fi

echo "==> Auditing Rust dependencies"
cargo audit

echo
echo "==> Checking dependency tree"
cargo tree --workspace --all-features

echo
echo "==> Running security-focused Clippy checks"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo
echo "==> Security audit completed successfully"
