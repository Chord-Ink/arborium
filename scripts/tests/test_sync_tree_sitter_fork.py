"""Sync invariants using local Git repositories; no network access required."""
import contextlib
import importlib.util
import io
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).resolve().parents[1] / "sync_tree_sitter_fork.py"
SPEC = importlib.util.spec_from_file_location("sync_tree_sitter_fork", SCRIPT)
sync = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = sync
SPEC.loader.exec_module(sync)


class SyncTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.origin = self.root / "origin"
        self.origin.mkdir()
        self.git(self.origin, "init", "--initial-branch=main")
        (self.origin / "lib").mkdir()
        (self.origin / "lib/input").write_text("old")
        self.git(self.origin, "add", ".")
        self.git(self.origin, "commit", "-m", "first")
        self.first = self.git(self.origin, "rev-parse", "HEAD")
        self.git(self.origin, "tag", "-a", "v1", "-m", "release")
        self.upstream = self.root / "upstream"
        self.git(self.root, "clone", str(self.origin), str(self.upstream))
        (self.origin / "lib/input").write_text("new")
        self.git(self.origin, "commit", "-am", "second")
        self.second = self.git(self.origin, "rev-parse", "HEAD")

    def git(self, cwd, *args):
        return subprocess.run(
            ["git", "-c", "user.name=Sync Test", "-c", "user.email=sync@example.invalid",
             "-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false", *args],
            cwd=cwd, check=True, capture_output=True, text=True,
        ).stdout.strip()

    def test_fetch_uses_remote_branch_instead_of_stale_local_branch(self):
        self.assertEqual(sync.resolve_upstream_revision(self.upstream, "main", False), self.second)
        self.assertEqual(self.git(self.upstream, "rev-parse", "main"), self.first)

    def test_offline_resolves_full_commit_and_peels_annotated_tag(self):
        for ref in [self.first, "refs/tags/v1"]:
            self.assertEqual(sync.resolve_upstream_revision(self.upstream, ref, True), self.first)

    def test_missing_revision_leaves_checkout_unchanged(self):
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            sync.resolve_upstream_revision(self.upstream, "missing", True)
        self.assertEqual(self.git(self.upstream, "rev-parse", "HEAD"), self.first)

    def test_patch_failure_preserves_existing_fork_and_restores_upstream(self):
        project = self.root / "project"
        target = project / sync.TARGET_REL
        target.mkdir(parents=True)
        (target / "README.md").write_text("existing fork")
        before = {p.name: p.read_bytes() for p in target.iterdir()}
        arguments = [str(project / "scripts/sync_tree_sitter_fork.py"), "--upstream",
                     str(self.upstream), "--rev", self.first, "--offline", "--apply", "--allow-dirty"]
        with patch.object(sync, "__file__", arguments[0]), patch.object(sys, "argv", arguments), \
             patch.object(sync, "patch_cargo_template", side_effect=RuntimeError("patch drift")), \
             contextlib.redirect_stdout(io.StringIO()), self.assertRaisesRegex(RuntimeError, "patch drift"):
            sync.main()
        self.assertEqual({p.name: p.read_bytes() for p in target.iterdir()}, before)
        self.assertEqual(self.git(self.upstream, "symbolic-ref", "--short", "HEAD"), "main")
        self.assertEqual(self.git(self.upstream, "rev-parse", "HEAD"), self.first)


if __name__ == "__main__":
    unittest.main()
