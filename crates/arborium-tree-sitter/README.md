# arborium-tree-sitter

Vendored tree-sitter Rust bindings for arborium.

## Purpose

This is a fork of the official `tree-sitter` crate, modified for arborium's needs:

- Custom build configuration for WASM targets
- Compatibility patches for the arborium plugin system
- Pinned version to ensure consistent behavior across all grammar crates

Most users should use the main `arborium` crate instead of depending on this directly.
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
