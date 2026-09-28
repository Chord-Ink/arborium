use std::{cell::Cell, rc::Rc};
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator, TextProvider};

struct Chunks<'a> {
    bytes: &'a [u8],
    width: usize,
    empty_next: bool,
    reads: Rc<Cell<usize>>,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.empty_next {
            self.empty_next = false;
            return Some(&[]);
        }
        if self.bytes.is_empty() {
            return None;
        }
        let len = self.width.min(self.bytes.len());
        let (chunk, rest) = self.bytes.split_at(len);
        self.bytes = rest;
        self.empty_next = true;
        self.reads.set(self.reads.get() + 1);
        Some(chunk)
    }
}

struct Provider<'a> {
    source: &'a [u8],
    width: usize,
    reads: Rc<Cell<usize>>,
}

impl<'a> TextProvider<&'a [u8]> for Provider<'a> {
    type I = Chunks<'a>;
    fn text(&mut self, node: Node) -> Self::I {
        Chunks {
            bytes: &self.source[node.byte_range()],
            width: self.width,
            empty_next: true,
            reads: self.reads.clone(),
        }
    }
}

#[test]
fn chunked_predicates_preserve_capture_output() {
    let language = tree_sitter_json::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let source = r#"[123,123,456,1,12,0,"日本語","日本語",""]"#.as_bytes();
    let tree = parser.parse(source, None).unwrap();
    for expression in [
        r#"((number) @n (#eq? @n "123"))"#,
        r#"((number) @n (#not-eq? @n "123"))"#,
        r#"(array (number) @a (number) @b (#eq? @a @b))"#,
        r#"(array (number) @a (number) @b (#not-eq? @a @b))"#,
        r#"((number) @n (#any-of? @n "123" "456" "123"))"#,
        r#"((number) @n (#not-any-of? @n "0" "1" "2" "3" "4" "5" "6" "7" "8" "9" "123"))"#,
        r#"((number) @n (#match? @n "^12"))"#,
        r#"((string_content) @s (#eq? @s "日本語"))"#,
        r#"(array (string) @a (string) @b (#eq? @a @b))"#,
        r#"(array (number)+ @n (#any-eq? @n "123"))"#,
        r#"(array (number)+ @n (#any-not-eq? @n "123"))"#,
        r#"(array (number)+ @n (#eq? @n "123"))"#,
    ] {
        let query = Query::new(&language, expression).unwrap();
        let mut cursor = QueryCursor::new();
        cursor.set_match_limit(64);
        let expected: Vec<_> = cursor
            .captures(&query, tree.root_node(), source)
            .map(|(m, i)| {
                (
                    m.pattern_index,
                    m.captures()[*i].index,
                    m.captures()[*i].node.byte_range(),
                )
            })
            .cloned()
            .collect();
        for width in [1, 2, 3, 7, 1024] {
            let provider = Provider {
                source,
                width,
                reads: Rc::new(Cell::new(0)),
            };
            let actual: Vec<_> = cursor
                .captures(&query, tree.root_node(), provider)
                .map(|(m, i)| {
                    (
                        m.pattern_index,
                        m.captures()[*i].index,
                        m.captures()[*i].node.byte_range(),
                    )
                })
                .cloned()
                .collect();
            assert_eq!(actual, expected, "{expression}, chunk width {width}");
        }
    }
}

#[test]
fn equality_rejects_a_long_capture_without_reading_its_suffix() {
    let language = tree_sitter_json::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let source = format!("\"{}\"", "a".repeat(1024 * 1024));
    let tree = parser.parse(&source, None).unwrap();
    let query = Query::new(&language, r#"((string) @s (#eq? @s "different"))"#).unwrap();
    let reads = Rc::new(Cell::new(0));
    let provider = Provider {
        source: source.as_bytes(),
        width: 1024,
        reads: reads.clone(),
    };
    let mut cursor = QueryCursor::new();
    assert!(
        cursor
            .captures(&query, tree.root_node(), provider)
            .next()
            .is_none()
    );
    assert_eq!(reads.get(), 1);
}

#[cfg(feature = "wasm")]
#[test]
fn native_wasm_host_parses_queries_and_resumes_after_cancellation() {
    use std::ops::ControlFlow;
    use tree_sitter::{ParseOptions, WasmStore, wasmtime};

    let bytes = std::fs::read(std::env::var("ARBORIUM_TEST_WASM").unwrap()).unwrap();
    let mut store = WasmStore::new(&wasmtime::Engine::default()).unwrap();
    let language = store.load_language("json", &bytes).unwrap();
    assert!(language.is_wasm());
    let mut parser = Parser::new();
    parser.set_wasm_store(store).unwrap();
    parser.set_language(&language).unwrap();
    let source = format!("\"{}\"", "a".repeat(20000));
    let calls = Cell::new(0);
    let mut cancel = |_: &tree_sitter::ParseState| {
        calls.set(calls.get() + 1);
        ControlFlow::Break(())
    };
    let cancelled = parser.parse_with_options(
        &mut |offset, _| &source.as_bytes()[offset..],
        None,
        Some(ParseOptions::new().progress_callback(&mut cancel)),
    );
    assert!(cancelled.is_none());
    assert_eq!(calls.get(), 1);
    let tree = parser.parse(&source, None).unwrap();
    assert!(!tree.root_node().has_error());
    assert_eq!(tree.root_node().end_byte(), source.len());
    let query = Query::new(&language, "(string) @string").unwrap();
    let mut cursor = QueryCursor::new();
    let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());
    assert_eq!(
        captures.next().unwrap().0.captures()[0].node.byte_range(),
        0..source.len()
    );
    assert!(captures.next().is_none());
}
