# arborium-highlight

Core syntax highlighting engine for arborium.

## Features

This crate provides the unified highlighting engine that works with both:

- **Statically linked Rust grammars**: For CLI tools and servers
- **Dynamically loaded WASM plugins**: For browser contexts

## Why Async?

The parsing is synchronous (tree-sitter fundamentally is), but *getting* a grammar
can be async in browser contexts where plugins are loaded from a CDN via dynamic
`import()`. The async trait supports both use cases with the same API.

## Usage

```rust
use arborium_highlight::{Span, spans_to_html, HtmlFormat};

// After getting spans from a grammar...
let html = spans_to_html(source, spans, &HtmlFormat::CustomElements);
```
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
