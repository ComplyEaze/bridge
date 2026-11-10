# Row 1381-R1P: check each of row 1381-R1's kept-local answers before it is pushed.
#
#   python -I check-push.py <row 1381-R1 folder> <report file>
#
# File work only: no request is sent and Tally is not touched. For each answer listed below it
# checks the size and sha256 against row 1381-R1's kept-local.txt, decodes it as UTF-16LE with no
# byte order mark, and counts, never prints:
#   - GSTIN-shaped values outside the synthetic identifier family;
#   - COMPANYNUMBER elements with any text;
#   - GUID-shaped values that are not the registered identifier of BRIDGE STOCK CREATE LAB (a
#     value that is that identifier followed by a hyphen and digits, as an item's GUID is, counts
#     as that identifier);
#   - drive-letter paths, home-folder paths (a Users folder between path separators), email shapes, and runs of 10 or more digits.
# An answer is PUSH only if its size and sha256 match and every count above is 0; otherwise it is
# KEEP with the names of the counts that are not 0. The report holds names, sizes, sha256s,
# counts and the verdicts only. It refuses to overwrite an existing report.

import hashlib
import os
import re
import sys

ANSWERS = [  # name, bytes, sha256, as row 1381-R1's kept-local.txt lists them (lab/1381-r1-answers at 3b50a15e)
    ("a-a1.xml", 8270, "92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4"),
    ("a-a1b1.xml", 10758, "2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd"),
    ("a-a1b2.xml", 15922, "f58ab8debba4d819c53e3d58df3d84b1bbfb050e35b78a9d506ee1cd74fc2ba3"),
    ("a-a1c.xml", 15998, "3bdb313e443c8de87fa6f9c593611307833f93e29dc2f348133e76f00c5b800c"),
    ("a-g1.xml", 4170, "b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a"),
    ("a-u1.xml", 4040, "cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39"),
    ("a-m0.xml", 3318, "ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1"),
]
# BRIDGE STOCK CREATE LAB's identifier, as #2419 lists it in scripts/merge-gate-synthetic-uuids.json.
REGISTERED = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1"

GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][0-9A-Z]Z[0-9A-Z]")  # count-ed.py's pattern
FAMILY = re.compile(r"\d{2}ZZZZZ0000Z[A-Za-z0-9]Z[A-Za-z0-9]")
GUID = re.compile(r"[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}")
COMPANYNUMBER = re.compile(r"<COMPANYNUMBER\b[^>]*>([^<]*)</COMPANYNUMBER>")
CHECKS = {
    "drive path": re.compile(r"(?<![A-Za-z])[A-Za-z]:\\"),
    "Users folder": re.compile(r"[\\/]Users[\\/]", re.I),
    "email shape": re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+\.[A-Za-z]{2,}"),
    "10+ digit run": re.compile(r"\d{10,}"),
}


def counts(text):
    found = {
        "GSTIN outside the family": sum(1 for m in GSTIN.finditer(text) if not FAMILY.fullmatch(m.group(0))),
        "COMPANYNUMBER with text": sum(1 for m in COMPANYNUMBER.finditer(text) if m.group(1).strip()),
        "other GUID": sum(1 for m in GUID.finditer(text) if m.group(0).lower() != REGISTERED),
    }
    for name, pattern in CHECKS.items():
        found[name] = len(pattern.findall(text))
    return found


def main():
    if len(sys.argv) != 3:
        print("REFUSED: usage: check-push.py <row 1381-R1 folder> <report file>")
        sys.exit(1)
    folder, report = sys.argv[1], sys.argv[2]
    if os.path.exists(report):
        print("REFUSED: " + report + " already exists")
        sys.exit(1)
    lines = ["Row 1381-R1P: row 1381-R1's kept-local answers checked before a push (check-push.py)"]
    push = 0
    for name, size, sha in ANSWERS:
        path = os.path.join(folder, "answers", name)
        if not os.path.isfile(path):
            lines.append("%s: KEEP (not found)" % name)
            continue
        body = open(path, "rb").read()
        got = hashlib.sha256(body).hexdigest()
        if len(body) != size or got != sha:
            lines.append("%s: KEEP (%d B, sha256 %s: not the kept-local entry)" % (name, len(body), got))
            continue
        if body[:2] in (b"\xff\xfe", b"\xfe\xff") or len(body) % 2:
            lines.append("%s: KEEP (not UTF-16LE without a byte order mark)" % name)
            continue
        text = body.decode("utf-16-le")
        found = counts(text)
        registered = text.lower().count(REGISTERED)
        bad = [k for k, v in found.items() if v]
        detail = ", ".join("%s %d" % (k, v) for k, v in found.items())
        verdict = "PUSH" if not bad else "KEEP (" + ", ".join(bad) + ")"
        push += not bad
        lines.append("%s: %s; %d B, sha256 %s; registered identifier %d; %s" % (name, verdict, size, sha, registered, detail))
    lines.append("PUSH %d of %d" % (push, len(ANSWERS)))
    with open(report + ".part", "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    os.replace(report + ".part", report)
    print("\n".join(lines))


main()
