# Nexus Browser Build Artifacts

This directory contains validated build artifacts produced by the Nexus
Browser build system.

Artifacts may include:

- Browser binaries
- Engine binaries
- Shared libraries
- WebAssembly modules
- MicroVM images
- Engine manifests
- VM manifests
- Security metadata
- Build manifests
- Checksums
- Signatures
- Debug symbols
- Release bundles

Artifacts should only be placed here after the appropriate build and
validation stages have completed.

The intended artifact lifecycle is:

Source
    ↓
Compile
    ↓
Build
    ↓
Test
    ↓
Validate
    ↓
Security checks
    ↓
Artifact
    ↓
Package
    ↓
Release

The artifact manager should maintain version information and associate each
artifact with its originating engine, build configuration, platform, and
architecture.

Examples of artifact metadata include:

- Artifact identifier
- Artifact version
- Source version
- Engine version
- Target operating system
- Target architecture
- Build profile
- Compiler version
- SHA-256 checksum
- Signature
- Creation timestamp
- Validation status
- Trust state

Generated release artifacts should normally be copied into the appropriate
packaging directory or release output rather than manually committed to the
source repository.
