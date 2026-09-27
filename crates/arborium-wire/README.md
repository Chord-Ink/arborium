# arborium-wire

Wire protocol types for arborium WASM plugins.

## Purpose

Defines the data structures used for communication between the arborium host
and grammar plugins. All types use serde for serialization.

## Wire Version

The `WIRE_VERSION` constant is checked by both host and plugins to ensure
compatibility. If versions don't match, the host rejects the plugin with
a clear error message.

## Types

- `Span`: A highlighted region with a capture name
- `Injection`: A point where another language should be parsed
- `Edit`: An incremental edit for re-parsing

This is an internal crate used by the plugin system.
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
