# Error-recovery fixtures

Copied from `test/fixtures/error_corpus` in tree-sitter v0.27.0 (6070dbfe),
under tree-sitter's MIT license. The JavaScript corpus includes the regression
for hidden error-node cost accounting from upstream commit 869638f6.

These fixtures run through Arborium's bundled grammars and patched C runtime,
not the separately installed tree-sitter CLI. Input and expected trees are
retained from upstream; field labels and formatting are ignored by the runner.

The Python expectations omit `expression_statement` wrappers, which Arborium's bundled Python grammar makes invisible. Inputs and recovery boundaries are unchanged.
