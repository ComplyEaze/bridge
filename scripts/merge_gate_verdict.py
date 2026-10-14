#!/usr/bin/env python3
"""Decide whether the owner's two machine lines stand on a pull request (merge-check spec v3).

A verdict is a pull request review or an issue comment (never an inline review comment) whose whole body, after CRLF to LF and at
most one trailing newline, is exactly one line, lowercase hex:
    MERGE-VERDICT CLEAN head=<40> base=<40>
    MERGE-ADVERSARY RECOMMEND head=<40> base=<40>
posted by the owner's numeric user id (a real User, not through an app). Both are needed for the current head and base; each is the
newest by server time; any edit, deletion or dismissal of ANY owner item, and any later owner line that is not a positive one, voids
every verdict that is not strictly newer. A pull request that carries either token in its title, body, commit messages, added lines or
anyone else's review or comment is refused. Unreadable evidence is indeterminate, never a pass.

Usage: merge_gate_verdict.py < DOC.json   (prints one JSON object; exit 0 pass, 1 block, 2 indeterminate)
"""
import json
import re
import sys
from datetime import datetime, timezone

OWNER_ID = 57982425
TOKENS = ("MERGE-VERDICT", "MERGE-ADVERSARY")
HEX40 = r"([0-9a-f]{40})"
CLEAN_RE = re.compile(r"MERGE-VERDICT CLEAN head=" + HEX40 + r" base=" + HEX40)
RECOMMEND_RE = re.compile(r"MERGE-ADVERSARY RECOMMEND head=" + HEX40 + r" base=" + HEX40)
VOID_RE = re.compile(r"MERGE-(?:VERDICT (?:NOT-CLEAN|RETRACT)|ADVERSARY (?:REJECT|RETRACT)) head=" + HEX40 + r" base=" + HEX40)
FOREVER = datetime.max.replace(tzinfo=timezone.utc)


class Unreadable(Exception):
    pass


def when(value):
    if not isinstance(value, str) or not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z", value):
        raise Unreadable("a timestamp could not be read")
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def one_line(body):
    """The single line a verdict must be, or None."""
    if not isinstance(body, str):
        return None
    text = body.replace("\r\n", "\n")
    if text.endswith("\n"):
        text = text[:-1]
    return None if "\n" in text or "\r" in text else text


def has_token(text):
    return isinstance(text, str) and any(token in text for token in TOKENS)


def is_owner(item):
    user = item.get("user")
    if not isinstance(user, dict):
        return False
    uid = user.get("id")
    return (type(uid) is int and uid == OWNER_ID and user.get("type") == "User"
            and item.get("performed_via_github_app") is None)


def evaluate(doc):
    head, base = doc["head"], doc["base"]
    owner_login = str(doc["owner_login"]).lower()
    reasons, refusals, unreadable = [], [], []
    voids = []
    positives = {"clean": [], "recommend": []}

    def item_time(item, kind):
        return when(item.get("submitted_at") if kind == "review" else item.get("created_at"))

    for kind, items in (("review", doc["reviews"]), ("comment", doc["comments"]), ("inline", doc["inline_comments"])):
        for item in items:
            body = item.get("body") or ""
            if not has_token(body):
                continue
            if not is_owner(item):
                refusals.append("a %s by someone other than the owner carries a merge-verdict token" % kind)
                continue
            try:
                at = item_time(item, kind) if kind != "inline" else when(item.get("created_at"))
            except Unreadable as error:
                unreadable.append(str(error))
                continue
            line = one_line(body)
            if kind == "inline":
                voids.append(at)  # an owner's inline comment is never a verdict; carrying the token voids what is older
                continue
            if line is not None and CLEAN_RE.fullmatch(line):
                positives["clean"].append((at, CLEAN_RE.fullmatch(line).groups()))
            elif line is not None and RECOMMEND_RE.fullmatch(line):
                positives["recommend"].append((at, RECOMMEND_RE.fullmatch(line).groups()))
            else:
                voids.append(at)  # a NOT-CLEAN, a RETRACT or any malformed owner line

    for label, text in (("title", doc["texts"]["title"]), ("body", doc["texts"]["body"])):
        if has_token(text):
            refusals.append("the pull request %s carries a merge-verdict token" % label)
    if any(has_token(m) for m in doc["texts"]["commit_messages"]):
        refusals.append("a commit message carries a merge-verdict token")
    if any(has_token(line) for line in doc["texts"]["added_lines"]):
        refusals.append("an added line carries a merge-verdict token")

    graph = doc.get("graphql")
    if not isinstance(graph, dict) or graph.get("complete") is not True:
        unreadable.append("the edit, deletion and dismissal history could not be read completely")
    else:
        try:
            for kind in ("reviews", "comments"):
                for node in graph[kind]:
                    if str(node.get("authorLogin") or "").lower() != owner_login:
                        continue
                    edited = node.get("lastEditedAt")
                    if edited is not None:
                        voids.append(when(edited))
                    elif node.get("editCount"):
                        voids.append(FOREVER)
            for event in graph["events"]:
                if event.get("type") == "CommentDeletedEvent" and str(event.get("authorLogin") or "").lower() == owner_login:
                    voids.append(when(event.get("createdAt")))
                elif event.get("type") == "ReviewDismissedEvent" and str(event.get("authorLogin") or "").lower() == owner_login:
                    voids.append(when(event.get("createdAt")))
            for review in doc["reviews"]:
                if is_owner(review) and review.get("state") == "DISMISSED":
                    dismissed = [when(e["createdAt"]) for e in graph["events"]
                                 if e.get("type") == "ReviewDismissedEvent" and str(e.get("authorLogin") or "").lower() == owner_login]
                    voids.append(max(dismissed) if dismissed else FOREVER)
        except (KeyError, TypeError, Unreadable) as error:
            unreadable.append("the edit, deletion and dismissal history was malformed (%s)" % type(error).__name__)

    newest_void = max(voids) if voids else None
    stands = {}
    for name, found in positives.items():
        mine = [at for at, (h, b) in found if h == head and b == base]
        if not mine:
            stands[name] = False
            reasons.append("no owner %s line for this head and merge base" % ("MERGE-VERDICT CLEAN" if name == "clean" else "MERGE-ADVERSARY RECOMMEND"))
        elif newest_void is not None and not max(mine) > newest_void:
            stands[name] = False
            reasons.append("the newest owner %s line is voided by a later or simultaneous edit, deletion, dismissal or withdrawal"
                           % ("MERGE-VERDICT CLEAN" if name == "clean" else "MERGE-ADVERSARY RECOMMEND"))
        else:
            stands[name] = True
    if unreadable:
        status = "indeterminate"
    elif refusals or not (stands["clean"] and stands["recommend"]):
        status = "block"
    else:
        status = "pass"
    return {"status": status, "clean": stands["clean"], "recommend": stands["recommend"],
            "reasons": reasons, "refusals": sorted(set(refusals)), "unreadable": unreadable}


if __name__ == "__main__":
    try:
        result = evaluate(json.load(sys.stdin))
    except (KeyError, TypeError, ValueError) as error:
        print(json.dumps({"status": "indeterminate", "unreadable": ["the evidence document was malformed (%s)" % type(error).__name__],
                          "reasons": [], "refusals": [], "clean": False, "recommend": False}))
        raise SystemExit(2)
    print(json.dumps(result, sort_keys=True))
    raise SystemExit({"pass": 0, "block": 1, "indeterminate": 2}[result["status"]])
