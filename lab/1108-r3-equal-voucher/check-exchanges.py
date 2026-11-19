"""#1108: checks a relay-keep.py folder after a row, and lists what it kept. Not for merge.

python -I check-exchanges.py RELAYDIR [FIRST LAST]

Reads RELAYDIR/relay.jsonl and every x-N and import-N file in RELAYDIR. FIRST and LAST bound
the exchanges listed and checked (both included); without them, every exchange is. A range that
holds no exchange (FIRST above LAST, FIRST below 1, or either not a whole number) fails: it is never
"ok 0". For each answered exchange the log holds, both files must exist, each must hold one HTTP
head and a body, and each body's size and sha256 must equal the log's `sent`/`sent_sha256` and
`answered`/`answer_sha256`. Exchange numbers must run 1, 2, 3 ... with no gap.

What a request is, from its own bytes: `GET /status` with an empty body is a status exchange (Bridge
sends it, and it has no HEADER). Any other request must be a POST whose body parses as XML; its
TALLYREQUEST, TYPE and ID are listed from its HEADER. A body that does not parse fails.

An import (the log names it with `number`) must also be kept as import-M, and import-M.request and
import-M.answer must be byte-equal to x-N.request and x-N.answer; an import-M file the log does not
name fails too.

One line per exchange (capped at CAP). A request kept with no answer is listed as NO ANSWER and
fails the check: a row needs every answer.

Exit 0 and `exchanges: ok N, FAIL 0` only when every check holds; the line also counts the imports
in the range. A folder or log that cannot be read prints NOT READ and exits 1, never "0 exchanges".
"""
import hashlib
import json
import os
import re
import sys
import xml.etree.ElementTree as ET

CAP = 400
NAME = re.compile(r"(x|import)-([0-9]+)\.(request|answer)\Z")


def not_read(text):
    print("NOT READ: " + text)
    sys.exit(1)


def read(path):
    try:
        with open(path, "rb") as source:
            return source.read()
    except OSError:
        return None


def split(data):
    if data is None or b"\r\n\r\n" not in data:
        return None, None
    head, body = data.split(b"\r\n\r\n", 1)
    return head, body


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


def what_of(head, body):
    """The request as listed, or None with the reason when it is neither a status exchange nor a parsed POST."""
    line = head.split(b"\r\n", 1)[0]
    if line == b"GET /status HTTP/1.1":
        return ("GET /status", None) if body == b"" else (None, "GET /status with a body")
    if not line.startswith(b"POST "):
        return None, "request line %r" % line[:60]
    try:
        return " ".join(header_of(body)), None
    except (UnicodeDecodeError, ET.ParseError) as error:
        return None, "HEADER not parsed (%s)" % type(error).__name__


def bounds(folder_last):
    if len(sys.argv) == 2:
        return 1, folder_last
    try:
        first, last = int(sys.argv[2]), int(sys.argv[3])
    except ValueError:
        return None
    if first < 1 or last < first:
        return None
    return first, last


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
    kept, imports_kept = {}, set()
    for name in names:
        found = NAME.match(name)
        if found and found.group(1) == "x":
            kept.setdefault(int(found.group(2)), set()).add(found.group(3))
        elif found:
            imports_kept.add(int(found.group(2)))
    if not kept:
        not_read("%s holds no x-N file" % folder)
    numbers = sorted(kept)
    failures = []
    if numbers != list(range(1, numbers[-1] + 1)):
        failures.append("exchange numbers have a gap: %s ... %s, %d files" % (numbers[0], numbers[-1], len(numbers)))
    span = bounds(numbers[-1])
    if span is None:
        print("FAIL range %s to %s holds no exchange" % tuple(sys.argv[2:4]))
        print("exchanges: ok 0, FAIL 1 (no range)")
        sys.exit(1)
    first, last = span
    logged = {r["x"]: r for r in records if "event" not in r and "x" in r}
    import_of = {r["x"]: r["number"] for r in records if r.get("number") is not None and "x" in r}
    named = set(import_of.values())
    for number in sorted(imports_kept - named):
        failures.append("import-%d: kept, but no log line names it" % number)
    shown = ok = imports = 0
    for number in range(first, last + 1):
        row = logged.get(number)
        line = "x-%d" % number
        raw_request = read(os.path.join(folder, "x-%d.request" % number))
        head, request = split(raw_request)
        if request is None:
            failures.append("%s: request not kept whole" % line)
            continue
        what, why = what_of(head, request)
        if what is None:
            failures.append("%s: %s" % (line, why))
            continue
        if row is None:
            failures.append("%s NO ANSWER: %s" % (line, what))
            continue
        raw_answer = read(os.path.join(folder, "x-%d.answer" % number))
        answer = split(raw_answer)[1]
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
        if number in import_of:
            imports += 1
            stem = os.path.join(folder, "import-%d" % import_of[number])
            if read(stem + ".request") != raw_request or read(stem + ".answer") != raw_answer:
                failures.append("%s: import-%d is not byte-equal to it" % (line, import_of[number]))
                continue
            what += " (import-%d, byte-equal)" % import_of[number]
        ok += 1
        shown += 1
        if shown <= CAP:
            print("%s conn %s request %s sent %d answered %d %s" % (
                line, row["conn"], row["request"], row["sent"], row["answered"], what))
        elif shown == CAP + 1:
            print("... more than %d exchanges: the rest are not listed; the count below is unchanged" % CAP)
    for failure in failures[:CAP]:
        print("FAIL " + failure)
    print("exchanges: ok %d, FAIL %d (x-%d to x-%d), imports %d" % (ok, len(failures), first, last, imports))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
