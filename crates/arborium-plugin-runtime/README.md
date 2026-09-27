# arborium-plugin-runtime

Runtime library for arborium grammar plugins.

## Purpose

Provides the core functionality for implementing a tree-sitter grammar as a
WASM plugin. This is linked into each grammar plugin.

## Features

- Session management (create/free)
- Parser state and tree storage
- Query execution to produce Span and Injection records
- Incremental parsing via edit application
- Cancellation support

This is an internal crate used by generated grammar plugins.
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
