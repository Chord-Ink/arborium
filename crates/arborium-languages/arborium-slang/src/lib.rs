//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_slang() -> *const ();
}

/// Returns the slang tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_slang) }
}



/// The highlights query for slang.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");




/// The injections query for slang (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";



/// The locals query for slang (empty - no locals available).
pub const LOCALS_QUERY: &str = "";



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "slang",

            HIGHLIGHTS_QUERY,

            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }

    #[test]
    fn test_corpus() {
        arborium_test_harness::test_corpus(language(), "slang", env!("CARGO_MANIFEST_DIR"));
    }
}
