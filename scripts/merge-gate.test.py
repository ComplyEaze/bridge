#!/usr/bin/env python3
"""Focused offline controls for the shrunk scripts/merge-gate.sh.

The gate was cut from 3977 lines of merge-gate.sh/.test.py/_fake_gh.py/
_privacy.py/_diff.py down to only what runs as a real required CI check:
compatibility-surface validation (schema 3 and the pinned-path acknowledgement),
the privacy/PII scan, and a minimal review-evidence-names-current-head check. This suite exercises only that
kept surface, plus a dedicated regression test for each of the 7 defects
fixed in the kept code (#346, #358, #360, #365, #366, #373, #377).

The fake `gh` (merge_gate_test_gh.py) models server responses; no network or
merge operation is used.
"""
from __future__ import annotations

import contextlib
import hashlib
import importlib.util
import json
import os
import re
import shlex
import shutil
import stat
import subprocess
import tempfile
import unittest
from unittest import mock
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "scripts" / "merge-gate.sh"
FAKE_GH = ROOT / "scripts" / "merge_gate_test_gh.py"
PRIVACY_MODULE_PATH = ROOT / "scripts" / "merge_gate_privacy.py"
HEAD = "0123456789abcdef0123456789abcdef01234567"
SHORT = HEAD[:7]
ACK_DIR = "docs/tally/compatibility/acks"
ACK_PATH = ACK_DIR + "/pr-321.txt"


def load_privacy_module():
    """Load scripts/merge_gate_privacy.py fresh, bypassing sys.modules.

    A fresh load (rather than a cached import) matters here: several of the
    PrivacyScannerFindingsPR335 tests are run by hand against a deliberately
    reverted copy of the file to prove they fail on the pre-fix behavior
    described in PR #335 review, then re-run against the restored file. A
    cached import would silently keep serving the first version loaded.
    """
    spec = importlib.util.spec_from_file_location("merge_gate_privacy", PRIVACY_MODULE_PATH)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

# The real mktemp is resolved here, before the stub's directory is put on
# PATH, and written into the stub. A lookup inside the stub would need
# `command -v -p`, which macOS's bash 3.2 ignores, so the stub found itself
# and called itself forever (#1043).
MKTEMP_WRAPPER = """#!/usr/bin/env bash
# Only scan-input-write-failure pre-occupies the privacy-scan-input path (as
# a directory) so the write inside merge-gate.sh's redirect group fails.
real={real}
if [ "${GATE_SCENARIO:-}" = "scan-input-write-failure" ] && [ "$1" = "-d" ]; then
  dir=$("$real" -d)
  mkdir "$dir/privacy-scan-input"
  printf '%s\\n' "$dir"
elif [ "${GATE_SCENARIO:-}" = "manifest-write-failure" ] && [ "$1" = "-d" ]; then
  dir=$("$real" -d)
  mkdir "$dir/synthetic-uuid-manifest.json"
  printf '%s\\n' "$dir"
else
  exec "$real" "$@"
fi
"""


class MergeGateControls(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix="merge-gate-controls-")
        cls.bin = Path(cls.tmp.name)
        # The gate runs only from origin/master's tree: build a throwaway repository whose origin/master holds exactly these scripts.
        cls.repo = cls.bin / "gate-repo"
        (cls.repo / "scripts").mkdir(parents=True)
        for name in ("merge-gate.sh", "merge_gate_diff.py", "merge_gate_privacy.py", "merge_gate_verdict.py", "review-tier.mjs", "review-tier-paths.json", "check-ai-attribution.mjs"):
            shutil.copyfile(ROOT / "scripts" / name, cls.repo / "scripts" / name)
        os.chmod(cls.repo / "scripts" / "merge-gate.sh", 0o755)
        cls.origin = cls.bin / "origin.git"
        subprocess.run(["git", "init", "-q", "--bare", "-b", "master", str(cls.origin)], check=True, capture_output=True)
        cls.git_in_repo("init", "-q", "-b", "master")
        cls.git_in_repo("add", "-A")
        cls.git_in_repo("commit", "-q", "-m", "scripts")
        cls.git_in_repo("remote", "add", "origin", str(cls.origin))
        cls.git_in_repo("push", "-q", "origin", "master")
        cls.git_in_repo("fetch", "-q", "origin", "master")
        cls.script = cls.repo / "scripts" / "merge-gate.sh"
        gh = cls.bin / "gh"
        shutil.copyfile(FAKE_GH, gh)
        gh.chmod(0o755)
        real_mktemp = shutil.which("mktemp")
        if not real_mktemp:
            raise RuntimeError("mktemp is required for merge-gate controls")
        mktemp = cls.bin / "mktemp"
        mktemp.write_text(MKTEMP_WRAPPER.replace("{real}", shlex.quote(real_mktemp)))
        mktemp.chmod(mktemp.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)
        if not shutil.which("jq"):
            raise RuntimeError("jq is required for merge-gate controls")
        real_node = shutil.which("node")
        if not real_node:
            raise RuntimeError("node is required for merge-gate controls (the review tier tool)")
        node = cls.bin / "node"
        node.write_text(
            "#!/usr/bin/env bash\n"
            'case "${GATE_SCENARIO:-}" in\n'
            "  tier-tool-fails) exit 2 ;;\n"
            '  tier-tool-error-object) echo \'{"tier":"one","error":"boom","files":[],"unclassified":[]}\'; exit 0 ;;\n'
            '  tier-tool-bad-tier) echo \'{"tier":"maybe","files":[]}\'; exit 0 ;;\n'
            "esac\n"
            "exec " + shlex.quote(real_node) + ' "$@"\n'
        )
        node.chmod(node.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)
        # The pin-list retry waits between tries (#2179). With GATE_SLEEPS set, a wait is recorded there instead of slept.
        real_sleep = shutil.which("sleep")
        if not real_sleep:
            raise RuntimeError("sleep is required for merge-gate controls")
        sleep = cls.bin / "sleep"
        sleep.write_text(
            "#!/usr/bin/env bash\n"
            'if [ -n "${GATE_SLEEPS:-}" ]; then printf \'%s\\n\' "$*" >>"$GATE_SLEEPS"; exit 0; fi\n'
            "exec " + shlex.quote(real_sleep) + ' "$@"\n'
        )
        sleep.chmod(sleep.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)

    @classmethod
    def git_in_repo(cls, *args):
        subprocess.run(["git", "-C", str(cls.repo), "-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", *args],
                       check=True, capture_output=True)

    @classmethod
    @contextlib.contextmanager
    def master_state(cls, tracked=None, symlinks=None, ignored=None, hidden_edits=None, deleted=None):
        """Put files on the throwaway origin/master (and in the gate's checkout), then put it back.

        tracked: path -> text, committed. symlinks: path -> target, committed as links. ignored: path -> text, written
        into the checkout only, with a committed .gitignore naming it (git status does not show it). hidden_edits: path -> text
        written over a tracked file in the checkout and hidden from git with skip-worktree (git diff and git status do not show it).
        deleted: path -> text, committed and then removed in a later commit, so master once held it.
        """
        base = subprocess.run(["git", "-C", str(cls.repo), "rev-parse", "HEAD"], check=True, capture_output=True, text=True).stdout.strip()
        try:
            if deleted:
                for path, text in deleted.items():
                    (cls.repo / path).parent.mkdir(parents=True, exist_ok=True)
                    (cls.repo / path).write_text(text)
                cls.git_in_repo("add", "-A")
                cls.git_in_repo("commit", "-q", "-m", "held")
                cls.git_in_repo("rm", "-q", *deleted)
                cls.git_in_repo("commit", "-q", "-m", "deleted")
            for path, text in (tracked or {}).items():
                (cls.repo / path).parent.mkdir(parents=True, exist_ok=True)
                (cls.repo / path).write_text(text)
            for path, target in (symlinks or {}).items():
                os.symlink(target, cls.repo / path)
            if ignored:
                (cls.repo / ".gitignore").write_text("".join(path + "\n" for path in ignored))
            cls.git_in_repo("add", "-A")
            cls.git_in_repo("commit", "-q", "-m", "state")
            for path, text in (ignored or {}).items():
                (cls.repo / path).write_text(text)
            cls.git_in_repo("push", "-q", "-f", "origin", "master")
            for path, text in (hidden_edits or {}).items():
                cls.git_in_repo("update-index", "--skip-worktree", path)
                (cls.repo / path).write_text(text)
            yield
        finally:
            for path in (hidden_edits or {}):
                cls.git_in_repo("update-index", "--no-skip-worktree", path)
            cls.git_in_repo("reset", "-q", "--hard", base)
            cls.git_in_repo("clean", "-fdxq")
            cls.git_in_repo("push", "-q", "-f", "origin", "master")

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def run_gate(self, scenario="pass", extra_args=(), cwd=None, repo="lamemustafa/bridge", env_extra=None):
        env = os.environ.copy()
        env["PATH"] = f"{self.bin}:{env['PATH']}"
        env["GATE_SCENARIO"] = scenario
        env.update(env_extra or {})
        return subprocess.run(
            [str(self.script), "321", "--repo", repo, *extra_args],
            cwd=str(cwd) if cwd else ROOT,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )

    def assert_pass(self, scenario, phrase=None, extra_args=()):
        result = self.run_gate(scenario, extra_args)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("MAY MERGE", result.stdout)
        if phrase:
            self.assertIn(phrase, result.stdout)
        return result

    def assert_blocked(self, scenario, phrase, extra_args=()):
        result = self.run_gate(scenario, extra_args)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn(phrase, result.stdout)
        self.assertNotIn("MAY MERGE", result.stdout)
        return result

    def assert_indeterminate(self, scenario, phrase, extra_args=()):
        result = self.run_gate(scenario, extra_args)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn(phrase, result.stdout)
        self.assertNotIn("MAY MERGE", result.stdout)
        return result

    # -- Baseline -----------------------------------------------------------

    def test_the_organization_name_is_the_same_policy_as_the_old_account_name(self):
        result = self.run_gate("pass", repo="ComplyEaze/bridge")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_any_other_repository_is_refused(self):
        for repo in ("someone/bridge", "ComplyEaze/other", "complyeaze/bridge"):
            result = self.run_gate("pass", repo=repo)
            self.assertEqual(result.returncode, 2, repo + result.stdout + result.stderr)
            self.assertIn("unsupported repository", result.stderr)

    def test_pass_scenario_may_merge(self):
        self.assert_pass("pass", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    # -- Compatibility-surface schema (schema 3 at head; 2 or 3 at the base) --

    def test_surface_fetch_failure_is_indeterminate(self):
        self.assert_indeterminate("surface-fetch-fail", "could not read and validate compatibility surface at " + SHORT)

    def test_surface_malformed_is_indeterminate(self):
        self.assert_indeterminate("surface-malformed", "could not read and validate compatibility surface at " + SHORT)

    def test_head_schema_3_is_read(self):
        self.assert_pass("pass", "validated schema-3 compatibility surface at head " + SHORT)

    def test_head_schema_3_with_a_stored_hash_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema3-extra-key", "could not read and validate compatibility surface at " + SHORT)

    def test_head_schema_3_with_an_extra_top_level_key_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema3-extra-top-level-key", "could not read and validate compatibility surface at " + SHORT)

    def test_head_schema_3_with_a_non_string_reason_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema3-reason-not-a-string", "could not read and validate compatibility surface at " + SHORT)

    def test_head_schema_3_with_a_duplicate_path_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema3-duplicate-path", "could not read and validate compatibility surface at " + SHORT)

    def test_head_schema_3_with_an_empty_path_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema3-empty-path", "could not read and validate compatibility surface at " + SHORT)

    def test_head_on_schema_2_is_indeterminate_and_says_to_merge_master_and_migrate(self):
        result = self.assert_indeterminate("surface-head-schema2", "could not read and validate compatibility surface at " + SHORT)
        self.assertIn("still schema 2", result.stdout)
        self.assertIn("merge master into the branch and migrate the pin list to schema 3, see docs/release-process.md", result.stdout)

    def test_head_on_schema_1_gets_no_migration_hint(self):
        result = self.assert_indeterminate("surface-head-schema1", "could not read and validate compatibility surface at " + SHORT)
        self.assertNotIn("merge master", result.stdout)

    def test_head_on_schema_1_is_indeterminate(self):
        self.assert_indeterminate("surface-head-schema1", "could not read and validate compatibility surface at " + SHORT)

    def test_base_on_schema_2_is_read(self):
        self.assert_pass("surface-base-schema2", "validated schema-2 compatibility surface at base aaaaaaa")

    def test_base_on_schema_1_is_indeterminate(self):
        self.assert_indeterminate("surface-base-schema1", "could not read and validate compatibility surface at base aaaaaaa")

    def test_base_on_schema_3_with_a_stored_hash_is_indeterminate(self):
        self.assert_indeterminate("surface-base-schema3-extra-key", "could not read and validate compatibility surface at base aaaaaaa")

    def test_an_unreadable_merge_base_is_indeterminate(self):
        self.assert_indeterminate("merge-base-fetch-fail", "could not read the merge base")

    # -- Pinned-path acknowledgement (pr-<N>.txt) ------------------------------

    def test_ack_missing_blocks(self):
        self.assert_blocked("ack-missing", "but the PR does not add " + ACK_PATH)

    def test_ack_exact_passes(self):
        result = self.assert_pass("ack-exact", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")
        self.assertIn(ACK_PATH + " lists the 1 touched pinned path(s), reviewer reviewer", result.stdout)

    def test_ack_with_an_extra_path_blocks(self):
        self.assert_blocked("ack-extra-path", "lists path(s) the PR does not touch: src/other.rs")

    def test_ack_missing_a_touched_path_blocks(self):
        self.assert_blocked("ack-missing-path", "omits touched pinned path(s): src/other.rs")

    def test_a_modified_ack_instead_of_an_added_one_blocks(self):
        self.assert_blocked("ack-modified", "but the PR does not add " + ACK_PATH)

    def test_editing_another_acknowledgement_blocks(self):
        self.assert_blocked("ack-other-modified", "acknowledgement files are append-only")

    def test_deleting_another_acknowledgement_while_touching_a_pin_blocks(self):
        self.assert_blocked("ack-other-deleted", "acknowledgement files are append-only")

    def test_ack_with_a_hash_token_blocks(self):
        self.assert_blocked("ack-with-hash-token", "contains a 64-hex token")

    def test_ack_with_unsorted_paths_blocks(self):
        self.assert_blocked("ack-unsorted", "paths must be sorted and unique")

    def test_ack_with_a_duplicate_path_blocks(self):
        self.assert_blocked("ack-duplicate-path", "paths must be sorted and unique")

    def test_ack_without_a_reviewer_line_blocks(self):
        self.assert_blocked("ack-no-reviewer", "needs exactly one 'reviewer: <github login>' line")

    def test_ack_with_two_reviewer_lines_blocks(self):
        self.assert_blocked("ack-two-reviewers", "needs exactly one 'reviewer: <github login>' line")

    def test_ack_with_a_line_that_is_not_a_repository_path_blocks(self):
        self.assert_blocked("ack-path-not-a-repository-path", "has a line that is not a repository path")

    def test_an_unreadable_ack_is_indeterminate(self):
        self.assert_indeterminate("ack-unreadable", "could not read " + ACK_PATH)

    def test_an_ack_with_no_pinned_path_touched_blocks(self):
        self.assert_blocked("ack-without-pinned-change", "no pinned path is touched, so no acknowledgement may be added")

    def test_an_ack_cleanup_that_only_deletes_acks_passes(self):
        self.assert_pass("ack-cleanup-only", "changed files contain no pinned path requiring an acknowledgement")

    def test_unpinned_change_needs_no_ack(self):
        self.assert_pass("pass", "changed files contain no pinned path requiring an acknowledgement")

    # -- Pin list edits: an added or removed pin is a touch ------------------------

    def test_pin_added_with_the_file_unchanged_needs_an_ack(self):
        self.assert_blocked("pin-only-change", "but the PR does not add " + ACK_PATH)

    def test_pin_added_with_an_ack_and_a_reason_passes(self):
        self.assert_pass("pin-only-change-acked", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_pin_added_without_a_reason_blocks(self):
        self.assert_blocked("pin-added-without-reason", "pin(s) added without a non-empty reason: src/new.rs")

    def test_pin_removed_without_a_removed_pin_line_blocks(self):
        self.assert_blocked("pin-removed-no-line", "lacks a removed-pin line for: src/other.rs")

    def test_pin_removed_with_a_removed_pin_line_passes(self):
        self.assert_pass("pin-removed-with-line", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    # -- Branch history: a pin the branch added and then lost ----------------------

    def test_a_pin_the_branch_added_and_lost_blocks_without_a_removed_pin_line(self):
        self.assert_blocked(
            "history-own-pin-lost-undeclared",
            "pinned by a commit of this PR but not in the pin list at the head; restore the pin, or declare the withdrawal with a removed-pin line: src/own.rs",
        )

    def test_a_pin_the_branch_added_and_lost_passes_when_the_withdrawal_is_declared(self):
        self.assert_pass("history-own-pin-lost-declared", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_a_lost_pin_named_as_a_path_in_the_ack_blocks(self):
        self.assert_blocked("history-own-pin-lost-ack-names-the-path", "lists path(s) the PR does not touch: src/own.rs")

    def test_a_lost_pin_needs_an_ack_even_when_nothing_else_pinned_changed(self):
        self.assert_blocked("history-own-pin-lost-no-ack", "pinned path(s) touched")

    def test_a_pin_the_branch_added_and_kept_is_not_withdrawn(self):
        self.assert_pass("history-pin-kept-at-the-head", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_a_merge_commit_is_compared_with_all_of_its_parents(self):
        self.assert_pass("history-merge-parent-compared-with-all", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_an_out_of_order_list_in_an_older_branch_commit_passes_once_the_head_list_is_sorted(self):
        self.assert_pass("history-out-of-order-then-sorted", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_the_same_out_of_order_history_blocks_when_the_pin_is_then_dropped(self):
        self.assert_blocked(
            "history-out-of-order-then-dropped",
            "pinned by a commit of this PR but not in the pin list at the head; restore the pin, or declare the withdrawal with a removed-pin line: src/own.rs",
        )

    def test_the_same_out_of_order_history_passes_when_the_withdrawal_is_declared(self):
        self.assert_pass("history-out-of-order-then-dropped-declared", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_an_unreadable_pin_list_in_the_branch_history_is_indeterminate(self):
        result = self.assert_indeterminate("history-commit-list-unreadable", "could not read and parse the pin list at every commit of the PR")
        # A failure with no HTTP status is not tried again, and the line names the commit and the error.
        self.assertIn("(commit c1c1c1c1c1c1, after 1 try: controlled history surface read failure)", result.stdout)

    # -- A pin-list read that fails and is tried again (#2179) ----------------------

    def run_flaky(self, scenario):
        """Run a flaky-read scenario; return the result, the waits the gate asked for, and the reads of the flaky commit."""
        with tempfile.TemporaryDirectory(prefix="merge-gate-flaky-") as folder:
            state, sleeps = Path(folder) / "calls", Path(folder) / "sleeps"
            result = self.run_gate(scenario, env_extra={"GATE_STATE": str(state), "GATE_SLEEPS": str(sleeps)})
            waits = sleeps.read_text().split() if sleeps.exists() else []
            calls = int(state.read_text()) if state.exists() else 0
        return result, waits, calls

    def assert_flaky_passes(self, scenario):
        result, waits, calls = self.run_flaky(scenario)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("MAY MERGE", result.stdout)
        self.assertNotIn("INDETERMINATE", result.stdout)
        self.assertEqual((waits, calls), (["2"], 2), "one failed read, one wait of 2 s, then the read that answered")

    def assert_flaky_not_retried(self, scenario, phrase):
        result, waits, calls = self.run_flaky(scenario)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn(phrase, result.stdout)
        self.assertEqual((waits, calls), ([], 1), "a failure a second try cannot cure is not tried again")

    def test_a_server_error_on_one_pin_list_read_is_tried_again_and_passes(self):
        self.assert_flaky_passes("history-flaky-502-once")

    def test_a_secondary_rate_limit_on_one_pin_list_read_is_tried_again_and_passes(self):
        self.assert_flaky_passes("history-flaky-rate-limit-once")

    def test_a_404_for_one_of_the_prs_own_commits_is_tried_again_and_passes(self):
        self.assert_flaky_passes("history-flaky-404-own-commit-once")

    def test_three_failed_reads_stay_indeterminate_and_name_the_commit_and_the_status(self):
        result, waits, calls = self.run_flaky("history-flaky-502-always")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertNotIn("MAY MERGE", result.stdout)
        self.assertIn("could not read and parse the pin list at every commit of the PR", result.stdout)
        self.assertIn("(commit c1c1c1c1c1c1, HTTP 502, after 3 tries: gh: Server Error (HTTP 502))", result.stdout)
        self.assertEqual((waits, calls), (["2", "4"], 3), "three tries, 2 s then 4 s apart, and no fourth")

    def test_a_403_that_is_not_a_rate_limit_is_not_tried_again(self):
        self.assert_flaky_not_retried("history-flaky-403-not-a-rate-limit", "(commit c1c1c1c1c1c1, HTTP 403, after 1 try: ")

    def test_a_404_for_a_commit_outside_the_pr_is_not_tried_again(self):
        self.assert_flaky_not_retried("history-flaky-404-outside-the-pr", "(commit d2d2d2d2d2d2, HTTP 404, after 1 try: gh: Not Found (HTTP 404))")

    def test_removed_pin_line_for_a_pin_that_is_kept_blocks(self):
        self.assert_blocked("pin-removed-line-for-a-kept-pin", "has removed-pin line(s) for pin(s) the PR does not remove: src/other.rs")

    # -- The review or comment must name the head, the ack and every touched path ----

    # -- Renames, stale branches, the cut-over PR -----------------------------------

    def test_rename_of_a_pinned_file_needs_an_ack_for_the_old_path(self):
        self.assert_blocked("rename-old-path-pinned", "but the PR does not add " + ACK_PATH)

    def test_rename_of_a_pinned_file_with_an_ack_for_the_old_path_passes(self):
        result = self.assert_pass("rename-old-path-acked", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")
        self.assertIn("lists the 1 touched pinned path(s)", result.stdout)

    def test_a_nested_gitattributes_blocks(self):
        self.assert_blocked("nested-gitattributes", "a nested .gitattributes can change the bytes of a pinned file")

    def test_pr_behind_a_pin_adding_master_commit_is_not_indeterminate(self):
        # The base tip pins one more path than the head, but the PR's merge base
        # does not: nothing was removed, so nothing is compared against the tip.
        result = self.assert_pass("behind-a-pin-adding-master-commit", "validated schema-3 compatibility surface at merge base bbbbbbb")
        self.assertNotIn("INDETERMINATE", result.stdout)

    def test_pr_behind_master_that_touches_a_path_master_pinned_needs_an_ack(self):
        self.assert_blocked("behind-master-pin-touched", "but the PR does not add " + ACK_PATH)

    def test_cut_over_pr_schema_2_base_schema_3_head_passes_with_an_ack(self):
        self.assert_pass("cut-over", "validated schema-2 compatibility surface at base aaaaaaa")

    def test_cut_over_pr_without_an_ack_blocks(self):
        self.assert_blocked("cut-over-without-ack", "but the PR does not add " + ACK_PATH)

    # -- .gitattributes is pinned like any file: the gate has no special case for it ----

    def test_gitattributes_pinned_and_edited_without_an_ack_blocks(self):
        self.assert_blocked("gitattributes-no-ack", "but the PR does not add " + ACK_PATH)

    def test_gitattributes_pinned_and_edited_with_an_ack_listing_it_passes(self):
        result = self.assert_pass("gitattributes-acked", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")
        self.assertIn("lists the 1 touched pinned path(s)", result.stdout)

    def test_gitattributes_pinned_and_edited_with_an_ack_that_omits_it_blocks(self):
        self.assert_blocked("gitattributes-wrong-ack", "omits touched pinned path(s): .gitattributes")

    def test_gitattributes_edited_but_not_pinned_needs_no_ack(self):
        self.assert_pass("gitattributes-unpinned", "changed files contain no pinned path requiring an acknowledgement")

    # -- removed-pin lines are sorted and unique, like the path lines --------------------

    def test_sorted_removed_pin_lines_pass(self):
        self.assert_pass("removed-pins-sorted", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_unsorted_removed_pin_lines_block(self):
        self.assert_blocked("removed-pins-unsorted", "removed-pin lines must be sorted and unique")

    def test_duplicate_removed_pin_lines_block(self):
        self.assert_blocked("removed-pins-duplicate", "removed-pin lines must be sorted and unique")

    # -- The reviewer login is a GitHub login (the same expression as the CI checker) ---------

    def test_reviewer_login_shapes(self):
        for scenario in ("reviewer-login-39-chars", "reviewer-login-bot"):
            self.assert_pass(scenario, "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")
        for scenario in ("reviewer-login-trailing-hyphen", "reviewer-login-leading-hyphen", "reviewer-login-40-chars"):
            self.assert_blocked(scenario, "needs exactly one 'reviewer: <github login>' line")

    # -- The reason on an added pin: non-empty, at most 500 characters, no control character --

    def test_pin_reason_of_501_characters_blocks(self):
        self.assert_blocked("pin-reason-501-chars", "pin(s) added with a reason over 500 characters or containing a control character: src/new.rs")

    def test_pin_reason_of_500_astral_characters_passes(self):
        self.assert_pass("pin-reason-500-astral-chars", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_pin_reason_with_a_control_character_blocks(self):
        self.assert_blocked("pin-reason-control-char", "reason over 500 characters or containing a control character")

    # -- An ack may only be added: a rename of an ack is a delete plus an add ------------------

    def test_an_ack_renamed_out_of_the_directory_blocks_with_a_pinned_change(self):
        self.assert_blocked("ack-renamed-out-with-pin-change", "acknowledgement files are append-only; the PR also changes: " + ACK_DIR + "/pr-100.txt")

    def test_an_ack_renamed_out_of_the_directory_blocks_without_a_pinned_change(self):
        self.assert_blocked("ack-renamed-out-no-pin-change", "no pinned path is touched, so no acknowledgement may be added or changed: " + ACK_DIR + "/pr-100.txt")

    def test_an_ack_that_git_pairs_with_an_unrelated_deleted_file_is_still_an_add(self):
        self.assert_pass("ack-added-as-a-rename-from-outside", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    # -- Copies: the destination counts, the source only when it is itself pinned ----------------

    def test_a_copy_of_a_pinned_file_counts_the_pinned_source(self):
        self.assert_blocked("copy-of-a-pinned-file", "but the PR does not add " + ACK_PATH)
        result = self.assert_pass("copy-of-a-pinned-file-acked", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")
        self.assertIn("lists the 1 touched pinned path(s)", result.stdout)

    def test_a_copy_of_an_unpinned_file_needs_no_ack(self):
        self.assert_pass("copy-of-an-unpinned-file", "changed files contain no pinned path requiring an acknowledgement")

    # -- A path is named in the review or comment only as a whole token ----------------------------

    # -- A large ack: a match must not be lost when grep exits early under pipefail -------------------

    def test_a_large_ack_still_reports_an_early_hex_token(self):
        self.assert_blocked("ack-large-with-early-hex-token", "contains a 64-hex token")

    def test_a_large_ack_still_reports_an_early_control_character(self):
        self.assert_blocked("ack-large-with-early-control-char", "contains a control character")

    # -- Ordering is by UTF-8 bytes (LC_ALL=C), not UTF-16 code units --------------------------------------

    def test_ack_paths_in_utf8_byte_order_pass(self):
        self.assert_pass("ack-utf8-byte-order", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_ack_paths_in_utf16_order_block(self):
        self.assert_blocked("ack-utf8-utf16-order", "paths must be sorted and unique")

    # -- Privacy / PII scan (KEEP #2) ----------------------------------------

    def test_email_in_title_blocks(self):
        self.assert_blocked("privacy-email-blocker", "customer email shape")

    def test_known_public_agent_address_in_a_well_formed_commit_trailer_is_identity_metadata(self):
        self.assert_pass("public-agent-coauthor-trailer", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_known_public_agent_address_in_a_well_formed_crlf_commit_trailer_is_identity_metadata(self):
        self.assert_pass("public-agent-coauthor-trailer-crlf", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    # -- AI attribution is a note, never a block (owner practice of 25 Oct 2026) ------------

    def assert_noted(self, scenario, text):
        result = self.run_gate(scenario)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("MAY MERGE", result.stdout)
        self.assertNotIn("BLOCK", result.stdout)
        self.assertIn("AI attribution: " + text, result.stdout)
        return result

    def test_a_generated_with_line_in_the_title_or_body_is_a_note_in_each_form(self):
        for scenario in ("attribution-body-plain", "attribution-body-link", "attribution-body-comment", "attribution-title", "attribution-body-markdown"):
            with self.subTest(scenario=scenario):
                self.assert_noted(scenario, "")

    def test_a_coauthor_line_in_the_body_is_a_note_that_says_a_squash_copies_the_body(self):
        self.assert_noted("attribution-body-trailer", "body: a Co-authored-by line")

    def test_the_body_is_noted_even_when_the_commit_list_is_not_proven(self):
        result = self.run_gate("attribution-body-with-unproven-commits")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("AI attribution: body: a \"Generated with\" line", result.stdout)
        self.assertIn("could not prove complete head-bound PR commit metadata", result.stdout)

    def test_a_trailer_on_a_commit_whatever_its_date_is_a_note_that_says_a_squash_can_copy_it(self):
        for scenario in ("attribution-trailer-after", "attribution-trailer-before", "attribution-trailer-undated"):
            with self.subTest(scenario=scenario):
                result = self.assert_noted(scenario, "commit ")
                self.assertIn("a squash merge can copy it into the commit on master", result.stdout)

    def test_a_committer_identity_naming_an_assistant_is_a_note(self):
        self.assert_noted("attribution-identity", "commit ")

    def test_a_check_that_crashes_is_not_a_pass(self):
        # The helper exits 1 with nothing on stdout (a crash, an old node): no answer, so the gate must say it could not determine.
        copy = self.private_copy("attribution-crash")
        for source in ("process.exit(1);\n", "process.exit(0);\n", "throw new Error('boom');\n", "console.log('REFUSE x');\nconsole.log('CHECKED 1');\nprocess.exit(0);\n",
                       "console.log('NOTE x');\nconsole.log('CHECKED 0');\nprocess.exit(1);\n", "console.log('CHECKED 9');\nprocess.exit(0);\n",
                       "console.log('CHECKED 1');\nconsole.log('NOTE after');\nprocess.exit(0);\n"):
            (copy / "scripts" / "check-ai-attribution.mjs").write_text(source)
            self.git_copy(copy, "add", "-A")
            self.git_copy(copy, "commit", "-q", "-m", "a broken helper")
            self.git_copy(copy, "push", "-q", "origin", "master")
            for scenario in ("attribution-trailer-after", "pass"):
                with self.subTest(scenario=scenario, helper=source):
                    result = self.run_copy(copy, scenario)
                    self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertNotIn("MAY MERGE", result.stdout)
                    self.assertIn("could not run the AI attribution check", result.stdout)

    def test_a_human_coauthor_after_the_rule_passes_and_a_clean_pull_request_says_so(self):
        self.assert_pass("attribution-trailer-human-after", "no AI attribution in the title, body or commits")
        self.assert_pass("pass", "no AI attribution in the title, body or commits")

    def test_arbitrary_customer_address_in_a_coauthor_trailer_still_blocks(self):
        self.assert_blocked("customer-coauthor-trailer", "customer email shape")

    def test_arbitrary_customer_address_in_a_crlf_coauthor_trailer_still_blocks(self):
        self.assert_blocked("customer-coauthor-trailer-crlf", "customer email shape")

    def test_known_public_agent_address_outside_a_commit_trailer_still_blocks(self):
        self.assert_blocked("public-agent-email-in-pr-body", "customer email shape")

    def test_known_public_agent_address_in_source_payload_still_blocks(self):
        self.assert_blocked("public-agent-email-in-source-payload", "customer email shape")

    def test_spoofed_coauthor_header_still_blocks(self):
        self.assert_blocked("public-agent-spoof-header", "customer email shape")

    def test_coauthor_line_outside_the_trailer_footer_still_blocks(self):
        self.assert_blocked("public-agent-nonfooter", "customer email shape")

    def test_crlf_coauthor_line_outside_the_trailer_footer_still_blocks(self):
        self.assert_blocked("public-agent-nonfooter-crlf", "customer email shape")

    def test_mixed_line_endings_still_block(self):
        self.assert_blocked("public-agent-mixed-line-endings", "customer email shape")

    def test_malformed_coauthor_trailer_still_blocks(self):
        self.assert_blocked("public-agent-malformed-trailer", "customer email shape")

    def test_malformed_crlf_coauthor_trailer_still_blocks(self):
        self.assert_blocked("public-agent-malformed-trailer-crlf", "customer email shape")

    def test_coauthor_trailer_with_extra_payload_still_blocks(self):
        self.assert_blocked("public-agent-trailer-extra-payload", "customer email shape")

    def test_public_agent_address_in_the_author_name_still_blocks(self):
        self.assert_blocked("public-agent-email-in-author-name", "customer email shape")

    def test_customer_address_in_author_name_with_public_agent_trailer_still_blocks(self):
        self.assert_blocked("customer-email-in-author-name-with-public-agent-trailer", "customer email shape")

    def test_credential_literal_blocks(self):
        self.assert_blocked("privacy-credential-blocker", "literal credential, bearer, or API token value")

    def test_unknown_uuid_is_indeterminate(self):
        result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
        self.assertNotIn("synthetic-UUID registry:", result.stdout)

    # -- The synthetic-UUID registry: scripts/merge-gate-synthetic-uuids.json on master -----------------------------------
    REGISTRY_PATH = "scripts/merge-gate-synthetic-uuids.json"
    LISTED = "11111111-2222-3333-4444-555555555555"

    def registry(self, *uuids, **override):
        document = {"schema_version": 1, "entries": [{"uuid": value, "note": "a synthetic test value"} for value in uuids]}
        document.update(override)
        return json.dumps(document)

    def test_a_listed_uuid_no_longer_makes_the_gate_indeterminate(self):
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            self.assert_pass("privacy-uuid-indeterminate", "synthetic-UUID registry: 1 listed value(s) applied")

    def test_an_unlisted_uuid_still_does_in_the_same_pull_request(self):
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            result = self.assert_indeterminate("privacy-two-uuids", "privacy scan found 1 non-nil UUID shape(s) not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry: 1 listed value(s) applied", result.stdout)

    def test_a_listed_uuid_in_credential_context_still_blocks(self):
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            self.assert_blocked("privacy-uuid-credential-context", "in credential, session, token, or bearer context")

    def test_a_listed_uuid_is_not_reported_as_a_hit_when_something_else_trips_the_scan(self):
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            result = self.assert_blocked("privacy-uuid-and-digit-run", "unexplained long digit run")
        self.assertIn("at docs/example.md:2: privacy scan found 0 identifier shape(s) and 1 unexplained long digit run(s)", result.stdout)
        self.assertNotIn("docs/example.md:1:", result.stdout)

    def test_an_invalid_registry_is_not_applied_and_says_so(self):
        entries = lambda *items: json.dumps({"schema_version": 1, "entries": list(items)})
        good = {"uuid": self.LISTED, "note": "a synthetic test value"}
        for name, text in (
            ("not json", "{"),
            ("wrong schema version", self.registry(self.LISTED, schema_version=2)),
            ("an extra top-level key", self.registry(self.LISTED, extra=1)),
            ("entries not a list", json.dumps({"schema_version": 1, "entries": {}})),
            ("a duplicate", entries(good, good)),
            ("an upper-case value", entries({"uuid": "AAAAAAAA-2222-3333-4444-555555555555", "note": "n"})),
            ("the nil value", entries({"uuid": "00000000-0000-0000-0000-000000000000", "note": "n"})),
            ("a trailing newline in the value", entries({"uuid": self.LISTED + "\n", "note": "n"})),
            ("a missing note", entries({"uuid": self.LISTED})),
            ("an empty note", entries({"uuid": self.LISTED, "note": " "})),
            ("an extra entry key", entries({"uuid": self.LISTED, "note": "n", "kind": "x"})),
            ("more than 500 entries", entries(*[{"uuid": "%08x-2222-3333-4444-555555555555" % number, "note": "n"} for number in range(501)])),
        ):
            with self.subTest(case=name), self.master_state(tracked={self.REGISTRY_PATH: text}):
                result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
                self.assertIn("synthetic-UUID registry not applied", result.stdout)

    def test_a_registry_that_is_a_symbolic_link_is_not_applied(self):
        with self.master_state(tracked={"scripts/real-registry.json": self.registry(self.LISTED)}, symlinks={self.REGISTRY_PATH: "real-registry.json"}):
            result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry not applied", result.stdout)

    def test_a_registry_that_is_only_valid_once_a_nul_byte_is_dropped_is_not_applied(self):
        # bash drops a NUL byte when a command's output is captured into a variable, so the check reads the bytes through a pipe.
        text = self.registry(self.LISTED).replace("a synthetic test value", "a synthetic\0 test value")
        self.assertIn("\0", text)
        with self.master_state(tracked={self.REGISTRY_PATH: text}):
            result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry not applied: it is not valid", result.stdout)

    def test_the_registry_on_master_is_used_not_a_different_file_in_the_checkout(self):
        # skip-worktree hides a local edit from git diff and git status, so the gate's own-tree check passes with it present.
        other = "66666666-7777-8888-9999-aaaaaaaaaaaa"
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(other)},
                               hidden_edits={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry: 1 listed value(s) applied", result.stdout)

    def test_a_link_whose_target_text_is_itself_a_valid_registry_is_not_applied(self):
        # A link's content is its target path. Without the regular-file check that text would be read as the registry.
        text = self.registry(self.LISTED)
        with self.master_state(symlinks={self.REGISTRY_PATH: text}):
            result = self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry not applied: it is not a regular file", result.stdout)

    def test_a_registry_file_only_in_the_checkout_is_not_read(self):
        # An ignored file is invisible to git status, so the gate's own-tree check passes with it present; the registry
        # is read from master's ref, not from the checkout.
        with self.master_state(ignored={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            self.assert_indeterminate("privacy-uuid-indeterminate", "not listed in the synthetic-UUID registry")

    def test_a_manifest_that_cannot_be_written_is_not_passed_to_the_scan(self):
        with self.master_state(tracked={self.REGISTRY_PATH: self.registry(self.LISTED)}):
            result = self.assert_indeterminate("manifest-write-failure", "not listed in the synthetic-UUID registry")
            self.assertIn("synthetic-UUID registry not applied", result.stdout)
            # The scan was not handed an unwritten list: it did not try to read one.
            self.assertNotIn("could not read exact current-head fixture UUID provenance", result.stdout)

    def test_binary_addition_without_attestation_blocks_and_names_the_path(self):
        result = self.assert_blocked("binary-missing-attestation", "require the owner's own --binary-review-sha attestation")
        self.assertIn("first 8 at most: docs/new.png", result.stdout)

    def test_a_privacy_hit_in_added_text_is_located_by_file_and_new_line_without_its_value(self):
        result = self.assert_blocked("digit-run-in-payload", "unexplained long digit run")
        self.assertIn("at docs/example.md:5: privacy scan found 0 identifier shape(s) and 1 unexplained long digit run(s)", result.stdout)
        self.assertNotIn("12345" + "678901", result.stdout + result.stderr)
        self.assertNotIn("harmless", result.stdout)

    def test_a_privacy_hit_in_pr_metadata_is_located_by_source_and_line(self):
        result = self.assert_blocked("privacy-email-blocker", "customer email shape")
        self.assertIn("at pr-title:1: privacy scan found 1 customer email shape(s)", result.stdout)
        self.assertNotIn("company" + ".test", result.stdout)

    def test_a_merge_commit_conflict_comment_block_after_the_trailer_is_not_a_customer_address(self):
        self.assert_pass("merge-commit-conflict-comment-block", "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head")

    def test_an_address_inside_the_conflict_comment_block_still_blocks(self):
        result = self.assert_blocked("merge-commit-conflict-block-customer-address", "customer email shape")
        self.assertIn("at commit-message-1:", result.stdout)
        self.assertNotIn("company" + ".test", result.stdout)

    def test_binary_addition_with_attestation_passes(self):
        self.assert_pass(
            "binary-with-attestation",
            "have the owner's current-head binary review attestation",
            extra_args=("--binary-review-sha", HEAD),
        )

    # -- The lab-capture manifest: scripts/merge-gate-lab-captures.json on master ---------------------------------------
    LAB_PATH = "scripts/merge-gate-lab-captures.json"
    LAB_BLOB = "1e0627036228d2a3032c1d66e96b9e997bea34af"
    LAB_BLOB_OFF = "1e0627036228d2a3032c1d66e96b9e997bea34ae"
    OTHER_BLOB = "9f2b1c4e7a6d03581e4c2b7a9d0f3e5c6b8a1d24"
    ATTEST = "1 binary addition/change(s) require the owner's own --binary-review-sha attestation"
    LAB_BYTES = b"lab capture bytes\n"
    LAB_SHA256 = hashlib.sha256(LAB_BYTES).hexdigest()
    LISTED_OK = "1 binary capture(s) are listed, path and blob, in the owner-approved lab-capture manifest on master, and their size and sha256 match: docs/new.png"
    LAB_NOT_VALID = "lab-capture manifest not applied: it is not valid"

    def lab_entry(self, path="docs/new.png", blob=None, **override):
        entry = {"blob": blob or self.LAB_BLOB, "capture_id": "sitting-1/answer-1", "company": "Lab Co", "lab_ref": "lab/sitting-1@abc",
                 "path": path, "seed_basis": "built from scratch in the lab", "sha256": self.LAB_SHA256, "size": len(self.LAB_BYTES)}
        entry.update(override)
        return entry

    def lab_manifest(self, *entries, **override):
        document = {"schema_version": 1, "entries": list(entries)}
        document.update(override)
        return json.dumps(document)

    def test_a_listed_binary_needs_no_attestation(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn("lab-capture manifest: 1 listed capture(s) (blob ", result.stdout)
        self.assertNotIn("--binary-review-sha attestation", result.stdout)

    # -- Stale manifest entries: a warning, never a block ---------------------------------------------------------------
    STALE = "lab-capture manifest: stale entry"
    PENDING = "entr(ies) not yet on master (pending)"

    def test_an_entry_whose_path_master_never_held_is_only_counted_as_pending(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn("lab-capture manifest: 1 " + self.PENDING, result.stdout)
        self.assertNotIn(self.STALE, result.stdout)

    def test_an_entry_whose_file_master_holds_at_the_listed_blob_is_neither_stale_nor_pending(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="docs/kept.bin"))
        with self.master_state(tracked={self.LAB_PATH: manifest, "docs/kept.bin": self.LAB_BYTES.decode()}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertNotIn(self.STALE, result.stdout)
        self.assertIn("lab-capture manifest: 1 " + self.PENDING, result.stdout)

    def test_an_entry_whose_path_master_holds_with_another_blob_is_stale_but_still_passes(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="docs/changed.bin"))
        with self.master_state(tracked={self.LAB_PATH: manifest, "docs/changed.bin": "other bytes\n"}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn(self.STALE + " 'docs/changed.bin': master holds another blob there", result.stdout)

    def test_an_entry_whose_path_is_a_directory_on_master_is_stale(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="docs/dir"))
        with self.master_state(tracked={self.LAB_PATH: manifest, "docs/dir/inner.txt": "x\n"}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn(self.STALE + " 'docs/dir': master holds another blob there", result.stdout)

    def test_an_entry_whose_path_master_deleted_is_stale_and_names_the_commit(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="docs/gone.bin"))
        with self.master_state(tracked={self.LAB_PATH: manifest}, deleted={"docs/gone.bin": "gone\n"}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertRegex(result.stdout, re.escape(self.STALE + " 'docs/gone.bin': master deleted it at ") + "[0-9a-f]{7,}")
        self.assertIn("lab-capture manifest: 1 " + self.PENDING, result.stdout)

    def test_a_path_with_pathspec_magic_is_read_literally(self):
        # ":(glob)docs/*" would match every docs file as a pathspec; read literally it names no file master ever held
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path=":(glob)docs/*"))
        with self.master_state(tracked={self.LAB_PATH: manifest, "docs/kept.bin": "x\n"}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertNotIn(self.STALE, result.stdout)
        self.assertIn("lab-capture manifest: 2 " + self.PENDING, result.stdout)

    def test_a_stale_entry_with_a_home_shaped_path_is_named_redacted(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="Users/someone/cap.bin"))
        with self.master_state(tracked={self.LAB_PATH: manifest, "Users/someone/cap.bin": "other\n"}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn(self.STALE + " '<home>/cap.bin'", result.stdout)
        self.assertNotIn("someone", result.stdout)

    COULD_NOT_RUN = "lab-capture manifest: the stale-entry check could not run (git could not look up an entry's path on master)"

    def test_a_git_failure_on_the_first_entry_says_the_check_could_not_run_and_changes_nothing_else(self):
        # git refuses a path outside the repository (exit 128); the schema allows "docs/../../x" because it does not start with ".."
        manifest = self.lab_manifest(self.lab_entry(path="docs/../../outside.bin"), self.lab_entry())
        with self.master_state(tracked={self.LAB_PATH: manifest}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn(self.COULD_NOT_RUN, result.stdout)
        self.assertNotIn(self.PENDING, result.stdout)
        self.assertNotIn(self.STALE, result.stdout)

    def test_a_git_failure_after_some_entries_says_the_check_stopped_partway(self):
        manifest = self.lab_manifest(self.lab_entry(), self.lab_entry(path="docs/../../outside.bin"))
        with self.master_state(tracked={self.LAB_PATH: manifest}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn("lab-capture manifest: the stale-entry check stopped partway, after 1 of 2 entries", result.stdout)
        self.assertNotIn(self.COULD_NOT_RUN, result.stdout)

    def test_a_current_entry_counts_as_checked_before_a_later_git_failure(self):
        # the first entry is current (master holds it at the listed blob); the failure comes after it
        manifest = self.lab_manifest(self.lab_entry(path="docs/kept.bin"), self.lab_entry(path="docs/../../outside.bin"), self.lab_entry())
        with self.master_state(tracked={self.LAB_PATH: manifest, "docs/kept.bin": self.LAB_BYTES.decode()}):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertIn("lab-capture manifest: the stale-entry check stopped partway, after 1 of 3 entries", result.stdout)
        self.assertNotIn(self.COULD_NOT_RUN, result.stdout)

    def run_with_failing_git(self, subcommand, manifest):
        # a git stub that fails only the stale check's own lookups (git --literal-pathspecs ... <subcommand>); every other call is real
        real_git = shutil.which("git")
        with tempfile.TemporaryDirectory() as stub_dir:
            stub = Path(stub_dir) / "git"
            stub.write_text("#!/bin/sh\n"
                            "case \" $* \" in *\" --literal-pathspecs \"*\" %s \"*) echo 'stub: refused' >&2; exit 128 ;; esac\n"
                            "exec %s \"$@\"\n" % (subcommand, shlex.quote(real_git)))
            stub.chmod(0o755)
            old_path = os.environ["PATH"]
            os.environ["PATH"] = stub_dir + os.pathsep + old_path
            try:
                with self.master_state(tracked={self.LAB_PATH: manifest}):
                    return self.assert_pass("binary-missing-attestation", self.LISTED_OK)
            finally:
                os.environ["PATH"] = old_path

    def test_a_failing_history_lookup_is_not_counted_as_pending(self):
        result = self.run_with_failing_git("log", self.lab_manifest(self.lab_entry()))
        self.assertIn(self.COULD_NOT_RUN, result.stdout)
        self.assertNotIn(self.PENDING, result.stdout)

    def test_a_failing_tree_lookup_is_not_counted_as_pending(self):
        result = self.run_with_failing_git("ls-tree", self.lab_manifest(self.lab_entry()))
        self.assertIn(self.COULD_NOT_RUN, result.stdout)
        self.assertNotIn(self.PENDING, result.stdout)

    def test_more_than_eight_stale_entries_are_named_eight_times_and_counted(self):
        paths = ["docs/s%d.bin" % i for i in range(10)]
        manifest = self.lab_manifest(self.lab_entry(), *[self.lab_entry(path=p) for p in paths])
        with self.master_state(tracked=dict({self.LAB_PATH: manifest}, **{p: "other\n" for p in paths})):
            result = self.assert_pass("binary-missing-attestation", self.LISTED_OK)
        self.assertEqual(result.stdout.count(self.STALE + " '"), 8)
        self.assertIn("lab-capture manifest: 10 stale entries in all (eight named above)", result.stdout)

    def test_a_listed_binary_in_a_renamed_file_is_judged_by_its_new_path(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            self.assert_pass("binary-renamed", self.LISTED_OK)
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(path="docs/old.png"))}):
            self.assert_blocked("binary-renamed", self.ATTEST)

    def test_a_blob_one_character_different_is_not_listed(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-one-char-off", self.ATTEST)
        self.assertIn("first 8 at most: docs/new.png", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_listed_blob_at_a_different_path_is_not_listed(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-other-path", self.ATTEST)
        self.assertIn("first 8 at most: docs/other.png", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_pointer_to_a_lab_branch_and_commit_in_the_pull_request_body_is_ignored(self):
        # Nothing reads a lab branch at gate time; only the manifest on master lists a binary, and it does not list this one.
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest()}):
            self.assert_blocked("binary-body-pointer", self.ATTEST)

    def test_a_pull_requests_own_manifest_edit_does_not_apply_to_its_own_run(self):
        # master lists nothing; the checkout (and so any pull request's branch) lists the capture
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest()},
                               hidden_edits={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            self.assert_blocked("binary-and-manifest-edit", self.ATTEST)
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest()},
                               hidden_edits={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-and-manifest-edit", "the merge is the owner's own act")
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_pull_request_that_edits_the_manifest_is_the_owners_own_merge(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest()}):
            self.assert_blocked("changes-the-lab-manifest", "the merge is the owner's own act")

    def test_a_force_moved_or_deleted_lab_branch_does_not_matter_because_bytes_not_branches_are_listed(self):
        # The owner listed the bytes (path and blob) when he merged the manifest; the gate never resolves lab_ref, so a lab
        # branch that is moved to other history, or deleted, changes nothing.
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(lab_ref="lab/sitting-1@" + "a" * 40))}):
            self.git_in_repo("push", "-q", "-f", "origin", "HEAD:refs/heads/lab/sitting-1")
            self.git_in_repo("commit", "-q", "--allow-empty", "-m", "other history")
            self.git_in_repo("push", "-q", "-f", "origin", "HEAD:refs/heads/lab/sitting-1")
            self.assert_pass("binary-missing-attestation", self.LISTED_OK)
            self.git_in_repo("push", "-q", "origin", "--delete", "lab/sitting-1")
            self.assert_pass("binary-missing-attestation", self.LISTED_OK)

    def test_a_blob_that_exists_only_locally_and_is_not_in_the_manifest_is_not_listed(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(path="docs/other.png", blob=self.OTHER_BLOB))}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
        self.assertIn("first 8 at most: docs/new.png", result.stdout)
        self.assertIn("lab-capture manifest: 1 listed capture(s)", result.stdout)

    def test_one_listed_and_one_unlisted_binary_block_and_name_the_unlisted_one(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-listed-and-unlisted", self.ATTEST)
        self.assertIn(self.LISTED_OK, result.stdout)
        self.assertIn("first 8 at most: docs/extra.bin)", result.stdout)
        self.assertNotIn("first 8 at most: docs/new.png", result.stdout)

    def test_the_unlisted_one_can_still_be_attested_beside_a_listed_one(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_pass("binary-listed-and-unlisted", self.LISTED_OK, extra_args=("--binary-review-sha", HEAD))
        self.assertIn("1 binary addition/change(s) have the owner's current-head binary review attestation", result.stdout)

    def test_a_removed_binary_is_never_listed_by_the_manifest(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-removed", self.ATTEST)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_missing_or_malformed_rest_blob_id_lists_nothing(self):
        for scenario in ("binary-no-rest-sha", "binary-malformed-rest-sha"):
            with self.subTest(scenario=scenario), self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
                result = self.assert_blocked(scenario, self.ATTEST)
                self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_listed_entry_whose_sha256_differs_from_the_blobs_bytes_is_not_exempt(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(sha256="0" * 64))}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
        self.assertIn("listed capture 'docs/new.png' not exempt: its bytes differ from the manifest (the sha256 differs)", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_listed_entry_whose_size_differs_from_the_blobs_bytes_is_not_exempt(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(size=len(self.LAB_BYTES) + 1))}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
        self.assertIn("not exempt: its bytes differ from the manifest (the size differs)", result.stdout)

    def test_a_blob_answer_whose_bytes_are_not_that_blob_exempts_nothing_even_if_the_manifest_records_them(self):
        # the answer's bytes match the entry's size and sha256, but do not hash to the blob id that was asked for
        other = b"other capture bytes\n"
        entry = self.lab_entry(size=len(other), sha256=hashlib.sha256(other).hexdigest())
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(entry)}):
            result = self.assert_blocked("binary-blob-not-that-blob", self.ATTEST)
        self.assertIn("listed capture 'docs/new.png' not exempt: could not check its bytes (the blob answer does not hash to its blob id)", result.stdout)
        self.assertNotIn("bytes differ", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_failed_blob_request_is_told_apart_from_differing_bytes_and_exempts_nothing(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-blob-fails", self.ATTEST)
        self.assertIn("listed capture 'docs/new.png' not exempt: could not check its bytes (the blob request failed)", result.stdout)
        self.assertNotIn("bytes differ", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_blob_answer_that_is_malformed_not_base64_or_has_a_wrong_size_field_exempts_nothing(self):
        for scenario in ("binary-blob-malformed", "binary-blob-not-base64", "binary-blob-size-field-differs"):
            with self.subTest(scenario=scenario), self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
                result = self.assert_blocked(scenario, self.ATTEST)
                self.assertIn("not exempt: could not check its bytes (the blob answer was malformed)", result.stdout)
                self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_path_that_looks_like_a_number_is_compared_as_text(self):
        # awk compares number-like strings as numbers: "100" would equal "1e2"
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(path="1e2"))}):
            self.assert_pass("binary-numeric-path", "1 binary capture(s) are listed")
            result = self.assert_blocked("binary-numeric-path-lookalike", self.ATTEST)
        self.assertIn("first 8 at most: 100)", result.stdout)
        self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_an_empty_manifest_lists_nothing(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest()}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
        self.assertIn("lab-capture manifest: 0 listed capture(s)", result.stdout)

    def test_no_manifest_on_master_lists_nothing_and_says_nothing(self):
        result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
        self.assertNotIn("lab-capture manifest", result.stdout)

    def test_an_invalid_manifest_is_not_applied_and_says_so(self):
        good = self.lab_entry()
        without = lambda key: {k: v for k, v in good.items() if k != key}
        cases = [
            ("not an object", json.dumps([good])),
            ("wrong schema version", self.lab_manifest(good, schema_version=2)),
            ("an extra top-level key", self.lab_manifest(good, extra=1)),
            ("entries not an array", json.dumps({"schema_version": 1, "entries": {}})),
            ("an entry that is not an object", self.lab_manifest("docs/new.png")),
            ("an extra entry key", self.lab_manifest(self.lab_entry(extra="x"))),
            ("a missing entry key", self.lab_manifest(without("seed_basis"))),
            ("an empty path", self.lab_manifest(self.lab_entry(path=""))),
            ("a path with a control character", self.lab_manifest(self.lab_entry(path="docs/new\u0001.png"))),
            ("a path with a C1 control", self.lab_manifest(self.lab_entry(path="docs/new\u0085.png"))),
            ("a path with a zero-width space", self.lab_manifest(self.lab_entry(path="docs/new\u200b.png"))),
            ("a path with a right-to-left mark", self.lab_manifest(self.lab_entry(path="docs/new\u200f.png"))),
            ("a path with a bidi override", self.lab_manifest(self.lab_entry(path="docs/new\u202e.png"))),
            ("a path with a bidi isolate", self.lab_manifest(self.lab_entry(path="docs/new\u2066.png"))),
            ("a path with a byte-order mark", self.lab_manifest(self.lab_entry(path="docs/new\ufeff.png"))),
            ("a path with a newline", self.lab_manifest(self.lab_entry(path="docs/new.png\n"))),
            ("an absolute path", self.lab_manifest(self.lab_entry(path="/docs/new.png"))),
            ("a path that climbs", self.lab_manifest(self.lab_entry(path="../docs/new.png"))),
            ("a path that is not a string", self.lab_manifest(self.lab_entry(path=5))),
            ("an uppercase blob", self.lab_manifest(self.lab_entry(blob=self.LAB_BLOB.upper()))),
            ("a short blob", self.lab_manifest(self.lab_entry(blob=self.LAB_BLOB[:39]))),
            ("a blob with a trailing newline", self.lab_manifest(self.lab_entry(blob=self.LAB_BLOB + "\n"))),
            ("an uppercase sha256", self.lab_manifest(self.lab_entry(sha256="E" * 64))),
            ("a sha256 with a trailing newline", self.lab_manifest(self.lab_entry(sha256="e" * 64 + "\n"))),
            ("a short sha256", self.lab_manifest(self.lab_entry(sha256="e" * 63))),
            ("a negative size", self.lab_manifest(self.lab_entry(size=-1))),
            ("a fractional size", self.lab_manifest(self.lab_entry(size=1.5))),
            ("a size that is text", self.lab_manifest(self.lab_entry(size="3"))),
            ("a blank capture_id", self.lab_manifest(self.lab_entry(capture_id="  "))),
            ("a blank company", self.lab_manifest(self.lab_entry(company=""))),
            ("a blank lab_ref", self.lab_manifest(self.lab_entry(lab_ref=" "))),
            ("a blank seed_basis", self.lab_manifest(self.lab_entry(seed_basis=""))),
            ("a company that is not text", self.lab_manifest(self.lab_entry(company=None))),
            ("a duplicate path", self.lab_manifest(self.lab_entry(), self.lab_entry(blob=self.OTHER_BLOB))),
            ("over 1000 entries", self.lab_manifest(*[self.lab_entry(path=f"docs/{n}.png") for n in range(1001)])),
            ("not JSON", "{"),
        ]
        for name, text in cases:
            with self.subTest(case=name), self.master_state(tracked={self.LAB_PATH: text}):
                result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
                self.assertIn(self.LAB_NOT_VALID, result.stdout)
                self.assertNotIn("binary capture(s) are listed", result.stdout)

    def test_a_manifest_that_is_a_symbolic_link_is_not_applied(self):
        with self.master_state(tracked={"scripts/real-captures.json": self.lab_manifest(self.lab_entry())}, symlinks={self.LAB_PATH: "real-captures.json"}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
            self.assertIn("lab-capture manifest not applied: it is not a regular file", result.stdout)
        # a link whose target text is itself a valid manifest
        with self.master_state(symlinks={self.LAB_PATH: self.lab_manifest(self.lab_entry())}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
            self.assertIn("lab-capture manifest not applied: it is not a regular file", result.stdout)

    def test_a_manifest_that_is_only_valid_once_a_nul_byte_is_dropped_is_not_applied(self):
        # A shell variable would drop the NUL and read this as valid; the bytes go straight into jq instead.
        text = self.lab_manifest(self.lab_entry()).replace("built from scratch", "built\0 from scratch")
        self.assertIn("\0", text)
        with self.master_state(tracked={self.LAB_PATH: text}):
            result = self.assert_blocked("binary-missing-attestation", self.ATTEST)
            self.assertIn(self.LAB_NOT_VALID, result.stdout)

    def test_the_manifest_on_master_is_used_not_a_different_file_in_the_checkout(self):
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())},
                               hidden_edits={self.LAB_PATH: self.lab_manifest()}):
            self.assert_pass("binary-missing-attestation", self.LISTED_OK)

    # -- The owner's merge verdict (spec v3) ------------------------------------

    VERDICT_PASS = "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head"
    VERDICT_BLOCK = "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND do not both stand"

    def test_the_owners_two_lines_as_comments_or_reviews_pass(self):
        self.assert_pass("pass", self.VERDICT_PASS)
        self.assert_pass("verdict-by-review", self.VERDICT_PASS)

    def test_either_line_missing_or_for_another_merge_base_blocks(self):
        for scenario in ("verdict-missing-clean", "verdict-missing-recommend-write-path", "verdict-for-another-base"):
            with self.subTest(scenario=scenario):
                self.assert_blocked(scenario, self.VERDICT_BLOCK)

    def test_a_later_not_clean_an_owner_edit_or_a_deleted_owner_comment_voids_the_lines(self):
        for scenario in ("verdict-then-not-clean", "verdict-then-owner-edit", "verdict-then-owner-comment-deleted", "owner-edit-by-a-renamed-account"):
            with self.subTest(scenario=scenario):
                self.assert_blocked(scenario, self.VERDICT_BLOCK)

    def test_unreadable_or_incomplete_history_is_indeterminate_never_a_pass(self):
        for scenario in ("history-unreadable", "history-has-another-page", "reviews-unreadable", "comments-unreadable", "inline-unreadable"):
            with self.subTest(scenario=scenario):
                self.assert_indeterminate(scenario, "could not")

    def test_a_token_in_the_title_body_added_lines_or_a_strangers_comment_is_refused(self):
        for scenario in ("token-in-a-previous-file-name", "token-in-the-title", "token-in-the-body", "token-in-an-added-line", "token-in-a-stranger-comment", "token-in-the-branch-name",
                         "token-in-a-file-name", "token-in-a-hidden-comment-in-the-body", "token-in-a-stranger-commit-comment"):
            with self.subTest(scenario=scenario):
                self.assert_blocked(scenario, "carries a merge-verdict token")

    def test_a_change_of_the_one_review_tier_needs_only_the_clean_line(self):
        result = self.assert_pass("verdict-missing-recommend", "the owner's MERGE-VERDICT CLEAN stands at head")
        self.assertIn("review tier: one", result.stdout)
        self.assertIn("MERGE-ADVERSARY RECOMMEND is not required", result.stdout)

    def test_a_change_that_is_not_of_the_one_tier_or_whose_tier_cannot_be_computed_still_needs_both_lines(self):
        for scenario in ("verdict-missing-recommend-write-path", "tier-unclassified-clean-only", "tier-rename-from-write-path", "tier-tool-fails",
                         "tier-tool-error-object", "tier-tool-bad-tier"):
            with self.subTest(scenario=scenario):
                result = self.assert_blocked(scenario, self.VERDICT_BLOCK)
                self.assertIn("review tier: full", result.stdout)
        self.assertIn("src-tauri/src/agent_lab_new.rs (write-path)", self.run_gate("verdict-missing-recommend-write-path").stdout)
        self.assertIn("mystery/unknown.txt (unclassified)", self.run_gate("tier-unclassified-clean-only").stdout)

    # -- (c) at the gate: the clean line of an older head, carried when the pull request's own patch is unchanged --------------
    def test_a_clean_line_for_an_older_head_is_carried_when_the_patch_is_unchanged(self):
        result = self.assert_pass("carry-same-patch", "carried")
        self.assertIn("byte-identical", result.stdout)

    def test_a_change_of_the_text_after_the_hunk_numbers_alone_still_carries_the_clean_line(self):
        self.assert_pass("carry-function-context-differs", "carried")

    def test_a_clean_line_is_not_carried_when_the_patch_changed_or_cannot_be_compared(self):
        for scenario in ("carry-patch-differs", "carry-whitespace-only", "carry-indent-only", "carry-mode-change", "carry-binary", "carry-compare-fails",
                         "carry-context-differs", "carry-hunk-numbers-differ", "carry-moved-insertion",
                         "carry-empty-diff", "carry-duplicate-file-sections", "carry-diff-cut-short"):
            with self.subTest(scenario=scenario):
                result = self.assert_blocked(scenario, self.VERDICT_BLOCK)
                self.assertNotIn("carried", result.stdout.replace("not carried", ""))

    # A pull request whose binaries are all in the lab-capture manifest: the compare diffs hide the bytes, so the gate compares the
    # full blob id of each binary, read from the compare JSON of each range.
    def listed_binary_state(self):
        return self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry())})

    def assert_not_carried(self, result):
        self.assertIn("is not carried", result.stdout)
        self.assertNotIn("is carried to head", result.stdout)

    def test_a_clean_line_is_carried_when_every_binary_is_listed_and_its_blob_is_the_same_in_both_ranges(self):
        for scenario in ("carry-listed-binary", "carry-listed-binary-patch-body", "carry-listed-binary-299-files"):
            with self.subTest(scenario=scenario), self.listed_binary_state():
                result = self.assert_pass(scenario, "is carried to head")
                self.assertIn("with 1 manifest-listed binary blob(s) unchanged", result.stdout)
                self.assertIn(self.LISTED_OK, result.stdout)
                self.assertNotIn("--binary-review-sha attestation", result.stdout)

    def test_a_clean_line_is_not_carried_when_a_listed_binary_has_another_blob_in_the_older_range(self):
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-blob-differs", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn(self.LISTED_OK, result.stdout)

    def test_a_clean_line_is_not_carried_for_an_unlisted_binary_even_with_the_attestation(self):
        # no manifest on master: the owner's attestation lets the gate itself pass, but never the carry
        result = self.assert_blocked("carry-listed-binary", self.VERDICT_BLOCK, extra_args=("--binary-review-sha", HEAD))
        self.assert_not_carried(result)
        with self.master_state(tracked={self.LAB_PATH: self.lab_manifest(self.lab_entry(path="docs/other.png"))}):
            result = self.assert_blocked("carry-listed-binary", self.VERDICT_BLOCK, extra_args=("--binary-review-sha", HEAD))
        self.assert_not_carried(result)
        # without the attestation the binary itself is what blocks
        result = self.assert_blocked("carry-listed-binary", self.ATTEST)
        self.assert_not_carried(result)

    def test_a_clean_line_is_not_carried_when_a_listed_binary_was_renamed(self):
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-renamed", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn(self.LISTED_OK, result.stdout)

    def test_a_clean_line_is_not_carried_when_the_compare_json_cannot_prove_the_blob(self):
        reasons = {
            "carry-listed-binary-json-fails": "the compare JSON request failed",
            "carry-listed-binary-truncated": "lists 300 or more files",
            "carry-listed-binary-path-missing": "is missing from the compare JSON or appears twice in it",
            "carry-listed-binary-path-missing-new": "is missing from the compare JSON or appears twice in it",
            "carry-listed-binary-path-twice": "is missing from the compare JSON or appears twice in it",
            "carry-listed-binary-uppercase-sha": "is not 40 lowercase hex",
            "carry-listed-binary-short-sha": "is not 40 lowercase hex",
            "carry-listed-binary-no-files-list": "the binary evidence could not be read",
        }
        for scenario, reason in reasons.items():
            with self.subTest(scenario=scenario), self.listed_binary_state():
                result = self.assert_blocked(scenario, self.VERDICT_BLOCK)
                self.assert_not_carried(result)
                self.assertIn("verdict not carried: ", result.stdout)
                self.assertIn(reason, result.stdout)

    # Only an ADDED binary is carried: the kept `new file mode` line binds an empty base, whereas the base blob of a modified binary is
    # only in the stripped `index` line, so B0 -> B1 and B2 -> B1 would hash alike.
    def test_a_clean_line_is_not_carried_for_a_modified_listed_binary_whatever_its_base(self):
        for scenario in ("carry-listed-binary-modified", "carry-listed-binary-modified-same-base"):
            with self.subTest(scenario=scenario), self.listed_binary_state():
                result = self.assert_blocked(scenario, self.VERDICT_BLOCK)
                self.assertIn(self.LISTED_OK, result.stdout)
                self.assert_not_carried(result)
                self.assertIn("verdict not carried: a manifest-exempted binary is not a plain addition", result.stdout)

    # One test per check of the carry, each set up so that only that check stands between the scenario and a carry.
    def test_the_binary_paths_of_the_diff_must_be_exactly_the_exempted_ones(self):
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-extra-in-compare", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn("verdict not carried: the binary paths of the diff are not exactly the manifest-exempted paths", result.stdout)

    def test_every_binary_of_the_pull_request_must_be_exempted_by_the_manifest(self):
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-and-unlisted", self.VERDICT_BLOCK, extra_args=("--binary-review-sha", HEAD))
        self.assert_not_carried(result)
        self.assertIn("verdict not carried: a binary change of the pull request is not exempted by the lab-capture manifest", result.stdout)

    def test_an_exempted_binary_that_is_not_a_plain_addition_in_the_files_list_is_not_carried(self):
        # the files API calls it renamed, the compare JSON (wrongly) added: only the gate's own record of the exemption is left to refuse
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-renamed-json-added", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn("verdict not carried: a manifest-exempted binary is not a plain addition (modified, renamed or copied)", result.stdout)

    def test_a_binary_that_is_not_an_addition_in_the_compare_json_is_not_carried(self):
        # the files API says added, the compare JSON modified: only the status check of the carry is left to refuse
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-json-modified", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn("verdict not carried: a binary file is not a plain addition in the compare JSON", result.stdout)

    def test_the_compare_json_blob_must_be_the_blob_the_manifest_check_verified(self):
        # the same other blob in both ranges, so the digests agree: only the binding to the verified blob is left to refuse
        with self.listed_binary_state():
            result = self.assert_blocked("carry-listed-binary-json-differs-from-files", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertIn("verdict not carried: a binary blob id in the compare JSON is not the blob the manifest check verified", result.stdout)

    def test_a_text_only_clean_line_is_carried_as_before(self):
        result = self.assert_pass("carry-same-patch", "is carried to head")
        self.assertNotIn("manifest-listed binary", result.stdout)
        self.assertIn("hunk places and context included (digest ", result.stdout)

    # The owner's RECOMMEND line for the older pair is carried with the CLEAN line, under the same proof.
    CARRIED_RECOMMEND = "the MERGE-ADVERSARY RECOMMEND line was carried to head"

    def test_a_carried_clean_line_still_needs_a_recommend_line_of_some_kind_in_the_full_tier(self):
        self.assert_blocked("carry-same-patch-write-path", self.VERDICT_BLOCK)

    def test_a_recommend_line_for_the_carried_pair_is_carried_in_the_full_tier(self):
        result = self.assert_pass("carry-rec-fullpath", "is carried to head")
        self.assertIn(self.CARRIED_RECOMMEND, result.stdout)
        self.assertIn("review tier: full", result.stdout)
        with self.listed_binary_state():
            result = self.assert_pass("carry-rec-listed-binary", "is carried to head")
        self.assertIn(self.CARRIED_RECOMMEND, result.stdout)
        self.assertIn("with 1 manifest-listed binary blob(s) unchanged", result.stdout)

    def test_a_recommend_line_for_another_older_pair_is_not_carried(self):
        result = self.assert_blocked("carry-rec-fullpath-other-pair", self.VERDICT_BLOCK)
        self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)
        self.assertIn("is carried to head", result.stdout)  # the CLEAN line was carried; the other pair's RECOMMEND was not

    def test_a_voided_recommend_line_is_not_carried(self):
        result = self.assert_blocked("carry-rec-fullpath-retracted", self.VERDICT_BLOCK)
        self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)

    def test_a_newer_recommend_line_at_the_current_pair_stands_on_its_own(self):
        result = self.assert_pass("carry-rec-fullpath-newer-current", "is carried to head")
        self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)

    def test_a_recommend_line_is_not_carried_when_nothing_is_carried(self):
        # the patch differs between the pairs: neither line is carried
        result = self.assert_blocked("carry-rec-fullpath-patch-differs", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)
        # a listed binary whose blob differs between the pairs
        with self.listed_binary_state():
            result = self.assert_blocked("carry-rec-listed-binary-blob-differs", self.VERDICT_BLOCK)
        self.assert_not_carried(result)
        self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)
        # an unlisted binary, with and without the attestation
        for extra in ((), ("--binary-review-sha", HEAD)):
            result = self.assert_blocked("carry-rec-listed-binary", self.VERDICT_BLOCK if extra else self.ATTEST, extra_args=extra)
            self.assert_not_carried(result)
            self.assertNotIn(self.CARRIED_RECOMMEND, result.stdout)

    def test_both_lines_still_pass_at_either_tier(self):
        self.assert_pass("pass", self.VERDICT_PASS)

    def test_commit_comments_that_cannot_be_read_or_whose_commit_list_is_unproven_are_indeterminate_and_an_owners_voids(self):
        self.assert_indeterminate("commit-comments-unreadable", "could not read")
        # sixty-one commits without the head among them: the list is not proven to be the pull request's
        self.assert_indeterminate("sixty-one-commits", "could not read")
        self.assert_blocked("owner-commit-comment-with-a-token", self.VERDICT_BLOCK)

    def test_a_pull_request_of_more_than_sixty_commits_is_judged_like_any_other(self):
        # a chain link carries its parents' commits (76 here, the head among them): the comments of all of them are read
        self.assert_pass("seventy-six-commits", self.VERDICT_PASS)
        # and a stranger's token on one of those commits that is not the head is still found
        self.assert_blocked("seventy-six-commits-with-a-token-on-the-first", "carries a merge-verdict token")

    def test_the_commit_comments_of_every_page_are_read_and_only_those_on_the_pull_requests_commits_count(self):
        self.assert_blocked("commit-comment-token-on-the-second-page", self.VERDICT_BLOCK)
        self.assert_pass("commit-comment-token-on-another-commit", self.VERDICT_PASS)
        # a comment that names no commit cannot be placed, so the read is not complete
        self.assert_indeterminate("commit-comment-without-a-commit", "could not read")

    def test_a_pull_request_that_changes_the_gate_or_a_workflow_is_the_owners_own_act(self):
        for scenario in ("changes-the-gate", "changes-a-workflow", "renames-a-workflow-away", "renames-the-verdict-module-away",
                         "changes-the-registry", "changes-the-registry-in-another-case", "changes-the-review-tier", "changes-the-lab-manifest", "changes-the-attribution-check"):
            with self.subTest(scenario=scenario):
                self.assert_blocked(scenario, "the merge is the owner's own act")

    def test_a_binary_attestation_for_another_commit_blocks(self):
        result = self.assert_blocked("binary-with-attestation", "binary review attestation names a different commit than the PR head",
                                     extra_args=("--binary-review-sha", "f" * 40))
        self.assertIn("require the owner's own --binary-review-sha attestation", result.stdout)

    def test_the_independent_review_flag_no_longer_exists(self):
        result = self.run_gate("pass", extra_args=("--independent-review-sha", HEAD))
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("unknown option", result.stderr)

    def run_copy(self, copy, scenario="pass", script_dir="scripts"):
        env = os.environ.copy()
        env["PATH"] = f"{self.bin}:{env['PATH']}"
        env["GATE_SCENARIO"] = scenario
        return subprocess.run([str(copy / script_dir / "merge-gate.sh"), "321", "--repo", "lamemustafa/bridge"], env=env, text=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)

    def git_copy(self, copy, *args):
        subprocess.run(["git", "-C", str(copy), "-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", *args],
                       check=True, capture_output=True)

    def private_copy(self, name):
        """A copy of the gate repository with an origin of its own, so a test can move origin without disturbing the others."""
        copy = Path(self.tmp.name) / name
        shutil.copytree(self.repo, copy)
        origin = Path(self.tmp.name) / (name + "-origin.git")
        subprocess.run(["git", "init", "-q", "--bare", "-b", "master", str(origin)], check=True, capture_output=True)
        self.git_copy(copy, "remote", "set-url", "origin", str(origin))
        self.git_copy(copy, "push", "-q", "origin", "master")
        self.git_copy(copy, "fetch", "-q", "origin", "master")
        return copy

    def test_the_gate_refuses_to_run_from_a_tree_that_is_not_origin_masters(self):
        for how in ("edit", "committed-edit", "untracked-file", "remote-has-moved", "stale-tracking-ref", "tag-named-master", "script-elsewhere", "no-remote"):
            with self.subTest(how=how):
                copy = self.private_copy("tree-" + how)
                target = copy / "scripts" / "merge_gate_verdict.py"
                if how == "edit":
                    target.write_text(target.read_text() + "\n# changed\n")
                elif how == "committed-edit":
                    target.write_text(target.read_text() + "\n# changed\n")
                    self.git_copy(copy, "add", "-A")
                    self.git_copy(copy, "commit", "-q", "-m", "local edit")
                elif how == "untracked-file":
                    (copy / "scripts" / "extra_helper.py").write_text("x = 1\n")
                elif how == "remote-has-moved":
                    other = Path(self.tmp.name) / "tree-other"
                    shutil.copytree(copy, other)
                    (other / "scripts" / "merge_gate_verdict.py").write_text(target.read_text() + "\n# newer on origin\n")
                    self.git_copy(other, "add", "-A")
                    self.git_copy(other, "commit", "-q", "-m", "newer")
                    self.git_copy(other, "push", "-q", "origin", "HEAD:refs/heads/master")
                elif how == "tag-named-master":
                    # a tag called master on origin must not stand in for the branch: this checkout IS the tagged commit, not the branch
                    target.write_text(target.read_text() + "\n# tagged\n")
                    self.git_copy(copy, "add", "-A")
                    self.git_copy(copy, "commit", "-q", "-m", "tagged")
                    self.git_copy(copy, "push", "-q", "origin", "HEAD:refs/tags/master")
                elif how == "script-elsewhere":
                    # the scripts directory equals master's, but the gate that runs is a copy placed elsewhere in the tree
                    (copy / "other").mkdir()
                    for helper in (copy / "scripts").glob("merge*"):
                        shutil.copy2(helper, copy / "other" / helper.name)
                    (copy / "other" / "merge_gate_verdict.py").write_text("raise SystemExit(0)\n")
                elif how == "stale-tracking-ref":
                    # a fetch refspec that leaves master out of refs/remotes/origin: the guard must still compare with what it just fetched
                    self.git_copy(copy, "config", "remote.origin.fetch", "+refs/heads/other:refs/remotes/origin/other")
                    other = Path(self.tmp.name) / "tree-other2"
                    shutil.copytree(copy, other)
                    (other / "scripts" / "merge_gate_verdict.py").write_text(target.read_text() + "\n# newer on origin\n")
                    self.git_copy(other, "add", "-A")
                    self.git_copy(other, "commit", "-q", "-m", "newer")
                    self.git_copy(other, "push", "-q", "origin", "HEAD:refs/heads/master")
                else:
                    self.git_copy(copy, "remote", "remove", "origin")
                result = self.run_copy(copy, script_dir="other" if how == "script-elsewhere" else "scripts")
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertTrue("run this gate from origin/master's tree" in result.stderr or "could not fetch the branch master of origin" in result.stderr, result.stderr)

    def test_a_module_planted_beside_the_helpers_cannot_shadow_the_standard_library(self):
        copy = self.private_copy("tree-shadow")
        (copy / "scripts" / "json.py").write_text("raise SystemExit(0)\n")
        self.git_copy(copy, "add", "-A")
        self.git_copy(copy, "commit", "-q", "-m", "plant")
        # the planted file is on this copy's origin/master, so the tree guard passes: only an isolated interpreter keeps it out of the helpers
        self.git_copy(copy, "push", "-q", "origin", "master")
        self.git_copy(copy, "fetch", "-q", "origin", "master")
        result = self.run_copy(copy)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("MAY MERGE", result.stdout)

    def test_gitlink_change_is_indeterminate(self):
        self.assert_indeterminate("gitlink-change", "gitlink change(s) require explicit provenance")

    # -- Minimal head-SHA review-evidence check (KEEP #3, new) ---------------

    # -- Regression tests for the 7 fixes in the kept code -------------------

    def test_346_parenthesized_date_range_not_flagged(self):
        # Before the fix, GROUPED_NUMBER_RE's captured boundary character
        # (here "(" and ")") was left in match.group(0), so DATE_RANGE_RE's
        # fullmatch never matched and the date range was misreported as an
        # unexplained long digit run.
        result = self.assert_pass("date-range-parens")
        self.assertNotIn("unexplained long digit run", result.stdout)

    def test_358_scan_input_write_failure_is_indeterminate(self):
        # Before the fix, `{ printf ...; cat ...; } >"$scan_input_file"` had
        # no `|| status=$?`, so a failed write was never detected and a
        # truncated/empty file was silently fed to the privacy classifier.
        self.assert_indeterminate("scan-input-write-failure", "could not assemble complete privacy scan input")

    def test_360_lfs_pointer_requires_binary_attestation(self):
        # Before the fix, an added Git LFS pointer (ordinary text, no "GIT
        # binary patch" marker) was reconciled as a normal textual change and
        # never required a binary-review attestation.
        self.assert_blocked("lfs-pointer-binary", "require the owner's own --binary-review-sha attestation")

    def test_365_removed_record_line_total_mismatch_is_flagged(self):
        # Before the fix, `[ "$status" = "removed" ] && continue` skipped a
        # removed REST record before its line totals were ever compared
        # against the diff, so a mismatch went unreported.
        self.assert_indeterminate("removed-file-mismatch", "line totals for 'docs/retired.md' differ from REST metadata")

    def test_366_coverage_example_redacts_identifier_shape(self):
        # Before the fix, only home-directory paths were redacted from a
        # coverage-issue filename before it was echoed into gate output; an
        # identifier-shaped filename reached the message verbatim. (The same
        # filename is also, independently, real scan input -- path_text is
        # always scanned raw -- so this PR is separately and correctly
        # BLOCKed by the classifier itself; that is not what this test
        # checks. It checks that the *coverage diagnostic message* never
        # echoes the raw identifier shape.)
        result = self.assert_blocked("coverage-example-redaction", "omits or duplicates 'docs/<identifier>.md'")
        self.assertNotIn("ABCDE1234F", result.stdout)

    def test_373_diff_destination_without_rest_record_is_flagged(self):
        # Before the fix, reconciliation only walked REST records looking for
        # a matching diff section; a diff section with no REST record at all
        # (the reverse direction) was never flagged.
        self.assert_indeterminate("diff-extra-destination", "diff destination 'docs/smuggled.md' has no corresponding REST record")

    def test_377_localhost_author_email_is_accepted(self):
        # Before the fix, the author/committer email regex required a dotted
        # two-letter TLD, so a Git-valid address like dev@localhost rejected
        # the whole commit-metadata fetch and the PR went INDETERMINATE.
        result = self.assert_pass("author-email-localhost")
        self.assertNotIn("could not prove complete head-bound PR commit metadata", result.stdout)

    def test_finding374_helpers_resolve_relative_to_the_script(self):
        """#374: the gate must locate its own helper scripts by $script_dir.

        Every other test runs with cwd=ROOT, so a working-directory-relative
        invocation of merge_gate_diff.py passes there and fails anywhere else.
        Running the gate from an unrelated directory is the only thing that
        exercises the difference.
        """
        with tempfile.TemporaryDirectory(prefix="merge-gate-cwd-") as elsewhere:
            result = self.run_gate("pass", cwd=elsewhere)
        combined = result.stdout + result.stderr
        self.assertNotIn("merge_gate_diff.py: No such file or directory", combined)
        self.assertNotIn("can't open file", combined)
        self.assertEqual(result.returncode, 0, combined)
        self.assertIn("MAY MERGE", result.stdout)



class PrivacySpacedPanWordBoundary(unittest.TestCase):
    """The spaced PAN shape (five letters, four digits, one letter, with spaces or hyphens) is read only where its letters are not
    part of a longer word, so the end of one word and the start of the next are not taken for it. The test texts are built from
    parts, because this file is scanned like any other added text and must hold no identifier-shaped text."""

    def setUp(self):
        self.privacy = load_privacy_module()

    def blockers(self, text):
        return self.privacy.scan(text, HEAD)["blockers"]

    def blocked(self, text):
        return any("identifier shape" in b for b in self.blockers(text))

    def pan(self, first=" ", second=" "):
        return "ABCDE" + first + "1234" + second + "F"

    def test_a_spaced_or_hyphenated_pan_shape_is_still_blocked(self):
        for text in (
            "PAN " + self.pan(),
            "PAN: " + self.pan("-", "-") + ".",
            self.pan().lower(),
            "(" + self.pan() + ")",
            self.pan("-", " "),
            "ref_" + self.pan(),
            "id9" + self.pan(),
            self.pan() + "9",
        ):
            self.assertTrue(self.blocked(text), text)

    def test_a_spaced_gstin_is_still_blocked(self):
        self.assertTrue(self.blocked("GSTIN 07 " + self.pan() + " 1Z5"))

    def test_letters_of_neighbouring_words_do_not_make_the_shape(self):
        for text in (
            "migration 0003 job",
            "Run migration 0003 jobs in order.",
            "The reconciliation 2024 report",
            "ABCDEF 1234 G",
            self.pan() + "x",
        ):
            self.assertFalse(self.blocked(text), text)

    def test_a_real_shape_beside_a_false_one_counts_once(self):
        found = self.privacy.IDENTIFIER_RE.findall("migration 0003 job and " + self.pan())
        self.assertEqual(len(found), 1, found)

    def test_the_redaction_masks_the_shape_and_leaves_the_words(self):
        self.assertEqual(self.privacy.redact_shapes("see " + self.pan() + " here"), "see <identifier> here")
        self.assertEqual(self.privacy.redact_shapes("migration 0003 job"), "migration 0003 job")

    def test_the_unspaced_shapes_are_unchanged(self):
        pan = "PAN " + "ABCDE" + "1234" + "F"
        gstin = "GSTIN " + "07" + "ABCDE" + "1234" + "F" + "1Z5"
        for text in (pan, gstin):
            self.assertIsNotNone(self.privacy.IDENTIFIER_RE.search(text), text)
            self.assertTrue(self.blocked(text), text)

def run_of(length, step=7, start=3):
    """A run of digits that is not one repeated digit and not a placeholder, built at run time so that the
    test source holds no digit run of its own (the merge check's privacy scan reads test source too)."""
    return "".join(str((start + step * index) % 10) for index in range(length))


def repeated(digit, length):
    """One digit repeated, built at run time."""
    return str(digit) * length


class PrivacyScannerSyntheticReferences1743(unittest.TestCase):
    """A reference that is visibly synthetic is admitted (#1743): a token made of a three or four letter X/Z
    prefix and digits only, or one digit repeated and nothing else. Each admitted family has a positive
    case; every neighbouring real-looking shape still blocks. Every value is built at run time."""

    def setUp(self):
        self.privacy = load_privacy_module()
        self.twelve = run_of(12)
        self.mobile = "".join(str(9 - index) for index in range(10))  # a ten-digit run led by a 9
        self.pan_letters = "".join(chr(ord("A") + index) for index in range(5))

    def blockers(self, text):
        return self.privacy.scan(text, HEAD)["blockers"]

    def assert_admitted(self, text):
        self.assertEqual(self.blockers(text), [], text)

    def assert_blocked(self, text):
        blockers = self.blockers(text)
        self.assertTrue(any("long digit run" in b or "identifier shape" in b for b in blockers), (text, blockers))

    def test_a_digit_run_inside_a_placeholder_token_is_admitted(self):
        for text in (
            f"utr ZZZZ{self.twelve}",
            f"utr zzzz{self.twelve}",
            f"utr XXXX{run_of(17)}",
            f'let utr = "ZZZZ{run_of(16)}";',
        ):
            self.assert_admitted(text)

    def test_one_digit_repeated_is_admitted_at_each_length_the_scan_reads(self):
        for text in (
            f"ref {repeated(1, 11)}",
            f"ref {repeated(2, 12)}",
            f"ref {repeated(0, 14)}",
            f"ref {repeated(9, 18)}",
            f"call {repeated(9, 10)}",
            f"call {repeated(6, 10)}",
        ):
            self.assert_admitted(text)

    def test_a_random_looking_run_a_mobile_shape_and_a_long_tail_still_block(self):
        counting = "".join(str((index + 1) % 10) for index in range(12))
        for text in (f"ref {self.twelve}", f"call {self.mobile}", f"utr {run_of(18)}", f"ref {counting}"):
            self.assert_blocked(text)

    def test_a_run_beside_but_not_inside_a_placeholder_token_still_blocks(self):
        for text in (
            f"ZZZZ {self.twelve}",
            f"ZZZZ-{self.twelve}",
            f"ZZZZ: {self.twelve}",
            f"{self.twelve} ZZZZ",
        ):
            self.assert_blocked(text)

    def test_a_repeated_run_that_is_only_part_of_a_longer_token_still_blocks(self):
        # The token is judged whole: a run of one digit inside digits that are not all that digit,
        # or glued to letters that do not make a placeholder, is not the admitted family.
        for text in (
            f"ref {repeated(1, 12)}{run_of(4)}",
            f"ref {run_of(4)}{repeated(1, 12)}",
            f"ref AB{repeated(1, 12)}",
            f"ref {repeated(1, 12)}AB",
        ):
            self.assert_blocked(text)

    def test_an_identifier_glued_after_or_around_a_prefix_still_blocks(self):
        # A PAN shape and a mobile shape behind two digits and an X, and a prefix that is too short, too
        # long, or followed by anything but digits.
        for text in (
            f"pan 12X1{self.pan_letters}{run_of(4)}F",
            f"id 11X{self.mobile}",
            f"acct X{self.twelve}",
            f"call X{self.mobile}",
            f"ref XX{self.twelve}",
            f"ref ZZZZZ{self.twelve}",
            f"ref ZZZZ{self.twelve}A",
            f"ref ZZZZ{self.twelve}ZZZZ",
            f"ref ZZZZ{repeated(1, 25)}",
        ):
            self.assert_blocked(text)

    def test_the_family_stays_narrow_two_digits_and_a_leading_digit_do_not_qualify(self):
        for text in (
            f"ref 1{repeated(0, 10)}2",
            f"ref {'12' * 6}",
            f"ref {repeated(1, 11)}2",
            f"call {repeated(9, 9)}0",
        ):
            self.assert_blocked(text)

    def test_the_enclosing_token_is_the_run_of_ascii_letters_and_digits(self):
        token = self.privacy.enclosing_token
        text = f"x ZZZZ{self.twelve} y"
        start = text.index(self.twelve)
        self.assertEqual(token(text, start, start + 12), f"ZZZZ{self.twelve}")
        text = f"ZZZZ-{self.twelve}"
        start = text.index(self.twelve)
        self.assertEqual(token(text, start, start + 12), self.twelve)
        self.assertEqual(token(self.twelve, 0, 12), self.twelve)


class PrivacyScannerFindingsPR335(unittest.TestCase):
    """Direct unit coverage of scripts/merge_gate_privacy.py, one pair of
    tests (positive + negative) per surviving PR #335 review finding against
    the promoted-to-required privacy/PII scanner. Thread ids are the ones
    from the PR #335 review; all 11 were investigated and found to already
    be fixed on this branch (see the PR description / task report for the
    commit-by-commit evidence) -- these tests exist to lock that behavior in
    as regression coverage, since no prior test exercised these specific
    sub-cases directly.

    Every positive test is paired with a negative test asserting a
    legitimate, similarly-shaped value is NOT flagged -- issue #328 was a
    phone normalizer that fused adjacent numbers and flagged ordinary date
    ranges, so a widened/whole-line/multi-match PII pattern gets a
    false-positive check alongside its detection check.
    """

    def setUp(self):
        self.privacy = load_privacy_module()

    def blockers(self, text):
        return self.privacy.scan(text, HEAD)["blockers"]

    def assert_blocked(self, text, phrase):
        blockers = self.blockers(text)
        self.assertTrue(any(phrase in b for b in blockers), blockers)

    def assert_not_blocked(self, text):
        blockers = self.blockers(text)
        self.assertEqual(blockers, [], blockers)

    # Finding 1 -- PRRT_kwDOTWMyis6h8G4J: normalize standard 3-digit-area-
    # code Indian landline formats (e.g. 022-23456789, 011 23456789).
    def test_finding1_standard_3digit_landline_blocked(self):
        self.assert_blocked("Customer landline: 022-23456789", "long digit run")
        self.assert_blocked("Customer landline: 011 23456789", "long digit run")

    def test_finding1_negative_no_landline_not_blocked(self):
        self.assert_not_blocked("This changelog entry references no landline number at all.")
        self.assert_not_blocked(
            "Build step 0-1 ran before step 2345 in the pipeline; step 6789 followed."
        )

    # Finding 2 -- PRRT_kwDOTWMyis6h8Z5U: recognize variable-length (4-digit)
    # landline area codes (e.g. 0120-2345678).
    def test_finding2_variable_length_area_code_blocked(self):
        self.assert_blocked("Customer landline: 0120-2345678", "long digit run")

    def test_finding2_negative_non_landline_shapes_not_blocked(self):
        # Leading digit isn't 0: not a landline area code.
        self.assert_not_blocked("Invoice 9120-2345678 was issued to a vendor.")
        # Subscriber half is only 6 digits: one short of the 7-digit form.
        self.assert_not_blocked("Ticket 0120-234567 was closed.")

    # Finding 3 -- PRRT_kwDOTWMyis6h8G4L: hold raw certificate payloads
    # (bare "-----BEGIN CERTIFICATE-----") for human review.
    def test_finding3_bare_certificate_envelope_blocked(self):
        self.assert_blocked(
            "-----BEGIN CERTIFICATE-----\n"
            "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0000000000000000\n"
            "-----END CERTIFICATE-----",
            "PEM certificate envelope",
        )

    def test_finding3_negative_plain_mention_not_blocked(self):
        self.assert_not_blocked("Renew the TLS certificate before it expires next month.")

    # Finding 4 -- PRRT_kwDOTWMyis6h8Z5W: block trusted-certificate PEM
    # envelopes ("-----BEGIN TRUSTED CERTIFICATE-----"), not just bare/X509.
    def test_finding4_trusted_certificate_envelope_blocked(self):
        self.assert_blocked(
            "-----BEGIN TRUSTED CERTIFICATE-----\n"
            "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0000000000000000\n"
            "-----END TRUSTED CERTIFICATE-----",
            "PEM certificate envelope",
        )

    def test_finding4_negative_non_certificate_pem_not_blocked(self):
        # A different PEM envelope kind (not a certificate) is out of this
        # finding's scope and must not trip the certificate-specific rule.
        self.assert_not_blocked(
            "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A\n-----END PUBLIC KEY-----"
        )

    # Finding 5 -- PRRT_kwDOTWMyis6h8Z5Z: block UUID-shaped credential
    # session IDs instead of exempting every UUID wholesale.
    def test_finding5_uuid_credential_session_id_blocked(self):
        self.assert_blocked(
            "credential_session_id: 3fae1c2b-9d4e-4a11-8f2c-7b6d5e4a3c21",
            "credential, session, token, or bearer",
        )

    def test_finding5_negative_nil_uuid_sentinel_not_blocked(self):
        # The all-zero UUID is an explicit, documented sentinel and must stay
        # exempt even in credential context, or every fixture using it as a
        # placeholder session id would wrongly block.
        self.assert_not_blocked("credential_session_id: 00000000-0000-0000-0000-000000000000")

    # Finding 6 -- PRRT_kwDOTWMyis6h_z-h: block non-UUID credential tokens
    # (credential context must not be UUID-only).
    def test_finding6_non_uuid_bearer_token_blocked(self):
        self.assert_blocked(
            "Authorization: Bearer example-live-token-abcdefghijklmnop",
            "literal credential, bearer, or API token",
        )

    def test_finding6_negative_env_substitution_not_blocked(self):
        self.assert_not_blocked("Authorization: Bearer $BEARER_TOKEN")

    # Finding 7 -- PRRT_kwDOTWMyis6h_z-m: detect customer email addresses in
    # added payloads, distinct from admitted commit-author identities.
    def test_finding7_customer_email_blocked(self):
        self.assert_blocked("Customer contact: jane.doe@customerdomain.test", "customer email shape")

    def test_finding7_negative_example_domain_not_blocked(self):
        self.assert_not_blocked("Reviewer contact: dev@example.com")

    # The address shape ends before a full stop that closes a sentence or a list. Each value is built from AT.
    def test_an_address_followed_by_closing_punctuation_counts(self):
        AT = "@"
        for line in ("Write to jane.doe" + AT + "customerdomain.test.",
                     "Write to jane.doe" + AT + "customerdomain.test.)",
                     "Write to jane.doe" + AT + "customerdomain.test..",
                     "Write to jane.doe" + AT + "customerdomain.test...",
                     "Write to jane.doe" + AT + "customerdomain.test.\nNext line",
                     # an address written against a package specifier after full stops
                     "Bumped vite" + AT + "8.3.3...jane.doe" + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3..jane" + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3-jane..doe" + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3." + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3.!" + AT + "customerdomain.test"):
            with self.subTest(line=line):
                self.assertEqual(self.privacy.customer_email_count(line), 1)
                self.assert_blocked(line, "customer email shape")

    def test_an_address_ends_before_any_character_that_cannot_continue_its_domain(self):
        AT = "@"
        for line in ("Write to _jane.doe" + AT + "customerdomain.test_ today",
                     "_jane.doe" + AT + "customerdomain.test_.",
                     "jane.doe" + AT + "customerdomain.test--",
                     "jane.doe" + AT + "customerdomain.test-",
                     "jane.doe" + AT + "customerdomain.test_",
                     "jane.doe" + AT + "customerdomain.test+x",
                     "jane.doe" + AT + "customerdomain.test%",
                     # an address written against an example address
                     "dev" + AT + "example.com_jane" + AT + "customerdomain.test",
                     "dev" + AT + "example.com+jane" + AT + "customerdomain.test",
                     "dev" + AT + "example.com%jane" + AT + "customerdomain.test",
                     "dev" + AT + "example.com-jane" + AT + "customerdomain.test",
                     "dev" + AT + "example.com..jane" + AT + "customerdomain.test",
                     "dev" + AT + "example.com." + AT + "customerdomain.test"):
            with self.subTest(line=line):
                self.assertEqual(self.privacy.customer_email_count(line), 1)
                self.assert_blocked(line, "customer email shape")

    def test_a_package_specifier_or_an_example_address_before_a_full_stop_still_does_not_count(self):
        AT = "@"
        for line in ("Bumped vite" + AT + "8.3.3.", "Reviewer contact: dev" + AT + "example.com."):
            with self.subTest(line=line):
                self.assertEqual(self.privacy.customer_email_count(line), 0)

    # A package specifier (pnpm lockfile) has the address shape; a domain that is a full semantic version is not an
    # address. Each value is built from AT so that this file's own lines do not carry the shape.
    def test_package_specifiers_with_a_semver_domain_are_not_emails(self):
        AT = "@"
        for line in ("      vite" + AT + "8.3.3(yaml" + AT + "2.9.1):",
                     "      version: 5.0.3(jsdom" + AT + "30.1.1)(vite" + AT + "8.3.3(yaml" + AT + "2.9.1))",
                     "  '" + AT + "types/node" + AT + "22.1.0':",
                     "  typescript" + AT + "5.9.0-beta.1:",
                     "  esbuild" + AT + "0.25.0:",
                     "  react" + AT + "19.0.0-rc-abc123-20240101:",
                     "  next" + AT + "15.0.0-canary.12:"):
            with self.subTest(line=line):
                self.assertEqual(self.privacy.customer_email_count(line), 0)
                self.assert_not_blocked(line)

    def test_an_address_next_to_semver_specifiers_or_with_a_near_semver_domain_still_counts(self):
        AT = "@"
        for line in ("      vite" + AT + "8.3.3(yaml" + AT + "2.9.1): jane.doe" + AT + "customerdomain.test",
                     "  pkg" + AT + "8.3:",
                     "  x" + AT + "01.2.3:",
                     "contact: user" + AT + "1und1.de",
                     "contact: user" + AT + "1.2.3.4",
                     # an address written against a specifier: the specifier's domain takes the address's local part
                     "  pkg" + AT + "1.2.3-jane.doe" + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3-ops.jane2" + AT + "customerdomain.test",
                     "  pkg" + AT + "1.2.3-ops.jane2!" + AT + "customerdomain.test",
                     # a prerelease that ends in a real top-level domain, in either case, or in an internationalised one
                     "contact: jane" + AT + "1.2.3-rc.mailhost.com",
                     "contact: ops" + AT + "0.0.1-a.co",
                     "contact: JANE" + AT + "1.2.3-RC.MAILHOST.COM",
                     "contact: jane" + AT + "1.2.3-rc.xn--p1ai"):
            with self.subTest(line=line):
                self.assertEqual(self.privacy.customer_email_count(line), 1)
                self.assert_blocked(line, "customer email shape")

    # Finding 8 -- PRRT_kwDOTWMyis6iBarl: recognize quoted credential keys
    # (e.g. {"api_key": "..."}), not just bare/unquoted assignments.
    def test_finding8_quoted_credential_key_blocked(self):
        self.assert_blocked(
            '{"api_key": "customerproductioncredential"}',
            "literal credential, bearer, or API token",
        )

    def test_finding8_negative_quoted_placeholder_not_blocked(self):
        self.assert_not_blocked('{"api_key": "your_api_key"}')

    # Finding 9 -- PRRT_kwDOTWMyis6iBarn: stop exempting credential values
    # that merely resemble type names (...Token, ...Secret) when lower-case.
    def test_finding9_lowercase_typelike_values_not_exempted(self):
        self.assert_blocked("access_token: productionToken", "literal credential, bearer, or API token")
        self.assert_blocked("client_secret: supersecret", "literal credential, bearer, or API token")

    def test_finding9_negative_real_type_annotation_not_blocked(self):
        # Genuine type-syntax spellings (leading-capital ...Token/...Secret,
        # or the established lower-case type names) must stay exempt.
        self.assert_not_blocked("access_token: AccessToken")
        self.assert_not_blocked("client_secret: ClientSecret")
        self.assert_not_blocked("client_secret: str")

    # Finding 10 -- PRRT_kwDOTWMyis6iC356: scan password/passphrase/private-
    # key assignments as credential literals, not just api_key/token/secret.
    def test_finding10_password_assignment_blocked(self):
        self.assert_blocked(
            '{"password":"customerproductionpassword"}',
            "literal credential, bearer, or API token",
        )

    def test_finding10_negative_password_placeholder_not_blocked(self):
        self.assert_not_blocked('{"password":"REDACTED"}')

    # Finding 11 -- PRRT_kwDOTWMyis6iC36H: inspect every credential
    # assignment on a line, not just the first.
    def test_finding11_second_assignment_on_line_inspected(self):
        mixed = self.blockers(
            '{"api_key":"example_api_key","client_secret":"customerproductionsecret"}'
        )
        self.assertEqual(len(mixed), 1, mixed)
        self.assertIn("1 literal credential", mixed[0])
        both_real = self.blockers(
            '{"api_key":"customerprodkeyabc","client_secret":"customerprodsecretxyz"}'
        )
        self.assertEqual(len(both_real), 1, both_real)
        self.assertIn("2 literal credential", both_real[0])

    def test_finding11_negative_all_placeholders_not_blocked(self):
        self.assert_not_blocked(
            '{"api_key":"your_api_key","client_secret":"replace_me_client_secret"}'
        )


def load_diff_module():
    spec = importlib.util.spec_from_file_location("merge_gate_diff", ROOT / "scripts" / "merge_gate_diff.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class PrivacyLocationAndCommentBlockTests(unittest.TestCase):
    """Where the scan's hits are (never the values), and a merge commit's conflict comment block."""

    AGENT = "noreply" + "@" + "anthropic.com"
    # Built from parts: this file is scanned by the gate it tests.
    CUSTOMER = "person" + "@" + "customer.test"
    RUN = "12345" + "678901"
    IDENTIFIER = "98765" + "43210"

    def setUp(self):
        self.privacy = load_privacy_module()

    def redacted(self, message):
        return self.privacy.redact_public_agent_attribution_trailer(message)

    def emails(self, message):
        return self.privacy.customer_email_count(self.redacted(message))

    def test_the_trailer_before_a_git_conflict_comment_block_is_still_the_terminal_trailer(self):
        message = "Merge master\n\nCo-Authored-By: Claude <" + self.AGENT + ">\n\n# Conflicts:\n#\tdocs/a.json\n#\ttools/b.rs"
        self.assertEqual(self.emails(message), 0)
        self.assertTrue(self.redacted(message).endswith("\n\n# Conflicts:\n#\tdocs/a.json\n#\ttools/b.rs"), "the block is returned unchanged")
        crlf = message.replace("\n", "\r\n")
        self.assertEqual(self.emails(crlf), 0)

    def test_an_address_inside_or_after_the_comment_block_is_still_counted(self):
        inside = "Merge\n\nCo-Authored-By: Claude <" + self.AGENT + ">\n\n# Conflicts:\n# " + self.CUSTOMER
        self.assertEqual(self.emails(inside), 1)
        agent_in_block = "Merge\n\nCo-Authored-By: Claude <" + self.AGENT + ">\n\n# note <" + self.AGENT + ">"
        self.assertEqual(self.emails(agent_in_block), 1)
        trailer_after_block = "Merge\n\n# Conflicts:\n#\ta\n\nCo-Authored-By: Claude <" + self.AGENT + ">"
        self.assertEqual(self.emails(trailer_after_block), 0, "a terminal trailer is unchanged behaviour")
        not_a_trailer = "Merge\n\nnote: see <" + self.AGENT + ">\n\n# Conflicts:\n#\ta"
        self.assertEqual(self.emails(not_a_trailer), 1)

    def test_explain_reports_location_and_categories_but_never_a_value(self):
        document = {
            "sources": [
                {"label": "pr-title", "text": "a fine title"},
                {"label": "commit-message-2", "text": "fine\nmail " + self.CUSTOMER + " now"},
            ],
            "added": [
                {"path": "docs/a.md", "line": 7, "text": "synthetic " + self.RUN + " run"},
                {"path": "docs/a.md", "line": 8, "text": "nothing here"},
                {"path": "docs/account-" + self.IDENTIFIER + ".md", "line": 0, "text": "docs/account-" + self.IDENTIFIER + ".md"},
            ],
        }
        result = self.privacy.explain(document, HEAD)
        wheres = [hit["where"] for hit in result["hits"]]
        self.assertEqual(wheres, ["commit-message-2:2", "docs/a.md:7", "docs/account-<identifier>.md"])
        rendered = repr(result)
        for value in (self.CUSTOMER, self.RUN, self.IDENTIFIER, "fine"):
            self.assertNotIn(value, rendered)
        self.assertEqual(result["truncated"], 0)

    def test_explain_is_capped_and_counts_the_rest(self):
        added = [{"path": "f.md", "line": n, "text": "synthetic " + self.RUN} for n in range(1, 61)]
        result = self.privacy.explain({"sources": [], "added": added}, HEAD)
        self.assertEqual((len(result["hits"]), result["truncated"]), (50, 10))

    def test_explain_cli_refuses_malformed_input(self):
        for bad in ('{"sources": 1, "added": []}', '{"sources": [], "added": [{"path": "a", "line": true, "text": "x"}]}', "not json"):
            result = subprocess.run(
                ["python3", str(PRIVACY_MODULE_PATH), "--head", HEAD, "--explain"],
                input=bad, capture_output=True, text=True,
            )
            self.assertNotEqual(result.returncode, 0, bad)

    def test_the_diff_parser_gives_each_added_line_its_new_file_line_number(self):
        diff = (
            "diff --git a/a.txt b/a.txt\nindex 1..2 100644\n--- a/a.txt\n+++ b/a.txt\n"
            "@@ -1,3 +1,5 @@\n keep\n+added one\n keep2\n+added two\n-gone\n+added three\n\\ No newline at end of file\n"
            "@@ -40,2 +42,3 @@\n ctx\n\n+tail\n"
        )
        parsed = load_diff_module().parse(diff.split("\n"))
        self.assertEqual(parsed["added_payload"], ["added one", "added two", "added three", "tail"])
        self.assertEqual(parsed["added_locations"], [["a.txt", 2], ["a.txt", 4], ["a.txt", 5], ["a.txt", 44]])


def load_verdict_module():
    spec = importlib.util.spec_from_file_location("merge_gate_verdict", ROOT / "scripts" / "merge_gate_verdict.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class MergeVerdictEvaluator(unittest.TestCase):
    """scripts/merge_gate_verdict.py: the owner's two machine lines, spec v3 (every case is a document the gate would build)."""

    OWNER = 57982425
    LOGIN = "lamemustafa"
    HEAD = "1" * 40
    BASE = "2" * 40
    CLEAN = "MERGE-VERDICT CLEAN head=" + "1" * 40 + " base=" + "2" * 40
    RECOMMEND = "MERGE-ADVERSARY RECOMMEND head=" + "1" * 40 + " base=" + "2" * 40
    T = ["2026-10-14T0%d:00:00Z" % n for n in range(1, 10)]

    def setUp(self):
        self.mod = load_verdict_module()

    def user(self, uid=None, kind="User"):
        return {"id": self.OWNER if uid is None else uid, "type": kind}

    def comment(self, body, at, uid=None, kind="User", app=None):
        return {"user": self.user(uid, kind), "body": body, "created_at": at, "performed_via_github_app": app}

    def review(self, body, at, uid=None, kind="User", state="COMMENTED"):
        return {"user": self.user(uid, kind), "body": body, "submitted_at": at, "state": state}

    def doc(self, reviews=(), comments=(), inline=(), title="t", body="b", commits=("c",), added=("a",),
            reviews_graph=(), comments_graph=(), events=(), complete=True):
        return {"head": self.HEAD, "base": self.BASE, "reviews": list(reviews), "comments": list(comments),
                "inline_comments": list(inline),
                "texts": {"title": title, "body": body, "commit_messages": list(commits), "added_lines": list(added)},
                "graphql": {"complete": complete, "reviews": list(reviews_graph), "comments": list(comments_graph), "events": list(events)}}

    def both(self):
        return self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])

    def status(self, doc):
        return self.mod.evaluate(doc)["status"]

    def test_both_lines_pass_as_comments_reviews_or_one_of_each(self):
        self.assertEqual(self.status(self.both()), "pass")
        self.assertEqual(self.status(self.doc(reviews=[self.review(self.CLEAN, self.T[1]), self.review(self.RECOMMEND, self.T[2])])), "pass")
        self.assertEqual(self.status(self.doc(reviews=[self.review(self.CLEAN, self.T[1])], comments=[self.comment(self.RECOMMEND, self.T[2])])), "pass")

    def test_either_line_alone_blocks(self):
        self.assertEqual(self.status(self.doc(comments=[self.comment(self.CLEAN, self.T[1])])), "block")
        self.assertEqual(self.status(self.doc(comments=[self.comment(self.RECOMMEND, self.T[1])])), "block")
        self.assertEqual(self.status(self.doc()), "block")

    def test_a_line_for_another_head_or_base_does_not_count(self):
        other_head = self.CLEAN.replace("head=" + "1" * 40, "head=" + "3" * 40)
        other_base = self.RECOMMEND.replace("base=" + "2" * 40, "base=" + "4" * 40)
        self.assertEqual(self.status(self.doc(comments=[self.comment(other_head, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])), "block")
        self.assertEqual(self.status(self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(other_base, self.T[2])])), "block")

    def test_only_a_body_that_is_exactly_the_line_counts(self):
        bad = [self.CLEAN + " ok", " " + self.CLEAN, self.CLEAN + "\n\n", "x\n" + self.CLEAN, self.CLEAN + "\n" + self.RECOMMEND,
               self.CLEAN.upper(), self.CLEAN.replace("1" * 40, "1" * 39), self.CLEAN.replace("CLEAN", "CLEAN,"),
               "> " + self.CLEAN, "`" + self.CLEAN + "`", "<!-- " + self.CLEAN + " -->", self.CLEAN.replace("1" * 40, "G" * 40)]
        for body in bad:
            with self.subTest(body=body[:60]):
                self.assertEqual(self.status(self.doc(comments=[self.comment(body, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])), "block")

    def test_crlf_and_one_trailing_newline_are_accepted(self):
        for body in (self.CLEAN + "\n", self.CLEAN + "\r\n"):
            with self.subTest(body=repr(body[-3:])):
                self.assertEqual(self.status(self.doc(comments=[self.comment(body, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])), "pass")

    def test_a_line_by_anyone_else_is_refused_not_just_ignored(self):
        for uid, kind, app in ((123, "User", None), (self.OWNER, "Bot", None), (self.OWNER, "User", {"id": 1}), (str(self.OWNER), "User", None), (True, "User", None)):
            with self.subTest(uid=uid, kind=kind, app=app):
                result = self.mod.evaluate(self.doc(comments=[self.comment(self.CLEAN, self.T[1], uid=uid, kind=kind, app=app), self.comment(self.RECOMMEND, self.T[2])]))
                self.assertEqual(result["status"], "block")
                self.assertTrue(result["refusals"])
        nobody = {"user": None, "body": self.CLEAN, "created_at": self.T[1]}
        self.assertEqual(self.status(self.doc(comments=[nobody, self.comment(self.RECOMMEND, self.T[2])])), "block")

    def test_the_tokens_anywhere_in_the_pull_request_text_are_refused_even_with_both_lines(self):
        for field in ("title", "body"):
            for token in ("MERGE-VERDICT", "MERGE-ADVERSARY"):
                with self.subTest(field=field, token=token):
                    result = self.mod.evaluate(dict(self.both(), texts=dict(self.both()["texts"], **{field: "see " + token})))
                    self.assertEqual(result["status"], "block")
                    self.assertTrue(result["refusals"])
        for texts in ({"commit_messages": ["MERGE-ADVERSARY"]}, {"added_lines": ["x MERGE-VERDICT CLEAN"]}):
            with self.subTest(texts=texts):
                result = self.mod.evaluate(dict(self.both(), texts=dict(self.both()["texts"], **texts)))
                self.assertEqual(result["status"], "block")

    def test_a_non_owner_review_comment_or_inline_comment_with_a_token_is_refused(self):
        stranger = {"user": self.user(7), "body": "MERGE-VERDICT CLEAN", "created_at": self.T[3], "submitted_at": self.T[3]}
        for kwargs in ({"comments": [stranger]}, {"reviews": [stranger]}, {"inline": [stranger]}):
            with self.subTest(where=list(kwargs)[0]):
                doc = self.both()
                key = {"inline": "inline_comments"}.get(list(kwargs)[0], list(kwargs)[0])
                doc[key] = doc[key] + list(kwargs.values())[0]
                self.assertEqual(self.status(doc), "block")

    def test_an_owner_inline_comment_with_a_token_voids_everything_and_is_never_a_verdict(self):
        inline = {"user": self.user(), "body": self.CLEAN, "created_at": self.T[5]}
        self.assertEqual(self.status(self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2])], inline=[inline])), "block")
        only_inline = self.doc(comments=[self.comment(self.RECOMMEND, self.T[2])], inline=[dict(inline, created_at=self.T[0])])
        self.assertEqual(self.status(only_inline), "block")
        # a void older than the verdicts still voids: an inline comment's time and history are not read

    def test_a_later_not_clean_or_retraction_voids_and_a_newer_pair_restores(self):
        for line in ("MERGE-VERDICT NOT-CLEAN head=" + "1" * 40 + " base=" + "2" * 40, "MERGE-VERDICT RETRACT head=" + "1" * 40 + " base=" + "2" * 40,
                     "MERGE-ADVERSARY REJECT head=" + "1" * 40 + " base=" + "2" * 40, "MERGE-ADVERSARY RETRACT head=" + "1" * 40 + " base=" + "2" * 40,
                     "MERGE-VERDICT SOMETHING ELSE"):
            with self.subTest(line=line[:40]):
                comments = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment(line, self.T[3])]
                self.assertEqual(self.status(self.doc(comments=comments)), "block")
                renewed = comments + [self.comment(self.CLEAN, self.T[4]), self.comment(self.RECOMMEND, self.T[5])]
                self.assertEqual(self.status(self.doc(comments=renewed)), "pass")
                only_one_renewed = comments + [self.comment(self.CLEAN, self.T[4])]
                self.assertEqual(self.status(self.doc(comments=only_one_renewed)), "block")

    def test_the_review_tier_one_needs_the_clean_line_alone_and_anything_else_needs_both(self):
        clean_only = self.doc(comments=[self.comment(self.CLEAN, self.T[1])])
        both = self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])
        self.assertEqual(self.status(dict(clean_only, tier="one")), "pass")
        self.assertEqual(self.status(dict(both, tier="one")), "pass")
        for tier in (None, "full", "ONE", "one ", "", 1, True, ["one"]):
            with self.subTest(tier=tier):
                self.assertEqual(self.status(dict(clean_only, tier=tier)), "block")
                self.assertEqual(self.status(dict(both, tier=tier)), "pass")
        self.assertEqual(self.status(clean_only), "block")

    def test_the_review_tier_one_still_needs_a_standing_clean_line_and_still_obeys_every_void(self):
        recommend_only = self.doc(comments=[self.comment(self.RECOMMEND, self.T[2])])
        self.assertEqual(self.status(dict(recommend_only, tier="one")), "block")
        other_head = self.CLEAN.replace("1" * 40, "3" * 40)
        self.assertEqual(self.status(dict(self.doc(comments=[self.comment(other_head, self.T[1])]), tier="one")), "block")
        for line in ("MERGE-ADVERSARY REJECT head=" + "1" * 40 + " base=" + "2" * 40, "MERGE-VERDICT RETRACT head=" + "1" * 40 + " base=" + "2" * 40):
            with self.subTest(line=line[:30]):
                voided = self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(line, self.T[3])])
                self.assertEqual(self.status(dict(voided, tier="one")), "block")
        token_by_someone_else = self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2], uid=7)])
        self.assertEqual(self.status(dict(token_by_someone_else, tier="one")), "block")

    # -- (a) a verdict line may open a longer comment ------------------------------------------------------------------------
    PROSE = "Review of head 1111, the whole change. CLEAN, 0 P1 / 0 P2.\n\n- the change does what it says\n"

    def test_a_verdict_line_that_opens_an_owner_review_with_prose_after_it_counts(self):
        for separator in ("\n", "\r\n"):
            with self.subTest(separator=repr(separator)):
                clean = self.CLEAN + separator + separator + self.PROSE
                self.assertEqual(self.status(dict(self.doc(comments=[self.comment(clean, self.T[1])]), tier="one")), "pass")
                both = [self.comment(clean, self.T[1]), self.comment(self.RECOMMEND + separator + "Adversary read: nothing at P1 or P2." + separator, self.T[2])]
                self.assertEqual(self.status(self.doc(comments=both)), "pass")
        as_review = self.doc(reviews=[self.review(self.CLEAN + "\n\n" + self.PROSE, self.T[1])], comments=[self.comment(self.RECOMMEND, self.T[2])])
        self.assertEqual(self.status(as_review), "pass")

    def test_prose_around_a_verdict_line_that_is_not_just_prose_voids_instead(self):
        token = "MERGE-VERDICT"
        cases = {
            "the line is not first": self.PROSE + self.CLEAN,
            "a second verdict token in the text": self.CLEAN + "\n\nsee also " + token + " CLEAN head=" + "1" * 40 + " base=" + "2" * 40,
            "a token inside a hidden comment": self.CLEAN + "\n\n<!-- " + token + " NOT-CLEAN -->\nProse.",
            "trailing spaces on the line": self.CLEAN + "  \n\n" + self.PROSE,
            "a leading space on the line": " " + self.CLEAN + "\n\n" + self.PROSE,
            "a different word after the verdict": "MERGE-VERDICT CLEANISH head=" + "1" * 40 + " base=" + "2" * 40 + "\nProse",
            "another line of the same kind first": "MERGE-VERDICT NOT-CLEAN head=" + "1" * 40 + " base=" + "2" * 40 + "\n" + self.CLEAN,
        }
        for name, body in cases.items():
            with self.subTest(case=name):
                both = self.doc(comments=[self.comment(body, self.T[1]), self.comment(self.RECOMMEND, self.T[2])])
                self.assertEqual(self.status(both), "block")

    def test_a_verdict_line_with_prose_by_anyone_else_is_refused_as_before(self):
        other = self.doc(comments=[self.comment(self.CLEAN + "\n\n" + self.PROSE, self.T[1], uid=7), self.comment(self.RECOMMEND, self.T[2])])
        self.assertEqual(self.status(other), "block")

    # -- (c) a clean line carried across a change of head and merge base the gate proved leaves the patch unchanged ----------
    OLD_HEAD = "3" * 40
    OLD_BASE = "4" * 40

    def old_clean(self):
        return "MERGE-VERDICT CLEAN head=" + self.OLD_HEAD + " base=" + self.OLD_BASE

    def test_a_clean_line_for_an_older_head_stands_only_when_the_gate_carried_exactly_that_pair(self):
        doc = self.doc(comments=[self.comment(self.old_clean(), self.T[1])])
        self.assertEqual(self.status(dict(doc, tier="one")), "block")
        carried = {"head": self.OLD_HEAD, "base": self.OLD_BASE}
        self.assertEqual(self.status(dict(doc, tier="one", carried=carried)), "pass")
        for bad in ({"head": "5" * 40, "base": self.OLD_BASE}, {"head": self.OLD_HEAD, "base": "5" * 40}, {"head": self.OLD_HEAD},
                    {"head": self.OLD_HEAD, "base": self.OLD_BASE, "x": 1},
                    "carried", None, [], {"head": 1, "base": 2}):
            with self.subTest(bad=bad):
                self.assertEqual(self.status(dict(doc, tier="one", carried=bad)), "block")

    def old_recommend(self, head=None, base=None):
        return "MERGE-ADVERSARY RECOMMEND head=" + (head or self.OLD_HEAD) + " base=" + (base or self.OLD_BASE)

    def carried_pair(self):
        return {"head": self.OLD_HEAD, "base": self.OLD_BASE}

    def test_a_recommend_line_for_the_carried_pair_is_carried_with_the_clean_line(self):
        both = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(self.old_recommend(), self.T[2])])
        for tier in ("full", "one", None):
            with self.subTest(tier=tier):
                result = self.mod.evaluate(dict(both, tier=tier, carried=self.carried_pair()))
                self.assertEqual(result["status"], "pass")
                self.assertEqual(result["recommend_carried"], tier != "one")
        # as reviews, too, and in either order of the two lines
        reviews = self.doc(reviews=[self.review(self.old_recommend(), self.T[1]), self.review(self.old_clean(), self.T[2])])
        self.assertEqual(self.status(dict(reviews, tier="full", carried=self.carried_pair())), "pass")

    def test_a_recommend_line_for_an_older_pair_is_not_carried_when_no_pair_was_carried(self):
        both = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(self.old_recommend(), self.T[2])])
        self.assertEqual(self.status(dict(both, tier="full")), "block")
        for bad in (None, {"head": self.OLD_HEAD}, {"head": "5" * 40, "base": self.OLD_BASE}, "carried", []):
            with self.subTest(bad=bad):
                self.assertEqual(self.status(dict(both, tier="full", carried=bad)), "block")

    def test_a_recommend_line_is_not_carried_when_the_clean_line_was_not(self):
        # CLEAN names the current pair (nothing carried for it) and RECOMMEND an older pair: the older RECOMMEND does not stand
        doc = self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.old_recommend(), self.T[2])])
        self.assertEqual(self.status(dict(doc, tier="full", carried=self.carried_pair())), "block")
        # CLEAN names another older pair than the one carried
        other = self.old_clean().replace(self.OLD_HEAD, "6" * 40)
        doc = self.doc(comments=[self.comment(other, self.T[1]), self.comment(self.old_recommend(), self.T[2])])
        self.assertEqual(self.status(dict(doc, tier="full", carried=self.carried_pair())), "block")

    def test_a_recommend_line_for_a_different_older_pair_than_the_carried_clean_line_is_not_carried(self):
        for line in (self.old_recommend(head="6" * 40), self.old_recommend(base="6" * 40), self.old_recommend(head="6" * 40, base="7" * 40)):
            with self.subTest(line=line[-90:]):
                doc = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(line, self.T[2])])
                result = self.mod.evaluate(dict(doc, tier="full", carried=self.carried_pair()))
                self.assertEqual(result["status"], "block")
                self.assertFalse(result["recommend_carried"])
        # an older RECOMMEND for the carried pair does not stand when the newest one names another pair
        doc = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(self.old_recommend(), self.T[2]),
                                 self.comment(self.old_recommend(head="6" * 40), self.T[3])])
        self.assertEqual(self.status(dict(doc, tier="full", carried=self.carried_pair())), "block")

    def test_two_recommend_lines_in_the_same_newest_second_do_not_carry(self):
        # one names the carried pair and one another pair, at the same second: ambiguous, so nothing carries
        other = self.old_recommend(head="6" * 40)
        for order in ((self.old_recommend(), other), (other, self.old_recommend())):
            with self.subTest(first=order[0][-60:]):
                doc = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(order[0], self.T[2]), self.comment(order[1], self.T[2])])
                result = self.mod.evaluate(dict(doc, tier="full", carried=self.carried_pair()))
                self.assertEqual(result["status"], "block")
                self.assertFalse(result["recommend_carried"])

    def test_a_carried_recommend_line_is_voided_like_the_carried_clean_line(self):
        carried = self.carried_pair()
        lines = [self.comment(self.old_clean(), self.T[1]), self.comment(self.old_recommend(), self.T[2])]
        retract = "MERGE-ADVERSARY RETRACT head=" + "1" * 40 + " base=" + "2" * 40
        stand = self.mod.evaluate(dict(self.doc(comments=lines), tier="full", carried=carried))
        self.assertEqual(stand["status"], "pass")
        voids = {
            "a later retraction": dict(comments=lines + [self.comment(retract, self.T[3])]),
            "a later NOT-CLEAN": dict(comments=lines + [self.comment("MERGE-VERDICT NOT-CLEAN head=" + "1" * 40 + " base=" + "2" * 40, self.T[3])]),
            "a void at the same second": dict(comments=lines + [self.comment(retract, self.T[2])]),
            "a later owner edit": dict(comments=lines, comments_graph=[{"authorId": self.OWNER, "lastEditedAt": self.T[3], "editCount": 1}]),
            "an owner edit of unknown time": dict(comments=lines, comments_graph=[{"authorId": self.OWNER, "lastEditedAt": None, "editCount": 1}]),
            "a later deletion": dict(comments=lines, events=[{"type": "CommentDeletedEvent", "createdAt": self.T[3], "authorId": self.OWNER}]),
            "a later dismissal": dict(comments=lines, events=[{"type": "ReviewDismissedEvent", "createdAt": self.T[3], "authorId": self.OWNER}]),
            "a later review asking for changes": dict(comments=lines, reviews=[self.review(self.CLEAN, self.T[3], state="CHANGES_REQUESTED")]),
            "an owner inline comment with a token": dict(comments=lines, inline=[self.comment("MERGE-VERDICT x", self.T[3])]),
            "an owner commit comment with a token": dict(comments=lines),
        }
        for name, kwargs in voids.items():
            with self.subTest(void=name):
                doc = self.doc(**kwargs)
                if name == "an owner commit comment with a token":
                    doc["commit_comments"] = [dict(self.comment("MERGE-ADVERSARY x", self.T[3]))]
                result = self.mod.evaluate(dict(doc, tier="full", carried=carried))
                self.assertEqual(result["status"], "block")
                self.assertFalse(result["recommend_carried"])
        # a void between the two lines voids the older one only: the RECOMMEND line is older here, so it does not stand
        between = self.doc(comments=[self.comment(self.old_recommend(), self.T[1]), self.comment(retract, self.T[2]), self.comment(self.old_clean(), self.T[3])])
        self.assertEqual(self.status(dict(between, tier="full", carried=carried)), "block")
        # a token by anyone else is refused
        other = self.doc(comments=lines + [self.comment(self.old_recommend(), self.T[3], uid=7)])
        self.assertEqual(self.status(dict(other, tier="full", carried=carried)), "block")

    def test_a_newer_recommend_line_at_the_current_pair_stands_on_its_own(self):
        doc = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(self.old_recommend(), self.T[2]),
                                 self.comment(self.RECOMMEND, self.T[3])])
        result = self.mod.evaluate(dict(doc, tier="full", carried=self.carried_pair()))
        self.assertEqual(result["status"], "pass")
        self.assertFalse(result["recommend_carried"])
        # and without any carried pair a RECOMMEND at the current pair never needed one, though the old CLEAN still does
        self.assertEqual(self.status(dict(doc, tier="full")), "block")

    def test_a_carried_clean_line_is_voided_like_any_other(self):
        carried = {"head": self.OLD_HEAD, "base": self.OLD_BASE}
        for line in ("MERGE-VERDICT RETRACT head=" + self.OLD_HEAD + " base=" + self.OLD_BASE, "MERGE-ADVERSARY REJECT head=" + "1" * 40 + " base=" + "2" * 40):
            with self.subTest(line=line[:30]):
                voided = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(line, self.T[3])])
                self.assertEqual(self.status(dict(voided, tier="one", carried=carried)), "block")

    def test_a_newer_clean_line_for_the_current_pair_wins_over_an_older_carried_one(self):
        carried = {"head": self.OLD_HEAD, "base": self.OLD_BASE}
        doc = self.doc(comments=[self.comment(self.CLEAN, self.T[1]), self.comment(self.old_clean(), self.T[2])])
        # the NEWEST clean line is the old pair: it stands because it was carried
        self.assertEqual(self.status(dict(doc, tier="one", carried=carried)), "pass")
        doc2 = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment(self.CLEAN, self.T[2])])
        self.assertEqual(self.status(dict(doc2, tier="one")), "pass")

    def test_the_result_names_the_clean_line_that_could_be_carried(self):
        result = self.mod.evaluate(dict(self.doc(comments=[self.comment(self.old_clean(), self.T[1])]), tier="one"))
        self.assertEqual(result["candidate_clean"], {"head": self.OLD_HEAD, "base": self.OLD_BASE})
        self.assertIsNone(self.mod.evaluate(dict(self.doc(comments=[self.comment(self.CLEAN, self.T[1])]), tier="one"))["candidate_clean"])
        voided = self.doc(comments=[self.comment(self.old_clean(), self.T[1]), self.comment("MERGE-VERDICT RETRACT head=" + "1" * 40 + " base=" + "2" * 40, self.T[3])])
        self.assertIsNone(self.mod.evaluate(dict(voided, tier="one"))["candidate_clean"])

    def test_a_void_at_the_same_second_as_a_verdict_voids_it(self):
        comments = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment("MERGE-VERDICT RETRACT head=" + "1" * 40 + " base=" + "2" * 40, self.T[2])]
        self.assertEqual(self.status(self.doc(comments=comments)), "block")

    def test_both_verdicts_voided_at_their_own_second_do_not_stand(self):
        comments = [self.comment(self.CLEAN, self.T[2]), self.comment(self.RECOMMEND, self.T[2]),
                    self.comment("MERGE-ADVERSARY RETRACT head=" + "1" * 40 + " base=" + "2" * 40, self.T[2])]
        self.assertEqual(self.status(self.doc(comments=comments)), "block")
        self.assertEqual(self.status(self.doc(comments=comments[:2])), "pass")

    def test_a_dismissal_event_alone_voids_what_is_older_and_not_what_is_newer(self):
        late = {"type": "ReviewDismissedEvent", "createdAt": self.T[6], "authorId": self.OWNER}
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[late]))), "block")
        early = dict(late, createdAt=self.T[0])
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[early]))), "pass")
        someone_else = dict(late, authorId=7)
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[someone_else]))), "pass")

    def test_the_owner_is_matched_by_id_so_a_renamed_account_still_voids(self):
        doc = self.both()
        doc["graphql"] = dict(doc["graphql"], comments=[{"authorId": self.OWNER, "authorLogin": "a-new-name", "lastEditedAt": self.T[6], "editCount": 1}])
        self.assertEqual(self.status(doc), "block")
        doc["graphql"] = dict(doc["graphql"], comments=[{"authorId": 7, "authorLogin": self.LOGIN, "lastEditedAt": self.T[6], "editCount": 1}])
        self.assertEqual(self.status(doc), "pass", "a node whose author id is not the owner's is ignored, whatever its login says")

    def test_the_newest_line_of_each_kind_counts_even_when_an_older_one_matched(self):
        other_head = self.CLEAN.replace("head=" + "1" * 40, "head=" + "3" * 40)
        comments = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment(other_head, self.T[3])]
        self.assertEqual(self.status(self.doc(comments=comments)), "block")
        again = comments + [self.comment(self.CLEAN, self.T[4])]
        self.assertEqual(self.status(self.doc(comments=again)), "pass")

    def test_an_edit_of_any_owner_item_after_a_verdict_voids_it_and_an_older_edit_or_anyone_elses_does_not(self):
        late = [{"authorId": self.OWNER, "lastEditedAt": self.T[6], "editCount": 1}]
        early = [{"authorId": self.OWNER, "lastEditedAt": self.T[0], "editCount": 1}]
        other = [{"authorId": 7, "lastEditedAt": self.T[8], "editCount": 1}]
        for graph_key in ("comments_graph", "reviews_graph"):
            with self.subTest(item=graph_key):
                self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], **{graph_key.replace("_graph", ""): late}))), "block")
                self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], **{graph_key.replace("_graph", ""): early}))), "pass")
                self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], **{graph_key.replace("_graph", ""): other}))), "pass")
        unknown_time = [{"authorId": self.OWNER, "lastEditedAt": None, "editCount": 2}]
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], comments=unknown_time))), "block")
        never_edited = [{"authorId": self.OWNER, "lastEditedAt": None, "editCount": 0}]
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], comments=never_edited))), "pass")

    def test_a_deleted_owner_comment_voids_and_someone_elses_does_not(self):
        deleted = {"type": "CommentDeletedEvent", "createdAt": self.T[6], "authorId": self.OWNER}
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[deleted]))), "block")
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[dict(deleted, authorId=7)]))), "pass")
        self.assertEqual(self.status(dict(self.both(), graphql=dict(self.both()["graphql"], events=[dict(deleted, createdAt=self.T[0])]))), "pass")

    def test_a_dismissed_owner_review_voids_with_or_without_its_event(self):
        dismissed = self.review("looks fine", self.T[0], state="DISMISSED")
        event = {"type": "ReviewDismissedEvent", "createdAt": self.T[6], "authorId": self.OWNER}
        doc = self.both(); doc["reviews"] = [dismissed]
        self.assertEqual(self.status(dict(doc, graphql=dict(doc["graphql"], events=[event]))), "block")
        self.assertEqual(self.status(doc), "block")
        early = dict(event, createdAt=self.T[0])
        self.assertEqual(self.status(dict(doc, graphql=dict(doc["graphql"], events=[early]))), "pass")

    def test_unreadable_evidence_is_indeterminate(self):
        self.assertEqual(self.status(self.doc(comments=self.both()["comments"], complete=False)), "indeterminate")
        no_graph = self.both(); no_graph.pop("graphql")
        self.assertEqual(self.status(no_graph), "indeterminate")
        bad_time = self.doc(comments=[self.comment(self.CLEAN, "yesterday"), self.comment(self.RECOMMEND, self.T[2])])
        self.assertEqual(self.status(bad_time), "indeterminate")
        bad_event = dict(self.both(), graphql=dict(self.both()["graphql"], events=[{"type": "CommentDeletedEvent", "authorId": self.OWNER}]))
        self.assertEqual(self.status(bad_event), "indeterminate")

    def test_the_command_line_exit_codes(self):
        script = str(ROOT / "scripts" / "merge_gate_verdict.py")
        run = lambda text: subprocess.run(["python3", script], input=text, capture_output=True, text=True)
        self.assertEqual(run(json.dumps(self.both())).returncode, 0)
        self.assertEqual(run(json.dumps(self.doc())).returncode, 1)
        self.assertEqual(run(json.dumps(self.doc(complete=False))).returncode, 2)
        for text in ("not json", "{}", "[]"):
            with self.subTest(text=text):
                self.assertEqual(run(text).returncode, 2)

    def test_an_edit_or_a_deletion_by_an_author_that_cannot_be_read_voids(self):
        ghost_edit = dict(self.both(), graphql=dict(self.both()["graphql"], comments=[{"authorId": None, "lastEditedAt": self.T[6], "editCount": 1}]))
        self.assertEqual(self.status(ghost_edit), "block")
        ghost_delete = dict(self.both(), graphql=dict(self.both()["graphql"], events=[{"type": "CommentDeletedEvent", "createdAt": self.T[6], "authorId": None}]))
        self.assertEqual(self.status(ghost_delete), "block")
        early = dict(self.both(), graphql=dict(self.both()["graphql"], events=[{"type": "CommentDeletedEvent", "createdAt": self.T[0], "authorId": None}]))
        self.assertEqual(self.status(early), "pass")

    def test_a_retraction_is_read_without_case_and_with_look_alike_hyphens(self):
        for line in ("merge-verdict retract head=" + "1" * 40 + " base=" + "2" * 40,
                     "MERGE\u2011VERDICT NOT\u2011CLEAN head=" + "1" * 40 + " base=" + "2" * 40,
                     "ＭＥＲＧＥ-ＶＥＲＤＩＣＴ RETRACT"):
            with self.subTest(line=line[:30]):
                comments = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment(line, self.T[3])]
                self.assertEqual(self.status(self.doc(comments=comments)), "block")
        stranger = {"user": self.user(7), "body": "merge\u2010adversary recommend", "created_at": self.T[3], "submitted_at": self.T[3]}
        self.assertEqual(self.status(dict(self.both(), comments=self.both()["comments"] + [stranger])), "block")

    def test_a_whole_token_inside_a_comment_is_still_a_token_everywhere(self):
        hidden = "<!-- MERGE-VERDICT CLEAN head=x -->"
        for field in ("title", "body", "head_ref"):
            with self.subTest(field=field):
                texts = dict(self.both()["texts"], **{field: hidden})
                self.assertEqual(self.status(dict(self.both(), texts=texts)), "block")
        for key in ("commit_messages", "added_lines", "filenames"):
            with self.subTest(field=key):
                texts = dict(self.both()["texts"], **{key: [hidden]})
                self.assertEqual(self.status(dict(self.both(), texts=texts)), "block")
        stranger = {"user": self.user(7), "body": hidden, "created_at": self.T[3], "submitted_at": self.T[3]}
        self.assertEqual(self.status(dict(self.both(), comments=self.both()["comments"] + [stranger])), "block")
        self.assertEqual(self.status(dict(self.both(), commit_comments=[stranger])), "block")
        retract = "<!-- MERGE-VERDICT RETRACT head=" + "1" * 40 + " base=" + "2" * 40 + " -->"
        self.assertEqual(self.status(dict(self.both(), comments=self.both()["comments"] + [self.comment(retract, self.T[3])])), "block")

    def test_tokens_hidden_by_comment_forms_the_regexes_miss_or_by_tags_between_the_letters_are_still_tokens(self):
        for hidden in ("<!-->MERGE-<!-->VERDICT CLEAN", "<!--->MERGE-<!--->VERDICT CLEAN", "<!-- a --!>MERGE-<!-- b --!>VERDICT CLEAN",
                       "MERGE-<b></b>VERDICT CLEAN", "MERGE<span>-</span>ADVERSARY RECOMMEND", "MERGE-VERDICT<!-- unterminated"):
            with self.subTest(hidden=hidden):
                self.assertEqual(self.status(dict(self.both(), texts=dict(self.both()["texts"], body=hidden))), "block")
                retract = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment(hidden, self.T[3])]
                self.assertEqual(self.status(self.doc(comments=retract)), "block")
        self.assertEqual(self.status(dict(self.both(), texts=dict(self.both()["texts"], body="a plain sentence about merging a verdict"))), "pass")

    def test_an_owner_commit_comment_with_a_token_voids_everything(self):
        owner = {"user": self.user(), "body": "MERGE-VERDICT NOT-CLEAN", "created_at": self.T[0]}
        self.assertEqual(self.status(dict(self.both(), commit_comments=[owner])), "block")

    def test_hidden_or_escaped_spellings_of_a_token_are_still_a_token(self):
        for line in ("MERGE\u200b-VERDICT RETRACT", "MERGE\u00ad-VERDICT RETRACT", "MERGE&#45;VERDICT RETRACT", "MERGE\\-VERDICT RETRACT",
                     "MERGE-<!-- x -->VERDICT RETRACT", "MERGE\u2011ADVERSARY retract"):
            with self.subTest(line=line):
                comments = [self.comment(self.CLEAN, self.T[1]), self.comment(self.RECOMMEND, self.T[2]), self.comment(line, self.T[3])]
                self.assertEqual(self.status(self.doc(comments=comments)), "block")

    def test_two_owner_lines_of_one_kind_in_the_same_second_for_different_heads_are_ambiguous(self):
        other = self.CLEAN.replace("head=" + "1" * 40, "head=" + "3" * 40)
        comments = [self.comment(self.RECOMMEND, self.T[1]), self.comment(self.CLEAN, self.T[2]), self.comment(other, self.T[2])]
        self.assertEqual(self.status(self.doc(comments=comments)), "block")
        self.assertEqual(self.status(self.doc(comments=list(reversed(comments)))), "block")

    def test_an_owner_review_that_asks_for_changes_or_was_dismissed_is_never_a_verdict(self):
        for state in ("CHANGES_REQUESTED", "DISMISSED"):
            with self.subTest(state=state):
                doc = self.doc(reviews=[self.review(self.CLEAN, self.T[1], state=state)], comments=[self.comment(self.RECOMMEND, self.T[2])])
                self.assertEqual(self.status(doc), "block")

    def test_an_owner_inline_comment_with_a_token_voids_everything_whatever_its_time(self):
        inline = {"user": self.user(), "body": "MERGE-VERDICT NOT-CLEAN", "created_at": self.T[0]}
        self.assertEqual(self.status(dict(self.both(), inline_comments=[inline])), "block")

    def test_a_malformed_item_is_indeterminate_not_a_block(self):
        doc = self.both(); doc["comments"] = doc["comments"] + ["not an object"]
        self.assertEqual(subprocess.run(["python3", str(ROOT / "scripts" / "merge_gate_verdict.py")], input=json.dumps(doc), capture_output=True, text=True).returncode, 2)

    def test_the_owner_id_is_the_numeric_id_of_the_owner_account(self):
        self.assertEqual(self.mod.OWNER_ID, 57982425)


class PrivacyScanDecodesUtf16(unittest.TestCase):
    """A value in text written in UTF-16 is spaced out by NUL bytes, which hides it from
    every pattern, so a caller that hands the classifier the raw bytes must still
    get it read as the text it stands for. Digits are synthetic and built from
    parts: this file is scanned by the gate it tests."""

    RUN = "12345" + "678901"
    LINE = "Account reference: " + RUN + "\n"

    def setUp(self):
        self.privacy = load_privacy_module()

    def utf8_view(self, raw):
        """What a reader that assumes UTF-8 sees: NUL is a valid UTF-8 character."""
        return raw.decode("utf-8")

    def blockers(self, text):
        return self.privacy.scan(text, HEAD)["blockers"]

    def run_main(self, raw, locale="C"):
        # A stream is decoded by the process locale unless the reader decodes the bytes itself.
        done = subprocess.run(
            ["python3", "-I", str(PRIVACY_MODULE_PATH), "--head", HEAD],
            input=raw, capture_output=True, env=dict(os.environ, LC_ALL=locale, LANG=locale),
        )
        self.assertEqual(done.returncode, 0, done.stderr)
        return json.loads(done.stdout)

    def test_a_long_digit_run_in_utf16le_without_a_bom_is_blocked(self):
        text = self.utf8_view(self.LINE.encode("utf-16-le"))
        self.assertTrue(any("long digit run" in b for b in self.blockers(text)))

    def test_a_long_digit_run_in_utf16be_without_a_bom_is_blocked(self):
        text = self.utf8_view(self.LINE.encode("utf-16-be"))
        self.assertTrue(any("long digit run" in b for b in self.blockers(text)))

    def test_a_utf16_segment_among_utf8_lines_is_decoded_and_the_rest_is_untouched(self):
        raw = b"title line\n+" + self.LINE.encode("utf-16-le") + b"+" + b"plain line\n"
        record = self.privacy.scan(self.utf8_view(raw), HEAD)
        self.assertTrue(any("long digit run" in b for b in record["blockers"]), record)
        self.assertTrue(any("UTF-16" in n for n in record["notes"]), record)

    def test_a_value_that_is_the_whole_text_loses_no_character_to_the_decoding(self):
        # The run starts at the first digit and ends at the last: dropping either one leaves ten.
        for codec in ("utf-16-le", "utf-16-be"):
            with self.subTest(codec=codec):
                text = self.utf8_view(self.RUN.encode(codec))
                self.assertTrue(any("long digit run" in b for b in self.blockers(text)))

    def test_digits_outside_ascii_after_a_byte_order_mark_are_decoded_whole(self):
        # Python reads these as digits too; each code unit has no NUL beside it, so only the mark finds them.
        digits = "".join(chr(0x0966 + int(c)) for c in "123456789012")
        for codec, bom in (("utf-16-le", b"\xff\xfe"), ("utf-16-be", b"\xfe\xff")):
            with self.subTest(codec=codec):
                record = self.run_main(bom + digits.encode(codec))
                self.assertTrue(any("long digit run" in b for b in record["blockers"]), record)

    def test_a_short_run_next_to_plain_digits_is_joined_to_them_on_either_side(self):
        # Each side of the NUL is needed: dropping one leaves the run undecoded and the digits split.
        # The digits are chosen so that neither half alone is an identifier shape or a long run.
        for raw in ("123456789".encode() + "01".encode("utf-16-le"), "01".encode("utf-16-be") + "987654321".encode()):
            with self.subTest(raw=raw):
                self.assertTrue(any("long digit run" in b for b in self.blockers(self.utf8_view(raw))))

    def test_latin_1_letters_beside_nul_bytes_are_not_read_as_utf16(self):
        # Valid UTF-16 text never reaches the scan this way: a code unit above 0x7f is not valid UTF-8 beside a NUL.
        record = self.privacy.scan("\u00e9\x00\u00e9\x00\u00e9\x00", HEAD)
        self.assertFalse(any("UTF-16" in n for n in record["notes"]), record)

    def test_decoding_a_run_never_hides_a_number_the_original_text_shows(self):
        # The decoded run is "ab", a word that now touches the digits and removes the boundary the
        # patterns need before a grouped number; the original, with its NULs, still has the boundary.
        run = "a\x00b\x00"
        for number in ("1234-5678-9012", "011-2345-6789", "022 23456789"):
            with self.subTest(number=number):
                text = run + number
                self.assertTrue(self.privacy.scan(text, HEAD)["blockers"], text)

    def test_text_that_has_a_nul_run_is_blocked_whenever_the_same_text_without_decoding_is(self):
        import random
        random.seed(20261015)
        parts = ["a\x00b\x00", "\x00x\x00y", " ", "-", "1234-5678-9012", "011-2345-6789", "022 23456789",
                 "98765" + "43210", "12345" + "678901", "(", ")", "ab", "x" + "@" + "example.com", "C" + ":", "/Us" + "ers/", "z"]
        original_only = mock.patch.object(self.privacy, "decode_utf16_runs", lambda text: (text, 0))
        checked = 0
        for _ in range(3000):
            text = "".join(random.choice(parts) for _ in range(random.randint(1, 6)))
            if "\x00" not in text:
                continue
            with original_only:
                before = bool(self.privacy.scan(text, HEAD)["blockers"])
            if before:
                checked += 1
                self.assertTrue(self.privacy.scan(text, HEAD)["blockers"], repr(text))
        self.assertGreater(checked, 100, "the generator must reach inputs the original reading blocks")

    def test_a_mark_never_hides_what_the_utf8_reading_shows(self):
        digits = self.RUN + "\n"
        for name, raw in (
            ("utf-8 digits after a utf-16 file", b"\xff\xfe" + "hello\n".encode("utf-16-le") + digits.encode()),
            ("plain digits after a bare mark", b"\xff\xfe" + digits.encode()),
            ("big-endian mark over little-endian body", b"\xfe\xff" + digits.encode("utf-16-le")),
            ("two marked files", b"\xff\xfe" + "ab\n".encode("utf-16-le") + b"\xfe\xff" + digits.encode("utf-16-be")),
        ):
            with self.subTest(name=name):
                record = self.run_main(raw)
                self.assertTrue(any("long digit run" in b for b in record["blockers"]), record)

    def test_harmless_utf16_text_is_not_blocked_and_is_noted(self):
        text = self.utf8_view("Release notes for the build.\n".encode("utf-16-le"))
        record = self.privacy.scan(text, HEAD)
        self.assertEqual(record["blockers"], [], record)
        self.assertTrue(any("UTF-16" in n for n in record["notes"]), record)

    def test_text_without_nul_bytes_is_scanned_exactly_as_before(self):
        record = self.privacy.scan("Release notes for the build.\n", HEAD)
        self.assertEqual(record["blockers"], [])
        self.assertFalse(any("UTF-16" in n for n in record["notes"]), record)

    def test_a_single_stray_nul_is_not_read_as_utf16(self):
        record = self.privacy.scan("a\x00b and nothing else\n", HEAD)
        self.assertEqual(record["blockers"], [])
        self.assertFalse(any("UTF-16" in n for n in record["notes"]), record)

    def test_a_located_line_in_utf16_is_reported_by_location_without_its_value(self):
        document = {"sources": [], "added": [{"path": "docs/example.md", "line": 5,
                    "text": self.utf8_view(self.LINE.encode("utf-16-le"))}]}
        result = self.privacy.explain(document, HEAD)
        self.assertEqual([h["where"] for h in result["hits"]], ["docs/example.md:5"])
        self.assertNotIn(self.RUN, json.dumps(result))

    def test_bytes_that_are_not_utf8_are_indeterminate_whatever_the_locale(self):
        unreadable = ["could not read privacy scan input"]
        for locale in ("C", "en_US.UTF-8"):
            for raw in (b"\x80\x81 not text", b"\xff\xfe" + b"abc"):
                with self.subTest(locale=locale, raw=raw):
                    record = self.run_main(raw, locale)
                    self.assertEqual(record["indeterminate"], unreadable)
                    self.assertEqual(record["blockers"], [])


class MergeWrapperControls(unittest.TestCase):
    """scripts/merge-gate-merge.sh: runs the gate, then ready and a head-pinned squash merge, refusing anything that does not match.

    The wrapper is copied beside a stub gate (it always runs the gate in its own directory) and called with a stub `gh` that logs
    every call, so each case can say which commands were and were not run.
    """

    GATED = "a" * 40
    GATE_PASS = (
        "PR #321 (ComplyEaze/bridge)  head=aaaaaaa  base=master\n"
        "  ok            something\n\n"
        "MAY MERGE \u2014 checks passed for aaaaaaa.\n"
        "Branch protection on master enforces the remaining required status checks separately.\n"
        '  [ "$(gh pr view 321 --repo ComplyEaze/bridge --json baseRefName -q .baseRefName)" = "master" ] \\\n'
        "    && gh pr merge 321 --repo ComplyEaze/bridge --squash --match-head-commit " + "a" * 40 + "\n"
    )
    VIEW = {"state": "OPEN", "isDraft": False, "headRefOid": "a" * 40, "baseRefName": "master"}

    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix="merge-wrapper-")
        cls.root = Path(cls.tmp.name)
        (cls.root / "scripts").mkdir()
        shutil.copyfile(ROOT / "scripts" / "merge-gate-merge.sh", cls.root / "scripts" / "merge-gate-merge.sh")
        os.chmod(cls.root / "scripts" / "merge-gate-merge.sh", 0o755)
        gate = cls.root / "scripts" / "merge-gate.sh"
        gate.write_text('#!/usr/bin/env bash\ncat "$STUB_GATE_OUT"\nexit "${STUB_GATE_EXIT:-0}"\n')
        gate.chmod(0o755)
        gh = cls.root / "gh"
        gh.write_text(
            "#!/usr/bin/env bash\n"
            'echo "$*" >>"$GH_LOG"\n'
            'case "$1 $2" in\n'
            '  "pr view") [ -z "${GH_VIEW_FAIL:-}" ] || exit 1; n=$(grep -c "^pr view" "$GH_LOG"); if [ "$n" -ge 3 ] && [ -f "${GH_VIEW3:-/nonexistent}" ]; then cat "$GH_VIEW3"; exit 0; fi; if [ "$n" -ge 2 ] && [ -n "${GH_VIEW2_FAIL:-}" ]; then exit 1; fi; if [ "$n" -ge 2 ] && [ -f "${GH_VIEW2:-/nonexistent}" ]; then cat "$GH_VIEW2"; else cat "$GH_VIEW"; fi ;;\n'
            '  "pr ready") exit "${GH_READY_EXIT:-0}" ;;\n'
            '  "pr merge") [ -z "${GH_MERGE_SLEEP:-}" ] || sleep "$GH_MERGE_SLEEP"; exit "${GH_MERGE_EXIT:-0}" ;;\n'
            "  *) exit 99 ;;\n"
            "esac\n"
        )
        gh.chmod(0o755)
        if not shutil.which("jq"):
            raise RuntimeError("jq is required for the merge wrapper controls")

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def run_wrapper(self, args=("321", "--repo", "ComplyEaze/bridge"), gate_out=None, gate_exit=0, view=None, view_fail=False,
                    merge_exit=0, ready_exit=0, view2=None, view2_fail=False, view3=None, merge_sleep=None, signal_after=None):
        work = Path(tempfile.mkdtemp(dir=self.root))
        (work / "gate-out").write_text(self.GATE_PASS if gate_out is None else gate_out)
        (work / "view").write_text(json.dumps(self.VIEW if view is None else view))
        log = work / "gh.log"
        log.write_text("")
        if view2 is not None:
            (work / "view2").write_text(json.dumps(view2))
        env = os.environ.copy()
        env["GH_VIEW2"] = str(work / "view2")
        if view2_fail:
            env["GH_VIEW2_FAIL"] = "1"
        if view3 is not None:
            (work / "view3").write_text(json.dumps(view3))
            env["GH_VIEW3"] = str(work / "view3")
        if merge_sleep is not None:
            env["GH_MERGE_SLEEP"] = str(merge_sleep)
        env.update(PATH=f"{self.root}:{env['PATH']}", STUB_GATE_OUT=str(work / "gate-out"), STUB_GATE_EXIT=str(gate_exit),
                   GH_LOG=str(log), GH_VIEW=str(work / "view"), GH_MERGE_EXIT=str(merge_exit), GH_READY_EXIT=str(ready_exit))
        if view_fail:
            env["GH_VIEW_FAIL"] = "1"
        if signal_after is not None:
            import signal as signal_module
            import time
            proc = subprocess.Popen([str(self.root / "scripts" / "merge-gate-merge.sh"), *args], env=env, text=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            deadline = time.time() + 20
            while time.time() < deadline and not any(line.startswith("pr merge") for line in log.read_text().splitlines()):
                time.sleep(0.05)
            proc.send_signal(signal_module.SIGTERM)
            out, err = proc.communicate(timeout=30)
            result = subprocess.CompletedProcess(proc.args, proc.returncode, out, err)
        else:
            result = subprocess.run([str(self.root / "scripts" / "merge-gate-merge.sh"), *args], env=env, text=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        calls = [line for line in log.read_text().splitlines() if line]
        return result, calls

    def assert_refused(self, result, calls, phrase):
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn(phrase, result.stdout + result.stderr)
        self.assertFalse([c for c in calls if c.startswith(("pr ready", "pr merge"))], calls)

    def test_a_passing_gate_and_a_matching_head_is_merged_pinned_to_the_gated_head(self):
        result, calls = self.run_wrapper()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual([c for c in calls if c.startswith(("pr ready", "pr merge"))],
                         ["pr merge 321 --repo ComplyEaze/bridge --squash --match-head-commit " + self.GATED])

    def test_a_draft_is_made_ready_first_and_only_then_merged(self):
        result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        acts = [c for c in calls if c.startswith(("pr ready", "pr merge"))]
        self.assertEqual(acts, ["pr ready 321 --repo ComplyEaze/bridge",
                                "pr merge 321 --repo ComplyEaze/bridge --squash --match-head-commit " + self.GATED])

    def test_a_ready_that_fails_stops_before_the_merge(self):
        result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True), ready_exit=1)
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse([c for c in calls if c.startswith("pr merge")], calls)

    def test_a_draft_stays_a_draft_when_the_merge_does_not_happen(self):
        # the merge fails after the pull request was made ready
        result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True), merge_exit=1)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual([c for c in calls if c.startswith("pr ready")], ["pr ready 321 --repo ComplyEaze/bridge", "pr ready 321 --repo ComplyEaze/bridge --undo"])
        # the head or base moves between the first read and the merge
        for name, moved in (("head", dict(self.VIEW, isDraft=True, headRefOid="b" * 40)), ("base", dict(self.VIEW, isDraft=True, baseRefName="other")),
                            ("state", dict(self.VIEW, isDraft=True, state="CLOSED"))):
            with self.subTest(moved=name):
                result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True), view2=moved)
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                self.assertEqual([c for c in calls if c.startswith("pr ready")], ["pr ready 321 --repo ComplyEaze/bridge", "pr ready 321 --repo ComplyEaze/bridge --undo"])
                self.assertFalse([c for c in calls if c.startswith("pr merge")], calls)

    def test_a_merge_that_gh_reports_as_failed_but_that_happened_is_not_undone(self):
        draft = dict(self.VIEW, isDraft=True)
        result, calls = self.run_wrapper(view=draft, merge_exit=1, view3=dict(draft, state="MERGED"))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("merge requested", result.stdout)
        self.assertFalse([c for c in calls if "--undo" in c], calls)

    def test_an_interrupt_during_the_merge_says_what_state_the_pull_request_may_be_in_and_does_not_undo_it(self):
        result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True), merge_sleep=2, signal_after=True)
        self.assertEqual(result.returncode, 143, result.stdout + result.stderr)
        self.assertIn("interrupted", result.stderr)
        self.assertFalse([c for c in calls if "--undo" in c], calls)

    def test_a_second_read_that_fails_refuses_and_returns_a_draft_to_draft(self):
        result, calls = self.run_wrapper(view=dict(self.VIEW, isDraft=True), view2_fail=True)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("pr ready 321 --repo ComplyEaze/bridge --undo", calls)
        self.assertFalse([c for c in calls if c.startswith("pr merge")], calls)

    def test_a_head_or_base_that_moves_after_the_first_read_is_refused_before_the_merge(self):
        for name, moved in (("head", dict(self.VIEW, headRefOid="b" * 40)), ("base", dict(self.VIEW, baseRefName="other"))):
            with self.subTest(moved=name):
                result, calls = self.run_wrapper(view2=moved)
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                self.assertFalse([c for c in calls if c.startswith(("pr ready", "pr merge"))], calls)

    def test_a_gate_that_blocks_or_cannot_decide_is_never_merged(self):
        for code in (1, 2, 3):
            with self.subTest(code=code):
                result, calls = self.run_wrapper(gate_exit=code, gate_out="MUST NOT MERGE\n")
                self.assert_refused(result, calls, "the gate did not pass")

    def test_a_zero_exit_without_the_may_merge_line_is_refused(self):
        result, calls = self.run_wrapper(gate_out=self.GATE_PASS.replace("MAY MERGE", "may merge"))
        self.assert_refused(result, calls, "MAY MERGE")

    def test_a_merge_command_that_is_missing_repeated_or_for_another_pull_request_is_refused(self):
        command = "    && gh pr merge 321 --repo ComplyEaze/bridge --squash --match-head-commit " + self.GATED + "\n"
        cases = {
            "missing": self.GATE_PASS.replace(command, ""),
            "repeated": self.GATE_PASS + command,
            "another pull request": self.GATE_PASS.replace("merge 321", "merge 322"),
            "another repository": self.GATE_PASS.replace("--repo ComplyEaze/bridge --squash", "--repo someone/bridge --squash"),
            "a short sha": self.GATE_PASS.replace(self.GATED, "a" * 39),
            "an upper-case sha": self.GATE_PASS.replace(self.GATED, "A" * 40),
            "a non-hex sha": self.GATE_PASS.replace(self.GATED, "g" * 40),
            "an extra flag": self.GATE_PASS.replace("--squash", "--squash --admin"),
        }
        for name, out in cases.items():
            with self.subTest(case=name):
                result, calls = self.run_wrapper(gate_out=out)
                self.assert_refused(result, calls, "merge command")

    def test_a_missing_or_repeated_base_check_line_is_refused(self):
        base_line = '  [ "$(gh pr view 321 --repo ComplyEaze/bridge --json baseRefName -q .baseRefName)" = "master" ] \\\n'
        for name, out in {"missing": self.GATE_PASS.replace(base_line, ""), "repeated": self.GATE_PASS + base_line}.items():
            with self.subTest(case=name):
                result, calls = self.run_wrapper(gate_out=out)
                self.assert_refused(result, calls, "base")

    def test_a_head_that_moved_since_the_gate_is_refused_and_nothing_is_run(self):
        result, calls = self.run_wrapper(view=dict(self.VIEW, headRefOid="b" * 40))
        self.assert_refused(result, calls, "head moved")

    def test_another_base_or_a_closed_pull_request_is_refused(self):
        for name, view in {"base": dict(self.VIEW, baseRefName="other"), "closed": dict(self.VIEW, state="CLOSED"),
                           "merged": dict(self.VIEW, state="MERGED")}.items():
            with self.subTest(case=name):
                result, calls = self.run_wrapper(view=view)
                self.assert_refused(result, calls, "pull request")

    def test_a_view_that_cannot_be_read_is_refused(self):
        result, calls = self.run_wrapper(view_fail=True)
        self.assert_refused(result, calls, "could not read")
        result, calls = self.run_wrapper(view={"state": "OPEN"})
        self.assert_refused(result, calls, "could not read")

    def test_a_merge_that_gh_refuses_is_a_failure_not_a_success(self):
        result, calls = self.run_wrapper(merge_exit=1)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("merged", result.stdout.lower().replace("not merged", ""))

    def test_a_dry_run_runs_the_gate_and_the_checks_but_neither_ready_nor_merge(self):
        result, calls = self.run_wrapper(args=("321", "--repo", "ComplyEaze/bridge", "--dry-run"), view=dict(self.VIEW, isDraft=True))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("dry run", result.stdout)
        self.assertFalse([c for c in calls if c.startswith(("pr ready", "pr merge"))], calls)

    def test_bad_arguments_are_a_usage_error_and_run_nothing(self):
        for args in ((), ("abc",), ("321", "--repo", "nonsense"), ("321", "--repo", "a/b", "--unknown"), ("321", "extra"),
                     ("321", "--repo")):
            with self.subTest(args=args):
                result, calls = self.run_wrapper(args=args)
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertEqual(calls, [])


if __name__ == "__main__":
    unittest.main()
