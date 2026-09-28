//! Upstream v0.27.0 error corpus, run through Arborium's actual bundled grammars.
use arborium::tree_sitter::Parser;

fn normalized(sexp: &str) -> String {
    sexp.split_whitespace()
        .filter(|word| !word.ends_with(':'))
        .collect()
}

fn check(language: &str, corpus: &str) {
    let mut parser = Parser::new();
    parser
        .set_language(&arborium::get_language(language).unwrap())
        .unwrap();
    let lines: Vec<_> = corpus.lines().collect();
    let mut index = 0;
    let mut count = 0;
    let mut failures = Vec::new();
    while index < lines.len() {
        if !lines[index].starts_with("===") {
            index += 1;
            continue;
        }
        index += 1;
        let name = lines[index];
        while index < lines.len() && !lines[index].starts_with("===") {
            index += 1;
        }
        index += 1;
        let start = index;
        while index < lines.len() && !lines[index].starts_with("---") {
            index += 1;
        }
        let source = lines[start..index].join("\n").trim_matches('\n').to_owned();
        index += 1;
        let start = index;
        while index < lines.len() && !lines[index].starts_with("===") {
            index += 1;
        }
        let expected = lines[start..index].join("\n");
        let actual = parser.parse(&source, None).unwrap().root_node().to_sexp();
        if normalized(&actual) != normalized(&expected) {
            failures.push(format!(
                "{name}:\n{source}\nexpected: {}\nactual: {actual}",
                normalized(&expected)
            ));
        }
        count += 1;
    }
    assert!(count > 0);
    assert!(
        failures.is_empty(),
        "{language}: {} of {count} failed:\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

macro_rules! corpus {
    ($name:ident, $language:literal) => {
        #[test]
        #[cfg(feature = $language)]
        fn $name() {
            check(
                stringify!($name),
                include_str!(concat!(
                    "fixtures/tree-sitter-errors/",
                    stringify!($name),
                    "_errors.txt"
                )),
            );
        }
    };
}

corpus!(c, "lang-c");
corpus!(javascript, "lang-javascript");
corpus!(json, "lang-json");
corpus!(python, "lang-python");
corpus!(ruby, "lang-ruby");
