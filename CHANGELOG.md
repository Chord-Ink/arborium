## 2.21.0 (2026-09-27)

- Optimize Tree-sitter query metadata lookup, chunked text predicates, subtree edit allocations, lexer cancellation, capture scratch retention, and reference-count ordering.
- Restore optional native Wasmtime grammar hosting and add runtime benchmarks and regression coverage.
- Maintain the vendored Tree-sitter source directly; remove the standalone patch and sync workflow while preserving all runtime fixes.
- Keep the parser ABI and all 119 generated grammars unchanged. Regeneration is not required for this release.

## 2.20.0 (2026-09-27)

- Upgrade the vendored tree-sitter runtime to upstream commit `dcdc8cc55e5dfedfc858080835f153999a29ec40`, including the UTF-16 surrogate-pair endianness fix.
- Fix error recovery, anchored query repetitions, descendant lookup, deep-tree traversal, repeated fields, visible aliases, disabled wildcard patterns, reverse navigation, and error-leaf status.
- Make capture-list reuse constant time and preserve the runtime fixes as replayable patches.
- Support exact commit pins and stage tree-sitter syncs before replacing the working fork. Existing grammar parsers remain compatible and unchanged.

## 0.2.2 (2025-12-04)



## 0.2.1 (2025-12-04)



## 0.2.0 (2025-12-04)
