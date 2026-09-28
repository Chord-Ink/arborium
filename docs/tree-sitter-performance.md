# Arborium Tree-sitter performance

The source of truth is `crates/arborium-tree-sitter`. These optimizations are
integrated into the vendored C runtime and Rust bindings; maintain them directly
in those files. Upstream upgrades will be handled separately. The current base
and maintenance policy are recorded in
[`UPSTREAM.md`](../crates/arborium-tree-sitter/UPSTREAM.md).
Cargo manifests remain generated from their source templates.

## Editor performance changes

These changes target the operations performed by Zed: repeated ranged queries,
text predicates over rope chunks, shared syntax snapshots, incremental edits,
pooled cursors, and parsers with a progress callback. They do not change Zed or
implement large-file buffering, paging, scheduling, or 64-bit tree offsets.

| Area | Change | Important constraint |
| --- | --- | --- |
| Query execution | Compute query node field, sibling, and supertype status only when a candidate step needs it. | Preserve the general query matcher, ordering, ranges, and match limits. |
| Rust predicates | Compare equality predicates directly across text chunks; sort and deduplicate literal membership sets, using binary search above eight entries. | Providers may return empty chunks and encodings different from node byte ranges. Regex predicates still materialize text. No match-result cache is introduced. |
| Subtree edits | Keep the first 16 subtree-edit work entries on the stack. | Spill to the heap for wider traversals; no new depth or width limit. |
| Cancellation | Check parser progress every 4096 lexer advances when a callback is installed. | Unwind scanners through EOF and discard cancelled tokens before caching. Scanners must cooperate by advancing/checking EOF. This is an operation bound, not a wall-clock deadline. |
| Scratch retention | Release exceptional capture-list capacity after eight consecutive low-use cursor resets. | Keep ordinary viewport working sets; use a 64-list floor when shrinking collections and tolerate capture capacities up to four times the working set, with a 256-capture floor. Memory is reclaimed on cursor reuse, not while a cursor is idle. |
| Reference counts | Use relaxed owning retains, release decrements, and acquire before destruction or unique mutation. | Keep unrelated atomic counters unchanged; never copy a concurrently changing reference count as plain bytes. |

### Reference ownership

Retaining requires an existing owning reference, so it does not publish new data.
Release decrements publish the departing owner's accesses. The final owner uses
an acquire load before destroying the subtree, and copy-on-write uniqueness checks
use acquire loads before permitting mutation. Cloning copies child pointers and
immutable metadata, then initializes its own reference count. An overflow guard
prevents wrapped counts. Windows keeps Interlocked operations; older compilers
keep the existing atomic fallback. TinyC retains upstream's single-threaded
fallback. Concurrent access still requires the public API's ownership rules.

### Native WASM hosting

`--features wasm` enables upstream's Wasmtime host implementation. This is
separate from building the runtime itself for `wasm32-unknown-unknown`.
The optional Wasmtime dependency follows the pinned upstream runtime; its compiler
requirements can exceed the base crate's `rust-version` (Wasmtime 48 requires Rust
1.95). Zed still needs to adopt a compatible Arborium runtime dependency to use
these changes; this work does not alter Zed's dependency graph.

## Verification

From the repository root:

```sh
source .envrc
python3 scripts/check_tree_sitter_runtime.py --sanitize --wasm
python3 scripts/check_wasm.py
cargo check --manifest-path crates/arborium-tree-sitter/Cargo.toml --no-default-features
cargo test --manifest-path crates/arborium-highlight/Cargo.toml --features tree-sitter
```

`check_tree_sitter_runtime.py` checks pool invariants, deep traversal, UTF-16,
recovery costs, scratch retention, and Rust predicates with empty/split chunks.
Its optional Wasmtime test compiles a JSON grammar module, loads it into a native
parser, and verifies cancellation, resumption, and querying.

Prepare a baseline runtime from the parent revision for comparisons:

```sh
mkdir -p .cache/tree-sitter-baseline
git archive 9e5a86c3abb9d458ad653083840f81895b2c3f12 crates/arborium-tree-sitter |
  tar -x --strip-components=2 -C .cache/tree-sitter-baseline
python3 scripts/check_tree_sitter_queries.py \
  --baseline .cache/tree-sitter-baseline --zed ../zed --sanitize
python3 scripts/bench_tree_sitter.py \
  --baseline .cache/tree-sitter-baseline --zed ../zed --runs 7
python3 scripts/bench_tree_sitter.py \
  --baseline crates/arborium-tree-sitter --zed ../zed --runs 1 --thread-sanitize
```

The thread-sanitized command deliberately uses the candidate on both sides:
it checks concurrent copy/edit/drop behavior, not timing or the baseline's races.
Address/undefined sanitizer comparisons use `--sanitize` instead.

Query comparisons use Zed's complete JSON, JavaScript, and Rust query files,
both capture and match iteration, ranged queries, and truncated source prefixes.
JavaScript follows Zed's TSX grammar mapping. Rust uses Zed's pinned grammar from
the Cargo cache (or `--rust-grammar`), since Arborium's Rust Orchard grammar differs.
The C harness compares structural captures; Rust text predicates are tested
separately. Cancellation fixtures cover long strings, whitespace, malformed
tokens, external scanners, UTF-8/UTF-16, resumption, and parser reset.

The benchmark alternates baseline/candidate order and reports medians. Its JSON
workload uses 4000 objects, 1024-byte input chunks, 100 viewport queries with a
match limit of 64, and 100 edits with a retained snapshot. It also checks a wide
deletion, shared-tree concurrency, allocation balance, and cancellation inside
a 16 MiB string. The allocation-tracking wrapper adds overhead to both runtimes.
These are runtime workload measurements, not end-to-end Zed latency or process
RSS measurements. Sanitizer timings are not performance results.

## Recorded results

Seven alternating runs on an Apple M3 Pro (arm64), Apple Clang 21, `-O3`,
against Arborium `9e5a86c3abb9d458ad653083840f81895b2c3f12`, using Zed
`0d78d9611cc48f7a0d498a00d8075d7774e2fc99` queries:

| Metric | Baseline median | Optimized median |
| --- | ---: | ---: |
| 100 viewport queries | 28.084 ms | 23.126 ms |
| Initial parse | 25.064 ms | 25.598 ms |
| Parse with a continuing progress callback | 25.074 ms | 25.712 ms |
| 100 edits and incremental reparses | 217.161 ms | 217.449 ms |
| Allocations during those edits | 404,726 | 404,626 |
| 100,000 snapshot copies and drops | 3.499 ms | 3.503 ms |
| Return from a cancelled 16 MiB string parse | 77.461 ms | 0.023 ms |

The baseline completed the long-token parse without invoking the cancellation
callback; the optimized runtime invoked it once and returned no tree. All 143,195
viewport captures matched, and tracked live allocations returned to zero.
Query time fell about 17.7%. This run did not show a parse or snapshot throughput
improvement: parsing was roughly 2–2.5% slower, and edit/copy times were similar.
Benchmark variance and application-level integration still need consideration.

The scratch regression retained 737,280 bytes after an exceptional query and
1,280 bytes after sustained low use. This is capture-pool capacity, not process
RSS. Normal benchmark peak allocation was effectively unchanged (40 bytes higher
in the optimized runtime). The Rust equality test rejected a 1 MiB capture after
reading its first 1024-byte chunk, without materializing its remaining text.

Validation passed: ASan/UBSan, ThreadSanitizer shared-tree copy/edit/drop checks,
29 Zed/supplemental query files across three grammars, three Rust/native-Wasm
regressions, 27 highlighting tests, `no_std` compilation, and the
existing WASM checks for all 119 bundled grammars. Native checks ran on macOS
arm64; this is not a cross-platform performance claim.
