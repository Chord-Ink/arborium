#!/usr/bin/env python3
"""Test Cargo Git dependencies using only files eligible for version control.

Includes local edits and untracked, non-ignored files for pre-commit checks.
Neither the snapshot nor the consumer runs xtask or tree-sitter generation.
"""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def run(args, cwd, **kwargs):
    return subprocess.run(args, cwd=cwd, check=True, **kwargs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--wasm", action="store_true", help="Also link a wasm32 consumer")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    with tempfile.TemporaryDirectory(prefix="arborium-git-check-") as directory:
        snapshot = Path(directory) / "repository"
        snapshot.mkdir()
        files = run(
            ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
            root, capture_output=True,
        ).stdout.decode().split("\0")
        for name in sorted(set(files) - {""}):
            source = root / name
            if source.is_file():
                target = snapshot / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source, target)
        run(["git", "init", "--quiet"], snapshot)
        run(["git", "add", "."], snapshot)
        run([
            "git", "-c", "user.name=Arborium test", "-c", "user.email=test@example.invalid",
            "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "Test snapshot",
        ], snapshot)

        consumer = Path(directory) / "consumer"
        (consumer / "src").mkdir(parents=True)
        git_url = json.dumps(snapshot.as_uri())
        (consumer / "Cargo.toml").write_text(f'''[package]
name = "arborium-git-consumer"
version = "0.0.0"
edition = "2024"

[dependencies]
arborium = {{ git = {git_url}, default-features = false, features = ["lang-json", "lang-rust", "lang-cpp", "lang-html", "lang-javascript", "lang-css", "lang-markdown", "lang-query"] }}
arborium-json = {{ git = {git_url} }}
''')
        (consumer / "src/main.rs").write_text(r'''
fn main() {
    let language = arborium_json::language().into();
    let mut parser = arborium::tree_sitter::Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(r#"{"hello": true}"#, None).unwrap();
    assert!(!tree.root_node().has_error());

    let mut highlighter = arborium::Highlighter::new();
    for (language, source) in [
        ("json", r#"{"hello": true}"#),
        ("rust", "fn main() { println!(\"hello\"); }"),
        ("cpp", "int main() { return 42; }"),
        ("html", "<script>const answer = 42;</script><style>body { color: red; }</style>"),
        ("markdown", "# Hello\n\n**world**\n"),
        ("query", "(identifier) @variable"),
    ] {
        let spans = highlighter.highlight_spans(language, source).unwrap();
        assert!(!spans.is_empty(), "no highlighting for {language}");
    }
}
''')
        env = dict(os.environ)
        env.setdefault("CARGO_TARGET_DIR", str(root / "target/git-dependency"))
        metadata = json.loads(run(
            ["cargo", "metadata", "--format-version", "1"], consumer,
            env=env, capture_output=True, text=True,
        ).stdout)
        resolved_ids = {node["id"] for node in metadata["resolve"]["nodes"]}
        packages = [p for p in metadata["packages"]
                    if p["id"] in resolved_ids and p["name"].startswith("arborium")
                    and p["name"] != "arborium-git-consumer"]
        for package in packages:
            assert (package["source"] or "").startswith("git+" + snapshot.as_uri()), package
        run(["cargo", "run", "--quiet"], consumer, env=env)
        if args.wasm:
            run(["cargo", "build", "--target", "wasm32-unknown-unknown"], consumer, env=env)
        print(f"Git dependency smoke test passed ({len(packages)} Arborium packages from Git).")


if __name__ == "__main__":
    main()
