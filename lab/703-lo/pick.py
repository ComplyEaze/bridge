"""Row 703-LO: pick the smallest, median and largest party by bills from a kept Receivable answer. Not for merge.

python -I pick.py ANSWER OUTDIR

ANSWER is row 2377-B's kept `answers/d2-ref-receivable.answer.xml` (BRIDGE CORPUS DENSE, Bills
Receivable to 31 March 2026). It is refused unless its size and sha256 are the ones that row
recorded. Nothing is sent to Tally.

Counts the BILLPARTY elements of the answer by their exact text, keeps the names
LedgerScopeName admits (ASCII letters and digits with single inner spaces, no leading or
trailing space, at most 1,024 characters, no formula word), and orders them by bills, then by
the name's UTF-8 bytes:
  s1  the first in ascending order (fewest bills);
  m1  the lower median of the ascending order, index (n - 1) // 2;
  l1  the first in descending order of bills (ties by the name's bytes), row 2377-B's top1 rule.
Writes OUTDIR/s1.name, m1.name and l1.name, each the name's UTF-8 bytes with no line terminator,
and prints the counts with no name. OUTDIR must be new.
"""
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET
from collections import Counter

SIZE = 13_679_490
SHA256 = "5a406cab1a2fe7f6581591a0e910215c48cf380ce04f7558b0896f109a560f5a"
NAME = re.compile(r"\A[A-Za-z0-9]+( [A-Za-z0-9]+)*\Z")
MAX_CHARS = 1024
FORMULA_WORDS = ("and", "or", "not", "is", "in", "contains", "like", "between", "starting", "ending", "with",
                 "equal", "greater", "less", "than")
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")


class Failed(Exception):
    pass


def keep_reference(match):
    body = match.group(1)
    code = int(body[1:], 16) if body[0] == "x" else int(body)
    return "" if code <= 8 or code in (11, 12) or 14 <= code <= 31 else match.group(0)


def decoded(data):
    if data.startswith(b"\xff\xfe"):
        body = data[2:].decode("utf-16-le")
    elif len(data) >= 2 and len(data) % 2 == 0 and data[1] == 0:
        body = data.decode("utf-16-le")
    else:
        raise Failed("the answer is not UTF-16LE")
    return REFERENCE.sub(keep_reference, FORBIDDEN_RAW.sub("", body))


def admitted(name):
    return (NAME.match(name) is not None and len(name) <= MAX_CHARS
            and not any(word.lower() in FORMULA_WORDS for word in name.split(" ")))


def main(answer, outdir):
    with open(answer, "rb") as handle:
        data = handle.read()
    digest = hashlib.sha256(data).hexdigest()
    if len(data) != SIZE or digest != SHA256:
        raise Failed("the answer is {} B, sha256 {}; row 2377-B recorded {} B, {}".format(len(data), digest, SIZE, SHA256))
    try:
        root = ET.fromstring(decoded(data))
    except ET.ParseError as error:
        raise Failed("the answer does not parse ({})".format(error))
    parties = [element.text or "" for element in root.iter("BILLPARTY")]
    names = Counter(parties)
    empty = names.pop("", 0)
    kept = sorted((n for n in names if admitted(n)), key=lambda n: (names[n], n.encode("utf-8")))
    if len(kept) < 3:
        raise Failed("{} admitted parties, fewer than three".format(len(kept)))
    picks = {
        "s1": kept[0],
        "m1": kept[(len(kept) - 1) // 2],
        "l1": sorted(kept, key=lambda n: (-names[n], n.encode("utf-8")))[0],
    }
    if len(set(picks.values())) != 3:
        raise Failed("the three picks are not three parties")
    os.makedirs(outdir)
    lines = ["bills {}; empty BILLPARTY {}; parties {}; admitted {}".format(len(parties), empty, len(names), len(kept))]
    histogram = Counter(names[n] for n in kept)
    lines.append("admitted parties by bills: " + ", ".join("{} bills x {}".format(b, histogram[b]) for b in sorted(histogram)))
    for label, name in picks.items():
        raw = name.encode("utf-8")
        with open(os.path.join(outdir, label + ".name"), "xb") as handle:
            handle.write(raw)
        lines.append("{} bills {} name {} B sha256 {}".format(label, names[name], len(raw), hashlib.sha256(raw).hexdigest()))
    return "\n".join(lines)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(__doc__)
        sys.exit(2)
    try:
        print(main(sys.argv[1], sys.argv[2]))
    except (Failed, OSError, ValueError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
