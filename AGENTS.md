# AGENTS Notes

## tree-sitter fork workflow (`crates/arborium-tree-sitter`)

- Maintain `crates/arborium-tree-sitter` directly; its checked-in source is authoritative.
- Apply fixes and optimizations in the vendored source. Do not maintain standalone patch files or a reset/replay sync workflow.
- Upstream Tree-sitter upgrades are separate, explicitly requested work. See `crates/arborium-tree-sitter/UPSTREAM.md` for the current base revision.
- Cargo manifests still follow the template workflow below.

## Generated Cargo manifests

- Most `Cargo.toml` files in this repo are generated from `Cargo.stpl.toml` templates.
- Prefer editing the corresponding `Cargo.stpl.toml` source template, then regenerating, rather than hand-editing generated `Cargo.toml` files.
- The same principle applies to generated plugin manifests (for example under language `npm/` directories): update templates/generation logic, then regenerate.

## C compiler requirement

- **A C compiler with WASM support is REQUIRED** for building plugins.
- Tree-sitter is inherently a C codebase - the parsers are C code that must be compiled to WASM.
- The `.envrc` file contains the magic to find the right clang with WASM support.
- DO NOT try to disable C compilation in the build process - it's fundamental to how tree-sitter works.
- On macOS, you need LLVM from Homebrew (not Apple's clang) for `wasm32-unknown-unknown` target support.
