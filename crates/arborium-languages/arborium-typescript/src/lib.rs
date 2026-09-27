//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_typescript() -> *const ();
}

/// Returns the typescript tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_typescript) }
}



/// The highlights query for typescript (base query only).
/// Use [`HIGHLIGHTS_QUERY`] for the full query including inherited queries.
const HIGHLIGHTS_QUERY_BASE: &str = include_str!("../queries/highlights.scm");

/// The highlights query for typescript.
/// Includes inherited queries from: arborium_javascript.
pub static HIGHLIGHTS_QUERY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    let mut query = String::new();

    query.push_str(&arborium_javascript::HIGHLIGHTS_QUERY);
    query.push('\n');

    query.push_str(HIGHLIGHTS_QUERY_BASE);
    query
});




/// The injections query for typescript (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";



/// The locals query for typescript.
pub const LOCALS_QUERY: &str = include_str!("../queries/locals.scm");



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "typescript",

            &HIGHLIGHTS_QUERY,

            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }

    #[test]
    fn test_corpus() {
        arborium_test_harness::test_corpus(language(), "typescript", env!("CARGO_MANIFEST_DIR"));
    }
}
