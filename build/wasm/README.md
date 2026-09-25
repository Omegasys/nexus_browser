# Nexus Browser WebAssembly Build Output

This directory contains generated WebAssembly modules used by Nexus Browser.

WASM source code and engine source code belong in their respective source
directories.

Generated files may include:

- .wasm modules
- .wat modules
- WASM component artifacts
- WASM engine adapters
- WASM interface definitions
- WASM optimization output
- WASM validation metadata
- WASM manifests
- Debug symbols

WASM modules should be validated before being promoted to the runtime.

Recommended pipeline:

WASM source
    ↓
WASM compiler
    ↓
build/wasm/
    ↓
WASM validator
    ↓
sandbox testing
    ↓
artifact manager
    ↓
build/artifacts/
