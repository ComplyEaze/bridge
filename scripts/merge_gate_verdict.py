#!/usr/bin/env python3
"""Decide whether the owner's two machine lines stand on a pull request (merge-check spec v3).

A verdict is a pull request review or an issue comment (never an inline review comment) whose whole body, after CRLF to LF and at
most one trailing newline, is exactly one line, lowercase hex:
    MERGE-VERDICT CLEAN head=<40> base=<40>
    MERGE-ADVERSARY RECOMMEND head=<40> base=<40>
posted by the owner's numeric user id (a real User, not through an app). Both are needed; the NEWEST line of each kind by server
time is the one that counts, and it must name the current head and base; any edit, deletion or dismissal of ANY owner item, and any later owner line that is not a positive one, voids
every verdict that is not strictly newer. A pull request that carries either token in its title, body, branch name, changed file
names, commit messages, commit comments, added lines or anyone else's review or comment is refused. Unreadable evidence is indeterminate, never a pass.

The line may also open a longer comment or review: its first line is the verdict line exactly, and the rest of the body (a reviewer's
written review) carries neither token, read the same way as everywhere else; a review that opens with the line is then the one thing the
reviewer posts. Prose before the line, a line with spaces around it or a token anywhere else in the body is a malformed owner line and voids.

The document may carry `carried`, {"head": H, "base": B}, which only the gate sets, after proving that the pull request's own diff is
byte-identical (apart from index lines and the text after the hunk numbers) at H and B and at the current head and merge base: a MERGE-VERDICT CLEAN line that names exactly that pair then stands
as if it named the current pair. The newest MERGE-ADVERSARY RECOMMEND line then stands too, under the same proof, but only when
the CLEAN line was carried and the RECOMMEND line names exactly the same older pair (not any other pair), is the newest of its kind,
is unambiguous and is not voided, judged exactly as the carried CLEAN line is. A RECOMMEND line that names the current pair stands on
its own, as ever. The result's `candidate_clean` names the pair of the newest standing-but-for-its-pair CLEAN line, which is what the
gate asks about; `recommend_carried` says whether the RECOMMEND line stands by the carry.

The document may carry `tier`. Only the exact string "one" (the review tier scripts/review-tier.mjs computes from the changed
paths) lets the MERGE-ADVERSARY RECOMMEND line be absent: a pull request of the one tier stands on MERGE-VERDICT CLEAN alone. Any
other value, or none, is the full tier and needs both lines. Every refusal and every void still applies at the one tier, and a
MERGE-ADVERSARY line that voids (a rejection or retraction) still voids the CLEAN line.

Usage: merge_gate_verdict.py < DOC.json   (prints one JSON object; exit 0 pass, 1 block, 2 indeterminate)
"""
import html
import json
from html.parser import HTMLParser
import re
import sys
import unicodedata
from datetime import datetime, timezone

OWNER_ID = 57982425
TOKENS = ("MERGE-VERDICT", "MERGE-ADVERSARY")
HEX40 = r"([0-9a-f]{40})"
CLEAN_RE = re.compile(r"MERGE-VERDICT CLEAN head=" + HEX40 + r" base=" + HEX40)
RECOMMEND_RE = re.compile(r"MERGE-ADVERSARY RECOMMEND head=" + HEX40 + r" base=" + HEX40)
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


def verdict_line(body):
    """The verdict line of an owner item: the whole body (as one_line) or, when more follows, its exact first line over prose with no token."""
    if not isinstance(body, str):
        return None
    text = body.replace("\r\n", "\n")
    if text.endswith("\n"):
        text = text[:-1]
    if "\r" in text:
        return None
    if "\n" not in text:
        return text
    first, rest = text.split("\n", 1)
    if (CLEAN_RE.fullmatch(first) or RECOMMEND_RE.fullmatch(first)) and rest.strip() and not has_token(rest):
        return first
    return None


DASHES = dict.fromkeys(list(range(0x2010, 0x2016)) + [0x2212, 0xFE58, 0xFE63, 0xFF0D], "-")


class _Readings(HTMLParser):
    """Text as the HTML parser reads it: `shown` is what a page shows, `all` also holds comment, declaration and processing text."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.shown, self.all = [], []

    def handle_data(self, data):
        self.shown.append(data)
        self.all.append(data)

    def handle_comment(self, data):
        self.all.append(data)

    def handle_decl(self, decl):
        self.all.append(decl)

    def unknown_decl(self, data):
        self.all.append(data)

    def handle_pi(self, data):
        self.all.append(data)


def parser_readings(text):
    """The two parser readings of the text; an unparseable text yields none (the regex readings still apply)."""
    try:
        parser = _Readings()
        parser.feed(text)
        parser.close()
    except Exception:
        return []
    return ["".join(parser.shown), "".join(parser.all)]


def has_token(text):
    """Whether the text carries either token, read without case and with every dash look-alike folded to a hyphen."""
    if not isinstance(text, str):
        return False
    readings = [re.sub(r"<!--.*?-->", "", text, flags=re.S),       # comments removed
                text.replace("<!--", "").replace("-->", ""),        # a token hidden inside a comment is still a token
                re.sub(r"<!--+>|--!>", "", text)] + parser_readings(text)   # forms the regexes do not know, and tags between the letters
    for reading in readings:
        plain = html.unescape(reading).replace("\\", "")
        plain = "".join(c for c in plain if unicodedata.category(c) != "Cf")
        folded = unicodedata.normalize("NFKC", plain).translate(DASHES).upper()
        if any(token in folded for token in TOKENS):
            return True
    return False


def is_owner_id(value):
    return type(value) is int and value == OWNER_ID


def is_owner(item):
    user = item.get("user")
    if not isinstance(user, dict):
        return False
    uid = user.get("id")
    return (type(uid) is int and uid == OWNER_ID and user.get("type") == "User"
            and item.get("performed_via_github_app") is None)


def evaluate(doc):
    head, base = doc["head"], doc["base"]
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
            line = verdict_line(body)
            if kind == "inline":
                voids.append(FOREVER)  # never a verdict; its time (a draft's) and its history are not read, so it voids everything
                continue
            if kind == "review" and item.get("state") in ("CHANGES_REQUESTED", "DISMISSED"):
                voids.append(at)  # a review that asks for changes or was dismissed is not a verdict, whatever its body says
                continue
            if line is not None and CLEAN_RE.fullmatch(line):
                positives["clean"].append((at, CLEAN_RE.fullmatch(line).groups()))
            elif line is not None and RECOMMEND_RE.fullmatch(line):
                positives["recommend"].append((at, RECOMMEND_RE.fullmatch(line).groups()))
            else:
                voids.append(at)  # a NOT-CLEAN, a RETRACT or any malformed owner line

    for label, text in (("title", doc["texts"]["title"]), ("body", doc["texts"]["body"]), ("branch name", doc["texts"].get("head_ref", ""))):
        if has_token(text):
            refusals.append("the pull request %s carries a merge-verdict token" % label)
    if any(has_token(name) for name in doc["texts"].get("filenames", [])):
        refusals.append("a changed file name carries a merge-verdict token")
    for item in doc.get("commit_comments", []):
        if has_token(item.get("body") or ""):
            if is_owner(item):
                voids.append(FOREVER)  # an owner's commit comment is never a verdict and its history is not read
            else:
                refusals.append("a commit comment by someone other than the owner carries a merge-verdict token")
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
                    unknown_author = node.get("authorId") is None
                    if not is_owner_id(node.get("authorId")) and not unknown_author:
                        continue
                    edited = node.get("lastEditedAt")
                    if edited is not None:
                        voids.append(when(edited))
                    elif node.get("editCount"):
                        voids.append(FOREVER)
            for event in graph["events"]:
                # an author that cannot be read (a deleted account) could be the owner: fail closed, it voids
                if event.get("type") in ("CommentDeletedEvent", "ReviewDismissedEvent") and (is_owner_id(event.get("authorId")) or event.get("authorId") is None):
                    voids.append(when(event.get("createdAt")))
            for review in doc["reviews"]:
                if is_owner(review) and review.get("state") == "DISMISSED":
                    dismissed = [when(e["createdAt"]) for e in graph["events"]
                                 if e.get("type") == "ReviewDismissedEvent" and is_owner_id(e.get("authorId"))]
                    voids.append(max(dismissed) if dismissed else FOREVER)
        except (KeyError, TypeError, Unreadable) as error:
            unreadable.append("the edit, deletion and dismissal history was malformed (%s)" % type(error).__name__)

    newest_void = max(voids) if voids else None
    recommend_required = doc.get("tier") != "one"
    carried = doc.get("carried")
    carried_pair = None
    if (isinstance(carried, dict) and set(carried) == {"head", "base"}
            and all(isinstance(carried[k], str) and re.fullmatch(r"[0-9a-f]{40}", carried[k]) for k in carried)):
        carried_pair = (carried["head"], carried["base"])
    candidate_clean = None
    clean_carried = False
    recommend_carried = False
    stands = {}
    for name, found in positives.items():
        newest = max(found, key=lambda entry: entry[0]) if found else None
        ambiguous = bool(found) and len({entry[1] for entry in found if entry[0] == newest[0]}) > 1
        if name == "recommend" and not recommend_required:
            stands[name] = bool(newest is not None and not ambiguous and newest[1] == (head, base)
                                and (newest_void is None or newest[0] > newest_void))
            reasons.append("review tier one: MERGE-ADVERSARY RECOMMEND is not required")
        elif name == "clean" and newest is not None and not ambiguous and carried_pair is not None and newest[1] == carried_pair \
                and (newest_void is None or newest[0] > newest_void):
            stands[name] = True
            clean_carried = True
            reasons.append("the newest owner MERGE-VERDICT CLEAN line names an older head and merge base and is carried: the gate proved the pull request's own patch unchanged")
        elif name == "recommend" and clean_carried and newest is not None and not ambiguous and newest[1] == carried_pair \
                and (newest_void is None or newest[0] > newest_void):
            # the same pair, the same proof, the same voids as the carried CLEAN line; a RECOMMEND line for any other older pair falls through
            stands[name] = True
            recommend_carried = True
            reasons.append("the newest owner MERGE-ADVERSARY RECOMMEND line names the same older head and merge base as the carried CLEAN line and is carried too: the gate proved the pull request's own patch unchanged")
        elif newest is None or ambiguous or newest[1] != (head, base):
            if name == "clean" and newest is not None and not ambiguous and (newest_void is None or newest[0] > newest_void):
                candidate_clean = {"head": newest[1][0], "base": newest[1][1]}
            stands[name] = False
            reasons.append("the newest owner %s line is missing or names another head or merge base" % ("MERGE-VERDICT CLEAN" if name == "clean" else "MERGE-ADVERSARY RECOMMEND"))
        elif newest_void is not None and not newest[0] > newest_void:
            stands[name] = False
            reasons.append("the newest owner %s line is voided by a later or simultaneous edit, deletion, dismissal or withdrawal"
                           % ("MERGE-VERDICT CLEAN" if name == "clean" else "MERGE-ADVERSARY RECOMMEND"))
        else:
            stands[name] = True
    if unreadable:
        status = "indeterminate"
    elif refusals or not (stands["clean"] and (stands["recommend"] or not recommend_required)):
        status = "block"
    else:
        status = "pass"
    return {"status": status, "clean": stands["clean"], "recommend": stands["recommend"], "recommend_required": recommend_required, "recommend_carried": recommend_carried, "candidate_clean": candidate_clean,
            "reasons": reasons, "refusals": sorted(set(refusals)), "unreadable": unreadable}


if __name__ == "__main__":
    try:
        result = evaluate(json.load(sys.stdin))
    except Exception as error:
        print(json.dumps({"status": "indeterminate", "unreadable": ["the evidence document was malformed (%s)" % type(error).__name__],
                          "reasons": [], "refusals": [], "clean": False, "recommend": False}))
        raise SystemExit(2)
    print(json.dumps(result, sort_keys=True))
    raise SystemExit({"pass": 0, "block": 1, "indeterminate": 2}[result["status"]])
