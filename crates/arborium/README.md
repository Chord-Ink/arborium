# arborium

Batteries-included [tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammar collection with HTML rendering and WASM support.

[![Crates.io](https://img.shields.io/crates/v/arborium.svg)](https://crates.io/crates/arborium)
[![Documentation](https://docs.rs/arborium/badge.svg)](https://docs.rs/arborium)
[![License](https://img.shields.io/crates/l/arborium.svg)](LICENSE-MIT)

## Features

- **112 language grammars** included out of the box
- **111 permissively licensed** (MIT/Apache-2.0/CC0/Unlicense) grammars enabled by default
- **WASM support** with custom allocator fix
- **Feature flags** for fine-grained control over included languages

### Using a Git dependency

The Rust manifests and generated build inputs are checked in. Cargo can build
this repository directly, without running `cargo xtask` or installing Node.js
or the tree-sitter CLI:

```toml
[dependencies]
arborium = { git = "https://github.com/Chord-Ink/arborium", default-features = false, features = ["lang-rust", "lang-javascript"] }
# Individual grammars are also available:
arborium-json = { git = "https://github.com/Chord-Ink/arborium" }
```

Add `rev = "<commit>"` to pin a revision. Builds require Rust 1.90 or newer and a
C compiler. For `wasm32-unknown-unknown`, use a clang with WebAssembly support
(see `.envrc` for macOS setup).


## Usage

```toml
[dependencies]
arborium = "2.18.2"
```

By default, all permissively-licensed grammars are included. To select specific languages:

```toml
[dependencies]
arborium = { version = "2.18.2", default-features = false, features = ["lang-rust", "lang-javascript"] }
```

## External Grammar Crates

You can ship Arborium-compatible grammar crates outside this repository. Arborium only needs a `language()` export plus highlight or injection query constants, which you can compile into a `CompiledGrammar` and register in a `GrammarStore`.

See the in-repo guide: [EXTERNAL_GRAMMAR_CRATES.md](https://github.com/bearcove/arborium/blob/main/EXTERNAL_GRAMMAR_CRATES.md)

## Browser Usage

Arborium can be used in the browser in two ways:

### Option 1: Drop-in Script (Easiest)

Add a single script tag and arborium auto-highlights all code blocks:

```html
<script src="https://cdn.jsdelivr.net/npm/@arborium/arborium/dist/arborium.iife.js"></script>
```

That's it! Arborium will:
- Auto-detect languages from `class="language-*"` or `data-lang="*"` attributes
- Load grammar WASM plugins on-demand from jsDelivr CDN
- Inject the default theme CSS

**Configuration via data attributes:**

```html
<script
  src="https://cdn.jsdelivr.net/npm/@arborium/arborium/dist/arborium.iife.js"
  data-theme="mocha"
  data-selector="pre code"
  data-manual
></script>
```

**Configuration via JavaScript:**

```html
<script>
  window.Arborium = {
    theme: 'tokyo-night',
    selector: 'pre code, .highlight',
    cdn: 'jsdelivr',  // or 'unpkg' or a custom URL
    version: '1', // or 'latest'
  };
</script>
<script src="https://cdn.jsdelivr.net/npm/@arborium/arborium/dist/arborium.iife.js"></script>
```

**Manual highlighting:**

```html
<script src="..." data-manual></script>
<script>
  // Highlight all code blocks
  arborium.highlightAll();

  // Highlight a specific element
  arborium.highlightElement(document.querySelector('code'), 'rust');
</script>
```

### Option 2: ESM Module (Programmatic)

For bundlers (Vite, webpack, etc.) or ESM-native environments:

```bash
npm install @arborium/arborium
```

```typescript
import { loadGrammar, highlight } from '@arborium/arborium';

// Load a grammar (fetched from CDN on first use)
const grammar = await loadGrammar('rust');

// Highlight code
const html = grammar.highlight('fn main() { println!("Hello!"); }');

// Or use the convenience function
const html = await highlight('rust', code);
```

### Option 3: Compile Rust to WASM (Maximum Control)

For complete control and offline-first apps, compile the Rust crate directly to WASM:

```toml
[dependencies]
arborium = { version = "2.18.2", default-features = false, features = ["lang-rust", "lang-javascript"] }
```

```bash
# Requires LLVM with WASM support (see FAQ below)
cargo build --target wasm32-unknown-unknown
```

This embeds selected grammars directly in your WASM binary - no CDN required at runtime.

### Themes

Arborium includes 33 built-in themes from popular color schemes.

**Dark themes:** `catppuccin-mocha`, `catppuccin-macchiato`, `catppuccin-frappe`, `dracula`, `evergarden-fall`, `tokyo-night`, `nord`, `one-dark`, `github-dark`, `gruvbox-dark`, `monokai`, `kanagawa-dragon`, `rose-pine-moon`, `ayu-dark`, `solarized-dark`, `ef-melissa-dark`, `melange-dark`, `cobalt2`, `zenburn`, `desert256`, `rustdoc-dark`, `rustdoc-ayu`

**Light themes:** `catppuccin-latte`, `github-light`, `gruvbox-light`, `ayu-light`, `solarized-light`, `melange-light`, `light-owl`, `lucius-light`, `dayfox`, `alabaster`, `rustdoc-light`

Import theme CSS:
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@arborium/arborium/dist/themes/tokyo-night.css">
```

Or let the IIFE bundle auto-inject it via the `data-theme` attribute.

**Theme Attribution:** All themes are adaptations of color schemes from their original projects. See the [arborium-theme crate README](https://github.com/bearcove/arborium/tree/main/crates/arborium-theme#built-in-themes) for full attribution and source links.

## Feature Flags

### Grammar Collections

| Feature | Description |
|---------|-------------|
| `mit-grammars` | All permissively licensed grammars (MIT, Apache-2.0, CC0) - **default** |
| `gpl-grammars` | GPL-licensed grammars (copyleft - may affect your project's license) |
| `all-grammars` | All grammars including GPL |

### Permissive Grammars (111)

These grammars use permissive licenses (MIT, Apache-2.0, CC0, Unlicense) and are included by default.

| Feature | Language | License | Source |
|---------|----------|---------|--------|

| `lang-ada` | Ada | MIT | [tree-sitter-ada](https://github.com/briot/tree-sitter-ada) |

| `lang-agda` | Agda | MIT | [tree-sitter-agda](https://github.com/tree-sitter/tree-sitter-agda) |

| `lang-asciidoc` | Asciidoc | Apache-2.0 | [tree-sitter-asciidoc](https://github.com/cathaysia/tree-sitter-asciidoc) |

| `lang-asm` | Assembly | MIT | [tree-sitter-asm](https://github.com/RubixDev/tree-sitter-asm) |

| `lang-awk` | AWK | MIT | [tree-sitter-awk](https://github.com/Beaglefoot/tree-sitter-awk) |

| `lang-bash` | Bash | MIT | [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) |

| `lang-batch` | Batch | MIT | [tree-sitter-batch](https://github.com/davidevofficial/tree-sitter-batch) |

| `lang-c` | C | MIT | [tree-sitter-c](https://github.com/tree-sitter/tree-sitter-c) |

| `lang-c-sharp` | C# | MIT | [tree-sitter-c-sharp](https://github.com/tree-sitter/tree-sitter-c-sharp) |

| `lang-caddy` | Caddyfile | MIT | [tree-sitter-caddy](https://github.com/Samonitari/tree-sitter-caddy) |

| `lang-capnp` | Cap&#039;n Proto | MIT | [tree-sitter-capnp](https://github.com/tree-sitter-grammars/tree-sitter-capnp) |

| `lang-cedar` | Cedar | MIT OR Apache-2.0 | [tree-sitter-cedar](https://github.com/DuskSystems/tree-sitter-cedar) |

| `lang-cedarschema` | Cedar Schema | MIT OR Apache-2.0 | [tree-sitter-cedarschema](https://github.com/DuskSystems/tree-sitter-cedar) |

| `lang-clojure` | Clojure | Unlicense | [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) |

| `lang-cmake` | CMake | MIT | [tree-sitter-cmake](https://github.com/uyha/tree-sitter-cmake) |

| `lang-cobol` | COBOL | MIT | [tree-sitter-cobol](https://github.com/zharinov/tree-sitter-cobol) |

| `lang-commonlisp` | Common Lisp | MIT | [tree-sitter-commonlisp](https://github.com/theHamsta/tree-sitter-commonlisp) |

| `lang-cpp` | C++ | MIT | [tree-sitter-cpp](https://github.com/tree-sitter/tree-sitter-cpp) |

| `lang-css` | CSS | MIT | [tree-sitter-css](https://github.com/tree-sitter/tree-sitter-css) |

| `lang-d` | D | MIT | [tree-sitter-d](https://github.com/gdamore/tree-sitter-d) |

| `lang-dart` | Dart | MIT | [tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart) |

| `lang-devicetree` | Device Tree | MIT | [tree-sitter-devicetree](https://github.com/joelspadin/tree-sitter-devicetree) |

| `lang-diff` | Diff | MIT | [tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff) |

| `lang-dockerfile` | Dockerfile | MIT | [tree-sitter-dockerfile](https://github.com/camdencheek/tree-sitter-dockerfile) |

| `lang-dot` | DOT/Graphviz | MIT | [tree-sitter-dot](https://github.com/rydesun/tree-sitter-dot) |

| `lang-elisp` | Emacs Lisp | MIT | [tree-sitter-elisp](https://github.com/Wilfred/tree-sitter-elisp) |

| `lang-elixir` | Elixir | Apache-2.0 | [tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir) |

| `lang-elm` | Elm | MIT | [tree-sitter-elm](https://github.com/elm-tooling/tree-sitter-elm) |

| `lang-erlang` | Erlang | Apache-2.0 | [tree-sitter-erlang](https://github.com/WhatsApp/tree-sitter-erlang) |

| `lang-fish` | Fish | Unlicense | [tree-sitter-fish](https://github.com/ram02z/tree-sitter-fish) |

| `lang-fsharp` | F# | MIT | [tree-sitter-fsharp](https://github.com/ionide/tree-sitter-fsharp) |

| `lang-gitattributes` | .gitattributes | MIT | [tree-sitter-gitattributes](https://github.com/tree-sitter-grammars/tree-sitter-gitattributes) |

| `lang-gleam` | Gleam | Apache-2.0 | [tree-sitter-gleam](https://github.com/gleam-lang/tree-sitter-gleam) |

| `lang-glsl` | GLSL | MIT | [tree-sitter-glsl](https://github.com/tree-sitter-grammars/tree-sitter-glsl) |

| `lang-go` | Go | MIT | [tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go) |

| `lang-graphql` | GraphQL | MIT | [tree-sitter-graphql](https://github.com/bkegley/tree-sitter-graphql) |

| `lang-groovy` | Groovy | MIT | [tree-sitter-groovy](https://github.com/murtaza64/tree-sitter-groovy) |

| `lang-haskell` | Haskell | MIT | [tree-sitter-haskell](https://github.com/tree-sitter-grammars/tree-sitter-haskell) |

| `lang-hcl` | HCL | Apache-2.0 | [tree-sitter-hcl](https://github.com/tree-sitter-grammars/tree-sitter-hcl) |

| `lang-hlsl` | HLSL | MIT | [tree-sitter-hlsl](https://github.com/tree-sitter-grammars/tree-sitter-hlsl) |

| `lang-html` | HTML | MIT | [tree-sitter-html](https://github.com/tree-sitter/tree-sitter-html) |

| `lang-idris` | Idris | MIT | [tree-sitter-idris](https://github.com/kayhide/tree-sitter-idris) |

| `lang-ini` | INI | Apache-2.0 | [tree-sitter-ini](https://github.com/justinmk/tree-sitter-ini) |

| `lang-java` | Java | MIT | [tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java) |

| `lang-javascript` | JavaScript | MIT | [tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript) |

| `lang-jinja2` | Jinja2 | Apache-2.0 | [tree-sitter-jinja2](https://github.com/dbt-labs/tree-sitter-jinja2) |

| `lang-jq` | jq | MIT | [tree-sitter-jq](https://github.com/flurie/tree-sitter-jq) |

| `lang-jsdoc` | JSDoc | MIT | [tree-sitter-jsdoc](https://github.com/tree-sitter/tree-sitter-jsdoc) |

| `lang-json` | JSON | MIT | [tree-sitter-json](https://github.com/tree-sitter/tree-sitter-json) |

| `lang-julia` | Julia | MIT | [tree-sitter-julia](https://github.com/tree-sitter/tree-sitter-julia) |

| `lang-just` | Just | MIT | [tree-sitter-just](https://github.com/casey/tree-sitter-just) |

| `lang-kconfig` | Kconfig | MIT | [tree-sitter-kconfig](https://github.com/tree-sitter-grammars/tree-sitter-kconfig) |

| `lang-kdl` | KDL | MIT | [tree-sitter-kdl](https://github.com/tree-sitter-grammars/tree-sitter-kdl) |

| `lang-kotlin` | Kotlin | MIT | [tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin) |

| `lang-lean` | Lean | MIT | [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean) |

| `lang-lua` | Lua | MIT | [tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua) |

| `lang-make` | Make | MIT | [tree-sitter-make](https://github.com/tree-sitter-grammars/tree-sitter-make) |

| `lang-markdown` | Markdown | MIT | [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown) |

| `lang-matlab` | MATLAB | MIT | [tree-sitter-matlab](https://github.com/acristoffers/tree-sitter-matlab) |

| `lang-meson` | Meson | MIT | [tree-sitter-meson](https://github.com/tree-sitter-grammars/tree-sitter-meson) |

| `lang-ninja` | Ninja | MIT | [tree-sitter-ninja](https://github.com/alemuller/tree-sitter-ninja) |

| `lang-nix` | Nix | MIT | [tree-sitter-nix](https://github.com/nix-community/tree-sitter-nix) |

| `lang-objc` | Objective-C | MIT | [tree-sitter-objc](https://github.com/tree-sitter-grammars/tree-sitter-objc) |

| `lang-ocaml` | OCaml | MIT | [tree-sitter-ocaml](https://github.com/tree-sitter/tree-sitter-ocaml) |

| `lang-odin` | Odin | MIT | [tree-sitter-odin](https://github.com/tree-sitter-grammars/tree-sitter-odin) |

| `lang-perl` | Perl | MIT | [tree-sitter-perl](https://github.com/tree-sitter-perl/tree-sitter-perl) |

| `lang-php` | PHP | MIT | [tree-sitter-php](https://github.com/tree-sitter/tree-sitter-php) |

| `lang-postscript` | PostScript | MIT | [tree-sitter-postscript](https://github.com/smoeding/tree-sitter-postscript) |

| `lang-powershell` | PowerShell | MIT | [tree-sitter-powershell](https://github.com/airbus-cert/tree-sitter-powershell) |

| `lang-prolog` | Prolog | MIT | [tree-sitter-prolog](https://codeberg.org/foxy/tree-sitter-prolog) |

| `lang-proto` | Protocol Buffers | MIT | [tree-sitter-proto](https://github.com/coder3101/tree-sitter-proto) |

| `lang-python` | Python | MIT | [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python) |

| `lang-query` | Tree-sitter Query | Apache-2.0 | [tree-sitter-query](https://github.com/tree-sitter-grammars/tree-sitter-query) |

| `lang-r` | R | MIT | [tree-sitter-r](https://github.com/r-lib/tree-sitter-r) |

| `lang-regex` | Regular Expression | MIT | [tree-sitter-regex](https://github.com/tree-sitter/tree-sitter-regex) |

| `lang-rego` | Rego | MIT | [tree-sitter-rego](https://github.com/FallenAngel97/tree-sitter-rego) |

| `lang-rescript` | ReScript | MIT | [tree-sitter-rescript](https://github.com/rescript-lang/tree-sitter-rescript) |

| `lang-ron` | RON | MIT OR Apache-2.0 | [tree-sitter-ron](https://github.com/tree-sitter-grammars/tree-sitter-ron) |

| `lang-ruby` | Ruby | MIT | [tree-sitter-ruby](https://github.com/tree-sitter/tree-sitter-ruby) |

| `lang-rust` | Rust | MIT | [tree-sitter-rust](https://codeberg.org/grammar-orchard/tree-sitter-rust-orchard) |

| `lang-scala` | Scala | MIT | [tree-sitter-scala](https://github.com/tree-sitter/tree-sitter-scala) |

| `lang-scheme` | Scheme | MIT | [tree-sitter-scheme](https://github.com/6cdh/tree-sitter-scheme) |

| `lang-scss` | SCSS | MIT | [tree-sitter-scss](https://github.com/serenadeai/tree-sitter-scss) |

| `lang-solidity` | Solidity | MIT | [tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity) |

| `lang-sparql` | SPARQL | MIT | [tree-sitter-sparql](https://github.com/GordianDziwis/tree-sitter-sparql) |

| `lang-sql` | SQL | MIT | [tree-sitter-sql](https://github.com/DerekStride/tree-sitter-sql) |

| `lang-ssh-config` | SSH Config | MIT | [tree-sitter-ssh-config](https://github.com/ObserverOfTime/tree-sitter-ssh-config) |

| `lang-starlark` | Starlark | MIT | [tree-sitter-starlark](https://github.com/tree-sitter-grammars/tree-sitter-starlark) |

| `lang-styx` | Styx | MIT OR Apache-2.0 | [tree-sitter-styx](https://github.com/bearcove/styx) |

| `lang-svelte` | Svelte | MIT | [tree-sitter-svelte](https://github.com/tree-sitter-grammars/tree-sitter-svelte) |

| `lang-swift` | Swift | MIT | [tree-sitter-swift](https://github.com/alex-pinkus/tree-sitter-swift) |

| `lang-textproto` | Text Proto | ISC | [tree-sitter-textproto](https://github.com/PorterAtGoogle/tree-sitter-textproto) |

| `lang-thrift` | Thrift | MIT | [tree-sitter-thrift](https://github.com/tree-sitter-grammars/tree-sitter-thrift) |

| `lang-tlaplus` | TLA+ | MIT | [tree-sitter-tlaplus](https://github.com/tlaplus-community/tree-sitter-tlaplus) |

| `lang-toml` | TOML | MIT | [tree-sitter-toml](https://github.com/tree-sitter-grammars/tree-sitter-toml) |

| `lang-tsx` | TSX | MIT | [tree-sitter-tsx](https://github.com/tree-sitter/tree-sitter-typescript) |

| `lang-typescript` | TypeScript | MIT | [tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript) |

| `lang-typst` | Typst | MIT | [tree-sitter-typst](https://github.com/uben0/tree-sitter-typst) |

| `lang-uiua` | Uiua | MPL-2.0 | [tree-sitter-uiua](https://github.com/shnarazk/tree-sitter-uiua) |

| `lang-vb` | Visual Basic | MIT | [tree-sitter-vb](https://github.com/CodeAnt-AI/tree-sitter-vb-dotnet) |

| `lang-verilog` | Verilog | MIT | [tree-sitter-verilog](https://github.com/tree-sitter/tree-sitter-verilog) |

| `lang-vhdl` | VHDL | MIT | [tree-sitter-vhdl](https://github.com/alemuller/tree-sitter-vhdl) |

| `lang-vim` | Vimscript | MIT | [tree-sitter-vim](https://github.com/tree-sitter-grammars/tree-sitter-vim) |

| `lang-vue` | Vue | MIT | [tree-sitter-vue](https://github.com/tree-sitter-grammars/tree-sitter-vue) |

| `lang-wit` | WIT | Apache-2.0 WITH LLVM-exception | [tree-sitter-wit](https://github.com/bytecodealliance/tree-sitter-wit) |

| `lang-x86asm` | x86 Assembly | MIT | local |

| `lang-xml` | XML | MIT | [tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml) |

| `lang-yaml` | YAML | MIT | [tree-sitter-yaml](https://github.com/tree-sitter-grammars/tree-sitter-yaml) |

| `lang-yuri` | Yuri | Apache-2.0 | [tree-sitter-yuri](https://git.gay/yuri/tree-sitter-yuri) |

| `lang-zig` | Zig | MIT | [tree-sitter-zig](https://github.com/tree-sitter-grammars/tree-sitter-zig) |

| `lang-zsh` | Zsh | MIT | [tree-sitter-zsh](https://github.com/georgeharker/tree-sitter-zsh) |


### GPL-Licensed Grammars (1)

These grammars are **not included by default** due to their copyleft license.
Enabling them may have implications for your project's licensing.

| Feature | Language | License | Source |
|---------|----------|---------|--------|

| `lang-nginx` | nginx | GPL-3.0 | [tree-sitter-nginx](https://gitlab.com/joncoole/tree-sitter-nginx) |


## HTML Tag Reference

Arborium renders syntax highlighting using custom HTML elements. When highlighting code, it wraps spans of text with tags like `<a-k>`, `<a-f>`, etc. These tags are styled by the theme CSS you choose.

### Tag Mappings

Each tag corresponds to a semantic code element. Here's the complete reference:

| Tag | Element Type | Description |
|-----|--------------|-------------|
| `<a-k>` | **Keyword** | Language keywords (if, else, while, class, fn, etc.) |
| `<a-f>` | **Function** | Function names and method calls |
| `<a-s>` | **String** | String literals and character literals |
| `<a-c>` | **Comment** | Comments (line and block) |
| `<a-t>` | **Type** | Type names and type annotations |
| `<a-v>` | **Variable** | Variable names and identifiers |
| `<a-co>` | **Constant** | Constants and boolean literals |
| `<a-n>` | **Number** | Numeric literals (integers and floats) |
| `<a-o>` | **Operator** | Operators (+, -, *, /, &&, etc.) |
| `<a-p>` | **Punctuation** | Delimiters and punctuation (parentheses, brackets, commas) |
| `<a-pr>` | **Property** | Object properties and struct fields |
| `<a-at>` | **Attribute** | Attributes and annotations (@, #[derive], etc.) |
| `<a-tg>` | **Tag** | HTML/XML tags |
| `<a-m>` | **Macro** | Macro names and invocations |
| `<a-l>` | **Label** | Labels and goto targets |
| `<a-ns>` | **Namespace** | Namespaces and modules |
| `<a-cr>` | **Constructor** | Constructor functions and type constructors |

### Markup Tags (Markdown, etc.)

| Tag | Element Type | Description |
|-----|--------------|-------------|
| `<a-tt>` | **Title** | Headings and titles |
| `<a-st>` | **Strong** | Bold text |
| `<a-em>` | **Emphasis** | Italic text |
| `<a-tu>` | **Link** | URLs and hyperlinks |
| `<a-tl>` | **Literal** | Code blocks and inline code |
| `<a-tx>` | **Strikethrough** | Strikethrough text |

### Diff Tags

| Tag | Element Type | Description |
|-----|--------------|-------------|
| `<a-da>` | **Diff Add** | Added lines in diffs |
| `<a-dd>` | **Diff Delete** | Deleted lines in diffs |

### Special Tags

| Tag | Element Type | Description |
|-----|--------------|-------------|
| `<a-eb>` | **Embedded** | Embedded language content |
| `<a-er>` | **Error** | Syntax errors |

### How It Works

Arborium uses tree-sitter grammars to parse code and identify semantic elements. Multiple capture names from tree-sitter queries (like `@keyword.function`, `@keyword.import`, `@conditional`) all map to the same theme slot. For example:

- `@keyword`, `@keyword.function`, `@include`, `@conditional` → all become `<a-k>` (keyword)
- `@function`, `@function.builtin`, `@method` → all become `<a-f>` (function)
- `@comment`, `@comment.documentation` → all become `<a-c>` (comment)

Adjacent spans with the same tag are automatically merged into a single element for efficiency.

### Styling Example

To create a custom theme, target these elements in your CSS:

```css
/* Keywords in blue */
a-k { color: #569cd6; }

/* Functions in yellow */
a-f { color: #dcdcaa; }

/* Strings in green */
a-s { color: #ce9178; }

/* Comments in gray */
a-c { color: #6a9955; font-style: italic; }

/* Types in cyan */
a-t { color: #4ec9b0; }
```

See the [included themes](https://github.com/bearcove/arborium/tree/main/packages/arborium/src/themes) for more examples.

## Sponsors

Thanks to all individual sponsors:

<p>
<a href="https://github.com/sponsors/fasterthanlime">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="./static/sponsors-v3/github-dark.svg">
<img src="./static/sponsors-v3/github-light.svg" height="40" alt="GitHub Sponsors">
</picture>
</a>
<a href="https://patreon.com/fasterthanlime">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="./static/sponsors-v3/patreon-dark.svg">
<img src="./static/sponsors-v3/patreon-light.svg" height="40" alt="Patreon">
</picture>
</a>
</p>

...along with corporate sponsors:

<p>
<a href="https://zed.dev">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="./static/sponsors-v3/zed-dark.svg">
<img src="./static/sponsors-v3/zed-light.svg" height="40" alt="Zed">
</picture>
</a>
<a href="https://depot.dev?utm_source=arborium">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="./static/sponsors-v3/depot-dark.svg">
<img src="./static/sponsors-v3/depot-light.svg" height="40" alt="Depot">
</picture>
</a>
</p>

## License

This project is dual-licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE).

The bundled grammar sources retain their original licenses - see [LICENSES.md](LICENSES.md) for details.

## WASM Support

Arborium supports building for `wasm32-unknown-unknown`. This requires compiling C code (tree-sitter core and grammar parsers) to WebAssembly.

### macOS

On macOS, the built-in Apple clang does **not** support the `wasm32-unknown-unknown` target. You need to install LLVM via Homebrew:

```bash
brew install llvm
```

Then ensure the Homebrew LLVM is in your PATH when building:

```bash
export PATH="$(brew --prefix llvm)/bin:$PATH"
cargo build --target wasm32-unknown-unknown
```

## Development

This project uses `cargo xtask` for most development and release tasks.

For detailed architecture, workflows, publishing order, and layout, see `DEVELOP.md`.

For a quick overview of available commands, run:

```bash
cargo xtask help
```