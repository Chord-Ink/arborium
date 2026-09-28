# Arborium Tree-sitter source

This directory is a directly maintained fork of the Tree-sitter runtime. Its
checked-in source includes Arborium's build integration, runtime fixes, and
performance optimizations. Apply changes here; there is no standalone patch
series or automatic upstream reset/replay workflow. Future upstream upgrades
are separate, explicitly requested work.

The upstream base is
[`dcdc8cc55e5dfedfc858080835f153999a29ec40`](https://github.com/tree-sitter/tree-sitter/commit/dcdc8cc55e5dfedfc858080835f153999a29ec40)
(2026-09-24), following v0.27.0. Upstream identifies this revision as 0.28.0
development; Arborium packages it under its own version.

- [Runtime fixes and regression evidence](../../docs/tree-sitter-issue-review.md)
- [Performance changes, benchmarks, and verification](../../docs/tree-sitter-performance.md)

The capture-list free-stack implementation adapts
[Eric Meadows-Jonsson's proposed fix](https://github.com/ericmj/tree-sitter/tree/capture-list-pool-free-stack).
Michael Sloan (mgsloan) reported the fixes for disabled wildcard patterns
([#5932](https://github.com/tree-sitter/tree-sitter/issues/5932)) and the
reverse-cursor index sentinel
([#5950](https://github.com/tree-sitter/tree-sitter/issues/5950)). Other local
runtime fixes are based on the reproductions documented in the issue review.

`Cargo.toml` remains generated from `Cargo.stpl.toml` through Arborium's normal
manifest generation workflow. Edit the template when changing dependencies or
features.
