#!/usr/bin/env python3
"""Link and execute direct-runtime and all-language WASM consumers without xtask.

Requires wasm32-unknown-unknown, a WASM-capable C compiler, and Node.js.
This tests final modules: building an rlib alone cannot detect missing imports.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    root = Path(__file__).resolve().parents[1]
    languages = sorted(p.parent.name.removeprefix('arborium-') for p in
                       (root / 'crates/arborium-languages').glob('*/grammar'))
    with tempfile.TemporaryDirectory(prefix='arborium-wasm-check-') as directory:
        consumer = Path(directory)
        (consumer / 'src').mkdir()
        (consumer / 'Cargo.toml').write_text(f'''[package]
name = "arborium-wasm-check"
version = "0.0.0"
edition = "2024"
[lib]
crate-type = ["cdylib"]
[features]
all = ["dep:arborium"]
[dependencies]
arborium-tree-sitter = {{ path = {json.dumps(str(root / 'crates/arborium-tree-sitter'))} }}
arborium = {{ path = {json.dumps(str(root / 'crates/arborium'))}, features = ["all-languages"], optional = true }}
''')
        language_literals = ', '.join(json.dumps(language) for language in languages)
        regression_source = json.dumps(str(root / 'scripts/tests/tree_sitter_wasm.rs'))
        (consumer / 'src/lib.rs').write_text(f'''#[cfg(feature = "all")]
#[path = {regression_source}]
mod tree_sitter_regressions;
''' + '''#[unsafe(no_mangle)]
pub extern "C" fn smoke_test() {
    // #212: direct consumers must link the sysroot, even in debug builds.
    assert!(arborium_tree_sitter::Parser::new().language().is_none());
    #[cfg(feature = "all")]
    {
        tree_sitter_regressions::run();
        let mut highlighter = arborium::Highlighter::new();
        for language in [''' + language_literals + '''] {
            highlighter.highlight(language, "").unwrap();
        }
        for (language, source) in [
            ("fsharp", "// comment\\nnamespace FSharp.Data\\n"),
            ("perl", "# no newline"),
            ("cobol", "!"),
            ("markdown", "**bold**"),
            ("powershell", "Write-Host \\\"hello\\\""),
            ("swift", "let value = 1"),
            ("vim", "let value = 1"),
            ("caddy", "localhost {\\n respond \\\"hello\\\"\\n}"),
        ] {
            highlighter.highlight(language, source).unwrap();
        }
    }
}
''')
        env = dict(os.environ)
        target = Path(env.get('CARGO_TARGET_DIR', root / 'target/wasm-check')).resolve()
        env['CARGO_TARGET_DIR'] = str(target)
        module = target / 'wasm32-unknown-unknown/debug/arborium_wasm_check.wasm'
        for features in [[], ['--features', 'all']]:
            subprocess.run(['cargo', 'build', '--target', 'wasm32-unknown-unknown', *features],
                           cwd=consumer, env=env, check=True)
            subprocess.run(['node', '--input-type=module', '-e', '''
import fs from 'node:fs';
import assert from 'node:assert/strict';
const module = await WebAssembly.compile(fs.readFileSync(process.argv[1]));
assert.deepEqual(WebAssembly.Module.imports(module), [], 'unexpected host imports');
const instance = await WebAssembly.instantiate(module, {});
instance.exports.smoke_test();
''', str(module)], check=True, timeout=60)
        print(f'WASM checks passed: direct runtime and {len(languages)} bundled grammars; zero host imports.')


if __name__ == '__main__':
    main()
