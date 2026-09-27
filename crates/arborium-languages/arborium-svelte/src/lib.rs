//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_svelte() -> *const ();
}

/// Returns the svelte tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_svelte) }
}



/// The highlights query for svelte (base query only).
/// Use [`HIGHLIGHTS_QUERY`] for the full query including inherited queries.
const HIGHLIGHTS_QUERY_BASE: &str = include_str!("../queries/highlights.scm");

/// The highlights query for svelte.
/// Includes inherited queries from: arborium_html.
pub static HIGHLIGHTS_QUERY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    let mut query = String::new();

    query.push_str(&arborium_html::HIGHLIGHTS_QUERY);
    query.push('\n');

    query.push_str(HIGHLIGHTS_QUERY_BASE);
    query
});




/// The injections query for svelte.
pub const INJECTIONS_QUERY: &str = include_str!("../queries/injections.scm");



/// The locals query for svelte.
pub const LOCALS_QUERY: &str = include_str!("../queries/locals.scm");



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "svelte",

            &HIGHLIGHTS_QUERY,

            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }

    #[test]
    fn test_corpus() {
        arborium_test_harness::test_corpus(language(), "svelte", env!("CARGO_MANIFEST_DIR"));
    }
}
