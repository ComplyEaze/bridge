# Row 2440-ED-D: make a derived copy of row 2440-ED's s1 answer whose GSTIN-shaped values are
# replaced by members of the synthetic identifier family, and report what changed.
#
#   python -I derive-gstin.py <s1 answer> <expected sha256> <expected count> <derived file> <report file>
#
# File work only: no request is sent and Tally is not touched. The answer is read as UTF-16LE
# with no byte order mark, as Tally answered it. Every GSTIN-shaped value (count-ed.py's pattern)
# that is not already a family member is replaced by a family member of the same length: the
# value's own two state digits, the PAN part ZZZZZ0000Z, an entity character numbering the
# distinct value in order of first appearance (1, 2, ...), Z, and the GSTIN check character
# computed for those fourteen characters. Equal values get equal replacements, so the answer
# keeps which rows share a GSTIN.
#
# The original values are never printed or written: the report gives only offsets, counts,
# sizes, sha256s and the replacements, which are family members. The script fails closed and
# writes nothing if the input's sha256 or the number of values differs from what is given, if
# either output already exists, or if any check after the replacement fails.

import hashlib
import os
import re
import sys
import xml.parsers.expat

GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][0-9A-Z]Z[0-9A-Z]")  # count-ed.py's pattern
FAMILY = re.compile(r"\d{2}ZZZZZ0000Z[A-Za-z0-9]Z[A-Za-z0-9]")
CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def check_character(first14):
    total = 0
    for i, ch in enumerate(first14):
        v = CHARS.index(ch) * (2 if i % 2 else 1)
        total += v // 36 + v % 36
    return CHARS[(36 - total % 36) % 36]


def elements(text):
    """Every element's name and attributes, in document order, by expat (never a text search).
    Tally writes control characters as numeric references XML 1.0 refuses; as in count-ed.py, each
    is blanked with spaces of the same length, in the copy expat reads only."""
    text = re.sub(r"&#(\d+);", lambda m: " " * len(m.group(0)) if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    seen = []
    p = xml.parsers.expat.ParserCreate("UTF-16LE")
    p.StartElementHandler = lambda name, attrs: seen.append((name, tuple(sorted(attrs.items()))))
    p.Parse(text.encode("utf-16-le"), True)
    return seen


def fail(why):
    print("REFUSED: " + why)
    sys.exit(1)


def main():
    if len(sys.argv) != 6:
        fail("usage: derive-gstin.py <s1 answer> <expected sha256> <expected count> <derived file> <report file>")
    src, want_sha, want_count, out, report = sys.argv[1], sys.argv[2].lower(), sys.argv[3], sys.argv[4], sys.argv[5]
    for path in (out, report):
        if os.path.exists(path):
            fail(path + " already exists")
    body = open(src, "rb").read()
    sha = hashlib.sha256(body).hexdigest()
    if sha != want_sha:
        fail("the input's sha256 is %s, not the expected one" % sha)
    if body[:2] in (b"\xff\xfe", b"\xfe\xff") or len(body) % 2:
        fail("the input is not UTF-16LE without a byte order mark")
    text = body.decode("utf-16-le")
    found = [m for m in GSTIN.finditer(text) if not FAMILY.fullmatch(m.group(0))]
    if str(len(found)) != want_count:
        fail("the input holds %d GSTIN-shaped values outside the family, not %s" % (len(found), want_count))

    distinct, replaced, rows = {}, list(text), []
    for m in found:
        value = m.group(0)
        if value not in distinct:
            if len(distinct) >= 9:
                fail("more than nine distinct values")
            first14 = value[:2] + "ZZZZZ0000Z" + str(len(distinct) + 1) + "Z"
            distinct[value] = first14 + check_character(first14)
        new = distinct[value]
        if len(new) != len(value) or not FAMILY.fullmatch(new):
            fail("a replacement is not a same-length family member")
        replaced[m.start():m.end()] = list(new)
        rows.append((m.start(), m.start() * 2, list(distinct).index(value) + 1, new))
    derived = "".join(replaced)
    data = derived.encode("utf-16-le")

    # Checks after the replacement: same length, nothing outside the family left, only the
    # replaced spans differ, and expat reads the same elements in the same order.
    if len(data) != len(body):
        fail("the derived file's size differs")
    if [m for m in GSTIN.finditer(derived) if not FAMILY.fullmatch(m.group(0))]:
        fail("a GSTIN-shaped value outside the family remains")
    spans = set()
    for m in found:
        spans.update(range(m.start(), m.end()))
    differing = [i for i in range(len(text)) if text[i] != derived[i]]
    if any(i not in spans for i in differing):
        fail("a code unit outside the replaced values differs")
    try:
        same = elements(text) == elements(derived)
    except xml.parsers.expat.ExpatError as e:
        fail("expat could not read the answer: %s" % e)
    if not same:
        fail("the elements differ")

    lines = [
        "Row 2440-ED-D: s1 with its GSTIN-shaped values replaced by synthetic family members (derive-gstin.py)",
        "input: %s, %d B, sha256 %s" % (os.path.basename(src), len(body), sha),
        "derived: %s, %d B, sha256 %s" % (os.path.basename(out), len(data), hashlib.sha256(data).hexdigest()),
        "values replaced: %d (%d distinct), each by a same-length family member" % (len(found), len(distinct)),
        "code units that differ: %d, all inside the replaced values" % len(differing),
        "elements by expat: %d in each, the same names and attributes in the same order" % len(elements(text)),
        "occurrences (code-unit offset, byte offset, distinct value, replacement):",
    ]
    lines += ["  %d, %d, %d, %s" % row for row in rows]
    tmp_out, tmp_report = out + ".part", report + ".part"
    with open(tmp_out, "wb") as f:
        f.write(data)
    with open(tmp_report, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    os.replace(tmp_out, out)
    os.replace(tmp_report, report)
    print("\n".join(lines))


main()
