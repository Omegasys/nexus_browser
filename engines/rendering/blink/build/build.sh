#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

BUILD_TYPE="${BUILD_TYPE:-Release}"
BUILD_DIR="${SCRIPT_DIR}/output"

GENERATOR="${CMAKE_GENERATOR:-Ninja}"

echo "Nexus Blink Engine"
echo "=================="
echo
echo "Project: ${PROJECT_DIR}"
echo "Build type: ${BUILD_TYPE}"
echo "Generator: ${GENERATOR}"
echo

if ! command -v cmake >/dev/null 2>&1; then
    echo "Error: CMake is not installed."
    exit 1
fi

mkdir -p "${BUILD_DIR}"

echo "Configuring..."

cmake \
    -S "${PROJECT_DIR}" \
    -B "${BUILD_DIR}" \
    -G "${GENERATOR}" \
    -DCMAKE_BUILD_TYPE="${BUILD_TYPE}" \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

echo
echo "Building..."

cmake \
    --build "${BUILD_DIR}" \
    --parallel

echo
echo "Build completed successfully."

echo
echo "Output directory:"
echo "${BUILD_DIR}"
