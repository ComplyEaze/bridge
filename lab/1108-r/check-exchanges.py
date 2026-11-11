"""#1108: checks a relay-keep.py folder after a row, and lists what it kept. Not for merge.

python -I check-exchanges.py RELAYDIR [FIRST LAST]

Reads RELAYDIR/relay.jsonl and every x-N file in RELAYDIR. FIRST and LAST bound the
exchanges listed and checked (both included); without them, every exchange is. For each
answered exchange the log holds, both files must exist, each must hold one HTTP head and
a body, and each body's size and sha256 must equal the log's `sent`/`sent_sha256` and
`answered`/`answer_sha256`. Exchange numbers must run 1, 2, 3 ... with no gap.

One line per exchange (capped at CAP): N, conn, request, bytes sent and answered, and
from the request's own HEADER, parsed as XML: TALLYREQUEST, TYPE and ID. A request kept
with no answer is listed as NO ANSWER and fails the check: a row needs every answer.

Exit 0 and `exchanges: ok N, FAIL 0` only when every check holds. A folder or log that
cannot be read prints NOT READ and exits 1, never "0 exchanges".
"""
import hashlib
import json
import os
import re
import sys
import xml.etree.ElementTree as ET

CAP = 400
NAME = re.compile(r"x-([0-9]+)\.(request|answer)\Z")


def not_read(text):
    print("NOT READ: " + text)
    sys.exit(1)


def split(data):
    if b"\r\n\r\n" not in data:
        return None
    return data.split(b"\r\n\r\n", 1)[1]


def header_of(body):
    if body[:2] == b"\xff\xfe":
        text = body[2:].decode("utf-16-le", "strict")
    elif len(body) > 1 and body[1:2] == b"\x00":
        text = body.decode("utf-16-le", "strict")
    else:
        text = body.decode("utf-8", "strict")
    root = ET.fromstring(text)

    def leaf(path):
        found = root.find(path)
        return (found.text or "").strip() if found is not None else "<absent>"

    return leaf("HEADER/TALLYREQUEST"), leaf("HEADER/TYPE"), leaf("HEADER/ID")


def main():
    if len(sys.argv) not in (2, 4):
        sys.exit("usage: check-exchanges.py RELAYDIR [FIRST LAST]")
    folder = sys.argv[1]
    try:
        names = os.listdir(folder)
        with open(os.path.join(folder, "relay.jsonl"), encoding="utf-8") as source:
            records = [json.loads(line) for line in source if line.strip()]
    except (OSError, ValueError) as error:
        not_read("%s: %s" % (folder, error))
    kept = {}
    for name in names:
        found = NAME.match(name)
        if found:
            kept.setdefault(int(found.group(1)), set()).add(found.group(2))
    if not kept:
        not_read("%s holds no x-N file" % folder)
    numbers = sorted(kept)
    failures = []
    if numbers != list(range(1, numbers[-1] + 1)):
        failures.append("exchange numbers have a gap: %s ... %s, %d files" % (numbers[0], numbers[-1], len(numbers)))
    first, last = (int(sys.argv[2]), int(sys.argv[3])) if len(sys.argv) == 4 else (1, numbers[-1])
    logged = {r["x"]: r for r in records if "event" not in r and "x" in r}
    shown = 0
    ok = 0
    for number in range(first, last + 1):
        row = logged.get(number)
        line = "x-%d" % number
        try:
            with open(os.path.join(folder, "x-%d.request" % number), "rb") as source:
                request = split(source.read())
        except OSError:
            request = None
        if request is None:
            failures.append("%s: request not kept whole" % line)
            continue
        try:
            what = " ".join(header_of(request))
        except (UnicodeDecodeError, ET.ParseError) as error:
            what = "HEADER not parsed (%s)" % type(error).__name__
        if row is None:
            failures.append("%s NO ANSWER: %s" % (line, what))
            continue
        try:
            with open(os.path.join(folder, "x-%d.answer" % number), "rb") as source:
                answer = split(source.read())
        except OSError:
            answer = None
        held = (
            answer is not None
            and len(request) == row["sent"]
            and hashlib.sha256(request).hexdigest() == row["sent_sha256"]
            and len(answer) == row["answered"]
            and hashlib.sha256(answer).hexdigest() == row["answer_sha256"]
        )
        if not held:
            failures.append("%s: kept bytes differ from the log" % line)
            continue
        ok += 1
        shown += 1
        if shown <= CAP:
            print("%s conn %s request %s sent %d answered %d %s" % (
                line, row["conn"], row["request"], row["sent"], row["answered"], what))
        elif shown == CAP + 1:
            print("... more than %d exchanges: the rest are not listed; the count below is unchanged" % CAP)
    for failure in failures[:CAP]:
        print("FAIL " + failure)
    print("exchanges: ok %d, FAIL %d (x-%d to x-%d)" % (ok, len(failures), first, last))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
