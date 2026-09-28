//! Reproductions for the tree-sitter runtime used by Arborium.
use arborium::tree_sitter::{Parser, Query, QueryCursor, StreamingIterator, Tree};

fn parse(language: &str, source: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&arborium::get_language(language).unwrap())
        .unwrap();
    parser.parse(source, None).unwrap()
}

#[test]
#[cfg(feature = "lang-javascript")]
fn utf16_surrogate_pairs_respect_input_endianness() {
    // Upstream #5912: both halves of a surrogate pair need byte swapping.
    let source = "const 𐐀 = 1;";
    let expected = parse("javascript", source).root_node().to_sexp();
    let mut parser = Parser::new();
    parser
        .set_language(&arborium::get_language("javascript").unwrap())
        .unwrap();
    for big_endian in [false, true] {
        let units: Vec<_> = source
            .encode_utf16()
            .map(|unit| {
                if big_endian {
                    unit.to_be()
                } else {
                    unit.to_le()
                }
            })
            .collect();
        let tree = if big_endian {
            parser.parse_utf16_be(&units, None)
        } else {
            parser.parse_utf16_le(&units, None)
        }
        .unwrap();
        assert_eq!(
            tree.root_node().to_sexp(),
            expected,
            "big_endian={big_endian}"
        );
        assert_eq!(tree.root_node().end_byte(), units.len() * 2);
    }
}

fn captures(language: &str, source: &str, query: &str) -> Vec<(String, String)> {
    let language_handle = arborium::get_language(language).unwrap();
    let query = Query::new(&language_handle, query).unwrap();
    let tree = parse(language, source);
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut result = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures() {
            result.push((
                query.capture_names()[capture.index as usize].to_owned(),
                capture
                    .node
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_owned(),
            ));
        }
    }
    result
}

fn assert_streaming_captures(
    language: &str,
    source: &str,
    pattern: &str,
    expected: &[(String, String)],
) {
    let query = Query::new(&arborium::get_language(language).unwrap(), pattern).unwrap();
    let tree = parse(language, source);
    let mut cursor = QueryCursor::new();
    let mut stream = cursor.captures(&query, tree.root_node(), source.as_bytes());
    let mut actual = Vec::new();
    while let Some((m, index)) = stream.next() {
        let capture = m.captures()[*index];
        actual.push((
            query.capture_names()[capture.index as usize].to_owned(),
            capture
                .node
                .utf8_text(source.as_bytes())
                .unwrap()
                .to_owned(),
        ));
    }
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    actual.sort_unstable();
    assert_eq!(actual, expected, "capture iteration: {pattern}: {source:?}");
}

#[test]
#[cfg(feature = "lang-json")]
fn disabled_wildcard_patterns_stay_disabled() {
    // tree-sitter#5932: removing entries must update the wildcard prefix length.
    let language = arborium::get_language("json").unwrap();
    let tree = parse("json", "[1, 2]");
    for query_source in [
        "(_) @any",
        "(_) @any (number) @number",
        "[_ (_)] @any (number) @number",
    ] {
        let mut query = Query::new(&language, query_source).unwrap();
        query.disable_pattern(0);
        query.disable_pattern(0); // Repeated disabling must not corrupt the prefix.
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), b"[1, 2]".as_slice());
        let mut count = 0;
        while let Some(m) = matches.next() {
            assert_eq!(m.pattern_index, 1);
            count += 1;
        }
        assert_eq!(count, if query.pattern_count() == 1 { 0 } else { 2 });
    }
}

#[test]
#[cfg(feature = "lang-rust")]
fn previous_sibling_crosses_256_child_boundaries() {
    // tree-sitter#5950: extra nodes do not get hidden repeat wrappers.
    let tree = parse("rust", &"// comment\n".repeat(800));
    let root = tree.root_node();
    let mut cursor = tree.walk();
    let children: Vec<_> = root.children(&mut cursor).collect();
    assert_eq!(children.len(), 800);
    cursor.reset(root);
    assert!(cursor.goto_last_child());
    for index in (0..children.len()).rev() {
        assert_eq!(cursor.node(), children[index]);
        assert_eq!(cursor.goto_previous_sibling(), index > 0, "child {index}");
    }
}

#[test]
#[cfg(feature = "lang-typescript")]
fn fields_do_not_leak_through_visible_aliases() {
    // tree-sitter#5948: `member_expression` is an alias of a hidden rule here.
    let source = "type T=typeof a.b";
    let tree = parse("typescript", source);
    let type_query = tree
        .root_node()
        .named_child(0)
        .unwrap()
        .child_by_field_name("value")
        .unwrap();
    assert_eq!(type_query.kind(), "type_query");
    assert!(type_query.child_by_field_name("object").is_none());
    let field = arborium::get_language("typescript")
        .unwrap()
        .field_id_for_name("object")
        .unwrap();
    assert!(type_query.child_by_field_id(field.get()).is_none());
    let member = type_query.named_child(0).unwrap();
    assert_eq!(
        member
            .child_by_field_name("object")
            .unwrap()
            .utf8_text(source.as_bytes())
            .unwrap(),
        "a"
    );
    assert_eq!(
        captures("typescript", source, "(type_query !object) @type"),
        vec![("type".into(), "typeof a.b".into())]
    );
}

#[test]
#[cfg(feature = "lang-javascript")]
fn child_with_descendant_rejects_self_ancestors_and_other_trees() {
    // tree-sitter#5949: byte ranges alone do not establish ancestry.
    let tree = parse("javascript", "(a)");
    let other_tree = parse("javascript", "(a)");
    let root = tree.root_node();
    let statement = root.named_child(0).unwrap();
    let expression = statement.named_child(0).unwrap();
    assert_eq!(root.child_with_descendant(root), None);
    assert_eq!(statement.child_with_descendant(root), None);
    assert_eq!(root.child_with_descendant(other_tree.root_node()), None);
    assert_eq!(root.child_with_descendant(expression), Some(statement));
    assert_eq!(
        statement.child_with_descendant(expression),
        Some(expression)
    );
}

#[test]
#[cfg(feature = "lang-javascript")]
fn descendant_lookup_agrees_with_cursor_ancestry() {
    // Include equal ranges, empty MISSING nodes, errors, anonymous nodes,
    // repeated rules, and comments. The cursor supplies an independent oracle.
    for source in [
        "(a); (b)",
        "a; b;",
        "({a: [1, 2]})",
        "function f( {",
        "(a",
        "// hi\nfoo()",
    ] {
        let tree = parse("javascript", source);
        let mut cursor = tree.walk();
        let mut nodes = vec![(cursor.node(), Vec::new())];
        let mut path = vec![cursor.node()];
        loop {
            if cursor.goto_first_child() {
                nodes.push((cursor.node(), path.clone()));
                path.push(cursor.node());
                continue;
            }
            loop {
                path.pop();
                if cursor.goto_next_sibling() {
                    nodes.push((cursor.node(), path.clone()));
                    path.push(cursor.node());
                    break;
                }
                if !cursor.goto_parent() {
                    path.clear();
                    break;
                }
            }
            if path.is_empty() {
                break;
            }
        }
        for (ancestor, _) in &nodes {
            for (descendant, ancestors) in &nodes {
                let expected = ancestors
                    .iter()
                    .position(|node| node == ancestor)
                    .map(|index| ancestors.get(index + 1).copied().unwrap_or(*descendant));
                assert_eq!(
                    ancestor.child_with_descendant(*descendant),
                    expected,
                    "{source:?}: {ancestor:?} -> {descendant:?}"
                );
            }
        }
    }
}

#[test]
#[cfg(all(feature = "lang-javascript", unix))]
fn deeply_nested_dot_graph_is_stack_safe() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let depth = 20_000;
            let source = format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
            let tree = parse("javascript", &source);
            assert!(!tree.root_node().has_error());
            tree.print_dot_graph(
                &std::fs::File::options()
                    .write(true)
                    .open("/dev/null")
                    .unwrap(),
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
#[cfg(feature = "lang-r")]
fn error_leaves_report_has_error() {
    // tree-sitter#3623: the error-cost heuristic is not the error predicate.
    let tree = parse("r", "1 + }");
    let mut nodes = vec![tree.root_node()];
    let mut errors = 0;
    while let Some(node) = nodes.pop() {
        if node.is_error() {
            errors += 1;
            assert!(node.has_error(), "{}", node.to_sexp());
        }
        let mut cursor = node.walk();
        nodes.extend(node.children(&mut cursor));
    }
    assert!(errors > 0);
}

#[test]
#[cfg(feature = "lang-json")]
fn nested_json_error_recovery() {
    let tree = parse("json", r#"{"a":{"b" "c"}}"#);
    assert_eq!(
        tree.root_node().to_sexp(),
        "(document (object (pair key: (string (string_content)) value: (object (ERROR (string (string_content)) (string (string_content)))))))"
    );
    let source = r#"{"one":{"bar" "baz"},"two":"bar"}"#;
    let tree = parse("json", source);
    assert_eq!(tree.root_node().kind(), "document");
    assert_eq!(
        captures("json", source, "(string) @string"),
        ["\"one\"", "\"bar\"", "\"baz\"", "\"two\"", "\"bar\""]
            .map(|text| ("string".into(), text.into()))
    );
    let object = tree.root_node().named_child(0).unwrap();
    assert_eq!(object.named_child_count(), 2);
    let second_pair = object.named_child(1).unwrap();
    assert!(!second_pair.has_error());
    assert_eq!(
        second_pair.utf8_text(source.as_bytes()).unwrap(),
        "\"two\":\"bar\""
    );
}

#[test]
#[cfg(feature = "lang-json")]
fn nested_json_recovers_after_incremental_repair() {
    use arborium::tree_sitter::{InputEdit, Point};
    let source = r#"{"one":{"bar" "baz"},"two":"bar"}"#;
    let offset = source.find(" \"baz\"").unwrap();
    let mut parser = Parser::new();
    parser
        .set_language(&arborium::get_language("json").unwrap())
        .unwrap();
    let mut tree = parser.parse(source, None).unwrap();
    tree.edit(&InputEdit {
        start_byte: offset,
        old_end_byte: offset,
        new_end_byte: offset + 1,
        start_position: Point::new(0, offset),
        old_end_position: Point::new(0, offset),
        new_end_position: Point::new(0, offset + 1),
    });
    let repaired = format!("{}:{}", &source[..offset], &source[offset..]);
    let incremental = parser.parse(&repaired, Some(&tree)).unwrap();
    assert!(!incremental.root_node().has_error());
    assert_eq!(
        incremental.root_node().to_sexp(),
        parse("json", &repaired).root_node().to_sexp()
    );
}

#[test]
#[cfg(feature = "lang-javascript")]
fn anchored_quantifiers() {
    for quantifier in ["", "?", "*", "+"] {
        for count in [0, 1, 2, 5] {
            let query = format!(
                "(program . (lexical_declaration) @self . (empty_statement){quantifier} @rest .) @whole"
            );
            let source = format!("const x = 1;{}", ";".repeat(count));
            let result = captures("javascript", &source, &query);
            let should_match = match quantifier {
                "" => count == 1,
                "?" => count <= 1,
                "*" => true,
                "+" => count > 0,
                _ => unreachable!(),
            };
            assert_eq!(
                result.iter().filter(|(name, _)| name == "whole").count(),
                usize::from(should_match),
                "{query}: {source}: {result:?}"
            );
            assert_eq!(
                result.iter().filter(|(name, _)| name == "rest").count(),
                if should_match { count } else { 0 },
                "{query}: {source}: {result:?}"
            );
            assert_streaming_captures("javascript", &source, &query, &result);
            // A later named sibling must prevent both the loop exit and zero skip.
            assert!(captures("javascript", &format!("{source}// end"), &query).is_empty());
            assert_streaming_captures("javascript", &format!("{source}// end"), &query, &[]);
        }
    }
}

#[test]
#[cfg(feature = "lang-javascript")]
fn anchored_repetitions_with_children_groups_and_alternatives() {
    for (source, pattern, expected) in [
        (
            "f();g();",
            ". (expression_statement (call_expression function: (identifier) @item))+ .",
            vec!["f", "g"],
        ),
        (
            "const x = 1;",
            ". (lexical_declaration (variable_declarator name: (identifier) @item)) . (empty_statement)* .",
            vec!["x"],
        ),
        (
            ";;;;",
            ". ((empty_statement) @item . (empty_statement) @item)+ .",
            vec![";", ";", ";", ";"],
        ),
        (
            ";/*a*/;",
            ". [(empty_statement) (comment)]* @item .",
            vec![";", "/*a*/", ";"],
        ),
        (
            ";;;",
            ". [(empty_statement)+ @item (comment)+ @item] .",
            vec![";", ";", ";"],
        ),
        (
            "/*a*//*b*/",
            ". [(empty_statement)+ @item (comment)+ @item] .",
            vec!["/*a*/", "/*b*/"],
        ),
        (
            ";;;",
            ". ((empty_statement)+ @item)* .",
            vec![";", ";", ";"],
        ),
        ("", ". (empty_statement)* @item .", vec![]),
    ] {
        let query = format!("(program {pattern}) @whole");
        let result = captures("javascript", source, &query);
        assert_eq!(
            result.iter().filter(|(name, _)| name == "whole").count(),
            1,
            "{query}: {source}: {result:?}"
        );
        assert_eq!(
            result
                .iter()
                .filter(|(name, _)| name == "item")
                .map(|(_, text)| text.as_str())
                .collect::<Vec<_>>(),
            expected,
            "{query}: {source}: {result:?}"
        );
        // Streaming captures exclude an empty root at the query range boundary.
        if !source.is_empty() {
            assert_streaming_captures("javascript", source, &query, &result);
        }
        assert!(
            captures("javascript", &format!("{source}let z = 2;"), &query).is_empty(),
            "{query}"
        );
        assert_streaming_captures("javascript", &format!("{source}let z = 2;"), &query, &[]);
    }
}

#[test]
#[cfg(feature = "lang-c")]
fn trailing_optional_anchor_preserves_upstream_regression() {
    let query = "(preproc_if (preproc_def)+ @def . (preproc_else)? @else .)";
    assert!(captures("c", "#if X\n#define A\n// c\n#endif\n", query).is_empty());
    assert_eq!(
        captures("c", "#if X\n#define A\n#endif\n", query),
        vec![("def".into(), "#define A\n".into())]
    );
}

#[test]
#[cfg(feature = "lang-java")]
fn valid_java_does_not_match_errors_or_missing_nodes() {
    for query in ["(superclass (MISSING)) @a", "(superclass (ERROR)) @a"] {
        assert!(captures("java", "class A extends B {}", query).is_empty());
    }
}

#[test]
#[cfg(feature = "lang-elm")]
fn repeated_fields_capture_every_parameter() {
    // tree-sitter#2984: the field belongs to a hidden repeat wrapper.
    for parameters in ["x", "x y", "a b c d e f g h"] {
        assert_eq!(
            captures(
                "elm",
                &format!("f {parameters} = 0"),
                "(function_declaration_left pattern: (_) @arg)"
            ),
            parameters
                .split_whitespace()
                .map(|p| ("arg".into(), p.into()))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
#[cfg(feature = "lang-html")]
fn capture_pool_reuse_preserves_matches_and_capture_order() {
    // tree-sitter#5951: keep many partial matches alive, then reuse the pool
    // across full iterations, abandoned iterations, and a low match limit.
    let language = arborium::get_language("html").unwrap();
    let query = Query::new(
        &language,
        "(element (start_tag (tag_name) @tag) (text) @text) (tag_name) @name",
    )
    .unwrap();
    let source = format!("{}hello{}", "<div>".repeat(96), "</div>".repeat(96));
    let tree = parse("html", &source);
    let mut cursor = QueryCursor::new();
    for _ in 0..3 {
        // Reset must reclaim captures held by unfinished query states.
        cursor
            .captures(&query, tree.root_node(), source.as_bytes())
            .next();
        let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
        let mut names = 0;
        let mut text_matches = 0;
        while let Some(m) = matches.next() {
            if m.pattern_index == 0 {
                text_matches += 1;
                assert_eq!(m.captures().len(), 2);
                assert_eq!(
                    m.captures()[1].node.utf8_text(source.as_bytes()).unwrap(),
                    "hello"
                );
            } else {
                names += 1;
            }
        }
        assert_eq!((names, text_matches), (192, 1));
        assert!(!cursor.did_exceed_match_limit());

        let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());
        let mut starts = Vec::new();
        while let Some((m, index)) = captures.next() {
            starts.push(m.captures()[*index].node.start_byte());
        }
        assert_eq!(starts.len(), 194);
        assert!(starts.windows(2).all(|pair| pair[0] <= pair[1]));
    }
    // The limit caps new allocations; lowering it does not shrink an existing
    // pool, so exercise exhaustion on a fresh cursor.
    let mut cursor = QueryCursor::new();
    cursor.set_match_limit(2);
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while matches.next().is_some() {}
    assert!(cursor.did_exceed_match_limit());
    cursor.set_match_limit(65536);
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut count = 0;
    while matches.next().is_some() {
        count += 1;
    }
    assert_eq!(count, 193);
    assert!(!cursor.did_exceed_match_limit());
}
