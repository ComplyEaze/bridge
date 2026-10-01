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

import importlib.util
import os
import shutil
import stat
import subprocess
import tempfile
import unittest
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

MKTEMP_WRAPPER = """#!/usr/bin/env bash
# Only scan-input-write-failure pre-occupies the privacy-scan-input path (as
# a directory) so the write inside merge-gate.sh's redirect group fails.
real=$(command -v -p mktemp)
if [ "${GATE_SCENARIO:-}" = "scan-input-write-failure" ] && [ "$1" = "-d" ]; then
  dir=$("$real" -d)
  mkdir "$dir/privacy-scan-input"
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
        gh = cls.bin / "gh"
        shutil.copyfile(FAKE_GH, gh)
        gh.chmod(0o755)
        mktemp = cls.bin / "mktemp"
        mktemp.write_text(MKTEMP_WRAPPER)
        mktemp.chmod(mktemp.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)
        if not shutil.which("jq"):
            raise RuntimeError("jq is required for merge-gate controls")

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def run_gate(self, scenario="pass", extra_args=(), cwd=None, repo="lamemustafa/bridge"):
        env = os.environ.copy()
        env["PATH"] = f"{self.bin}:{env['PATH']}"
        env["GATE_SCENARIO"] = scenario
        return subprocess.run(
            [str(SCRIPT), "321", "--repo", repo, *extra_args],
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
        self.assert_pass("pass", "review evidence names the current head")

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
        result = self.assert_pass("ack-exact", "review evidence names the current head")
        self.assertIn(ACK_PATH + " lists the 1 touched pinned path(s), reviewer reviewer", result.stdout)
        self.assertIn("names " + SHORT + ", " + ACK_PATH + " and all pinned paths touched", result.stdout)

    def test_ack_exact_via_a_review_body_passes(self):
        self.assert_pass("ack-exact-via-review", "all pinned paths touched")

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
        self.assert_pass("pin-only-change-acked", "all pinned paths touched")

    def test_pin_added_without_a_reason_blocks(self):
        self.assert_blocked("pin-added-without-reason", "pin(s) added without a non-empty reason: src/new.rs")

    def test_pin_removed_without_a_removed_pin_line_blocks(self):
        self.assert_blocked("pin-removed-no-line", "lacks a removed-pin line for: src/other.rs")

    def test_pin_removed_with_a_removed_pin_line_passes(self):
        self.assert_pass("pin-removed-with-line", "all pinned paths touched")

    # -- Branch history: a pin the branch added and then lost ----------------------

    def test_a_pin_the_branch_added_and_lost_blocks_without_a_removed_pin_line(self):
        self.assert_blocked(
            "history-own-pin-lost-undeclared",
            "pinned by a commit of this PR but not in the pin list at the head; restore the pin, or declare the withdrawal with a removed-pin line: src/own.rs",
        )

    def test_a_pin_the_branch_added_and_lost_passes_when_the_withdrawal_is_declared(self):
        self.assert_pass("history-own-pin-lost-declared", "all pinned paths touched")

    def test_a_lost_pin_named_as_a_path_in_the_ack_blocks(self):
        self.assert_blocked("history-own-pin-lost-ack-names-the-path", "lists path(s) the PR does not touch: src/own.rs")

    def test_a_lost_pin_needs_an_ack_even_when_nothing_else_pinned_changed(self):
        self.assert_blocked("history-own-pin-lost-no-ack", "pinned path(s) touched")

    def test_a_pin_the_branch_added_and_kept_is_not_withdrawn(self):
        self.assert_pass("history-pin-kept-at-the-head", "all pinned paths touched")

    def test_a_merge_commit_is_compared_with_all_of_its_parents(self):
        self.assert_pass("history-merge-parent-compared-with-all", "all pinned paths touched")

    def test_an_unreadable_pin_list_in_the_branch_history_is_indeterminate(self):
        self.assert_indeterminate("history-commit-list-unreadable", "could not read and parse the pin list at every commit of the PR")

    def test_removed_pin_line_for_a_pin_that_is_kept_blocks(self):
        self.assert_blocked("pin-removed-line-for-a-kept-pin", "has removed-pin line(s) for pin(s) the PR does not remove: src/other.rs")

    # -- The review or comment must name the head, the ack and every touched path ----

    def test_comment_missing_a_touched_file_blocks(self):
        self.assert_blocked("comment-missing-file", "no review or comment names every pinned path touched; the closest lacks: src/other.rs")

    def test_comment_naming_every_touched_file_passes(self):
        self.assert_pass("comment-names-all", "names " + SHORT + ", " + ACK_PATH + " and all pinned paths touched")

    def test_comment_missing_the_ack_path_blocks(self):
        self.assert_blocked("comment-missing-ack-path", "the closest lacks: " + ACK_PATH)

    def test_comment_naming_another_head_blocks(self):
        self.assert_blocked("comment-names-another-head", "no review or comment names head " + SHORT + " together with " + ACK_PATH)

    def test_reviewer_who_is_not_the_acks_reviewer_blocks(self):
        self.assert_blocked("reviewer-not-matching", "is by someone-else, not the acknowledgement's reviewer 'reviewer'")

    def test_reviewer_login_comparison_ignores_case(self):
        self.assert_pass("reviewer-login-case-differs", "all pinned paths touched")

    # -- Renames, stale branches, the cut-over PR -----------------------------------

    def test_rename_of_a_pinned_file_needs_an_ack_for_the_old_path(self):
        self.assert_blocked("rename-old-path-pinned", "but the PR does not add " + ACK_PATH)

    def test_rename_of_a_pinned_file_with_an_ack_for_the_old_path_passes(self):
        result = self.assert_pass("rename-old-path-acked", "all pinned paths touched")
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
        result = self.assert_pass("gitattributes-acked", "all pinned paths touched")
        self.assertIn("lists the 1 touched pinned path(s)", result.stdout)

    def test_gitattributes_pinned_and_edited_with_an_ack_that_omits_it_blocks(self):
        self.assert_blocked("gitattributes-wrong-ack", "omits touched pinned path(s): .gitattributes")

    def test_gitattributes_edited_but_not_pinned_needs_no_ack(self):
        self.assert_pass("gitattributes-unpinned", "changed files contain no pinned path requiring an acknowledgement")

    # -- removed-pin lines are sorted and unique, like the path lines --------------------

    def test_sorted_removed_pin_lines_pass(self):
        self.assert_pass("removed-pins-sorted", "all pinned paths touched")

    def test_unsorted_removed_pin_lines_block(self):
        self.assert_blocked("removed-pins-unsorted", "removed-pin lines must be sorted and unique")

    def test_duplicate_removed_pin_lines_block(self):
        self.assert_blocked("removed-pins-duplicate", "removed-pin lines must be sorted and unique")

    # -- The reviewer login is a GitHub login (the same expression as the CI checker) ---------

    def test_reviewer_login_shapes(self):
        for scenario in ("reviewer-login-39-chars", "reviewer-login-bot"):
            self.assert_pass(scenario, "all pinned paths touched")
        for scenario in ("reviewer-login-trailing-hyphen", "reviewer-login-leading-hyphen", "reviewer-login-40-chars"):
            self.assert_blocked(scenario, "needs exactly one 'reviewer: <github login>' line")

    # -- The reason on an added pin: non-empty, at most 500 characters, no control character --

    def test_pin_reason_of_501_characters_blocks(self):
        self.assert_blocked("pin-reason-501-chars", "pin(s) added with a reason over 500 characters or containing a control character: src/new.rs")

    def test_pin_reason_of_500_astral_characters_passes(self):
        self.assert_pass("pin-reason-500-astral-chars", "all pinned paths touched")

    def test_pin_reason_with_a_control_character_blocks(self):
        self.assert_blocked("pin-reason-control-char", "reason over 500 characters or containing a control character")

    # -- An ack may only be added: a rename of an ack is a delete plus an add ------------------

    def test_an_ack_renamed_out_of_the_directory_blocks_with_a_pinned_change(self):
        self.assert_blocked("ack-renamed-out-with-pin-change", "acknowledgement files are append-only; the PR also changes: " + ACK_DIR + "/pr-100.txt")

    def test_an_ack_renamed_out_of_the_directory_blocks_without_a_pinned_change(self):
        self.assert_blocked("ack-renamed-out-no-pin-change", "no pinned path is touched, so no acknowledgement may be added or changed: " + ACK_DIR + "/pr-100.txt")

    def test_an_ack_that_git_pairs_with_an_unrelated_deleted_file_is_still_an_add(self):
        self.assert_pass("ack-added-as-a-rename-from-outside", "all pinned paths touched")

    # -- Copies: the destination counts, the source only when it is itself pinned ----------------

    def test_a_copy_of_a_pinned_file_counts_the_pinned_source(self):
        self.assert_blocked("copy-of-a-pinned-file", "but the PR does not add " + ACK_PATH)
        result = self.assert_pass("copy-of-a-pinned-file-acked", "all pinned paths touched")
        self.assertIn("lists the 1 touched pinned path(s)", result.stdout)

    def test_a_copy_of_an_unpinned_file_needs_no_ack(self):
        self.assert_pass("copy-of-an-unpinned-file", "changed files contain no pinned path requiring an acknowledgement")

    # -- A path is named in the review or comment only as a whole token ----------------------------

    def test_a_longer_path_does_not_name_a_shorter_one(self):
        self.assert_blocked("comment-names-a-longer-path", "the closest lacks: src/example.rs")

    def test_a_longer_ack_path_does_not_name_the_ack(self):
        self.assert_blocked("comment-names-a-longer-ack-path", "the closest lacks: " + ACK_PATH)

    def test_paths_in_backticks_quotes_parentheses_commas_and_colons_are_named(self):
        self.assert_pass("comment-names-paths-in-punctuation", "all pinned paths touched")

    def test_a_path_with_spaces_and_parentheses_can_be_named(self):
        self.assert_pass("comment-names-a-path-with-spaces", "all pinned paths touched")

    # -- A large ack: a match must not be lost when grep exits early under pipefail -------------------

    def test_a_large_ack_still_reports_an_early_hex_token(self):
        self.assert_blocked("ack-large-with-early-hex-token", "contains a 64-hex token")

    def test_a_large_ack_still_reports_an_early_control_character(self):
        self.assert_blocked("ack-large-with-early-control-char", "contains a control character")

    # -- Ordering is by UTF-8 bytes (LC_ALL=C), not UTF-16 code units --------------------------------------

    def test_ack_paths_in_utf8_byte_order_pass(self):
        self.assert_pass("ack-utf8-byte-order", "all pinned paths touched")

    def test_ack_paths_in_utf16_order_block(self):
        self.assert_blocked("ack-utf8-utf16-order", "paths must be sorted and unique")

    # -- Privacy / PII scan (KEEP #2) ----------------------------------------

    def test_email_in_title_blocks(self):
        self.assert_blocked("privacy-email-blocker", "customer email shape")

    def test_known_public_agent_address_in_a_well_formed_commit_trailer_is_identity_metadata(self):
        self.assert_pass("public-agent-coauthor-trailer", "review evidence names the current head")

    def test_known_public_agent_address_in_a_well_formed_crlf_commit_trailer_is_identity_metadata(self):
        self.assert_pass("public-agent-coauthor-trailer-crlf", "review evidence names the current head")

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
        self.assert_indeterminate("privacy-uuid-indeterminate", "without exact current-head fixture provenance")

    def test_binary_addition_without_attestation_blocks_and_names_the_path(self):
        result = self.assert_blocked("binary-missing-attestation", "require matching --binary-review-sha and --independent-review-sha")
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
        self.assert_pass("merge-commit-conflict-comment-block", "review evidence names the current head")

    def test_an_address_inside_the_conflict_comment_block_still_blocks(self):
        result = self.assert_blocked("merge-commit-conflict-block-customer-address", "customer email shape")
        self.assertIn("at commit-message-1:", result.stdout)
        self.assertNotIn("company" + ".test", result.stdout)

    def test_binary_addition_with_attestation_passes(self):
        self.assert_pass(
            "binary-with-attestation",
            "have explicit current-head binary and independent review attestations",
            extra_args=("--binary-review-sha", HEAD, "--independent-review-sha", HEAD),
        )

    def test_gitlink_change_is_indeterminate(self):
        self.assert_indeterminate("gitlink-change", "gitlink change(s) require explicit provenance")

    # -- Minimal head-SHA review-evidence check (KEEP #3, new) ---------------

    def test_review_object_naming_head_passes(self):
        self.assert_pass("review-names-head-via-review", "review evidence names the current head")

    def test_summary_comment_naming_head_passes(self):
        self.assert_pass("review-names-head-via-comment", "review evidence names the current head")

    def test_no_review_evidence_blocks(self):
        self.assert_blocked("review-missing", "no review evidence (review or comment) names current head")
        self.assertIn("#317", self.run_gate("review-missing").stdout)

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
        self.assert_blocked("lfs-pointer-binary", "require matching --binary-review-sha and --independent-review-sha")

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


if __name__ == "__main__":
    unittest.main()
