//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_agda() -> *const ();
}

/// Returns the agda tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_agda) }
}



/// The highlights query for agda.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");




/// The injections query for agda (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";



/// The locals query for agda (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

