# arborium-test-harness

Test harness for arborium grammar crates.

## Purpose

Provides utilities for testing tree-sitter grammars and their highlight/injection queries.

## Usage

In your grammar crate's tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "rust",
            HIGHLIGHTS_QUERY,
            INJECTIONS_QUERY,
            LOCALS_QUERY,
        );
    }
}
```

The harness validates that:
- The grammar compiles correctly
- Highlight queries are syntactically valid
- Injection queries parse without errors
---

Part of the [arborium](https://github.com/bearcove/arborium) project. See [arborium.bearcove.eu](https://arborium.bearcove.eu) for more information.
