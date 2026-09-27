# arborium-rustdoc

Post-process rustdoc HTML to add syntax highlighting for non-Rust code blocks.

## Purpose

Rustdoc already highlights Rust code using rustc's parser, but code blocks in
other languages (Python, JavaScript, TOML, etc.) are left unhighlighted.
This tool fixes that.

## Usage

```bash
# Generate rustdoc as usual
cargo doc

# Post-process to add highlighting
arborium-rustdoc ./target/doc ./target/doc-highlighted
```

## How It Works

1. **CSS Generation**: Appends theme CSS rules to rustdoc's stylesheet
2. **HTML Transformation**: Uses streaming HTML parsing (lol_html) to find
   `<pre class="language-*">` elements and replace their content with
   syntax-highlighted HTML

The output uses arborium's custom elements (`<a-k>`, `<a-f>`, etc.) which
are styled by the injected CSS.
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
