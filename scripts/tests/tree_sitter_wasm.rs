//! Execute the patched C runtime through Arborium's WASM Rust bindings.
use arborium_tree_sitter::{Parser, Query, QueryCursor, StreamingIterator, Tree};

fn parse(language: &str, source: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&arborium::get_language(language).unwrap())
        .unwrap();
    parser.parse(source, None).unwrap()
}

pub fn run() {
    // #5910: keep recovery local and preserve the following pair's token ranges.
    let source = r#"{"one":{"bar" "baz"},"two":"bar"}"#;
    let tree = parse("json", source);
    assert_eq!(tree.root_node().kind(), "document");
    let object = tree.root_node().named_child(0).unwrap();
    assert_eq!(object.named_child_count(), 2);
    let pair = object.named_child(1).unwrap();
    assert!(!pair.has_error());
    assert_eq!(
        pair.utf8_text(source.as_bytes()).unwrap(),
        "\"two\":\"bar\""
    );

    // #5416: anchor the last repetition, including zero-occurrence skips.
    let language = arborium::get_language("javascript").unwrap();
    for quantifier in ["?", "*", "+"] {
        let query = Query::new(
            &language,
            &format!(
                "(program . (lexical_declaration) . (empty_statement){quantifier} @rest .) @whole"
            ),
        )
        .unwrap();
        for count in [0, 1, 2, 5] {
            let source = format!("const x = 1;{}", ";".repeat(count));
            let tree = parse("javascript", &source);
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
            let should_match = match quantifier {
                "?" => count <= 1,
                "*" => true,
                _ => count > 0,
            };
            if should_match {
                assert_eq!(matches.next().unwrap().captures().len(), count + 1);
            }
            assert!(matches.next().is_none());
        }
    }

    // #5949: equal ranges do not establish ancestry.
    let tree = parse("javascript", "(a)");
    let root = tree.root_node();
    let statement = root.named_child(0).unwrap();
    assert!(root.child_with_descendant(root).is_none());
    assert!(statement.child_with_descendant(root).is_none());
    assert_eq!(
        root.child_with_descendant(statement.named_child(0).unwrap()),
        Some(statement)
    );
    let copy = tree.clone();
    assert!(copy.root_node().child_with_descendant(statement).is_none());

    // Deep-tree parsing and navigation on WASM. Native #5807 tests additionally
    // force deep identity searches, field backtracking, empty suffixes and DOT output.
    let source = format!("{}1{}", "(".repeat(12_000), ")".repeat(12_000));
    let tree = parse("javascript", &source);
    let root = tree.root_node();
    let mut cursor = tree.walk();
    for _ in 0..12_002 {
        assert!(cursor.goto_first_child());
        // Parenthesized expressions begin with an anonymous opening delimiter.
        if !cursor.node().is_named() {
            assert!(cursor.goto_next_sibling());
        }
    }
    assert_eq!(cursor.node().kind(), "number");
    assert_eq!(
        root.child_with_descendant(cursor.node()),
        root.named_child(0)
    );

    // #5932: used to read beyond the pattern map (or abort with assertions).
    let language = arborium::get_language("json").unwrap();
    let mut query = Query::new(&language, "(_) @any").unwrap();
    query.disable_pattern(0);
    let tree = parse("json", "1");
    let mut cursor = QueryCursor::new();
    assert!(
        cursor
            .matches(&query, tree.root_node(), b"1".as_slice())
            .next()
            .is_none()
    );

    // #5950: check three full-width child-index boundaries.
    let tree = parse("rust", &"// comment\n".repeat(800));
    let mut walk = tree.walk();
    assert!(walk.goto_last_child());
    for index in (0..800).rev() {
        assert_eq!(walk.node().start_position().row, index);
        assert_eq!(walk.goto_previous_sibling(), index > 0);
    }

    // #5948: do not inherit an aliased child's fields.
    let tree = parse("typescript", "type T=typeof a.b");
    let type_query = tree
        .root_node()
        .named_child(0)
        .unwrap()
        .child_by_field_name("value")
        .unwrap();
    assert!(type_query.child_by_field_name("object").is_none());

    // #3623: the unexpected terminal is an error even when its cost is zero.
    let tree = parse("r", "1 + }");
    let mut nodes = vec![tree.root_node()];
    let mut errors = 0;
    while let Some(node) = nodes.pop() {
        if node.is_error() {
            errors += 1;
            assert!(node.has_error());
        }
        nodes.extend(node.children(&mut node.walk()));
    }
    assert!(errors > 0);

    // #2984, #5951: all repeated fields survive querying and cursor reuse.
    let language = arborium::get_language("elm").unwrap();
    let query = Query::new(&language, "(function_declaration_left pattern: (_) @arg)").unwrap();
    let source = "f a b c d e f g h = 0";
    let tree = parse("elm", source);
    for _ in 0..10 {
        let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
        let mut actual = Vec::new();
        while let Some(m) = matches.next() {
            actual.push(m.captures()[0].node.utf8_text(source.as_bytes()).unwrap());
        }
        assert_eq!(actual, vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
    }
}
