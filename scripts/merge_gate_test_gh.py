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
# git blob ids of lab captures, as the files API reports them in "sha": LAB_BLOB is the one the manifest tests list, LAB_BLOB_OFF
# differs from it in the last character only.
LAB_BLOB = "1e0627036228d2a3032c1d66e96b9e997bea34af"
LAB_BLOB_OFF = "1e0627036228d2a3032c1d66e96b9e997bea34ae"
# the bytes the fake serves for LAB_BLOB (LAB_BLOB is their git blob id); the manifest tests record their size and sha256.
# OTHER_LAB_BYTES are served for LAB_BLOB in one scenario: an answer whose bytes are not that blob.
LAB_BYTES = b"lab capture bytes\n"
OTHER_LAB_BYTES = b"other capture bytes\n"
OTHER_BLOB = "9f2b1c4e7a6d03581e4c2b7a9d0f3e5c6b8a1d24"
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
DEFAULT_REVIEWS = []
DEFAULT_COMMENTS = []
OWNER_USER = {"login": "lamemustafa", "id": 57982425, "type": "User"}
T1, T2, T3 = "2026-10-14T09:00:00Z", "2026-10-14T10:00:00Z", "2026-10-14T11:00:00Z"
VERDICT_CLEAN = "MERGE-VERDICT CLEAN head=" + HEAD + " base=" + BASE_TIP
OLD_HEAD = "9" * 40
OLD_BASE = "8" * 40
CARRY_DIFF = (
    "diff --git a/docs/x.md b/docs/x.md\n"
    "index 1111111..2222222 100644\n"
    "--- a/docs/x.md\n"
    "+++ b/docs/x.md\n"
    "@@ -1,2 +1,2 @@\n"
    " keep\n"
    "-old\n"
    "+changed\n"
)
VERDICT_RECOMMEND = "MERGE-ADVERSARY RECOMMEND head=" + HEAD + " base=" + BASE_TIP


def owner_comment(body, at):
    return {"id": 1, "user": dict(OWNER_USER), "body": body, "created_at": at, "updated_at": at, "performed_via_github_app": None}


# The owner's two lines, served in addition to whatever a scenario puts in its own comments and reviews.
DEFAULT_OWNER_ITEMS = [owner_comment(VERDICT_CLEAN, T1), owner_comment(VERDICT_RECOMMEND, T2)]
EMPTY_HISTORY = {"data": {"repository": {"pullRequest": {
    "reviews": {"pageInfo": {"hasNextPage": False}, "nodes": []},
    "comments": {"pageInfo": {"hasNextPage": False}, "nodes": []},
    "timelineItems": {"pageInfo": {"hasNextPage": False}, "nodes": []}}}}}
DEFAULT_BODY = "## Outcome and reason\n\nA bounded merge preflight.\n"
DEFAULT_COMMIT = {
    "sha": HEAD,
    "parents": [{"sha": BASE_TIP}],
    "commit": {
        "message": "safe commit metadata",
        "author": {"name": "Maintainer", "email": "maintainer@example.invalid"},
        "committer": {"name": "Maintainer", "email": "maintainer@example.invalid", "date": "2026-10-01T09:00:00Z"},
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
    global scenario  # a carry-rec-* scenario continues as the scenario it names
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
        "surface_at_flaky": {},          # commit sha -> (failures before a success, gh's error line); counted in $GATE_STATE
        "merge_base_fail": False,
        "ack_text": None,
        "ack_fail": False,
        "reviews": list(DEFAULT_REVIEWS),
        "comments": list(DEFAULT_COMMENTS),
        "owner_items": [dict(i) for i in DEFAULT_OWNER_ITEMS],   # served as issue comments after "comments"
        "owner_reviews": [],
        "head_ref": "feature-branch",
        "commit_comments": [],
        "commit_comments_fail": False,
        "commit_comment_pages": None,    # pages of the repository's commit comments, when a scenario needs more than one
        "inline": [],
        "history": EMPTY_HISTORY,
        "history_fail": False,
        "pages_fail": set(),
        "commits": [dict(DEFAULT_COMMIT)],
        "carry": None,                   # {"old": diff served for the older pair, "new": diff for the current pair, "fail": bool}
        "carry_json": None,              # {"old": compare JSON for the older pair, "new": for the current pair, "fail": bool}
    }

    if scenario == "surface-fetch-fail":
        s["surface_head_fail"] = True

    elif scenario == "verdict-missing-clean":
        s["owner_items"] = [owner_comment(VERDICT_RECOMMEND, T2)]

    elif scenario == "verdict-missing-recommend":
        s["owner_items"] = [owner_comment(VERDICT_CLEAN, T1)]

    elif scenario in {"verdict-missing-recommend-write-path", "tier-unclassified-clean-only", "tier-rename-from-write-path", "tier-tool-fails", "tier-tool-error-object", "tier-tool-bad-tier"}:
        # Only the CLEAN line, on a change that is NOT of the one tier (a write-path file, a path no rule classifies, a rename away
        # from a write-path file) or whose tier tool fails: the full tier, which needs both lines.
        s["owner_items"] = [owner_comment(VERDICT_CLEAN, T1)]
        if scenario == "verdict-missing-recommend-write-path":
            set_changes(s, ("added", "src-tauri/src/agent_lab_new.rs", "x"))
        elif scenario == "tier-unclassified-clean-only":
            set_changes(s, ("added", "mystery/unknown.txt", "x"))
        elif scenario == "tier-rename-from-write-path":
            set_changes(s, ("renamed", "src-tauri/src/agent_lab.rs", "docs/moved.md"))

    elif scenario.startswith("carry-"):
        # The owner's CLEAN line names an OLDER head and merge base; the gate may carry it when the pull request's own added and
        # removed lines (merge base...head) are byte-identical at both. The compare answers below are the two diffs.
        s["owner_items"] = [owner_comment("MERGE-VERDICT CLEAN head=" + OLD_HEAD + " base=" + OLD_BASE, T1)]
        # "carry-rec-<scenario>[-other-pair|-retracted|-newer-current]" is that scenario with the owner's RECOMMEND line for the same
        # older pair too (or for another pair, retracted after, or followed by a RECOMMEND for the current pair)
        rec = None
        if scenario.startswith("carry-rec-"):
            rec = scenario[len("carry-rec-"):]
            rec_flag = next((f for f in ("other-pair", "retracted", "newer-current") if rec.endswith("-" + f)), None)
            if rec_flag:
                rec = rec[:-len(rec_flag) - 1]
            scenario = "carry-" + rec
        set_changes(s, ("modified", "docs/x.md"))
        # Only the index line differs between the two: the pull request's own change is the same, in the same place.
        old = CARRY_DIFF.replace("index 1111111..2222222", "index 3333333..4444444")
        new = CARRY_DIFF
        if scenario == "carry-function-context-differs":
            # the text after the hunk numbers names the enclosing function and moves with master's renames: it does not count
            old = old.replace("@@ -1,2 +1,2 @@\n", "@@ -1,2 +1,2 @@ fn renamed_by_master()\n")
        elif scenario == "carry-hunk-numbers-differ":
            old = old.replace("@@ -1,2 +1,2 @@", "@@ -11,2 +11,2 @@")
        elif scenario == "carry-context-differs":
            old = old.replace(" keep\n", " keep (master changed this line)\n")
        elif scenario == "carry-moved-insertion":
            # the same added line, put where the same text also stands: only the hunk's place differs
            old = old.replace("@@ -1,2 +1,2 @@", "@@ -21,2 +21,2 @@")
        elif scenario == "carry-patch-differs":
            new = CARRY_DIFF.replace("+changed", "+changed differently")
        elif scenario == "carry-whitespace-only":
            new = CARRY_DIFF.replace("+changed", "+chan ged")
        elif scenario == "carry-indent-only":
            new = CARRY_DIFF.replace("+changed", "+  changed")
        elif scenario == "carry-binary":
            # the picture is one of the pull request's files, so the file list agrees; only the binary-diff guard can refuse
            set_changes(s, ("modified", "docs/x.md"), ("modified", "docs/x.png"))
            old = new = CARRY_DIFF + "diff --git a/docs/x.png b/docs/x.png\nindex 1111111..2222222 100644\nBinary files a/docs/x.png and b/docs/x.png differ\n"
        elif scenario in ("carry-fullpath", "carry-fullpath-patch-differs"):
            # a change of the full tier (a write-path file) whose own patch is the same at both pairs, or differs
            path = "src-tauri/src/agent_lab_new.rs"
            set_changes(s, ("modified", path))
            old = old.replace("docs/x.md", path)
            new = CARRY_DIFF.replace("docs/x.md", path)
            if scenario.endswith("patch-differs"):
                new = new.replace("+changed", "+changed differently")
        elif scenario.startswith("carry-listed-binary"):
            # The pull request also adds docs/new.png (LAB_BLOB, which the lab-capture manifest tests list). The compare diffs hide its
            # bytes, so the gate reads the full blob id of each range from the compare JSON (old_json and new_json below).
            name = "docs/new.png"
            # a binary is never of the one tier (no rule classifies it), so the full tier also needs the RECOMMEND line, at the current pair
            if rec is None:
                s["owner_items"] = s["owner_items"] + [owner_comment(VERDICT_RECOMMEND, T2)]
            renamed = scenario in ("carry-listed-binary-renamed", "carry-listed-binary-renamed-json-added")
            # a listed binary that is MODIFIED (not added): the diff then has no `new file mode` line and its base blob is only in the
            # stripped `index` line; "-modified" has other base blobs in the two ranges, "-modified-same-base" the same one
            modified = scenario.startswith("carry-listed-binary-modified")
            patch_body = scenario == "carry-listed-binary-patch-body"
            source = "docs/old.png" if renamed else name
            record = {"filename": name, "status": "renamed" if renamed else ("modified" if modified else "added"), "additions": 0, "deletions": 0}
            if renamed:
                record["previous_filename"] = source
            record["sha"] = LAB_BLOB
            privacy = f"diff --git a/{source} b/{name}\n" + (
                f"similarity index 90%\nrename from {source}\nrename to {name}\nindex 1111111..2222222 100644\n" if renamed
                else "index 1111111..2222222 100644\n" if modified
                else "new file mode 100644\nindex 0000000..1111111\n") + "GIT binary patch\nliteral 3\nQwerty\n"
            set_changes(s, ("modified", "docs/x.md"))
            s["files"] = [record] + s["files"]
            s["diff"] = privacy + s["diff"]
            extra_old = extra_new = ""
            if scenario == "carry-listed-binary-extra-in-compare":
                # docs/y.bin is a text change to the privacy scan (no binary to attest) but a binary section in both compare diffs, and
                # its REST blob is the one the compare JSON gives: only "the binary paths are exactly the exempted ones" is left to refuse
                s["files"].append({"filename": "docs/y.bin", "status": "modified", "additions": 1, "deletions": 1, "sha": "c" * 40})
                s["diff"] += modified_diff("docs/y.bin")
                extra_old = extra_new = "diff --git a/docs/y.bin b/docs/y.bin\nindex 1111111..2222222 100644\nBinary files a/docs/y.bin and b/docs/y.bin differ\n"
            elif scenario == "carry-listed-binary-and-unlisted":
                # docs/extra.bin is a second, UNLISTED binary of the pull request in REST and in the privacy diff, but a text section in the
                # compare diffs: only "every binary was exempted" (binary_count) is left to refuse (the owner's attestation is passed)
                s["files"].append({"filename": "docs/extra.bin", "status": "added", "additions": 0, "deletions": 0, "sha": OTHER_BLOB})
                s["diff"] += ("diff --git a/docs/extra.bin b/docs/extra.bin\nnew file mode 100644\nindex 0000000..1111111\n"
                              "GIT binary patch\nliteral 3\nQwerty\n")
                extra_old = extra_new = added_diff("docs/extra.bin", "x\n")
            s["changed_files_expected"] = len(s["files"])

            def binary_section(index, body):
                head = f"diff --git a/{source} b/{name}\n"
                if renamed:
                    head += f"similarity index 90%\nrename from {source}\nrename to {name}\n"
                elif not modified:
                    head += "new file mode 100644\n"
                return head + f"index {index}\n" + body

            marker = (f"Binary files a/{source} and b/{name} differ\n" if renamed or modified else f"Binary files /dev/null and b/{name} differ\n")
            old_body, new_body = (marker, marker)
            if patch_body:
                old_body, new_body = ("GIT binary patch\nliteral 5\nAbcdef\n\n", "GIT binary patch\nliteral 3\nQwerty\n\n")
            old_index, new_index = ("3333333..4444444", "0000000..1111111")
            if modified:
                old_index, new_index = ("5555555..1111111", "5555555..1111111" if scenario.endswith("same-base") else "6666666..1111111")
            old = binary_section(old_index, old_body) + CARRY_DIFF.replace("index 1111111..2222222", "index 3333333..4444444") + extra_old
            new = binary_section(new_index, new_body) + CARRY_DIFF + extra_new
            binary_status = {"carry-listed-binary-json-modified": "modified", "carry-listed-binary-renamed-json-added": "added"}.get(scenario, record["status"])
            old_json_files = [{"filename": name, "sha": LAB_BLOB, "status": binary_status}, {"filename": "docs/x.md", "sha": "a" * 40, "status": "modified"}]
            if scenario == "carry-listed-binary-extra-in-compare":
                old_json_files.append({"filename": "docs/y.bin", "sha": "c" * 40, "status": "added"})
            elif scenario == "carry-listed-binary-and-unlisted":
                old_json_files.append({"filename": "docs/extra.bin", "sha": OTHER_BLOB, "status": "added"})
            new_json_files = [dict(f) for f in old_json_files]
            if scenario == "carry-listed-binary-json-differs-from-files":
                # the same blob in both ranges, but not the one the files API gave (and the manifest check verified)
                old_json_files[0]["sha"] = new_json_files[0]["sha"] = OTHER_BLOB
            elif scenario == "carry-listed-binary-blob-differs":
                old_json_files[0]["sha"] = OTHER_BLOB
            elif scenario == "carry-listed-binary-path-missing":
                old_json_files = old_json_files[1:]
            elif scenario == "carry-listed-binary-path-missing-new":
                new_json_files = new_json_files[1:]
            elif scenario == "carry-listed-binary-path-twice":
                old_json_files.append(dict(old_json_files[0]))
            elif scenario == "carry-listed-binary-uppercase-sha":
                old_json_files[0]["sha"] = LAB_BLOB.upper()
            elif scenario == "carry-listed-binary-short-sha":
                new_json_files[0]["sha"] = LAB_BLOB[:7]
            elif scenario == "carry-listed-binary-truncated":
                # GitHub stops at 300 files: the list is then not proven to hold every file
                old_json_files += [{"filename": f"pad/{i}.txt", "sha": "b" * 40, "status": "added"} for i in range(298)]
                new_json_files += [{"filename": f"pad/{i}.txt", "sha": "b" * 40, "status": "added"} for i in range(298)]
            elif scenario == "carry-listed-binary-299-files":
                old_json_files += [{"filename": f"pad/{i}.txt", "sha": "b" * 40, "status": "added"} for i in range(297)]
                new_json_files += [{"filename": f"pad/{i}.txt", "sha": "b" * 40, "status": "added"} for i in range(297)]
            elif scenario == "carry-listed-binary-no-files-list":
                old_json_files = None
            s["carry_json"] = {"old": None if old_json_files is None else {"files": old_json_files}, "new": {"files": new_json_files},
                               "fail": scenario == "carry-listed-binary-json-fails"}
        elif scenario == "carry-empty-diff":
            old = new = ""
        elif scenario == "carry-duplicate-file-sections":
            old = new = CARRY_DIFF + CARRY_DIFF
        elif scenario == "carry-diff-cut-short":
            # The pull request changes two files; the compare diff names only one of them.
            set_changes(s, ("modified", "docs/x.md"), ("modified", "docs/y.md"))
        elif scenario == "carry-mode-change":
            new = CARRY_DIFF.replace("index 1111111..2222222 100644", "old mode 100644\nnew mode 100755\nindex 1111111..2222222")
        s["carry"] = {"old": old, "new": new, "fail": scenario == "carry-compare-fails"}
        if scenario == "carry-same-patch-write-path":
            set_changes(s, ("added", "src-tauri/src/agent_lab_new.rs", "x"))
        if rec is not None:
            other = OLD_HEAD if rec_flag != "other-pair" else "7" * 40
            s["owner_items"] = s["owner_items"] + [owner_comment("MERGE-ADVERSARY RECOMMEND head=" + other + " base=" + OLD_BASE, T2)]
            if rec_flag == "retracted":
                s["owner_items"] = s["owner_items"] + [owner_comment("MERGE-ADVERSARY RETRACT head=" + OLD_HEAD + " base=" + OLD_BASE, T3)]
            elif rec_flag == "newer-current":
                s["owner_items"] = s["owner_items"] + [owner_comment(VERDICT_RECOMMEND, T3)]

    elif scenario == "verdict-for-another-base":
        s["owner_items"] = [owner_comment(VERDICT_CLEAN.replace(BASE_TIP, "c" * 40), T1), owner_comment(VERDICT_RECOMMEND.replace(BASE_TIP, "c" * 40), T2)]

    elif scenario == "verdict-by-review":
        s["owner_items"] = []
        s["owner_reviews"] = [{"id": 2, "user": dict(OWNER_USER), "body": VERDICT_CLEAN.replace(BASE_TIP, "BASE"), "submitted_at": T1, "state": "COMMENTED"},
                              {"id": 3, "user": dict(OWNER_USER), "body": VERDICT_RECOMMEND.replace(BASE_TIP, "BASE"), "submitted_at": T2, "state": "COMMENTED"}]
        s["reviews_use_base"] = True

    elif scenario == "verdict-then-not-clean":
        s["owner_items"] += [owner_comment("MERGE-VERDICT NOT-CLEAN head=" + HEAD + " base=" + BASE_TIP, T3)]

    elif scenario == "verdict-then-owner-edit":
        s["history"] = {"data": {"repository": {"pullRequest": {
            "reviews": {"pageInfo": {"hasNextPage": False}, "nodes": []},
            "comments": {"pageInfo": {"hasNextPage": False}, "nodes": [{"databaseId": 1, "lastEditedAt": T3, "userContentEdits": {"totalCount": 1}, "author": {"login": "lamemustafa", "databaseId": 57982425}}]},
            "timelineItems": {"pageInfo": {"hasNextPage": False}, "nodes": []}}}}}

    elif scenario == "verdict-then-owner-comment-deleted":
        s["history"] = {"data": {"repository": {"pullRequest": {
            "reviews": {"pageInfo": {"hasNextPage": False}, "nodes": []},
            "comments": {"pageInfo": {"hasNextPage": False}, "nodes": []},
            "timelineItems": {"pageInfo": {"hasNextPage": False}, "nodes": [{"__typename": "CommentDeletedEvent", "createdAt": T3, "deletedCommentAuthor": {"login": "lamemustafa", "databaseId": 57982425}}]}}}}}

    elif scenario == "history-has-another-page":
        s["history"] = {"data": {"repository": {"pullRequest": {
            "reviews": {"pageInfo": {"hasNextPage": True}, "nodes": []},
            "comments": {"pageInfo": {"hasNextPage": False}, "nodes": []},
            "timelineItems": {"pageInfo": {"hasNextPage": False}, "nodes": []}}}}}

    elif scenario == "history-unreadable":
        s["history_fail"] = True

    elif scenario in {"reviews-unreadable", "comments-unreadable", "inline-unreadable"}:
        s["pages_fail"] = {scenario.split("-")[0]}

    elif scenario == "token-in-the-title":
        s["title"] = "Fix a thing MERGE-VERDICT"

    elif scenario == "token-in-the-body":
        s["body"] = DEFAULT_BODY + "MERGE-ADVERSARY RECOMMEND\n"

    elif scenario == "token-in-a-stranger-comment":
        s["comments"] = [{"id": 9, "user": {"login": "stranger", "id": 7, "type": "User"}, "body": "MERGE-VERDICT CLEAN", "created_at": T1, "updated_at": T1}]

    elif scenario == "token-in-the-branch-name":
        s["head_ref"] = "fix/merge-verdict-parser"

    elif scenario == "token-in-a-file-name":
        set_changes(s, ("added", "docs/MERGE-ADVERSARY-notes.md", "A plain line."))

    elif scenario == "token-in-a-previous-file-name":
        set_changes(s, ("renamed", "docs/MERGE-VERDICT-old.md", "docs/plain-new.md"))

    elif scenario == "commit-comments-unreadable":
        s["commit_comments_fail"] = True

    elif scenario == "seventy-six-commits":
        # A pull request in a chain carries its parents' commits: more than the old cap of 60, the head among them.
        s["commits"] = [dict(DEFAULT_COMMIT, sha="%040x" % (n + 1)) for n in range(75)] + [dict(DEFAULT_COMMIT)]

    elif scenario == "seventy-six-commits-with-a-token-on-the-first":
        # The same chain link, with a stranger's token on its first commit, which is not the head: the list of commits is
        # what the comments are filtered by, so a list of the head alone would pass this.
        s["commits"] = [dict(DEFAULT_COMMIT, sha="%040x" % (n + 1)) for n in range(75)] + [dict(DEFAULT_COMMIT)]
        s["commit_comments"] = [{"id": 61, "user": {"login": "stranger", "id": 7, "type": "User"}, "body": "MERGE-ADVERSARY RECOMMEND", "created_at": T1, "updated_at": T1, "commit_id": "%040x" % 1}]

    elif scenario == "commit-comment-token-on-the-second-page":
        owner_token = {"id": 31, "user": dict(OWNER_USER), "body": "MERGE-VERDICT NOT-CLEAN", "created_at": T1, "updated_at": T1, "commit_id": HEAD}
        benign = [{"id": 100 + n, "user": {"login": "someone", "id": 9, "type": "User"}, "body": "looks fine", "created_at": T1, "updated_at": T1, "commit_id": HEAD} for n in range(100)]
        s["commit_comment_pages"] = [benign, [owner_token]]

    elif scenario == "commit-comment-without-a-commit":
        s["commit_comments"] = [{"id": 51, "user": {"login": "stranger", "id": 7, "type": "User"}, "body": "hello", "created_at": T1, "updated_at": T1, "commit_id": None}]

    elif scenario == "commit-comment-token-on-another-commit":
        s["commit_comments"] = [{"id": 41, "user": {"login": "stranger", "id": 7, "type": "User"}, "body": "MERGE-ADVERSARY RECOMMEND", "created_at": T1, "updated_at": T1, "commit_id": "9" * 40}]

    elif scenario == "sixty-one-commits":
        s["commits"] = [dict(DEFAULT_COMMIT, sha="%040x" % (n + 1)) for n in range(61)]

    elif scenario == "owner-commit-comment-with-a-token":
        s["commit_comments"] = [{"id": 12, "user": dict(OWNER_USER), "body": "MERGE-VERDICT NOT-CLEAN", "created_at": T1, "updated_at": T1}]

    elif scenario == "token-in-a-hidden-comment-in-the-body":
        s["body"] = DEFAULT_BODY + "<!-- MERGE-VERDICT CLEAN head=x -->\n"

    elif scenario == "token-in-a-stranger-commit-comment":
        s["commit_comments"] = [{"id": 11, "user": {"login": "stranger", "id": 7, "type": "User"}, "body": "MERGE-ADVERSARY RECOMMEND", "created_at": T1, "updated_at": T1}]

    elif scenario == "token-in-an-added-line":
        s["diff"] = DEFAULT_DIFF.replace("+Safe content line.\n", "+MERGE-VERDICT CLEAN\n")

    elif scenario == "changes-the-gate":
        set_changes(s, ("modified", "scripts/merge-gate.sh"))

    elif scenario == "changes-the-registry":
        set_changes(s, ("modified", "scripts/merge-gate-synthetic-uuids.json"))

    elif scenario == "changes-the-lab-manifest":
        set_changes(s, ("modified", "scripts/merge-gate-lab-captures.json"))

    elif scenario == "changes-the-registry-in-another-case":
        set_changes(s, ("added", "Scripts/Merge-Gate-Synthetic-Uuids.json", "{}"))

    elif scenario == "changes-the-review-tier":
        set_changes(s, ("modified", "scripts/review-tier-paths.json"))

    elif scenario == "changes-the-attribution-check":
        set_changes(s, ("modified", "scripts/check-ai-attribution.mjs"))

    elif scenario == "changes-a-workflow":
        set_changes(s, ("modified", ".github/workflows/ci.yml"))

    elif scenario in {"renames-a-workflow-away", "renames-the-verdict-module-away"}:
        old = ".github/workflows/ci.yml" if scenario == "renames-a-workflow-away" else "scripts/merge_gate_verdict.py"
        set_changes(s, ("renamed", old, "elsewhere/renamed-file"))

    elif scenario == "owner-edit-by-a-renamed-account":
        s["history"] = {"data": {"repository": {"pullRequest": {
            "reviews": {"pageInfo": {"hasNextPage": False}, "nodes": []},
            "comments": {"pageInfo": {"hasNextPage": False}, "nodes": [
                {"databaseId": 1, "lastEditedAt": T3, "userContentEdits": {"totalCount": 1}, "author": {"login": "a-new-name", "databaseId": 57982425}}]},
            "timelineItems": {"pageInfo": {"hasNextPage": False}, "nodes": []}}}}}

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
        if scenario == "binary-body-pointer":
            # a pointer to a lab branch and commit is only text: the gate reads no branch, so it exempts nothing
            s["body"] = DEFAULT_BODY + "\nLab capture source: lab/1050-identity-answers @ " + "c" * 40 + "\n"
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

        # A pin-list read that fails the first N times with gh's own error line, then answers (#2179).
        flaky = {
            "history-flaky-502-once": (c1, 1, "gh: Server Error (HTTP 502)"),
            "history-flaky-502-always": (c1, 99, "gh: Server Error (HTTP 502)"),
            "history-flaky-rate-limit-once": (c1, 1, "gh: You have exceeded a secondary rate limit. (HTTP 403)"),
            "history-flaky-404-own-commit-once": (c1, 1, "gh: Not Found (HTTP 404)"),
            "history-flaky-403-not-a-rate-limit": (c1, 99, "gh: Resource not accessible by integration (HTTP 403)"),
            "history-flaky-404-outside-the-pr": (p2, 99, "gh: Not Found (HTTP 404)"),
        }
        if scenario in ("history-merge-parent-compared-with-all", "history-flaky-404-outside-the-pr"):
            # The pin z reached the branch through a merge's second parent, a master-side commit.
            merge = "e3" * 20
            s["commits"] = [commit_at(merge, [BASE_TIP, p2]), commit_at(HEAD, [merge])]
            s["surface_at"] = {merge: with_own, p2: with_own}
            set_changes(s, ("modified", "src/example.rs"))
            with_ack(s, ack_text(["src/example.rs"]))
            if scenario in flaky:
                sha, failures, message = flaky[scenario]
                s["surface_at_flaky"] = {sha: (failures, message)}
        elif scenario in flaky:
            # As history-pin-kept-at-the-head, which passes, with one commit's read failing first.
            s["commits"] = [commit_at(c1, [BASE_TIP]), commit_at(HEAD, [c1])]
            s["surface_at"] = {c1: with_own}
            s["surface_head"] = with_own
            set_changes(s, ("modified", "src/example.rs"), ("modified", SURFACE_PATH))
            with_ack(s, ack_text(["src/example.rs", own]))
            sha, failures, message = flaky[scenario]
            s["surface_at_flaky"] = {sha: (failures, message)}
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
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-coauthor-trailer-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\r\n\r\n"
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\r\n"
        )
        s["commits"] = [commit]

    elif scenario in ("attribution-body-plain", "attribution-body-link", "attribution-body-comment", "attribution-title"):
        # The line the owner banned on 25 Oct 2026, in both of its forms, in the body (visible and inside an HTML comment) and the title.
        plain = "\U0001F916 Generated with Claude Code"
        link = "\U0001F916 Generated with [Claude Code](https://claude.com/claude-code)"
        if scenario == "attribution-title":
            s["title"] = "Fix a thing " + plain
        else:
            s["body"] = DEFAULT_BODY + {"attribution-body-plain": plain, "attribution-body-link": link, "attribution-body-comment": "<!-- " + link + " -->"}[scenario] + "\n"

    elif scenario in ("attribution-body-trailer", "attribution-body-markdown", "attribution-body-with-unproven-commits"):
        # The body is copied into the commit on master by a squash merge, so a co-author line there counts; Markdown does not hide the line.
        line = {"attribution-body-trailer": "Co-Authored-By: Claude",
                "attribution-body-markdown": "Generated with **[Claude Code](https://claude.com/claude-code)**",
                "attribution-body-with-unproven-commits": "Generated with Claude Code"}[scenario]
        s["body"] = DEFAULT_BODY + line + "\n"
        if scenario == "attribution-body-with-unproven-commits":
            s["commits"] = []

    elif scenario == "attribution-identity":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["committer"] = {"name": "Claude", "email": PUBLIC_AGENT_ADDRESS, "date": "2026-10-01T09:00:00Z"}
        s["commits"] = [commit]

    elif scenario in ("attribution-trailer-after", "attribution-trailer-before", "attribution-trailer-undated", "attribution-trailer-human-after"):
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        committer = dict(DEFAULT_COMMIT["commit"]["committer"])
        co_author = "Co-Authored-By: " + ("A Person <person@example.invalid>" if scenario == "attribution-trailer-human-after" else "Claude <" + PUBLIC_AGENT_ADDRESS + ">")
        commit["commit"]["message"] = "Keep the ledger-tag refusal typed.\n\n" + co_author + "\n"
        committer["date"] = {"attribution-trailer-after": "2026-10-25T13:00:00Z", "attribution-trailer-human-after": "2026-10-26T09:00:00Z",
                             "attribution-trailer-before": "2026-10-25T12:59:59Z", "attribution-trailer-undated": ""}[scenario]
        commit["commit"]["committer"] = committer
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
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\n\n"
            "# Conflicts:\n" + extra
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-spoof-header":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "X-Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-nonfooter":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\n\n"
            "This is ordinary commit body text, not a trailer footer.\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-nonfooter-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\r\n\r\n"
            "This is ordinary commit body text, not a trailer footer.\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-mixed-line-endings":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + ">\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-malformed-trailer":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + "\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-malformed-trailer-crlf":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\r\n\r\n"
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + "\r\n"
        )
        s["commits"] = [commit]

    elif scenario == "public-agent-trailer-extra-payload":
        commit = dict(DEFAULT_COMMIT)
        commit["commit"] = dict(DEFAULT_COMMIT["commit"])
        commit["commit"]["message"] = (
            "Keep the ledger-tag refusal typed.\n\n"
            "Co-Authored-By: Claude <" + PUBLIC_AGENT_ADDRESS + "> extra\n"
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

    elif scenario in {"privacy-two-uuids", "privacy-uuid-and-digit-run", "privacy-uuid-credential-context"}:
        first = "+fixture reference 11111111-2222-3333-4444-555555555555\n"
        if scenario == "privacy-uuid-credential-context":
            first = "+session token 11111111-2222-3333-4444-555555555555\n"
        second = {"privacy-two-uuids": "+other reference 66666666-7777-8888-9999-aaaaaaaaaaaa\n",
                  "privacy-uuid-and-digit-run": "+synthetic account " + "12345" + "678901" + " for the test\n",
                  "privacy-uuid-credential-context": "+plain line\n"}[scenario]
        s["files"] = [{"filename": "docs/example.md", "status": "added", "additions": 2, "deletions": 0}]
        s["changed_files_expected"] = 1
        s["diff"] = (
            "diff --git a/docs/example.md b/docs/example.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/docs/example.md\n"
            "@@ -0,0 +1,2 @@\n" + first + second
        )

    elif scenario in {"privacy-uuid-indeterminate", "manifest-write-failure"}:
        s["diff"] = (
            "diff --git a/docs/example.md b/docs/example.md\n"
            "new file mode 100644\n"
            "index 0000000..1111111\n"
            "--- /dev/null\n"
            "+++ b/docs/example.md\n"
            "@@ -0,0 +1 @@\n"
            "+fixture reference 11111111-2222-3333-4444-555555555555\n"
        )

    elif scenario in {"binary-missing-attestation", "binary-with-attestation", "binary-no-rest-sha", "binary-malformed-rest-sha",
                      "binary-one-char-off", "binary-other-path", "binary-listed-and-unlisted", "binary-removed", "binary-renamed",
                      "binary-and-manifest-edit", "binary-body-pointer", "binary-blob-fails", "binary-blob-malformed",
                      "binary-blob-not-base64", "binary-blob-size-field-differs", "binary-blob-not-that-blob", "binary-numeric-path",
                      "binary-numeric-path-lookalike"}:
        def binary_file(name, sha, status="added", previous=None):
            record = {"filename": name, "status": status, "additions": 0, "deletions": 0}
            if sha is not None:
                record["sha"] = sha
            if previous:
                record["previous_filename"] = previous
            section = f"diff --git a/{previous or name} b/{name}\n"
            if status == "removed":
                section += "deleted file mode 100644\nindex 1111111..0000000\n"
            elif previous:
                section += f"similarity index 90%\nrename from {previous}\nrename to {name}\nindex 1111111..2222222 100644\n"
            else:
                section += "new file mode 100644\nindex 0000000..1111111\n"
            return record, section + "GIT binary patch\nliteral 3\nQwerty\n"

        wanted = {
            "binary-no-rest-sha": [binary_file("docs/new.png", None)],
            "binary-malformed-rest-sha": [binary_file("docs/new.png", "3B18E512DBA79E4C8300DD08AEB37F8E728B8DAD")],
            "binary-one-char-off": [binary_file("docs/new.png", LAB_BLOB_OFF)],
            "binary-other-path": [binary_file("docs/other.png", LAB_BLOB)],
            "binary-listed-and-unlisted": [binary_file("docs/new.png", LAB_BLOB), binary_file("docs/extra.bin", OTHER_BLOB)],
            "binary-removed": [binary_file("docs/new.png", LAB_BLOB, "removed")],
            "binary-renamed": [binary_file("docs/new.png", LAB_BLOB, "renamed", "docs/old.png")],
            "binary-and-manifest-edit": [binary_file("docs/new.png", LAB_BLOB)],
            "binary-numeric-path": [binary_file("1e2", LAB_BLOB)],
            "binary-numeric-path-lookalike": [binary_file("100", LAB_BLOB)],
        }.get(scenario, [binary_file("docs/new.png", LAB_BLOB)])
        s["files"] = [record for record, _ in wanted]
        s["diff"] = "".join(section for _, section in wanted)
        if scenario == "binary-and-manifest-edit":
            # the pull request lists its own capture: the edit is judged as a change to the gate, and does not apply to its own run
            manifest = "scripts/merge-gate-lab-captures.json"
            s["files"].append({"filename": manifest, "status": "modified", "additions": 1, "deletions": 1})
            s["diff"] += modified_diff(manifest)
        s["changed_files_expected"] = len(s["files"])

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
          "changedFiles": S["changed_files_expected"], "headRefName": S["head_ref"]})

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
                if history_ref in S["surface_at_flaky"]:
                    failures, message = S["surface_at_flaky"][history_ref]
                    state = os.environ.get("GATE_STATE")
                    if not state:
                        fail("a flaky scenario needs GATE_STATE, the file that counts its calls")
                    calls = int(open(state).read() or "0") if os.path.exists(state) else 0
                    with open(state, "w") as handle:
                        handle.write(str(calls + 1))
                    if calls < failures:
                        fail(message)
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
    elif "/git/blobs/" in joined:
        if scenario == "binary-blob-fails":
            fail("controlled blob failure")
        if not joined.rstrip().endswith("/git/blobs/" + LAB_BLOB):
            fail("unknown blob fixture")
        encoded = base64.encodebytes(LAB_BYTES).decode()
        if scenario == "binary-blob-malformed":
            emit({"encoding": "base64", "content": 5, "size": len(LAB_BYTES)})
        elif scenario == "binary-blob-not-base64":
            emit({"encoding": "utf-8", "content": LAB_BYTES.decode(), "size": len(LAB_BYTES)})
        elif scenario == "binary-blob-size-field-differs":
            emit({"encoding": "base64", "content": encoded, "size": len(LAB_BYTES) + 1})
        elif scenario == "binary-blob-not-that-blob":
            emit({"sha": LAB_BLOB, "encoding": "base64", "content": base64.encodebytes(OTHER_LAB_BYTES).decode(), "size": len(OTHER_LAB_BYTES)})
        else:
            emit({"sha": LAB_BLOB, "encoding": "base64", "content": encoded, "size": len(LAB_BYTES)})
    elif "/compare/" in joined and "vnd.github.diff" in joined:
        carry = S["carry"]
        if carry is None or carry["fail"]:
            fail("controlled compare diff failure")
        spec = next(a for a in args if "/compare/" in a).split("/compare/")[1]
        merge_base = BEHIND_MERGE_BASE if S["surface_merge_base"] is not None else BASE_TIP
        if spec == OLD_BASE + "..." + OLD_HEAD:
            emit(carry["old"])
        elif spec == merge_base + "..." + HEAD:
            emit(carry["new"])
        else:
            fail("unknown compare diff fixture: " + spec)
    elif "/compare/" in joined and "--jq" not in joined:
        # the compare JSON of the verdict carry: the full blob id of each file, for the same two ranges as the diffs
        carry_json = S["carry_json"]
        if carry_json is None or carry_json["fail"]:
            fail("controlled compare JSON failure")
        spec = next(a for a in args if "/compare/" in a).split("/compare/")[1]
        merge_base = BEHIND_MERGE_BASE if S["surface_merge_base"] is not None else BASE_TIP
        if spec == OLD_BASE + "..." + OLD_HEAD:
            document = carry_json["old"]
        elif spec == merge_base + "..." + HEAD:
            document = carry_json["new"]
        else:
            fail("unknown compare JSON fixture: " + spec)
        emit(document if document is not None else {"status": "ahead"})
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
        if "reviews" in S["pages_fail"]:
            fail("controlled reviews failure")
        merge_base = BEHIND_MERGE_BASE if S["surface_merge_base"] is not None else BASE_TIP
        emit([S["reviews"] + [dict(r, body=r["body"].replace("base=BASE", "base=" + merge_base)) for r in S["owner_reviews"]]])
    elif "/issues/321/comments" in joined:
        if "comments" in S["pages_fail"]:
            fail("controlled comments failure")
        merge_base = BEHIND_MERGE_BASE if S["surface_merge_base"] is not None else BASE_TIP
        emit([S["comments"] + [dict(i, body=i["body"].replace("base=" + BASE_TIP, "base=" + merge_base)) for i in S["owner_items"]]])
    elif "/commits/" in joined and joined.rstrip().split("?")[0].endswith("/comments"):
        fail("a commit comment read per commit: the gate reads the repository's commit comments once")
    elif any(a.endswith("/comments?per_page=100") and "/issues/" not in a and "/pulls/" not in a and "/commits/" not in a for a in args):
        # the repository's commit comments, each naming the commit it is on (the pull request's head unless a scenario says otherwise)
        if S["commit_comments_fail"]:
            fail("controlled commit comments failure")
        pages = S["commit_comment_pages"] if S["commit_comment_pages"] is not None else [S["commit_comments"]]
        emit([[dict({"commit_id": HEAD}, **c) for c in page] for page in pages])
    elif "/pulls/321/comments" in joined:
        if "inline" in S["pages_fail"]:
            fail("controlled inline comments failure")
        emit([S["inline"]])
    elif "graphql" in joined:
        if S["history_fail"]:
            fail("controlled history failure")
        emit(S["history"])
    elif "branches/master" in joined:
        emit(BASE_TIP)
    else:
        fail("unknown API fixture")
else:
    fail("unknown command fixture")
