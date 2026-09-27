//! Reproductions from the upstream issue tracker.
use arborium::Highlighter;

fn assert_capture(language: &str, source: &str, text: &str, capture: &str) {
    let spans = Highlighter::new()
        .highlight_spans(language, source)
        .unwrap();
    let start = source.find(text).unwrap() as u32;
    let end = start + text.len() as u32;
    assert!(
        spans
            .iter()
            .any(|span| span.start <= start && span.end >= end && span.capture == capture),
        "{language}: missing {capture} for {text:?}: {spans:?}"
    );
}

#[test]
#[cfg(all(feature = "lang-javascript", feature = "lang-typescript"))]
fn shorthand_properties() {
    for language in ["javascript", "typescript"] {
        assert_capture(language, "const value = { foo };", "foo", "property");
        assert_capture(language, "const { foo } = value;", "foo", "property");
    }
}

#[test]
#[cfg(feature = "lang-markdown")]
fn markdown_inline_injections() {
    // This must also pass with ONLY lang-markdown enabled.
    for (source, text, capture) in [
        ("**bold**", "bold", "text.strong"),
        ("*italic*", "italic", "text.emphasis"),
        ("`code`", "code", "text.literal"),
        (
            "[link](https://example.com)",
            "https://example.com",
            "text.uri",
        ),
    ] {
        assert_capture("markdown", source, text, capture);
    }
}

#[test]
#[cfg(feature = "lang-fsharp")]
fn fsharp_namespace() {
    assert_capture(
        "fsharp",
        "// comment\nnamespace FSharp.Data\n",
        "// comment",
        "comment",
    );
}

#[test]
#[cfg(feature = "lang-perl")]
fn perl_comment_at_eof() {
    assert_capture(
        "perl",
        "# no final newline",
        "# no final newline",
        "comment",
    );
}

#[test]
#[cfg(feature = "lang-cobol")]
fn cobol_truncated_sequence_area() {
    // Previously looped forever when error recovery reached EOF before column 6.
    for source in ["!", "12345", "12345\n", "     !"] {
        Highlighter::new().highlight("cobol", source).unwrap();
    }
}

#[test]
#[cfg(feature = "lang-zsh")]
fn zsh_external_scanner() {
    assert_capture("zsh", "cat <<EOF\nhello\nEOF\n", "hello", "string");
}

#[test]
#[cfg(feature = "lang-scss")]
fn scss_mixin() {
    let source = "@mixin u-padding($value) { padding: $value; }";
    assert_capture("scss", source, "$value", "variable");
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("scss").unwrap())
        .unwrap();
    assert!(!parser.parse(source, None).unwrap().root_node().has_error());
}

#[test]
#[cfg(feature = "lang-clojure")]
fn clojure_calls_and_definitions() {
    let source = "(ns example.core)\n(defn greet [name] (str \"Hello \" name))";
    assert_capture("clojure", source, "ns", "keyword");
    assert_capture("clojure", source, "example.core", "namespace");
    assert_capture("clojure", source, "defn", "keyword");
    assert_capture("clojure", source, "greet", "function");
    assert_capture("clojure", source, "str", "function");
}

#[test]
#[cfg(feature = "lang-elixir")]
fn elixir_semantic_tags() {
    let html = Highlighter::new()
        .highlight(
            "elixir",
            "defmodule Phoenix.Controller do\n  import Plug.Conn\n  @moduledoc \"Docs\"\nend",
        )
        .unwrap();
    assert!(html.contains("<a-k>defmodule</a-k>"), "{html}");
    assert!(html.contains("<a-ns>Phoenix.Controller</a-ns>"), "{html}");
    assert!(html.contains("<a-k>import</a-k>"), "{html}");
}

#[test]
#[cfg(feature = "lang-batch")]
fn batch_prompt_does_not_break_following_lines() {
    let source =
        "prompt $E[1;32;49m$P$$$E[1;30;49m$S$E[0m\n:: comment after prompt\n:CLINK_FINISH\n";
    assert_capture("batch", source, ":: comment after prompt", "comment");
    assert_capture("batch", source, ":CLINK_FINISH", "function");
}

#[test]
#[cfg(feature = "lang-actionscript")]
fn actionscript_sample() {
    let source = "package {\n  public class Hello {\n    public function greet(name:String):void {\n      trace(\"Hello, \" + name);\n    }\n  }\n}\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("actionscript").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("actionscript", source)
            .unwrap()
            .is_empty()
    );
}

#[test]
#[cfg(feature = "lang-slang")]
fn slang_sample() {
    let source = "float4 main(float4 position : POSITION) : SV_Position { return position; }\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("slang").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("slang", source)
            .unwrap()
            .is_empty()
    );
}

#[test]
#[cfg(feature = "lang-koto")]
fn koto_sample() {
    let source = "square = |x| x * x\nprint square(5)\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("koto").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("koto", source)
            .unwrap()
            .is_empty()
    );
}

#[test]
#[cfg(feature = "lang-wat")]
fn wat_sample() {
    let source = "(module\n  (func (export \"answer\") (result i32)\n    i32.const 42))\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("wat").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("wat", source)
            .unwrap()
            .is_empty()
    );
}

#[test]
#[cfg(feature = "lang-crystal")]
fn crystal_sample() {
    let source = "def greet(name : String)\n  puts \"Hello, #{name}!\"\nend\ngreet(\"world\")\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("crystal").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("crystal", source)
            .unwrap()
            .is_empty()
    );
}

#[test]
#[cfg(feature = "lang-pug")]
fn pug_sample() {
    let source = "doctype html\nhtml\n  body\n    h1 Hello\n    p Welcome to Pug\n";
    let mut parser = arborium::tree_sitter::Parser::new();
    parser
        .set_language(&arborium::get_language("pug").unwrap())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert!(
        !Highlighter::new()
            .highlight_spans("pug", source)
            .unwrap()
            .is_empty()
    );
}
