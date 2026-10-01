#!/usr/bin/env python3
"""Offline GitHub CLI fixture for the shrunk ``merge-gate.test.py``.

Replaces ``merge_gate_fake_gh.py`` (deleted: it existed only to test the
identity-binding/PR-body/skipped-CI concerns that were cut). This double
covers only what the shrunk ``merge-gate.sh`` still calls: PR metadata, the
diff, the changed-files list, the compatibility surface (at the head, the base
tip and the PR's merge base), the pinned-path acknowledgement file, the merge
base, reviews, and issue comments.
"""
import base64
import json
import os
import sys

args = sys.argv[1:]
scenario = os.environ.get("GATE_SCENARIO", "pass")

HEAD = "0123456789abcdef0123456789abcdef01234567"
SHORT = HEAD[:7]
BASE_TIP = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
BEHIND_MERGE_BASE = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
DIGEST = "a" * 64
SURFACE_PATH = "docs/tally/compatibility/compatibility-surface.json"
ACK_DIR = "docs/tally/compatibility/acks"
ACK_PATH = ACK_DIR + "/pr-321.txt"
UTF8_LOW = "src/" + chr(0xE000) + ".rs"       # one UTF-16 unit above the surrogates
UTF8_HIGH = "src/" + chr(0x10000) + ".rs"     # UTF-16 sorts this first; UTF-8 bytes sort it last


def emit(value):
    print(value if isinstance(value, str) else json.dumps(value))


def fail(message="controlled API failure"):
    print(message, file=sys.stderr)
    raise SystemExit(1)


def b64(obj_or_text):
    text = obj_or_text if isinstance(obj_or_text, str) else json.dumps(obj_or_text)
    return base64.b64encode(text.encode()).decode()


# ---------------------------------------------------------------------------
# Per-scenario fixture state. Every scenario starts from DEFAULT and overrides
# only what it needs to exercise.
# ---------------------------------------------------------------------------
DEFAULT_FILES = [{"filename": "docs/example.md", "status": "added", "additions": 1, "deletions": 0}]
DEFAULT_DIFF = (
    "diff --git a/docs/example.md b/docs/example.md\n"
    "new file mode 100644\n"
    "index 0000000..1111111\n"
    "--- /dev/null\n"
    "+++ b/docs/example.md\n"
    "@@ -0,0 +1 @@\n"
    "+Safe content line.\n"
)
DEFAULT_PINS = ["src/example.rs", "src/other.rs"]
DEFAULT_SURFACE = {"schema_version": 3, "files": [{"path": p} for p in DEFAULT_PINS]}
DEFAULT_REVIEWS = [{"user": {"login": "reviewer", "type": "User"}, "commit_id": HEAD, "state": "COMMENTED"}]
DEFAULT_COMMENTS = []
DEFAULT_BODY = "## Outcome and reason\n\nA bounded merge preflight.\n"
DEFAULT_COMMIT = {
    "sha": HEAD,
    "parents": [{"sha": BASE_TIP}],
    "commit": {
        "message": "safe commit metadata",
        "author": {"name": "Maintainer", "email": "maintainer@example.invalid"},
        "committer": {"name": "Maintainer", "email": "maintainer@example.invalid"},
    },
    "author": {"login": "author"},
    "committer": {"login": "author"},
}
PUBLIC_AGENT_ADDRESS = "noreply" + "@" + "anthropic.com"
CUSTOMER_ADDRESS = "customer" + "@" + "company.test"


def schema2(paths):
    return {"schema_version": 2, "files": [{"path": p, "sha256": DIGEST} for p in paths]}


def schema3(paths, reasons=None):
    reasons = reasons or {}
    return {"schema_version": 3,
            "files": [({"path": p, "reason": reasons[p]} if p in reasons else {"path": p}) for p in paths]}


def modified_diff(path):
    return (
        f"diff --git a/{path} b/{path}\n"
        "index 1111111..2222222 100644\n"
        f"--- a/{path}\n"
        f"+++ b/{path}\n"
        "@@ -1 +1 @@\n"
        "-old\n"
        "+new\n"
    )


def added_diff(path, text):
    lines = text.rstrip("\n").split("\n")
    return (
        f"diff --git a/{path} b/{path}\n"
        "new file mode 100644\n"
        "index 0000000..1111111\n"
        "--- /dev/null\n"
        f"+++ b/{path}\n"
        f"@@ -0,0 +1,{len(lines)} @@\n"
        + "".join(f"+{line}\n" for line in lines)
    )


def renamed_diff(old, new):
    return (
        f"diff --git a/{old} b/{new}\n"
        "similarity index 100%\n"
        f"rename from {old}\n"
        f"rename to {new}\n"
    )


def ack_text(paths, reviewer="reviewer", removed=()):
    """A well-formed acknowledgement: sorted unique paths, removed pins, one reviewer."""
    lines = sorted(set(paths)) + [f"removed-pin: {p}" for p in sorted(set(removed))]
    lines.append(f"reviewer: {reviewer}")
    return "\n".join(lines) + "\n"


def review_body(*names, head=None):
    """A review/comment body naming the head, the acknowledgement path and each name."""
    return f"Reviewed `{SHORT}`. Acknowledgement `{ACK_PATH}`. Pinned: " + ", ".join(names)


def set_changes(s, *changes):
    """Replace the changed files (and the matching diff) from (kind, ...) tuples.

    ("modified", path) / ("added", path, text) / ("renamed", old, new) / ("removed", path)
    """
    files, diff = [], ""
    for change in changes:
        kind = change[0]
        if kind == "modified":
            files.append({"filename": change[1], "status": "modified", "additions": 1, "deletions": 1})
            diff += modified_diff(change[1])
        elif kind == "added":
            n = len(change[2].rstrip("\n").split("\n"))
            files.append({"filename": change[1], "status": "added", "additions": n, "deletions": 0})
            diff += added_diff(change[1], change[2])
        elif kind == "renamed":
            files.append({"filename": change[2], "status": "renamed", "additions": 0, "deletions": 0,
                          "previous_filename": change[1]})
            diff += renamed_diff(change[1], change[2])
        elif kind == "removed":
            files.append({"filename": change[1], "status": "removed", "additions": 0, "deletions": 1})
            diff += (
                f"diff --git a/{change[1]} b/{change[1]}\n"
                "deleted file mode 100644\n"
                "index 1111111..0000000\n"
                f"--- a/{change[1]}\n"
                "+++ /dev/null\n"
                "@@ -1 +0,0 @@\n"
                "-gone\n"
            )
        else:
            raise SystemExit(f"unknown change kind {kind}")
    s["files"] = files
    s["changed_files_expected"] = len(files)
    s["diff"] = diff


def with_ack(s, text, status="added"):
    """Add the PR's own acknowledgement (as a changed file and as served content)."""
    s["ack_text"] = text
    n = len(text.rstrip("\n").split("\n"))
    s["files"] = s["files"] + [{"filename": ACK_PATH, "status": status, "additions": n, "deletions": 0}]
    s["changed_files_expected"] = len(s["files"])
    s["diff"] += added_diff(ACK_PATH, text)
    s["comments"] = [{"user": {"login": "reviewer", "type": "User"}, "body": review_body(*sorted(
        {ln for ln in text.splitlines() if ln and not ln.startswith(("reviewer:", "removed-pin:"))}
        | {ln[len("removed-pin: "):] for ln in text.splitlines() if ln.startswith("removed-pin: ")}))}]


def state():
    s = {
        "title": "Safe merge gate control",
        "body": DEFAULT_BODY,
        "changed_files_expected": 1,
        "files": list(DEFAULT_FILES),
        "diff": DEFAULT_DIFF,
        "surface_head": dict(DEFAULT_SURFACE),
        "surface_base": dict(DEFAULT_SURFACE),
        "surface_head_fail": False,
        "surface_head_malformed": False,
        "surface_merge_base": None,      # None: the merge base is the base tip
        "surface_at": {},                # commit sha -> surface served at that ref (branch history)
        "surface_at_fail": set(),        # commit shas whose pin list cannot be read
        "merge_base_fail": False,
        "ack_text": None,
        "ack_fail": False,
        "reviews": list(DEFAULT_REVIEWS),
        "comments": list(DEFAULT_COMMENTS),
        "commits": [dict(DEFAULT_COMMIT)],
    }

    if scenario == "surface-fetch-fail":
        s["surface_head_fail"] = True

    elif scenario == "surface-malformed":
        s["surface_head_malformed"] = True

    # -- Surface schema -----------------------------------------------------
    elif scenario == "surface-head-schema3-extra-key":
        # Schema 3 stores no hashes: a `sha256` on a row, or a top-level digest, is refused.
        s["surface_head"] = {"schema_version": 3, "files": [{"path": "src/example.rs", "sha256": DIGEST}]}

    elif scenario == "surface-head-schema3-extra-top-level-key":
        s["surface_head"] = {"schema_version": 3, "manifest_sha256": DIGEST, "files": [{"path": "src/example.rs"}]}

    elif scenario == "surface-head-schema3-reason-not-a-string":
        s["surface_head"] = {"schema_version": 3, "files": [{"path": "src/example.rs", "reason": 5}]}

    elif scenario == "surface-head-schema3-duplicate-path":
        s["surface_head"] = {"schema_version": 3, "files": [{"path": "src/example.rs"}, {"path": "src/example.rs"}]}

    elif scenario == "surface-head-schema3-empty-path":
        s["surface_head"] = {"schema_version": 3, "files": [{"path": ""}]}

    elif scenario == "surface-head-schema2":
        # Schema 2 is read at the base tip only; a head still on it is refused.
        s["surface_head"] = schema2(DEFAULT_PINS)

    elif scenario == "surface-head-schema1":
        s["surface_head"] = {"schema_version": 1, "manifest_sha256": DIGEST,
                             "files": [{"path": "src/example.rs", "sha256": DIGEST}]}

    elif scenario == "surface-base-schema2":
        # The cut-over base (schema 2) is read next to a schema-3 head.
        s["surface_base"] = schema2(DEFAULT_PINS)

    elif scenario == "surface-base-schema1":
        s["surface_base"] = {"schema_version": 1, "manifest_sha256": DIGEST,
                             "files": [{"path": "src/example.rs", "sha256": DIGEST}]}

    elif scenario == "surface-base-schema3-extra-key":
        s["surface_base"] = {"schema_version": 3, "files": [{"path": "src/example.rs", "sha256": DIGEST}]}

    elif scenario == "merge-base-fetch-fail":
        s["merge_base_fail"] = True

    # -- Acknowledgement: touched-set rules ----------------------------------
    elif scenario == "ack-missing":
        set_changes(s, ("modified", "src/example.rs"))

    elif scenario == "ack-exact":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))

    elif scenario == "ack-exact-via-review":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["comments"] = []
        s["reviews"] = [{"user": {"login": "reviewer", "type": "User"}, "commit_id": HEAD, "state": "COMMENTED",
                         "body": review_body("src/example.rs")}]

    elif scenario == "ack-extra-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs", "src/other.rs"]))

    elif scenario == "ack-missing-path":
        set_changes(s, ("modified", "src/example.rs"), ("modified", "src/other.rs"))
        with_ack(s, ack_text(["src/example.rs"]))

    elif scenario == "ack-modified":
        # The PR changes an ack that already exists instead of adding its own.
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]), status="modified")

    elif scenario == "ack-other-modified":
        # It adds its own ack AND edits another PR's ack (append-only).
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        other = ACK_DIR + "/pr-100.txt"
        s["files"] = s["files"] + [{"filename": other, "status": "modified", "additions": 1, "deletions": 1}]
        s["changed_files_expected"] = len(s["files"])
        s["diff"] += modified_diff(other)

    elif scenario == "ack-other-deleted":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        other = ACK_DIR + "/pr-100.txt"
        s["files"] = s["files"] + [{"filename": other, "status": "removed", "additions": 0, "deletions": 1}]
        s["changed_files_expected"] = len(s["files"])
        s["diff"] += (
            f"diff --git a/{other} b/{other}\n"
            "deleted file mode 100644\n"
            "index 1111111..0000000\n"
            f"--- a/{other}\n"
            "+++ /dev/null\n"
            "@@ -1 +0,0 @@\n"
            "-gone\n"
        )

    elif scenario == "ack-with-hash-token":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, "src/example.rs\n" + DIGEST + "\nreviewer: reviewer\n")

    elif scenario == "ack-unsorted":
        set_changes(s, ("modified", "src/example.rs"), ("modified", "src/other.rs"))
        with_ack(s, "src/other.rs\nsrc/example.rs\nreviewer: reviewer\n")

    elif scenario == "ack-duplicate-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, "src/example.rs\nsrc/example.rs\nreviewer: reviewer\n")

    elif scenario == "ack-no-reviewer":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, "src/example.rs\n")

    elif scenario == "ack-two-reviewers":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, "src/example.rs\nreviewer: reviewer\nreviewer: another\n")

    elif scenario == "ack-path-not-a-repository-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, "../src/example.rs\nreviewer: reviewer\n")

    elif scenario == "ack-unreadable":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["ack_fail"] = True

    elif scenario == "ack-without-pinned-change":
        # No pinned path is touched, so no acknowledgement may be added.
        set_changes(s, ("modified", "docs/example.md"))
        with_ack(s, ack_text(["docs/example.md"]))

    elif scenario == "ack-cleanup-only":
        # No pinned path touched and only ack files deleted: an old-ack cleanup passes.
        set_changes(s, ("modified", "docs/example.md"))
        other = ACK_DIR + "/pr-100.txt"
        s["files"] = s["files"] + [{"filename": other, "status": "removed", "additions": 0, "deletions": 1}]
        s["changed_files_expected"] = len(s["files"])
        s["diff"] += (
            f"diff --git a/{other} b/{other}\n"
            "deleted file mode 100644\n"
            "index 1111111..0000000\n"
            f"--- a/{other}\n"
            "+++ /dev/null\n"
            "@@ -1 +0,0 @@\n"
            "-gone\n"
        )

    # -- Pin list edits ------------------------------------------------------
    elif scenario in {"pin-only-change", "pin-only-change-acked", "pin-added-without-reason"}:
        # A pin is added at head; the pinned file itself is unchanged, so only the
        # surface file appears among the changed files.
        reasons = {} if scenario == "pin-added-without-reason" else {"src/new.rs": "Carries the new admission rule."}
        s["surface_head"] = schema3(DEFAULT_PINS + ["src/new.rs"], reasons)
        set_changes(s, ("modified", SURFACE_PATH))
        if scenario != "pin-only-change":
            with_ack(s, ack_text(["src/new.rs"]))

    elif scenario in {"pin-removed-no-line", "pin-removed-with-line"}:
        s["surface_head"] = schema3(["src/example.rs"])
        set_changes(s, ("modified", SURFACE_PATH))
        if scenario == "pin-removed-no-line":
            with_ack(s, ack_text([]))
            s["comments"] = [{"user": {"login": "reviewer", "type": "User"}, "body": review_body("src/other.rs")}]
        else:
            with_ack(s, ack_text([], removed=["src/other.rs"]))

    # -- Branch history: a pin the branch added and then lost ---------------------
    elif scenario.startswith("history-"):
        c1 = "c1" * 20
        p2 = "d2" * 20
        own = "src/own.rs"
        with_own = schema3(DEFAULT_PINS + [own], {own: "decides what is posted"})

        def commit_at(sha, parents):
            commit = dict(DEFAULT_COMMIT)
            commit["sha"] = sha
            commit["parents"] = [{"sha": p} for p in parents]
            return commit

        if scenario == "history-merge-parent-compared-with-all":
            # The pin z reached the branch through a merge's second parent, a master-side commit.
            merge = "e3" * 20
            s["commits"] = [commit_at(merge, [BASE_TIP, p2]), commit_at(HEAD, [merge])]
            s["surface_at"] = {merge: with_own, p2: with_own}
            set_changes(s, ("modified", "src/example.rs"))
            with_ack(s, ack_text(["src/example.rs"]))
        else:
            s["commits"] = [commit_at(c1, [BASE_TIP]), commit_at(HEAD, [c1])]
            s["surface_at"] = {c1: with_own}
            if scenario == "history-own-pin-lost-undeclared":
                set_changes(s, ("modified", "src/example.rs"))
                with_ack(s, ack_text(["src/example.rs"]))
            elif scenario == "history-own-pin-lost-declared":
                set_changes(s, ("modified", "src/example.rs"))
                with_ack(s, ack_text(["src/example.rs"], removed=[own]))
            elif scenario == "history-own-pin-lost-ack-names-the-path":
                set_changes(s, ("modified", "src/example.rs"))
                with_ack(s, ack_text(["src/example.rs", own]))
            elif scenario == "history-own-pin-lost-no-ack":
                pass  # nothing else pinned changed, and no acknowledgement
            elif scenario == "history-pin-kept-at-the-head":
                s["surface_head"] = with_own
                set_changes(s, ("modified", "src/example.rs"), ("modified", SURFACE_PATH))
                with_ack(s, ack_text(["src/example.rs", own]))
            elif scenario.startswith("history-out-of-order-"):
                # An older commit of the branch pinned the new file with the list out of order. Only
                # its paths matter there (scripts/check-surface-ack.mjs reads the same thing).
                unsorted = {"schema_version": 3, "files": [
                    {"path": "src/example.rs"}, {"path": own, "reason": "decides what is posted"}, {"path": "src/other.rs"}]}
                s["surface_at"] = {c1: unsorted}
                if scenario == "history-out-of-order-then-sorted":
                    s["surface_head"] = with_own
                    set_changes(s, ("modified", SURFACE_PATH))
                    with_ack(s, ack_text([own]))
                elif scenario == "history-out-of-order-then-dropped":
                    set_changes(s, ("modified", SURFACE_PATH))
                    with_ack(s, ack_text([own]))
                elif scenario == "history-out-of-order-then-dropped-declared":
                    set_changes(s, ("modified", SURFACE_PATH))
                    with_ack(s, ack_text([], removed=[own]))
                else:
                    fail("unknown history scenario")
            elif scenario == "history-commit-list-unreadable":
                s["surface_at"] = {}
                s["surface_at_fail"] = {c1}
            else:
                fail("unknown history scenario")

    elif scenario == "pin-removed-line-for-a-kept-pin":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"], removed=["src/other.rs"]))

    # -- Review / comment evidence -------------------------------------------
    elif scenario == "comment-missing-file":
        set_changes(s, ("modified", "src/example.rs"), ("modified", "src/other.rs"))
        with_ack(s, ack_text(["src/example.rs", "src/other.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"}, "body": review_body("src/example.rs")}]

    elif scenario == "comment-names-all":
        set_changes(s, ("modified", "src/example.rs"), ("modified", "src/other.rs"))
        with_ack(s, ack_text(["src/example.rs", "src/other.rs"]))

    elif scenario == "comment-missing-ack-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"},
                          "body": f"Reviewed `{SHORT}`: src/example.rs"}]

    elif scenario == "comment-names-another-head":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"},
                          "body": f"Reviewed `fffffff`. {ACK_PATH} src/example.rs"}]
        s["reviews"] = []  # the default review is bound to the head commit and would name it

    elif scenario == "reviewer-not-matching":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"], reviewer="reviewer"))
        s["comments"] = [{"user": {"login": "someone-else", "type": "User"}, "body": review_body("src/example.rs")}]

    elif scenario == "reviewer-login-case-differs":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"], reviewer="Reviewer"))

    # -- Renames, stale branches, the cut-over PR -----------------------------
    elif scenario in {"rename-old-path-pinned", "rename-old-path-acked"}:
        set_changes(s, ("renamed", "src/example.rs", "src/renamed.rs"))
        if scenario == "rename-old-path-acked":
            with_ack(s, ack_text(["src/example.rs"]))

    elif scenario == "nested-gitattributes":
        set_changes(s, ("added", "src/.gitattributes", "*.rs eol=crlf\n"))

    elif scenario in {"behind-a-pin-adding-master-commit", "behind-master-pin-touched"}:
        # master added a pin after this branch was cut: the tip has one more pin than
        # the merge base and the head. The PR removed nothing.
        s["surface_base"] = schema3(DEFAULT_PINS + ["src/added-on-master.rs"])
        s["surface_merge_base"] = schema3(DEFAULT_PINS)
        if scenario == "behind-master-pin-touched":
            # Merged, the PR would change a file the tip pins: it needs an acknowledgement.
            set_changes(s, ("modified", "src/added-on-master.rs"))

    elif scenario == "cut-over":
        # Schema-2 base and merge base, schema-3 head, a pinned file changed, its ack and review.
        s["surface_base"] = schema2(DEFAULT_PINS)
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))

    elif scenario == "cut-over-without-ack":
        s["surface_base"] = schema2(DEFAULT_PINS)
        set_changes(s, ("modified", "src/example.rs"))

    # -- .gitattributes is pinned like any file: no special case in the gate ----------
    elif scenario in {"gitattributes-no-ack", "gitattributes-acked", "gitattributes-wrong-ack",
                      "gitattributes-unpinned"}:
        if scenario != "gitattributes-unpinned":
            pins = sorted(DEFAULT_PINS + [".gitattributes"])
            s["surface_head"] = schema3(pins)
            s["surface_base"] = schema3(pins)
        set_changes(s, ("modified", ".gitattributes"))
        if scenario == "gitattributes-acked":
            with_ack(s, ack_text([".gitattributes"]))
        elif scenario == "gitattributes-wrong-ack":
            with_ack(s, ack_text(["src/example.rs"]))

    # -- removed-pin lines: sorted and unique, like the path lines ----------------------
    elif scenario in {"removed-pins-sorted", "removed-pins-unsorted", "removed-pins-duplicate"}:
        s["surface_base"] = schema3(DEFAULT_PINS + ["src/third.rs"])
        s["surface_head"] = schema3(["src/example.rs"])
        set_changes(s, ("modified", SURFACE_PATH))
        lines = {
            "removed-pins-sorted": ["removed-pin: src/other.rs", "removed-pin: src/third.rs"],
            "removed-pins-unsorted": ["removed-pin: src/third.rs", "removed-pin: src/other.rs"],
            "removed-pins-duplicate": ["removed-pin: src/other.rs", "removed-pin: src/other.rs",
                                       "removed-pin: src/third.rs"],
        }[scenario]
        with_ack(s, "\n".join(lines + ["reviewer: reviewer"]) + "\n")

    # -- reviewer login shape ---------------------------------------------------------------
    elif scenario.startswith("reviewer-login-"):
        login = {
            "reviewer-login-trailing-hyphen": "reviewer-",
            "reviewer-login-leading-hyphen": "-reviewer",
            "reviewer-login-40-chars": "a" * 40,
            "reviewer-login-39-chars": "a" * 39,
            "reviewer-login-bot": "dependabot[bot]",
        }.get(scenario)
        if login is None:
            fail("unknown scenario")
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"], reviewer=login))
        s["comments"] = [{"user": {"login": login, "type": "User"}, "body": review_body("src/example.rs")}]

    # -- the reason cap on an added pin ------------------------------------------------------------
    elif scenario in {"pin-reason-501-chars", "pin-reason-500-astral-chars", "pin-reason-control-char"}:
        reason = {
            "pin-reason-501-chars": "x" * 501,
            "pin-reason-500-astral-chars": chr(0x1F600) * 500,
            "pin-reason-control-char": "bad" + chr(7) + "reason",
        }[scenario]
        s["surface_head"] = schema3(DEFAULT_PINS + ["src/new.rs"], {"src/new.rs": reason})
        set_changes(s, ("modified", SURFACE_PATH))
        with_ack(s, ack_text(["src/new.rs"]))

    # -- an ack may only be added: renames of acks --------------------------------------------------
    elif scenario == "ack-renamed-out-with-pin-change":
        set_changes(s, ("modified", "src/example.rs"), ("renamed", ACK_DIR + "/pr-100.txt", "docs/moved-ack.txt"))
        with_ack(s, ack_text(["src/example.rs"]))

    elif scenario == "ack-renamed-out-no-pin-change":
        set_changes(s, ("modified", "docs/example.md"), ("renamed", ACK_DIR + "/pr-100.txt", "docs/moved-ack.txt"))

    elif scenario == "ack-added-as-a-rename-from-outside":
        # git pairs the added ack with an unrelated deleted file of similar content: still an add.
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["files"][-1].update({"status": "renamed", "previous_filename": "docs/deleted-notes.md"})
        s["files"].append({"filename": "docs/deleted-notes.md", "status": "removed", "additions": 0, "deletions": 1})
        s["changed_files_expected"] = len(s["files"])
        s["diff"] += (
            "diff --git a/docs/deleted-notes.md b/docs/deleted-notes.md\n"
            "deleted file mode 100644\n"
            "index 1111111..0000000\n"
            "--- a/docs/deleted-notes.md\n"
            "+++ /dev/null\n"
            "@@ -1 +0,0 @@\n"
            "-gone\n"
        )

    # -- copies: the destination counts; the source only if it is itself pinned ------------------
    elif scenario in {"copy-of-a-pinned-file", "copy-of-a-pinned-file-acked", "copy-of-an-unpinned-file"}:
        source = "docs/example.md" if scenario == "copy-of-an-unpinned-file" else "src/example.rs"
        set_changes(s, ("added", "src/copied.rs" if source != "docs/example.md" else "docs/copied.md", "copied text\n"))
        s["files"][-1].update({"status": "copied", "previous_filename": source})
        if scenario == "copy-of-a-pinned-file-acked":
            with_ack(s, ack_text(["src/example.rs"]))

    # -- a path is named in the review only as a whole token --------------------------------------
    elif scenario == "comment-names-a-longer-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"}, "body": review_body("src/example.rs.bak")}]

    elif scenario == "comment-names-a-longer-ack-path":
        set_changes(s, ("modified", "src/example.rs"))
        with_ack(s, ack_text(["src/example.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"},
                          "body": f"Reviewed `{SHORT}`. Acknowledgement elsewhere/{ACK_PATH}.bak Pinned: src/example.rs"}]

    elif scenario == "comment-names-paths-in-punctuation":
        set_changes(s, ("modified", "src/example.rs"), ("modified", "src/other.rs"))
        with_ack(s, ack_text(["src/example.rs", "src/other.rs"]))
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"},
                          "body": f'Reviewed `{SHORT}`: ({ACK_PATH}) "src/example.rs", \'src/other.rs\':'}]

    elif scenario == "comment-names-a-path-with-spaces":
        odd = "docs/My File (1).md"
        s["surface_head"] = schema3(sorted(DEFAULT_PINS + [odd]))
        s["surface_base"] = schema3(sorted(DEFAULT_PINS + [odd]))
        set_changes(s, ("modified", odd))
        with_ack(s, ack_text([odd]))

    # -- a large ack: a match must not be lost to SIGPIPE under pipefail -------------------------------
    elif scenario in {"ack-large-with-early-hex-token", "ack-large-with-early-control-char"}:
        set_changes(s, ("modified", "src/example.rs"))
        early = DIGEST if scenario == "ack-large-with-early-hex-token" else "src/" + chr(7)
        with_ack(s, early + "\n" + ("x" * 200000) + "\nsrc/example.rs\nreviewer: reviewer\n")
        s["comments"] = [{"user": {"login": "reviewer", "type": "User"}, "body": review_body("src/example.rs")}]

    # -- ordering is by UTF-8 bytes -------------------------------------------------------------------
    elif scenario in {"ack-utf8-byte-order", "ack-utf8-utf16-order"}:
        s["surface_head"] = schema3([UTF8_LOW, UTF8_HIGH])
        s["surface_base"] = schema3([UTF8_LOW, UTF8_HIGH])
        set_changes(s, ("modified", UTF8_LOW), ("modified", UTF8_HIGH))
        order = [UTF8_LOW, UTF8_HIGH] if scenario == "ack-utf8-byte-order" else [UTF8_HIGH, UTF8_LOW]
        with_ack(s, "\n".join(order + ["reviewer: reviewer"]) + "\n")

    elif scenario == "privacy-email-blocker":
        s["title"] = "Customer contact: customer@company.test"

    elif scenario == "public-agent-coauthor-trailer":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-coauthor-trailer-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\r\n\r\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "customer-coauthor-trailer":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Customer Contributor <" + CUSTOMER_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "customer-coauthor-trailer-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\r\n\r\n"
            "Co-Authored-By: Customer Contributor <" + CUSTOMER_ADDRESS + ">\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-email-in-pr-body":
        s["body"] = DEFAULT_BODY + "Public agent contact: " + PUBLIC_AGENT_ADDRESS + "\n"

    elif scenario == "public-agent-email-in-source-payload":
        s["diff"] = (
            "diff --git a/docs/example.md b/docs/example.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/docs/example.md\n"
            "@@ -0,0 +1 @@\n"
            "+Public agent contact: " + PUBLIC_AGENT_ADDRESS + "\n"
        )

    elif scenario == "digit-run-in-payload":
        s["files"] = [{"filename": "docs/example.md", "status": "added", "additions": 3, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/docs/example.md b/docs/example.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/docs/example.md\n"
            "@@ -0,0 +4,3 @@\n"
            "+a harmless first line\n"
            "+synthetic account " + "12345" + "678901" + " for the test\n"
            "+a harmless last line\n"
        )

    elif scenario in {"merge-commit-conflict-comment-block", "merge-commit-conflict-block-customer-address"}:
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        extra = "#\tdocs/x.json\n" if scenario == "merge-commit-conflict-comment-block" else "# contact customer" + "@" + "company.test\n"
        commit["commit"]["message"] = (
            "Merge master into the lane branch\n\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\n\n"
            "# Conflicts:\n" + extra
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-spoof-header":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "X-Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-nonfooter":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\n\n"
            "This is ordinary commit body text, not a trailer footer.\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-nonfooter-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\r\n\r\n"
            "This is ordinary commit body text, not a trailer footer.\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-mixed-line-endings":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + ">\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-malformed-trailer":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + "\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-malformed-trailer-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\r\n\r\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + "\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-trailer-extra-payload":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude Opus 5 <" + PUBLIC_AGENT_ADDRESS + "> extra\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-email-in-author-name":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: " + PUBLIC_AGENT_ADDRESS + " <dev@example.invalid>\n"
        )
        s["commits"] = [commit]

    elif scenario == "customer-email-in-author-name-with-public-agent-trailer":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Customer " + CUSTOMER_ADDRESS + " <" + PUBLIC_AGENT_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "privacy-credential-blocker":
        s["diff"] = (
            "diff --git a/config/settings.py b/config/settings.py\n"
            "index 1111111..2222222 100644\n"
            "--- a/config/settings.py\n"
            "+++ b/config/settings.py\n"
            "@@ -1 +1 @@\n"
            "-api_key = \"placeholder\"\n"
            "+api_key = \"sk_live_abcdef1234567890abcdef\"\n"
        )
        s["files"] = [{"filename": "config/settings.py", "status": "modified", "additions": 1, "deletions": 1}]

    elif scenario == "privacy-uuid-indeterminate":
        s["diff"] = (
            "diff --git a/docs/example.md b/docs/example.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/docs/example.md\n"
            "@@ -0,0 +1 @@\n"
            "+fixture reference 11111111-2222-3333-4444-555555555555\n"
        )

    elif scenario in {"binary-missing-attestation", "binary-with-attestation"}:
        s["files"] = [{"filename": "docs/new.png", "status": "added", "additions": 0, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/docs/new.png b/docs/new.png\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "GIT binary patch\n"
            "literal 3\nQwerty\n"
        )

    elif scenario == "gitlink-change":
        s["files"] = [{"filename": "vendor/module", "status": "modified", "additions": 1, "deletions": 1}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/vendor/module b/vendor/module\n"
            "index 1111111..2222222 160000\n"
            "--- a/vendor/module\n"
            "+++ b/vendor/module\n"
            "@@ -1 +1 @@\n"
            "-Subproject commit 1111111111111111111111111111111111111111\n"
            "+Subproject commit 2222222222222222222222222222222222222222\n"
        )

    elif scenario == "review-names-head-via-review":
        s["reviews"] = [{"user": {"login": "reviewer", "type": "User"}, "commit_id": HEAD, "state": "APPROVED"}]

    elif scenario == "review-names-head-via-comment":
        s["reviews"] = []
        s["comments"] = [{"user": {"login": "bot", "type": "Bot"},
                           "body": f"codex-pull-request-review-summary\n| 📝 | ✅ **Completed** | `{SHORT}` |"}]

    elif scenario == "review-missing":
        s["reviews"] = []
        s["comments"] = []

    elif scenario == "date-range-parens":
        s["body"] = DEFAULT_BODY + "\nSprint window (0101-2026 0201-2026) is locked.\n"

    elif scenario == "scan-input-write-failure":
        pass  # handled entirely by the fake mktemp wrapper; fixtures stay default

    elif scenario == "lfs-pointer-binary":
        s["files"] = [{"filename": "assets/logo.psd", "status": "added", "additions": 3, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/assets/logo.psd b/assets/logo.psd\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/assets/logo.psd\n"
            "@@ -0,0 +1,3 @@\n"
            "+version https://git-lfs.github.com/spec/v1\n"
            "+oid sha256:" + "b" * 64 + "\n"
            "+size 12345\n"
        )

    elif scenario == "removed-file-mismatch":
        s["files"] = [{"filename": "docs/retired.md", "status": "removed", "additions": 0, "deletions": 5}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/docs/retired.md b/docs/retired.md\n"
            "deleted file mode 100644\n"
            "index 1111111..0000000\n"
            "--- a/docs/retired.md\n"
            "+++ /dev/null\n"
            "@@ -1,2 +0,0 @@\n"
            "-line one\n"
            "-line two\n"
        )

    elif scenario == "diff-extra-destination":
        s["files"] = [{"filename": "docs/example.md", "status": "added", "additions": 1, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = DEFAULT_DIFF + (
            "diff --git a/docs/smuggled.md b/docs/smuggled.md\n"
            "new file mode 100644\n"
            "index 0000000..3333333\n"
            "--- /dev/null\n"
            "+++ b/docs/smuggled.md\n"
            "@@ -0,0 +1 @@\n"
            "+smuggled content\n"
        )

    elif scenario == "coverage-example-redaction":
        # The diff has no section at all for this REST filename, so it is
        # reported as a coverage issue; the filename itself carries an
        # identifier shape and must be redacted before it reaches gate output.
        s["files"] = [{"filename": "docs/ABCDE1234F.md", "status": "added", "additions": 1, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/unrelated.md b/unrelated.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/unrelated.md\n"
            "@@ -0,0 +1 @@\n"
            "+content\n"
        )

    elif scenario == "author-email-localhost":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = {
            "message": "safe commit metadata",
            "author": {"name": "Developer", "email": "dev@localhost"},
            "committer": {"name": "Developer", "email": "dev@localhost"},
        }
        s["commits"] = [commit]

    return s


S = state()


def has(*needles):
    joined = " ".join(args)
    return all(n in joined for n in needles)


if args[:2] == ["pr", "view"]:
    emit({"headRefOid": HEAD, "baseRefName": "master", "title": S["title"], "body": S["body"],
          "changedFiles": S["changed_files_expected"]})

elif args[:2] == ["pr", "diff"]:
    emit(S["diff"])

elif args and args[0] == "api":
    joined = " ".join(args)
    if "/contents/" in joined:
        if SURFACE_PATH in joined:
            history_ref = next((ref for ref in list(S["surface_at"]) + list(S["surface_at_fail"]) if f"ref={ref}" in joined), None)
            if history_ref is not None:
                if history_ref in S["surface_at_fail"]:
                    fail("controlled history surface read failure")
                emit({"encoding": "base64", "content": b64(S["surface_at"][history_ref])})
            elif f"ref={HEAD}" in joined:
                if S["surface_head_fail"]:
                    fail("controlled surface read failure")
                if S["surface_head_malformed"]:
                    emit({"content": "not-base64"})
                else:
                    emit({"encoding": "base64", "content": b64(S["surface_head"])})
            elif f"ref={BEHIND_MERGE_BASE}" in joined:
                emit({"encoding": "base64", "content": b64(S["surface_merge_base"])})
            else:
                emit({"encoding": "base64", "content": b64(S["surface_base"])})
        elif f"/contents/{ACK_PATH}?ref={HEAD}" in joined:
            if S["ack_text"] is None or S["ack_fail"]:
                fail("controlled acknowledgement read failure")
            emit({"encoding": "base64", "content": b64(S["ack_text"])})
        else:
            fail("unknown contents fixture")
    elif "/compare/" in joined:
        if S["merge_base_fail"]:
            fail("controlled merge-base failure")
        # `gh api --jq .merge_base_commit.sha` prints the bare SHA, as branches/master does.
        emit(BEHIND_MERGE_BASE if S["surface_merge_base"] is not None else BASE_TIP)
    elif "/pulls/321/commits" in joined:
        emit([S["commits"]])
    elif any(a.endswith("/pulls/321") for a in args):
        emit({"commits": len(S["commits"]), "head": {"sha": HEAD}})
    elif "/pulls/321/files" in joined:
        emit([S["files"]])
    elif "/pulls/321/reviews" in joined:
        emit([S["reviews"]])
    elif "/issues/321/comments" in joined:
        emit([S["comments"]])
    elif "branches/master" in joined:
        emit(BASE_TIP)
    else:
        fail("unknown API fixture")
else:
    fail("unknown command fixture")
