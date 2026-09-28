#!/usr/bin/env python3
"""Run native runtime invariants and Rust predicate regressions."""
import argparse
import gzip
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runtime', type=Path, default=root / 'crates/arborium-tree-sitter')
    parser.add_argument('--sanitize', action='store_true')
    parser.add_argument('--wasm', action='store_true', help='Also test native Wasmtime grammar hosting (source .envrc first)')
    args = parser.parse_args()
    runtime = args.runtime.resolve()
    with tempfile.TemporaryDirectory(prefix='arborium-runtime-check-') as directory:
        temporary = Path(directory)
        for source in ['capture_list_pool.c', 'tree_traversal.c', 'tree_sitter_scratch.c']:
            binary = temporary / source.removesuffix('.c')
            command = [os.environ.get('CC', 'cc'), '-std=c11', '-D_POSIX_C_SOURCE=200809L',
                       '-O2', '-g', '-I' + str(runtime / 'include'), '-I' + str(runtime / 'src'),
                       str(root / 'scripts/tests' / source), '-o', str(binary)]
            if args.sanitize:
                command += ['-fsanitize=address,undefined', '-fno-omit-frame-pointer']
            subprocess.run(command, check=True)
            subprocess.run([str(binary)], check=True, timeout=60)
        (temporary / 'Cargo.toml').write_text(f'''[package]
name = "arborium-runtime-regressions"
version = "0.0.0"
edition = "2024"
[features]
wasm = ["tree-sitter/wasm"]
[dependencies]
tree-sitter = {{ package = "arborium-tree-sitter", path = {json.dumps(str(runtime))} }}
tree-sitter-json = "=0.24.8"
[[test]]
name = "predicates"
path = {json.dumps(str(root / 'scripts/tests/tree_sitter_predicates.rs'))}
''')
        env = dict(os.environ)
        env['CARGO_TARGET_DIR'] = str(root / 'target/runtime-regressions')
        features = []
        if args.wasm:
            grammar = root / 'crates/arborium-languages/arborium-json/grammar/src'
            grammar_c = temporary / 'json.c'
            grammar_c.write_bytes(gzip.decompress((grammar / 'parser.c.gz').read_bytes()))
            module = temporary / 'json.wasm'
            object_file = temporary / 'json.o'
            subprocess.run([env.get('CC_wasm32_unknown_unknown', 'clang'), '--target=wasm32-unknown-unknown',
                            '-c', '-fPIC', '-O2',
                            '-I' + str(root / 'crates/arborium-sysroot/wasm-sysroot'),
                            '-I' + str(grammar), str(grammar_c), '-o', str(object_file)], check=True)
            sysroot = Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
            linker = next(sysroot.glob('lib/rustlib/*/bin/rust-lld'))
            linker_env = dict(env)
            for variable in ['DYLD_LIBRARY_PATH', 'LD_LIBRARY_PATH']:
                linker_env[variable] = os.pathsep.join(filter(None, [str(sysroot / 'lib'), env.get(variable)]))
            subprocess.run([str(linker), '-flavor', 'wasm', '--shared', '--export=tree_sitter_json',
                            str(object_file), '-o', str(module)], env=linker_env, check=True)
            env['ARBORIUM_TEST_WASM'] = str(module)
            features = ['--features', 'wasm']
        subprocess.run(['cargo', 'test', '--manifest-path', str(temporary / 'Cargo.toml'), *features], env=env, check=True)


if __name__ == '__main__':
    main()
