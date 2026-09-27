//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_hlsl() -> *const ();
}

/// Returns the hlsl tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_hlsl) }
}



/// The highlights query for hlsl (base query only).
/// Use [`HIGHLIGHTS_QUERY`] for the full query including inherited queries.
const HIGHLIGHTS_QUERY_BASE: &str = include_str!("../queries/highlights.scm");

/// The highlights query for hlsl.
/// Includes inherited queries from: arborium_cpp.
pub static HIGHLIGHTS_QUERY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    let mut query = String::new();

    query.push_str(&arborium_cpp::HIGHLIGHTS_QUERY);
    query.push('\n');

    query.push_str(HIGHLIGHTS_QUERY_BASE);
    query
});




/// The injections query for hlsl (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";



/// The locals query for hlsl (empty - no locals available).
pub const LOCALS_QUERY: &str = "";



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "hlsl",

            &HIGHLIGHTS_QUERY,

            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }

    #[test]
    fn test_corpus() {
        arborium_test_harness::test_corpus(language(), "hlsl", env!("CARGO_MANIFEST_DIR"));
    }
}
