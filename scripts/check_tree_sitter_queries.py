#!/usr/bin/env python3
"""Compare Zed query output and check cancellation across native grammars."""
import argparse
import gzip
import os
from pathlib import Path
import re
import subprocess
import tempfile
import tomllib

FIXTURES = {
    'json': ('{"key":[1,2,true,false,null],"text":"日本語\\u1234"}',
             '(pair key: (string) @key value: (_) @value) (array . (number)* @n .)',
             ['"' + 'a' * 20000 + '"', ' ' * 20000 + '{}', '"' + 'a' * 20000 + '\\']),
    'javascript': ('''/** docs */
class Thing extends Base { method({a, b = 2}, ...rest) { return {a, b, rest}; } }
const [x, y] = [1, 2]; let text = `hello ${x + y}`;
async function example() { for (const v of [x, y]) { await f(v?.key); } }
const view = <div key="a">{text}</div>;
''', '(expression) @expr (primary_expression/identifier) @id (arguments . (_) @first . (_) @second .)',
                   ['/*' + 'a' * 20000 + '*/ const x = 1;', '`' + 'a' * 20000 + '`']),
    'rust': ('''/// docs
use std::{fmt, io};
struct Thing<T> { field: Option<T> }
impl<T: fmt::Debug> Thing<T> { fn get(&self) -> &Option<T> { &self.field } }
fn main() { let values = [1, 2, 3]; for x in values { match x { 1..=2 => println!("{x}"), _ => () } } }
''', '(_expression) @expr (function_item name: (identifier) @name) (arguments . (_) @first .)',
             ['/* nested /*' + 'a' * 20000 + '*/ */ fn main() {}', 'fn main() { let s = r###"' + 'a' * 20000 + '"###; }']),
}


def main():
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, default=root / 'crates/arborium-tree-sitter')
    parser.add_argument('--zed', type=Path, required=True)
    parser.add_argument('--rust-grammar', type=Path, help='Zed-pinned tree-sitter-rust crate directory; defaults to Cargo cache')
    parser.add_argument('--sanitize', action='store_true')
    args = parser.parse_args()
    if args.rust_grammar is None:
        lock = tomllib.loads((args.zed / 'Cargo.lock').read_text())
        version = next(p['version'] for p in lock['package'] if p['name'] == 'tree-sitter-rust')
        cargo_home = Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo'))
        args.rust_grammar = next(cargo_home.glob(f'registry/src/*/tree-sitter-rust-{version}'), None)
        if args.rust_grammar is None:
            parser.error(f'Pass --rust-grammar with the tree-sitter-rust {version} crate used by Zed')
    with tempfile.TemporaryDirectory(prefix='arborium-query-check-') as directory:
        temporary = Path(directory)
        for language, (source, extra_query, cancellation_sources) in FIXTURES.items():
            configuration = tomllib.loads((args.zed / f'crates/grammars/src/{language}/config.toml').read_text())
            grammar_name = configuration['grammar']
            grammar = root / f'crates/arborium-languages/arborium-{grammar_name}/grammar'
            if language == 'rust':
                # Arborium's rust-orchard grammar does not accept Zed's Rust queries.
                grammar = args.rust_grammar.resolve()
            grammar_c = temporary / f'{language}.c'
            if (grammar / 'src/parser.c.gz').exists():
                grammar_c.write_bytes(gzip.decompress((grammar / 'src/parser.c.gz').read_bytes()))
            else:
                grammar_c.write_bytes((grammar / 'src/parser.c').read_bytes())
            export = re.search(r'TS_PUBLIC const TSLanguage \*(tree_sitter_\w+)\(void\)', grammar_c.read_text()).group(1)
            fixture = temporary / f'{language}.txt'
            fixture.write_text(source)
            extra = temporary / f'{language}.scm'
            extra.write_text(extra_query)
            queries = sorted((args.zed / f'crates/grammars/src/{language}').glob('*.scm')) + [extra]
            expected = None
            for name, runtime in [('baseline', args.baseline), ('candidate', args.candidate)]:
                runtime = runtime.resolve()
                binary = temporary / f'{language}-{name}'
                command = [os.environ.get('CC', 'cc'), '-std=c11', '-D_POSIX_C_SOURCE=200809L', '-O2', '-g',
                           f'-DLANGUAGE={export}', '-I' + str(runtime / 'include'),
                           '-I' + str(runtime / 'src'), '-I' + str(grammar / 'src'),
                           str(root / 'scripts/tests/tree_sitter_queries.c'), str(runtime / 'src/lib.c'),
                           str(grammar_c), '-o', str(binary)]
                for scanner in [grammar / 'scanner.c', grammar / 'src/scanner.c']:
                    if scanner.exists():
                        command.append(str(scanner))
                        break
                if args.sanitize:
                    command += ['-fsanitize=address,undefined', '-fno-omit-frame-pointer']
                subprocess.run(command, check=True)
                output = subprocess.check_output([str(binary), str(fixture), *map(str, queries)], timeout=60)
                if expected is not None and output != expected:
                    raise RuntimeError(f'{language}: query output differs from baseline')
                expected = output
                if name == 'candidate':
                    for source in cancellation_sources:
                        fixture.write_text(source)
                        subprocess.run([str(binary), str(fixture), 'cancel'], check=True, timeout=30)
            print(f'{language}: {len(queries)} queries match; UTF-8/UTF-16 cancellation and resume pass', flush=True)


if __name__ == '__main__':
    main()
