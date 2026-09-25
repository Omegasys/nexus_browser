# Nexus Browser MicroVM Build Output

This directory contains generated MicroVM images and supporting artifacts.

MicroVM source and management code belongs under:

microvm/

Generated files may include:

- Renderer VM images
- JavaScript VM images
- Network VM images
- DNS VM images
- Storage VM images
- GPU VM images
- Identity VM images
- VM kernels
- VM initramfs images
- VM configuration files
- VM snapshots
- VM manifests
- VM recovery images

The MicroVM build pipeline should produce isolated artifacts that can
subsequently be validated, signed, versioned, and promoted into the Nexus
Browser runtime.

Typical build flow:

microvm/
    ↓
VM builder
    ↓
build/microvm/
    ↓
VM validation
    ↓
VM security checks
    ↓
build/artifacts/
