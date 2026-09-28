# Tree-sitter issue review — 2026-09-27

Screened the **105 open issues** returned by the
[tree-sitter issue tracker](https://github.com/tree-sitter/tree-sitter/issues)
on this date, then examined relevant runtime reports, reproductions, and
discussion. This is separate from the review of Arborium's own upstream issues.
The review started from **v0.27.0, commit 6070dbfe**. The fork is now pinned to
upstream master commit
[`dcdc8cc55e5dfedfc858080835f153999a29ec40`](https://github.com/tree-sitter/tree-sitter/commit/dcdc8cc55e5dfedfc858080835f153999a29ec40)
(2026-09-24), the newest commit listed in the
[upstream history](https://github.com/tree-sitter/tree-sitter/commits) when upgraded.
The upstream workspace identifies itself as 0.28.0 development; Arborium ships
this runtime in version **2.20.0**.

Arborium ships C parsers and Rust query bindings, compiles highlight/injection
queries once, and reuses parsers and query cursors. It has its own highlighting
engine and wasm-bindgen plugin/session layer. Correct captures, predictable
query costs, malformed-input handling, and native/WASM consistency matter most.
The upstream CLI, generator, web-tree-sitter JS bindings, and Wasmtime loader
are not the runtime that Arborium ships.

## Fixed in this fork

| Issue | Why it is worth carrying | Fix and evidence |
| --- | --- | --- |
| [#5910: malformed nested JSON destroys the root](https://github.com/tree-sitter/tree-sitter/issues/5910) | Recovery quality controls highlighting of partially written code. | Charge skipped lexical errors consistently with other visible tokens. Preserve upstream `869638f6` hidden-error accounting. Tests cover localized errors, exact string boundaries, following pairs, incremental repair, 30 upstream recovery fixtures across five languages, and cost-neutral hidden grouping. |
| [#5416: quantified nodes with trailing anchors](https://github.com/tree-sitter/tree-sitter/issues/5416) | Anchored queries must retain every repeated capture and reject incomplete matches. | Apply the trailing constraint to repetition exits and zero-occurrence skips. Test `?`, `*`, `+`, groups, alternatives, nested child patterns, later named siblings, and both match and capture iteration. |
| [#5949: `child_with_descendant` accepts non-descendants](https://github.com/tree-sitter/tree-sitter/issues/5949) | Correctness of the exposed tree API, including empty and equal-range nodes. | Reject self and foreign trees; check identity for equal/empty ranges. Keep the ordinary nonempty containment shortcut. Compare with cursor ancestry, and stress deep empty branches and shared tree copies. |
| [#5807: recursive node operations overflow](https://github.com/tree-sitter/tree-sitter/issues/5807) | Deep input must not exhaust a native or WASM call stack. | Replace recursion in field lookup, ancestry lookup, trailing-empty-descendant search, and DOT output with iterative traversal. Branches use heap-backed frames; ordinary tail descents avoid allocations. All four paths pass 20,000-level stress tests on a 256 KiB native stack under ASan/UBSan. |
| [#5951: capture-list pool scans](https://github.com/tree-sitter/tree-sitter/issues/5951) | Shared hot path for highlight/injection queries; complex and custom queries can retain many partial matches. | Maintain a stack of free capture-list IDs. Acquire/release are amortized O(1); reset remains O(pool size). Pool invariants, ordered captures, partial iteration, cursor reuse, and match-limit exhaustion are tested. |
| [#2984: repeated fields only match once](https://github.com/tree-sitter/tree-sitter/issues/2984) | Drops valid captures, including Elm function parameters. | Account for later visible children when a field comes from a hidden wrapper. Reproduced with the bundled Elm grammar; one, two, and eight parameters now all match. |
| [#5948: fields cross visible aliases](https://github.com/tree-sitter/tree-sitter/issues/5948) | Field lookup also controls negated-field queries, so the bug can suppress highlight patterns. | Stop inherited field lookup at a visible alias. Test TypeScript `typeof a.b`, lookup by name/ID, and a negated-field query. The member expression's own field still resolves. |
| [#5932: disabled wildcard patterns still execute](https://github.com/tree-sitter/tree-sitter/issues/5932) | Public query API correctness, including custom consumers. In Arborium's debug build the reproducer aborts on an out-of-bounds pattern-map access. | Decrement the wildcard prefix count as entries are removed. Test wildcard-only, mixed, and alternating patterns, including repeated disabling. |
| [#5950: reverse cursor stops every 256 children](https://github.com/tree-sitter/tree-sitter/issues/5950) | Inexpensive correctness fix for the exposed tree API; large comment lists reproduce it without a synthetic grammar. | Compare the full-width unsigned sentinel. Walk backward through 800 Rust comments, crossing three boundaries. |
| [#3623: error leaves report no error](https://github.com/tree-sitter/tree-sitter/issues/3623) | Consumers inspecting malformed input must be able to trust `has_error()`. | Check the ERROR symbol as well as recovery cost. Reproduced on bundled R with `1 + }`; the unexpected terminal had zero cost. |

These are local fixes; upstream issues have not been closed or commented on.
The capture-list free stack adapts
[Eric Meadows-Jonsson's proposed fix](https://github.com/ericmj/tree-sitter/tree/capture-list-pool-free-stack).
The wildcard and reverse-cursor fixes were reported by Michael Sloan (mgsloan).
The other fixes are based on local reproductions.

## Relevant but unresolved

The remaining reports below need separate reproductions or broader changes.
The four previously deferred issues requested for follow-up are now fixed above;
none of their regression tests remain ignored.

| Issue | Assessment |
| --- | --- |
| [#5140: excessive Clojure captures](https://github.com/tree-sitter/tree-sitter/issues/5140), [#2822](https://github.com/tree-sitter/tree-sitter/issues/2822), [#1591](https://github.com/tree-sitter/tree-sitter/issues/1591) | Query-combination/quantifier behavior needs its own corpus and performance investigation. The free-list fix addresses allocation overhead, not combinatorial state growth or duplicate-match semantics. |
| [#5925: keyword extraction changes tokenization](https://github.com/tree-sitter/tree-sitter/issues/5925) | Potentially relevant to bundled grammars, but the proposed change spans generation and runtime behavior and changes generated parsers. Do not treat it as a drop-in C-runtime patch; regenerate and test affected grammars together. |
| [#4534](https://github.com/tree-sitter/tree-sitter/issues/4534), [#637](https://github.com/tree-sitter/tree-sitter/issues/637) | Incremental parsing reports matter to browser sessions, but require their custom grammar/edit reproductions and a broader parser-reuse investigation. No claim that these are fixed. |

The native Java `QueryCursor::matches` reproduction of
[#5079](https://github.com/tree-sitter/tree-sitter/issues/5079) passes: neither
`(MISSING)` nor `(ERROR)` matches a clean superclass. A broader follow-up check
found that streaming `QueryCursor::captures` still returns a superclass capture
for `(superclass (MISSING)) @a`. This separate streaming issue remains unresolved;
the passing match-iterator test does not establish that #5079 is fixed.

## Lower priority or a different component

- Generator-only bugs and requests (for example
  [#5953](https://github.com/tree-sitter/tree-sitter/issues/5953),
  [#5021](https://github.com/tree-sitter/tree-sitter/issues/5021), and
  [#4999](https://github.com/tree-sitter/tree-sitter/issues/4999)) belong in the
  separately installed generator. They do not justify modifying the vendored
  `lib/` runtime. They can matter during grammar maintenance and should be
  revisited when a bundled grammar reproduces them.
- Upstream npm/CLI installers, Bun binding templates, Swift packaging, Windows
  exports, documentation, playground limits, and web-tree-sitter scratch-cursor
  ownership are separate deliverables. In particular,
  [#5547](https://github.com/tree-sitter/tree-sitter/issues/5547) does not describe
  Arborium's per-context Rust cursors, and
  [#5954](https://github.com/tree-sitter/tree-sitter/issues/5954) concerns an
  Emscripten build rather than Arborium's `wasm32-unknown-unknown` sysroot.
- Upstream tree-sitter-highlight feature requests and injection API proposals
  are not fixes for Arborium's own rendering/injection implementation.
- Grammar-specific token boundaries, alias validation, and query-semantics
  reports such as [#4558](https://github.com/tree-sitter/tree-sitter/issues/4558),
  [#3966](https://github.com/tree-sitter/tree-sitter/issues/3966),
  [#2641](https://github.com/tree-sitter/tree-sitter/issues/2641), and
  [#1453](https://github.com/tree-sitter/tree-sitter/issues/1453) need their own
  current-grammar reproductions. Do not infer they are fixed by nearby patches.
- Proposed changes to how extras, predicates, changed ranges, or very large
  files work are broader contracts or application policies. Changing them
  globally would risk existing queries and integrations.

## Source maintenance

All ten runtime fixes are integrated directly into `crates/arborium-tree-sitter`,
along with the six [editor performance optimizations](tree-sitter-performance.md).
The checked-in source is authoritative. Edit it directly; standalone patches
and the reset/replay sync script have been removed. Future upstream upgrades
will be handled as separate, explicitly requested work. The current base and
attributions are recorded in
[`UPSTREAM.md`](../crates/arborium-tree-sitter/UPSTREAM.md).

None of the ten fixes was removed as an upstream duplicate during the latest
upgrade. The runtime delta from v0.27.0 includes upstream #5912's UTF-16
surrogate-pair endianness fix; the vendored web-binding dependency and Nix
metadata updates are also included.

Regenerate Cargo manifests from their templates after changing package metadata
(`cargo xtask gen json` is sufficient for the shared manifests). For the 2.20.0
version bump, `cargo xtask gen --version 2.20.0` updated all package metadata.
It reused all 119 cached parsers; none needed regeneration. The parser ABI and
generated parser interface are unchanged, and all 361 committed grammar source
files retained identical hashes.

## Validation

- `cargo test --manifest-path crates/arborium/Cargo.toml --all-features`:
  **197 passed**, including all 119 grammar-query checks, 16 runtime regression
  tests, and five language corpus tests covering 30 upstream recovery fixtures.
  Eight existing documentation examples remain ignored.
- `source .envrc && python3 scripts/check_wasm.py`: direct runtime and all
  **119 grammars** link and execute in Node with **zero host imports**. The added
  WASM assertions exercise wildcard disabling, reverse navigation, alias fields,
  error leaves, repeated-field captures, cursor reuse, localized JSON recovery,
  anchored quantifiers, descendant identity, and 12,000-level tree navigation.
- The internal capture-pool and tree-traversal tests pass with AddressSanitizer
  and UndefinedBehaviorSanitizer. The latter also verifies hidden error grouping
  preserves recovery costs for ordinary and lexical-error tokens, and checks
  UTF-16 surrogate pairs in both byte orders. The old decoder failed the direct
  big-endian reproduction; the upgraded decoder passes. This is focused
  runtime coverage, not a sanitizer run of every grammar.
- DOT output matches upstream on three representative JavaScript trees after
  normalizing pointer IDs.
- In the isolated `-O2` pool benchmark, 20,000 release/acquire pairs with 32,768
  allocated lists took **192.049 ms before / 0.012 ms after** on this machine.
  Checksums match. This demonstrates removal of pool-size-dependent scanning;
  it is **not an end-to-end highlighting speedup claim**. The extra free-ID
  storage costs approximately four bytes per allocated capture list, plus
  array capacity overhead.
- Python syntax, Rust formatting, and `git diff --check` pass.

Run the pool test/benchmark locally (keep assertions enabled):

```sh
cc -std=c11 -D_DEFAULT_SOURCE -D_POSIX_C_SOURCE=200112L -O2 \
  -I crates/arborium-tree-sitter/src -I crates/arborium-tree-sitter/include \
  scripts/tests/capture_list_pool.c -o /tmp/arborium-capture-pool
/tmp/arborium-capture-pool
```

For sanitizers, replace `-O2` with
`-O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer`.

Run the deep traversal and recovery-cost invariants with sanitizers:

```sh
cc -std=c11 -D_DEFAULT_SOURCE -D_POSIX_C_SOURCE=200112L \
  -O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer -pthread \
  -I crates/arborium-tree-sitter/src -I crates/arborium-tree-sitter/include \
  scripts/tests/tree_traversal.c -o /tmp/arborium-tree-traversal
/tmp/arborium-tree-traversal
```
