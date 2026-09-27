# Development Guide

This document covers the architecture and development workflow for arborium.

## Architecture Overview

### Crate Structure

Arborium consists of several types of crates:

**Pre-group crates** (publish first):
- `crates/arborium-tree-sitter` - Patched tree-sitter core
- `crates/arborium-highlight` - Highlighting engine and renderers
- `crates/arborium-sysroot` - WASM sysroot for grammar crates
- `crates/arborium-test-harness` - Test utilities for grammars

**Grammar crates** (in `crates/arborium-languages/arborium-*/`):
- Each grammar is an independent crate (e.g., `arborium-rust`, `arborium-svelte`)
- Grammar inheritance and query composition can introduce dependencies on other grammars
- Source definitions are organized into groups under `langs/`
- Each grammar crate and its corresponding WASM plugin crate (in `npm/`) are independent,
  with their own `target/` directories for maximum build parallelism.

**Post-group crates** (publish last):
- `crates/arborium` - Umbrella crate with feature flags for all grammars

### Language Injections

Languages like HTML, Svelte, and Vue support **language injections** - embedding one
language inside another (e.g., JavaScript in `<script>` tags, CSS in `<style>` tags).

#### How Injections Work

Injection queries (in `def/queries/injections.scm`) reference other languages **by name**:

```scheme
; From svelte's injections.scm
((script_element
  (raw_text) @injection.content)
  (#set! injection.language "javascript"))

((style_element
  (raw_text) @injection.content)
 (#set! injection.language "css"))
```

Queries refer to injected languages by name. Generated language crates also expose
optional injection dependencies, enabled by their default `injections` feature.
The umbrella crate's `lang-*` features activate the injected languages in its registry.

#### Injection Resolution by Platform

**Native Rust crate (`arborium`):**
- All grammars compiled into one binary via feature flags
- `Highlighter` struct has a HashMap of language configs
- Injection callback looks up languages from this HashMap
- Feature dependencies ensure required languages are included:
  ```toml
  lang-svelte = ["dep:arborium-svelte", "lang-javascript", "lang-css", "lang-typescript"]
  ```

**WASM demo:**
- Uses `arborium` compiled to WASM with `all-languages` feature
- Same as native - all grammars in one binary, injections resolved internally

**Individual npm packages (`@arborium/svelte`, etc.):**
- Each is a standalone WASM module with just that grammar
- **Cannot** resolve injections on their own
- Host application must:
  1. Load multiple grammar WASM modules
  2. Parse injection queries
  3. Route injection requests to appropriate grammar modules

### Publishing Order

Publish shared prerequisites first, followed by grammars in dependency order and
then the umbrella and integration crates. The publisher's dependency graph accounts
for grammar inheritance, query composition, and injections:

```bash
# 1. Publish pre-group crates first
cargo xtask publish crates --group pre

# 2. Publish language groups after their dependencies
cargo xtask publish crates --group acorn
cargo xtask publish crates --group birch
# ... etc

# 3. Publish post-group crates last
cargo xtask publish crates --group post

# 4. Publish npm packages (no ordering constraints)
cargo xtask publish npm
```

## Development Workflow

### Adding a New Grammar

1. Create the grammar definition in `langs/group-<name>/<lang>/def/`
2. Run `cargo xtask gen` to generate crate files
3. Build or test the generated grammar crate, e.g.:
   `cargo check --manifest-path crates/arborium-languages/arborium-<lang>/Cargo.toml`
4. Run `cargo xtask build <lang>` to build the WASM plugin
5. Test with `cargo xtask serve`

### Modifying xtask

After modifying xtask code, the next `cargo xtask` invocation will recompile automatically.

Commands show "next steps" hints after completion to guide the workflow.

## Directory Layout

```
arborium/
├── crates/
│   ├── arborium/           # Umbrella crate
│   ├── arborium-sysroot/   # WASM sysroot
│   └── arborium-test-harness/
├── langs/
│   ├── group-acorn/        # Web languages (html, css, js, json, etc.)
│   ├── group-birch/        # Systems languages (c, cpp, rust, go, zig)
│   ├── group-cedar/        # JVM languages (java, scala, kotlin, clojure)
│   ├── group-fern/         # Functional languages (haskell, ocaml, elixir)
│   ├── group-hazel/        # Scripting languages (python, ruby, lua, bash)
│   ├── group-maple/        # Config/data languages (toml, yaml, json, etc.)
│   ├── group-moss/         # Scientific languages (r, julia, matlab)
│   ├── group-pine/         # Misc modern languages (swift, dart, rescript)
│   ├── group-sage/         # Legacy/enterprise (c-sharp, vb, elisp)
│   └── group-willow/       # Markup/templating (markdown, svelte, vue)
├── demo/                   # WASM demo site
└── xtask/                  # Build tooling
```

## Grammar Repository Structure

Language definitions remain in `langs/group-*/<lang>/def/`, including
`arborium.yaml`, grammar JavaScript, scanners, queries, and test samples.
`cargo xtask gen` writes standalone Rust crates into
`crates/arborium-languages/arborium-<lang>/`. Commit those outputs along with
changes to their definitions or generation templates.

The generated Cargo manifests, Rust sources, README files, queries, headers,
and parser sources are checked in, including the shared and umbrella crates.
Parser sources use deterministic `grammar/src/parser.c.gz` files to keep the
repository manageable and avoid GitHub's per-file size limit. Each crate's
`build.rs` decompresses its parser into Cargo's `OUT_DIR` before compiling it.
Consumers need Rust 1.90+ and a C compiler, with no generation tools required.

WASM plugin packages remain under `langs/group-*/<lang>/npm/` and are generated
on demand. Their build still requires a clang with WASM support.

Run `python3 scripts/check_git_dependency.py` to test the current files through
a temporary Git repository and a separate Cargo consumer. This check excludes
ignored files and never runs generation. Add `--wasm` to link the consumer for
`wasm32-unknown-unknown` as well (`source .envrc` first on macOS).

### Key Commands

```bash
# Regenerate all grammar crates (local dev)
cargo xtask gen

# Regenerate with specific version (for releases)
cargo xtask gen --version 1.1.11

# Regenerate specific grammar only
cargo xtask gen rust

# Build and serve WASM demo
cargo xtask serve --dev

# Build WASM plugins
cargo xtask plugins build
```

### Local Development Workflow

```bash
# 1. Edit grammar source files
#    - arborium.yaml (config, license, metadata)
#    - grammar/grammar.js (tree-sitter grammar)
#    - queries/highlights.scm (syntax highlighting)

# 2. Regenerate crate files
cargo xtask gen

# 3. Build and test
cargo build --manifest-path crates/arborium/Cargo.toml --features lang-rust
cargo test --manifest-path crates/arborium-languages/arborium-rust/Cargo.toml
```

### Version Management

**Versions don't matter locally** - path dependencies ignore version numbers.

For releases, CI parses the version from the git tag and runs:
```bash
cargo xtask gen --version $VERSION
```

This updates all `Cargo.toml` files with the correct version before publishing.

See [PUBLISH.md](PUBLISH.md) for full release workflow details.

### arborium.yaml Format

Each language definition has an `arborium.yaml` file as its source of truth:

```yaml
repo: https://github.com/tree-sitter/tree-sitter-rust
commit: abc123...
license: MIT
grammars:
  - id: rust
    name: Rust
    tag: code
    tier: 1
    icon: devicon-plain:rust
    aliases: [rs]
    has_scanner: true
    generate_plugin: true
    samples:
      - path: samples/example.rs
        description: Example code
        license: MIT
```

**Key fields:**
- `license` - SPDX license for the grammar (used in generated Cargo.toml)
- `generate_plugin: true` - Include in WASM plugin builds
- `has_scanner: true` - Grammar has external scanner (scanner.c)
- `tier` - 1-5, groups languages in generated documentation

## Building the CLI from a checkout

The committed language crates and CLI lockfile allow ordinary Cargo builds,
including sandboxed package builds, without running `xtask` or Node.js:

```sh
cargo build --locked --release --manifest-path crates/arborium-cli/Cargo.toml
cargo install --locked --path crates/arborium-cli
```

For Nix packaging, use the repository root as `src`, set
`cargoRoot = "crates/arborium-cli"`, and use
`cargoLock.lockFile = ./crates/arborium-cli/Cargo.lock` with `buildRustPackage`.
The C compiler and vendored parser sources remain required. Parser generation and
network access to grammar repositories are not part of the package build.

## WASM preflight and regressions

On macOS, install Homebrew LLVM (`brew install llvm`) and run `source .envrc`
(or `direnv allow`). Some Apple clang versions lack the WASM backend.
`cargo xtask build` compiles a small probe with the compiler chosen by cc-rs before
starting plugin builds, honoring `CC_wasm32_unknown_unknown` and other cc-rs overrides.

```sh
cargo test --manifest-path crates/arborium/Cargo.toml --all-features
python3 scripts/check_wasm.py
python3 scripts/check_corpus.py
```

The first command compiles every grammar's queries and runs issue reproductions.
The second links and executes both a direct tree-sitter consumer and an all-language
WASM consumer in Node, rejecting unexpected host imports. The third runs the
vendored upstream corpora against committed parsers with the tree-sitter CLI.
Both corpus tests and CI's nextest runs impose timeouts on external scanners.

The standalone Nix expression is `nix/package.nix`:

```nix
pkgs.callPackage ./nix/package.nix {
  languageGrammars = [ "rust" "javascript" ];
}
```

Omit `languageGrammars` for the complete bundle (including the GPL Nginx grammar).
Flake users can run `nix build` or `nix run . -- --help`.
