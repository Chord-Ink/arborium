# arborium-ratatui

Render Arborium syntax highlighting as owned Ratatui `Line`s, using Arborium themes.
Enable language features on your `arborium` dependency. Compatible with Ratatui 0.30.

```rust
use arborium::{Highlighter, theme::builtin};

let lines = arborium_ratatui::highlight(
    &mut Highlighter::new(),
    &builtin::catppuccin_mocha(),
    "rust",
    "fn main() {}",
)?;
// Pass lines to ratatui::widgets::Paragraph::new(lines).
# Ok::<(), arborium::Error>(())
```

`spans_to_lines` also accepts precomputed Arborium spans. Both APIs preserve blank
lines and Unicode, resolve nested highlights, and apply colors and text modifiers.
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
