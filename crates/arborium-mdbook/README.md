# arborium-mdbook

[mdBook](https://rust-lang.github.io/mdBook/) preprocessor for syntax highlighting with arborium.

## Purpose

Replaces mdBook's built-in syntax highlighting with arborium's tree-sitter
based highlighter, providing better accuracy and more language support.

## Installation

```bash
cargo install arborium-mdbook
```

## Configuration

Add to your `book.toml`:

```toml
[preprocessor.arborium]
command = "arborium-mdbook"
```

## Features

- Highlights all fenced code blocks with language annotations
- Supports all languages available in arborium
- Uses arborium's custom HTML elements for styling
- Compatible with mdBook's standard themes
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
