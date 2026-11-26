#!/usr/bin/env python3
"""Send one of row 1381-RR's five reads to the lab's Tally, in the row's fixed order, and keep the answer exactly as received.

Row 1381-RR, from row 1381-RA's send.py (lab/1381-ra-sheet at 30f5c820, lab/1381-ra/send.py) with these changes: the
five labels M1 M2 I G U in place of RA's ten (no count read); the label pattern admits a one-letter label; RA's C1 gate
is replaced by three: M2 needs M1's state met and the restore's after-record (restore.py after), and I needs M2's
state met. Everything else is RA's: the label names its request (no file argument, so nothing outside the five reads
can be sent); each label is sent once in the row's life, whatever happened to it; a failed send stops every later one;
the answer, its headers and the transport record are three files, so a failed request never reads as an empty answer
(AGENTS.md P5); everything goes under local/. Not product code. Python 3.10, standard library only.

  python -I -X utf8 send.py LABEL         LABEL is one of M1 M2 I G U, sent in that order
  python -I -X utf8 send.py --selftest SCRATCH
      the verdict and order rules on synthetic cases, in SCRATCH (under local/, or outside the row folder); sends
      nothing

Before a send: the request's sha256 and size must equal its line in SHA256.txt; the send log must hold exactly the
labels before LABEL, each SENT-OK, and not LABEL itself; M2 needs local/evidence/m1-state.txt to hold the lines
`BOOK STATE MET` and `exit 0` (measure.py state M1) and local/evidence/r-after.txt to hold a line beginning
`AFTER RECORDED: ` and `exit 0` (restore.py after); I needs local/evidence/m2-state.txt to hold `BOOK STATE MET` and
`exit 0` (measure.py state M2); no answer, header or transport file of LABEL may exist. Any miss: nothing is sent, exit 2.

The send: LAB-COMMON v3 C3's curl line, to 127.0.0.1:9001 only, with no client timeout:
  curl.exe -sS -w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}" --data-binary @<request>
           -H "Content-Type: text/xml; charset=utf-16" -D <headers> http://127.0.0.1:9001 -o <answer>
Then local/answers/a-<LABEL>.transport.txt records curl's exit code, its -w line, the last HTTP status line and
CONTENT-LENGTH of the headers, the answer's bytes and sha256, curl's error output whole, and one verdict line:
`VERDICT SENT-OK`, or `VERDICT REQUEST-FAILED: <reasons>` when curl exited non-zero, the status is not 200,
CONTENT-LENGTH is missing or differs from the bytes written, curl's size differs, the answer is empty, or it is above
2,000,000 bytes (LAB-COMMON v3 C3's default bound; the file is kept and not used). One line goes to
local/evidence/send-log.txt. Exit 0: SENT-OK. Exit 1: REQUEST-FAILED (STOP-REQUEST-FAILED; never sent again).
"""

import hashlib
import pathlib
import re
import subprocess
import sys
from datetime import datetime, timezone

ROOT = pathlib.Path(__file__).resolve().parent
URL = "http://127.0.0.1:9001"
BOUND = 2_000_000
ORDER = (("M1", "requests/marks.utf16le.xml"), ("M2", "requests/marks.utf16le.xml"),
         ("I", "requests/items.utf16le.xml"), ("G", "requests/groups.utf16le.xml"),
         ("U", "requests/units.utf16le.xml"))
# The record each gated label needs, and the two lines that record must hold.
GATES = {"M2": (("m1-state.txt", "BOOK STATE MET"), ("r-after.txt", "AFTER RECORDED")),
         "I": (("m2-state.txt", "BOOK STATE MET"),)}
LOG_LINE = re.compile(r"\S+ (?P<label>[A-Z][0-9]?) \S+ .* \| VERDICT (?P<verdict>SENT-OK|REQUEST-FAILED)\b.*")


class Refused(Exception):
    pass


def utc():
    t = datetime.now(timezone.utc)
    return t.strftime("%Y-%m-%dT%H:%M:%S.") + f"{t.microsecond // 1000:03d}Z"


def listed(root):
    rows = {}
    for row in (root / "SHA256.txt").read_text(encoding="utf-8").splitlines():
        parts = row.split("  ")
        if len(parts) == 3:
            rows[parts[2]] = (parts[0], int(parts[1]))
    return rows


def log_entries(log):
    """The send log's (label, verdict) pairs in order; a line that does not parse refuses everything after it."""
    if not log.exists():
        return []
    entries = []
    for number, row in enumerate(log.read_text(encoding="utf-8").splitlines(), 1):
        match = LOG_LINE.fullmatch(row)
        if not match:
            raise Refused(f"send-log.txt line {number} is not a send line: nothing is sent")
        entries.append((match["label"], match["verdict"]))
    return entries


def admit(label, entries, records):
    """Why LABEL may not be sent now, or None. entries: the log so far; records: {record name: its text or None}."""
    labels = [name for name, _ in ORDER]
    if label not in labels:
        return f"{label!r} is not one of {' '.join(labels)}"
    if any(name == label for name, _ in entries):
        return f"{label} is in the send log already: a read is sent once in the row's life"
    failed = [name for name, verdict in entries if verdict != "SENT-OK"]
    if failed:
        return f"{failed[0]} failed: nothing more is sent (STOP-REQUEST-FAILED)"
    want = labels[: labels.index(label)]
    if [name for name, _ in entries] != want:
        return f"the log holds {[n for n, _ in entries]}, not {want}: out of order"
    for record, line in GATES.get(label, ()):
        lines = (records.get(record) or "").splitlines()
        if not any(x == line or x.startswith(line + ": ") for x in lines) or "exit 0" not in lines:
            return f"local/evidence/{record} does not hold a line `{line}` (or `{line}: ...`) and `exit 0`"
    return None


def verdict(curl_exit, written, headers, body):
    """The reasons a send failed ([] when it did not), from curl's exit, its -w line, the headers and the body."""
    reasons = []
    fields = written.split()
    status = fields[0] if fields else ""
    if curl_exit != 0:
        reasons.append(f"curl exit {curl_exit}")
    if status != "200":
        reasons.append(f"status {status or 'none'}")
    lengths = re.findall(r"(?im)^content-length:\s*(\d+)\s*$", headers)
    if not lengths:
        reasons.append("no CONTENT-LENGTH")
    elif int(lengths[-1]) != len(body):
        reasons.append(f"CONTENT-LENGTH {lengths[-1]} but {len(body)} bytes written")
    if len(fields) < 2 or fields[1] != str(len(body)):
        reasons.append(f"curl's size {fields[1] if len(fields) > 1 else 'none'} but {len(body)} bytes written")
    if not body:
        reasons.append("empty answer")
    if len(body) > BOUND:
        reasons.append(f"{len(body)} bytes, above the {BOUND}-byte bound: kept, not used")
    return reasons


def send(label):
    local = ROOT / "local"
    answers, evidence = local / "answers", local / "evidence"
    request = dict(ORDER).get(label)
    try:  # Everything before curl starts: a failure here sends nothing.
        records = {}
        for record, _ in GATES.get(label, ()):
            path = evidence / record
            records[record] = path.read_text(encoding="utf-8") if path.exists() else None
        reason = admit(label, log_entries(evidence / "send-log.txt"), records)
        if reason:
            raise Refused(reason)
        body = (ROOT / request).read_bytes()
        digest = hashlib.sha256(body).hexdigest()
        if listed(ROOT).get(request) != (digest, len(body)):
            raise Refused(f"{request} differs from its SHA256.txt line, or has none")
    except OSError as error:
        raise Refused(f"{type(error).__name__}: {error}")
    answer = answers / f"a-{label}.xml"
    headers = answers / f"a-{label}.headers.txt"
    record = answers / f"a-{label}.transport.txt"
    for path in (answer, headers, record):
        if path.exists():
            raise Refused(f"{path.relative_to(ROOT).as_posix()} exists: never overwritten")
    answers.mkdir(parents=True, exist_ok=True)
    evidence.mkdir(parents=True, exist_ok=True)
    start = utc()
    try:  # curl.exe that cannot start has sent nothing.
        done = subprocess.run(["curl.exe", "-sS", "-w", "%{http_code} %{size_download} %{time_starttransfer} %{time_total}",
                               "--data-binary", f"@{ROOT / request}", "-H", "Content-Type: text/xml; charset=utf-16",
                               "-D", str(headers), URL, "-o", str(answer)], capture_output=True, text=True)
    except OSError as error:
        raise Refused(f"curl.exe did not start ({type(error).__name__}: {error})")
    end = utc()
    got = answer.read_bytes() if answer.exists() else b""
    head = headers.read_text(encoding="latin-1") if headers.exists() else ""
    reasons = verdict(done.returncode, done.stdout.strip(), head, got)
    statuses = re.findall(r"(?m)^HTTP/\S+ .*$", head)
    lengths = re.findall(r"(?im)^content-length:\s*(\d+)\s*$", head)
    said = "SENT-OK" if not reasons else "REQUEST-FAILED: " + "; ".join(reasons)
    record.write_text("\n".join([
        f"label {label}", f"request {request} {len(body)} B sha256 {digest}", f"start {start}", f"end {end}",
        f"curl exit {done.returncode}", f"curl -w line: {done.stdout.strip()!r}",
        f"last status line: {statuses[-1].strip() if statuses else 'none'}",
        f"CONTENT-LENGTH: {lengths[-1] if lengths else 'none'}",
        f"answer file: {'present' if answer.exists() else 'absent'}, {len(got)} B sha256 {hashlib.sha256(got).hexdigest()}",
        f"curl error output ({len(done.stderr)} characters): {done.stderr!r}", f"VERDICT {said}"]) + "\n",
        encoding="utf-8", newline="\n")
    fields = (done.stdout.strip().split() + ["", "", "", ""])[:4]
    row = (f"{start} {label} {request} {len(body)} B sha256 {digest} | curl exit {done.returncode} status {fields[0]} "
           f"CONTENT-LENGTH {lengths[-1] if lengths else 'none'} answer {len(got)} B sha256 "
           f"{hashlib.sha256(got).hexdigest()} | first byte {fields[2]} s total {fields[3]} s | VERDICT {said}")
    with open(evidence / "send-log.txt", "a", encoding="utf-8", newline="\n") as log:
        log.write(row + "\n")
    print(row)
    return 0 if not reasons else 1


def selftest(scratch):
    scratch = pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    scratch.mkdir(parents=True, exist_ok=True)
    ok = "HTTP/1.1 200 OK\r\nCONTENT-LENGTH: 4\r\n\r\n"
    cases = [
        ("a whole answer", (0, "200 4 0.01 0.01", ok, b"\x00" * 4), []),
        ("connection refused", (7, "000 0 0 0", "", b""), ["curl exit 7", "status 000", "no CONTENT-LENGTH", "empty answer"]),
        ("status 500", (0, "500 4 0.01 0.01", ok.replace("200 OK", "500 Error"), b"\x00" * 4), ["status 500"]),
        ("a cut answer", (18, "200 2 0.01 0.01", ok, b"\x00" * 2),
         ["curl exit 18", "CONTENT-LENGTH 4 but 2 bytes written"]),
        ("an empty 200", (0, "200 0 0.01 0.01", ok.replace(": 4", ": 0"), b""), ["empty answer"]),
        ("no CONTENT-LENGTH", (0, "200 4 0.01 0.01", "HTTP/1.1 200 OK\r\n\r\n", b"\x00" * 4), ["no CONTENT-LENGTH"]),
        ("curl's size differs", (0, "200 5 0.01 0.01", ok, b"\x00" * 4), ["curl's size 5 but 4 bytes written"]),
        ("above the bound", (0, f"200 {BOUND + 2} 0 0", ok.replace(": 4", f": {BOUND + 2}"), b"\x00" * (BOUND + 2)),
         [f"{BOUND + 2} bytes, above the {BOUND}-byte bound: kept, not used"]),
    ]
    right = 0
    for what, args, want in cases:
        got = verdict(*args)
        right += got == want
        print(f"{'RIGHT' if got == want else 'WRONG'} verdict, {what}: {got}")
    met, after = "BOOK STATE MET\nexit 0\n", "AFTER RECORDED: synthetic\nexit 0\n"
    both = {"m1-state.txt": met, "r-after.txt": after}
    sent = [(name, "SENT-OK") for name, _ in ORDER]
    order_cases = [
        ("M1 first", "M1", [], {}, True),
        ("M2 before M1", "M2", [], both, False),
        ("M2 after M1, state met and the restore recorded", "M2", sent[:1], both, True),
        ("M2 after M1, no state file", "M2", sent[:1], {"r-after.txt": after}, False),
        ("M2 after M1, state without exit 0", "M2", sent[:1], {"m1-state.txt": "BOOK STATE MET\nexit 1\n",
                                                                "r-after.txt": after}, False),
        ("M2 after M1, state a STOP", "M2", sent[:1], {"m1-state.txt": "STOP-BOOK-STATE: ALTMSTID '211'\nexit 1\n",
                                                        "r-after.txt": after}, False),
        ("M2 after M1, no restore record", "M2", sent[:1], {"m1-state.txt": met}, False),
        ("M2 after M1, the restore a STOP", "M2", sent[:1], {"m1-state.txt": met,
                                                              "r-after.txt": "STOP-RESTORE: absent\nexit 1\n"}, False),
        ("M1 again", "M1", sent[:1], {}, False),
        ("I after M2, its state met", "I", sent[:2], {"m2-state.txt": met}, True),
        ("I after M2, its state a STOP", "I", sent[:2], {"m2-state.txt": "STOP-RESTORED-STATE: x\nexit 1\n"}, False),
        ("I after M2, no state file", "I", sent[:2], {}, False),
        ("G before I", "G", sent[:2], {}, False),
        ("U last", "U", sent[:4], {}, True),
        ("anything after U", "M1", sent, {}, False),
        ("G after I failed", "G", sent[:2] + [("I", "REQUEST-FAILED")], {}, False),
        ("a label not in the row", "C1", [], {}, False),
    ]
    for what, label, entries, records, allowed in order_cases:
        why = admit(label, entries, records)
        good = (why is None) == allowed
        right += good
        print(f"{'RIGHT' if good else 'WRONG'} order, {what}: {'admitted' if why is None else 'refused: ' + why}")
    # The log reader, on lines written as send() writes them (a two-character and a one-character label), and on a
    # line that is not a send line.
    log = scratch / "send-log.txt"
    tail = (" sha256 x | curl exit 0 status 200 CONTENT-LENGTH 4 answer 4 B sha256 y | first byte 0.01 s total 0.01 s"
            " | VERDICT SENT-OK\n")
    log.write_text("2026-11-26T00:00:00.000Z M1 requests/marks.utf16le.xml 990 B" + tail +
                   "2026-11-26T00:00:01.000Z M2 requests/marks.utf16le.xml 990 B" + tail +
                   "2026-11-26T00:00:02.000Z I requests/items.utf16le.xml 1330 B" + tail, encoding="utf-8", newline="\n")
    good = log_entries(log) == [("M1", "SENT-OK"), ("M2", "SENT-OK"), ("I", "SENT-OK")]
    right += good
    print(f"{'RIGHT' if good else 'WRONG'} log reader, three SENT-OK lines, one with a one-letter label")
    good = admit("G", log_entries(log), {}) is None
    right += good
    print(f"{'RIGHT' if good else 'WRONG'} log reader then order: G admitted after M1 M2 I")
    with open(log, "a", encoding="utf-8", newline="\n") as f:
        f.write("a line typed by hand\n")
    try:
        log_entries(log)
        good = False
    except Refused:
        good = True
    right += good
    print(f"{'RIGHT' if good else 'WRONG'} log reader, a stray line refuses")
    total = len(cases) + len(order_cases) + 3
    print(f"{total} cases, {right} right")
    return 0 if right == total else 1


def main(argv):
    if len(argv) == 2 and argv[0] == "--selftest":
        return selftest(argv[1])
    if len(argv) != 1:
        print("usage: send.py LABEL | --selftest SCRATCH")
        return 2
    try:
        return send(argv[0])
    except Refused as error:
        print(f"REFUSED, nothing sent: {error}")
        return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
