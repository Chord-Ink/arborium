//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_koto() -> *const ();
}

/// Returns the koto tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_koto) }
}



/// The highlights query for koto.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");




/// The injections query for koto.
pub const INJECTIONS_QUERY: &str = include_str!("../queries/injections.scm");



/// The locals query for koto.
pub const LOCALS_QUERY: &str = include_str!("../queries/locals.scm");



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "koto",

            HIGHLIGHTS_QUERY,

            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }

    #[test]
    fn test_corpus() {
        arborium_test_harness::test_corpus(language(), "koto", env!("CARGO_MANIFEST_DIR"));
    }
}
