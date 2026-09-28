#!/usr/bin/env python3
"""Compare native runtimes using Zed's JSON queries and editor-shaped operations."""
import argparse
import gzip
import json
import os
from pathlib import Path
import statistics
import subprocess
import tempfile


def main():
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True, help='Baseline runtime directory')
    parser.add_argument('--candidate', type=Path, default=root / 'crates/arborium-tree-sitter')
    parser.add_argument('--zed', type=Path, required=True, help='Zed checkout (read-only)')
    parser.add_argument('--runs', type=int, default=5)
    sanitizer = parser.add_mutually_exclusive_group()
    sanitizer.add_argument('--sanitize', action='store_true')
    sanitizer.add_argument('--thread-sanitize', action='store_true')
    args = parser.parse_args()
    if args.runs < 1:
        parser.error('--runs must be positive')
    query = args.zed / 'crates/grammars/src/json/highlights.scm'
    grammar = root / 'crates/arborium-languages/arborium-json/grammar/src'
    with tempfile.TemporaryDirectory(prefix='arborium-runtime-bench-') as directory:
        temporary = Path(directory)
        grammar_c = temporary / 'json.c'
        grammar_c.write_bytes(gzip.decompress((grammar / 'parser.c.gz').read_bytes()))
        binaries = {}
        captures = None
        for name, runtime in [('baseline', args.baseline), ('candidate', args.candidate)]:
            runtime = runtime.resolve()
            binary = temporary / name
            command = [os.environ.get('CC', 'cc'), '-std=c11', '-D_POSIX_C_SOURCE=200809L',
                       '-O2' if args.sanitize else '-O3', '-g', '-pthread',
                       '-I' + str(runtime / 'include'), '-I' + str(runtime / 'src'),
                       '-I' + str(grammar), str(root / 'scripts/tests/tree_sitter_performance.c'),
                       str(runtime / 'src/lib.c'), str(grammar_c), '-o', str(binary)]
            if args.sanitize:
                command += ['-fsanitize=address,undefined', '-fno-omit-frame-pointer']
            if args.thread_sanitize:
                command += ['-fsanitize=thread', '-fno-omit-frame-pointer']
            subprocess.run(command, check=True)
            binaries[name] = binary
            output = subprocess.check_output([str(binary), str(query), 'captures'], timeout=90)
            if captures is not None and output != captures:
                raise RuntimeError('Candidate capture output differs from baseline')
            captures = output
        samples = {name: [] for name in binaries}
        for iteration in range(args.runs):
            # Alternate order to reduce drift from temperature and CPU scheduling.
            order = list(binaries) if iteration % 2 == 0 else list(reversed(binaries))
            for name in order:
                binary = binaries[name]
                output = subprocess.check_output([str(binary), str(query), 'bench'], text=True, timeout=90)
                samples[name].append({key: float(value) for key, value in (line.split() for line in output.splitlines())})
        results = {name: {key: statistics.median(s[key] for s in values) for key in values[0]}
                   for name, values in samples.items()}
        print(json.dumps({'capture_output_equal': True, 'runs': args.runs, **results}, indent=2))


if __name__ == '__main__':
    main()
