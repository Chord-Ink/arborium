//! THIS FILE IS GENERATED FROM xtask/templates/lib.stpl.rs; DO NOT EDIT MANUALLY

#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_idris() -> *const ();
}

/// Returns the idris tree-sitter [`LanguageFn`].
pub const fn language() -> LanguageFn {
    unsafe { LanguageFn::from_raw(tree_sitter_idris) }
}



/// The highlights query for idris.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");




/// The injections query for idris (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";



/// The locals query for idris (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

