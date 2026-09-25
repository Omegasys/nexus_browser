#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> Building Nexus Browser engines"

ENGINE_ROOT="$ROOT_DIR/engines"

if [[ ! -d "$ENGINE_ROOT" ]]; then
    echo "ERROR: Engine directory does not exist:"
    echo "    $ENGINE_ROOT"
    exit 1
fi

build_rust_engine() {
    local engine="$1"
    local engine_dir="$ENGINE_ROOT/rendering/$engine"

    if [[ -f "$engine_dir/Cargo.toml" ]]; then
        echo
        echo "==> Building Rust engine: $engine"
        cargo build --manifest-path "$engine_dir/Cargo.toml"
    else
        echo
        echo "==> No standalone Cargo.toml for $engine; skipping Rust build"
    fi
}

build_cpp_engine() {
    local engine="$1"
    local engine_dir="$ENGINE_ROOT/rendering/$engine"
    local build_file="$engine_dir/build.json"

    if [[ ! -d "$engine_dir" ]]; then
        echo "WARNING: Engine directory not found: $engine_dir"
        return
    fi

    echo
    echo "==> Preparing C/C++ engine: $engine"

    if [[ -f "$build_file" ]]; then
        echo "    Build manifest: $build_file"
    else
        echo "    WARNING: No build.json found"
    fi

    if [[ -f "$engine_dir/CMakeLists.txt" ]]; then
        local build_dir="$engine_dir/build"

        cmake -S "$engine_dir" -B "$build_dir"
        cmake --build "$build_dir" --parallel
    else
        echo "    No CMakeLists.txt found."
        echo "    Engine source is present but has no standalone build system yet."
    fi
}

build_cpp_engine "blink"
build_cpp_engine "gecko"
build_rust_engine "servo"

echo
echo "==> Engine build process completed"
