#!/usr/bin/env python3
"""Run vendored upstream tree-sitter corpora against the committed parser sources.

Runs independently of generation. Requires the tree-sitter CLI and a C compiler.
Upstream fixtures live in langs/group-*/*/def/test/corpus and retain their format.
"""
import concurrent.futures
import gzip
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


def check(corpus):
    root = Path(__file__).resolve().parents[1]
    language = corpus.parents[2].name
    grammar = root / 'crates/arborium-languages' / f'arborium-{language}' / 'grammar'
    with tempfile.TemporaryDirectory(prefix=f'arborium-corpus-{language}-') as directory:
        work = Path(directory)
        shutil.copytree(grammar, work, dirs_exist_ok=True)
        # Arborium keeps scanner helpers beside grammar/src; the CLI expects them in src.
        for source in grammar.iterdir():
            if source.name != 'src':
                target = work / 'src' / source.name
                if source.is_dir():
                    shutil.copytree(source, target, dirs_exist_ok=True)
                else:
                    shutil.copy2(source, target)
        (work / 'src/parser.c').write_bytes(gzip.decompress((grammar / 'src/parser.c.gz').read_bytes()))
        shutil.copytree(corpus, work / 'test/corpus')
        grammar_name = json.loads((grammar / 'src/grammar.json').read_text())['name']
        (work / 'tree-sitter.json').write_text(json.dumps({
            'grammars': [{'name': grammar_name, 'path': '.', 'scope': f'source.{language}'}],
            'metadata': {'version': '0.0.0', 'license': 'MIT'},
        }))
        try:
            result = subprocess.run(['tree-sitter', 'test', '--grammar-path', str(work), '--overview-only'],
                                    cwd=work, capture_output=True, text=True, timeout=180)
            return language, result.returncode, result.stdout + result.stderr
        except subprocess.TimeoutExpired:
            return language, 1, 'Parser corpus exceeded the 180-second time limit.'


def main():
    root = Path(__file__).resolve().parents[1]
    corpora = sorted(root.glob('langs/group-*/*/def/test/corpus'))
    failed = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for language, status, output in pool.map(check, corpora):
            print(f'{language}: {"FAIL" if status else "PASS"}', flush=True)
            if status:
                failed.append(language)
                print(output, flush=True)
    if failed:
        raise SystemExit(f'Failed corpora: {", ".join(failed)}')
    print(f'Upstream corpora passed for {len(corpora)} grammars.')


if __name__ == '__main__':
    main()
