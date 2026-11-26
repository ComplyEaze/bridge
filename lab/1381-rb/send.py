#!/usr/bin/env python3
"""Send one of row 1381-RB's eleven requests to the lab's Tally, in the row's fixed order, and keep the answer exactly
as received.

Row 1381-RB, from row 1381-RR's send.py (lab/1381-rr-sheet at 62858add, lab/1381-rr/send.py) with these changes: the
eleven labels M1 I1 G1 U1 W M2 I2 M3 I3 G3 U3 in place of RR's five; the one write W, its file pinned here by sha256
and bytes (master's fixture, byte-identical) as well as by SHA256.txt, and parsed before it is sent: W's file must be
an import and every other label's an export (nothing else may import); RR's gates replaced by GATES below (the write
only after the bind and a met precheck; M2 only after the write's grade and the screenshot taken straight after it;
I2 and M3 only after a write graded ADMITTED); a record line `exit 0` counts only as the record's last line; and a
start marker, a-<LABEL>.started.txt, made before curl starts, so that a send whose answer never came still counts as
sent: that label is never sent again, and nothing is sent after it. Everything else is RR's: the label names its
request (no file argument); each label is sent once in the row's life, whatever happened to it; a failed send stops
every later one; the answer, its headers and the transport record are three files, so a failed request never reads
as an empty answer (AGENTS.md P5); everything goes under local/. Not product code. Python 3.10, standard library only.

  python -I -X utf8 send.py LABEL         LABEL is one of M1 I1 G1 U1 W M2 I2 M3 I3 G3 U3, sent in that order
  python -I -X utf8 send.py --selftest SCRATCH
      the verdict, order, gate and request rules on synthetic cases and on the five request files, and send()
      itself on a scratch row folder by a program that does not exist, in SCRATCH (under local/, or outside the
      row folder); sends nothing

Before a send: the request's sha256 and size must equal its line in SHA256.txt (and, for W, WRITE_SHA and
WRITE_BYTES); the request must parse as an export (or, for W, an import); the send log must hold exactly the labels
before LABEL, each SENT-OK, and not LABEL itself; no label may have a start marker without a log line; each record
GATES names for LABEL must hold one of its lines (the line, or the line then `: `) and end with the line `exit 0`, and
each file it names with no line must exist; no answer, header, transport or start file of LABEL may exist. Any miss:
nothing is sent, exit 2.

The send: LAB-COMMON v3 C3's curl line, to 127.0.0.1:9001 only, with no client timeout:
  curl.exe -sS -w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}" --data-binary @<request>
           -H "Content-Type: text/xml; charset=utf-16" -D <headers> http://127.0.0.1:9001 -o <answer>
Then local/answers/a-<LABEL>.transport.txt records curl's exit code, its -w line, the last HTTP status line and
CONTENT-LENGTH of the headers, the answer's bytes and sha256, curl's error output whole, and one verdict line:
`VERDICT SENT-OK`, or `VERDICT REQUEST-FAILED: <reasons>` when curl exited non-zero, the status is not 200,
CONTENT-LENGTH is missing or differs from the bytes written, curl's size differs, the answer is empty, or it is above
2,000,000 bytes (LAB-COMMON v3 C3's default bound; the file is kept and not used). One line goes to
local/evidence/send-log.txt. Exit 0: SENT-OK. Exit 1: REQUEST-FAILED (STOP-REQUEST-FAILED; never sent again).
A SENT-OK of W says only that its answer arrived whole: what it says is measure.py write's.
"""

import hashlib
import pathlib
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from datetime import datetime, timezone

ROOT = pathlib.Path(__file__).resolve().parent
URL = "http://127.0.0.1:9001"
BOUND = 2_000_000
ORDER = (("M1", "requests/marks.utf16le.xml"), ("I1", "requests/items.utf16le.xml"),
         ("G1", "requests/groups.utf16le.xml"), ("U1", "requests/units.utf16le.xml"),
         ("W", "requests/create.utf16le.xml"), ("M2", "requests/marks.utf16le.xml"),
         ("I2", "requests/items.utf16le.xml"), ("M3", "requests/marks.utf16le.xml"),
         ("I3", "requests/items.utf16le.xml"), ("G3", "requests/groups.utf16le.xml"),
         ("U3", "requests/units.utf16le.xml"))
# The one write: master's stock-create-lab-plain-create.request.utf16le.xml, byte-identical (git blob 090d427f).
WRITE = "W"
WRITE_SHA = "8f1e50a6f4a996adf951b6b30cee4f0b1366e9925f6bc3e6f016ecb2d8b28f93"
WRITE_BYTES = 3646
# Each gated label: the records under local/ it needs, and the lines one of which each must hold (None: must exist).
GATES = {"I1": (("evidence/m1-state.txt", ("BOOK STATE MET",)),),
         "W": (("evidence/bind.txt", ("BIND MET",)), ("evidence/r-precheck.txt", ("PRECHECK MET",)),
               ("precheck.txt", None)),
         "M2": (("evidence/w-grade.txt", ("WRITE ADMITTED", "WRITE REFUSED", "WRITE UNCLEAR")), ("shots/s3-write", None)),
         "I2": (("evidence/w-grade.txt", ("WRITE ADMITTED",)), ("evidence/m2-state.txt", ("BOOK STATE MET",))),
         "M3": (("evidence/w-grade.txt", ("WRITE ADMITTED",)), ("evidence/r-after.txt", ("AFTER RECORDED",))),
         "I3": (("evidence/m3-state.txt", ("BOOK STATE MET",)),)}
SHOT_TYPES = (".png", ".jpg")  # a screenshot record is shots/<name> with one of these (the screen tool's format)
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


def kind(body):
    """'import', 'export', or why the request is neither, from the parsed request (never a text search)."""
    try:
        root = ET.fromstring((body[2:] if body[:2] == b"\xff\xfe" else body).decode("utf-16-le"))
    except (UnicodeDecodeError, ET.ParseError) as error:
        return f"does not parse ({type(error).__name__})"
    request = (root.findtext("./HEADER/TALLYREQUEST") or "").strip().lower()
    writes = [el.tag for el in root.iter() if el.tag in ("IMPORTDATA", "TALLYMESSAGE") or "ACTION" in el.attrib]
    if root.tag == "ENVELOPE" and request in ("import", "import data") and writes:
        return "import"
    if root.tag == "ENVELOPE" and request in ("export", "export data") and not writes:
        return "export"
    return f"neither an import nor an export (TALLYREQUEST {request!r}, {len(writes)} write elements)"


def check_request(label, body, row):
    """Why LABEL's request may not be sent (None when it may): ROW is its SHA256.txt (sha256, bytes) or None."""
    digest = hashlib.sha256(body).hexdigest()
    if row != (digest, len(body)):
        return "the request differs from its SHA256.txt line, or has none"
    if label == WRITE and (digest, len(body)) != (WRITE_SHA, WRITE_BYTES):
        return "the write's file is not the pinned one (WRITE_SHA, WRITE_BYTES)"
    found = kind(body)
    if found != ("import" if label == WRITE else "export"):
        return f"the request is {found!r}: only W imports, and every other label exports"
    return None


def admit(label, entries, records, started=()):
    """Why LABEL may not be sent now, or None. entries: the log so far; records: {path under local/: its text, '' for
    a file that exists, or None}; started: the labels with a start marker."""
    labels = [name for name, _ in ORDER]
    if label not in labels:
        return f"{label!r} is not one of {' '.join(labels)}"
    logged = [name for name, _ in entries]
    if label in logged or label in started:
        return f"{label} was sent already: a request is sent once in the row's life"
    hung = [name for name in labels if name in started and name not in logged]
    if hung:
        return f"{hung[0]} was started and has no log line (its answer never came): nothing more is sent (STOP-NO-ANSWER)"
    failed = [name for name, verdict in entries if verdict != "SENT-OK"]
    if failed:
        return f"{failed[0]} failed: nothing more is sent (STOP-REQUEST-FAILED)"
    want = labels[: labels.index(label)]
    if logged != want:
        return f"the log holds {logged}, not {want}: out of order"
    for record, lines in GATES.get(label, ()):
        text = records.get(record)
        if text is None:
            return f"local/{record} is missing"
        if lines is None:
            continue
        held = [x.strip() for x in text.splitlines() if x.strip()]
        if not any(x == line or x.startswith(line + ": ") for x in held for line in lines) or held[-1] != "exit 0":
            return f"local/{record} does not hold one of {list(lines)} (or the line then ': ...') and end `exit 0`"
    return None


def records_for(label, local):
    """The records GATES names for LABEL, read from LOCAL: {path: text, '' when it only must exist, None when absent}."""
    records = {}
    for record, lines in GATES.get(label, ()):
        if record.startswith("shots/"):
            records[record] = "" if any((local / (record + t)).is_file() for t in SHOT_TYPES) else None
            continue
        path = local / record
        records[record] = None if not path.is_file() else ("" if lines is None else path.read_text(encoding="utf-8"))
    return records


def send(label, root=ROOT, curl="curl.exe"):
    """Send LABEL's request from the row folder by curl (the self-test passes a scratch folder and a program that
    does not exist, so that it sends nothing)."""
    local = root / "local"
    answers, evidence = local / "answers", local / "evidence"
    request = dict(ORDER).get(label)
    try:  # Everything before curl starts: a failure here sends nothing.
        started = {p.name[2:-len(".started.txt")] for p in answers.glob("a-*.started.txt")} if answers.is_dir() else set()
        reason = admit(label, log_entries(evidence / "send-log.txt"), records_for(label, local), started)
        if reason:
            raise Refused(reason)
        body = (root / request).read_bytes()
        reason = check_request(label, body, listed(root).get(request))
        if reason:
            raise Refused(f"{request}: {reason}")
        digest = hashlib.sha256(body).hexdigest()
        answer = answers / f"a-{label}.xml"
        headers = answers / f"a-{label}.headers.txt"
        record = answers / f"a-{label}.transport.txt"
        for path in (answer, headers, record):
            if path.exists():
                raise Refused(f"{path.relative_to(root).as_posix()} exists: never overwritten")
        answers.mkdir(parents=True, exist_ok=True)
        evidence.mkdir(parents=True, exist_ok=True)
        start = utc()
        with open(answers / f"a-{label}.started.txt", "x", encoding="utf-8", newline="\n") as marker:
            marker.write(f"label {label}\nrequest {request} {len(body)} B sha256 {digest}\nstart {start}\n")
    except (OSError, UnicodeDecodeError) as error:
        raise Refused(f"{type(error).__name__}: {error}")
    try:  # curl.exe that cannot start has sent nothing; the start marker says it was tried.
        done = subprocess.run([curl, "-sS", "-w", "%{http_code} %{size_download} %{time_starttransfer} %{time_total}",
                               "--data-binary", f"@{root / request}", "-H", "Content-Type: text/xml; charset=utf-16",
                               "-D", str(headers), URL, "-o", str(answer)], capture_output=True, text=True)
        code, written, said_err = done.returncode, done.stdout.strip(), done.stderr
    except OSError as error:
        code, written, said_err = -1, "", f"{curl} did not start ({type(error).__name__}: {error})"
    end = utc()
    got = answer.read_bytes() if answer.exists() else b""
    head = headers.read_text(encoding="latin-1") if headers.exists() else ""
    reasons = verdict(code, written, head, got)
    statuses = re.findall(r"(?m)^HTTP/\S+ .*$", head)
    lengths = re.findall(r"(?im)^content-length:\s*(\d+)\s*$", head)
    said = "SENT-OK" if not reasons else "REQUEST-FAILED: " + "; ".join(reasons)
    record.write_text("\n".join([
        f"label {label}", f"request {request} {len(body)} B sha256 {digest}", f"start {start}", f"end {end}",
        f"curl exit {code}", f"curl -w line: {written!r}",
        f"last status line: {statuses[-1].strip() if statuses else 'none'}",
        f"CONTENT-LENGTH: {lengths[-1] if lengths else 'none'}",
        f"answer file: {'present' if answer.exists() else 'absent'}, {len(got)} B sha256 {hashlib.sha256(got).hexdigest()}",
        f"curl error output ({len(said_err)} characters): {said_err!r}", f"VERDICT {said}"]) + "\n",
        encoding="utf-8", newline="\n")
    fields = (written.split() + ["", "", "", ""])[:4]
    row = (f"{start} {label} {request} {len(body)} B sha256 {digest} | curl exit {code} status {fields[0]} "
           f"CONTENT-LENGTH {lengths[-1] if lengths else 'none'} answer {len(got)} B sha256 "
           f"{hashlib.sha256(got).hexdigest()} | first byte {fields[2]} s total {fields[3]} s | VERDICT {said}")
    with open(evidence / "send-log.txt", "a", encoding="utf-8", newline="\n") as log:
        log.write(row + "\n")
    print(row)
    return 0 if not reasons else 1


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


def selftest(scratch):
    scratch = pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    scratch.mkdir(parents=True, exist_ok=True)
    right = total = 0

    def case(what, good):
        nonlocal right, total
        total += 1
        right += good
        print(f"{'RIGHT' if good else 'WRONG'} {what}")

    ok = "HTTP/1.1 200 OK\r\nCONTENT-LENGTH: 4\r\n\r\n"
    for what, args, want in (
        ("a whole answer", (0, "200 4 0.01 0.01", ok, b"\x00" * 4), []),
        ("connection refused", (7, "000 0 0 0", "", b""), ["curl exit 7", "status 000", "no CONTENT-LENGTH", "empty answer"]),
        ("status 500", (0, "500 4 0.01 0.01", ok.replace("200 OK", "500 Error"), b"\x00" * 4), ["status 500"]),
        ("a cut answer", (18, "200 2 0.01 0.01", ok, b"\x00" * 2), ["curl exit 18", "CONTENT-LENGTH 4 but 2 bytes written"]),
        ("an empty 200", (0, "200 0 0.01 0.01", ok.replace(": 4", ": 0"), b""), ["empty answer"]),
        ("no CONTENT-LENGTH", (0, "200 4 0.01 0.01", "HTTP/1.1 200 OK\r\n\r\n", b"\x00" * 4), ["no CONTENT-LENGTH"]),
        ("curl's size differs", (0, "200 5 0.01 0.01", ok, b"\x00" * 4), ["curl's size 5 but 4 bytes written"]),
        ("above the bound", (0, f"200 {BOUND + 2} 0 0", ok.replace(": 4", f": {BOUND + 2}"), b"\x00" * (BOUND + 2)),
         [f"{BOUND + 2} bytes, above the {BOUND}-byte bound: kept, not used"]),
    ):
        got = verdict(*args)
        case(f"verdict, {what}: {got}", got == want)
    met = "BOOK STATE MET: synthetic\nexit 0\n"
    bind = {"evidence/bind.txt": "BIND MET: synthetic\nexit 0\n", "evidence/r-precheck.txt": "PRECHECK MET: synthetic\n"
            "exit 0\n", "precheck.txt": ""}
    graded = lambda word: {"evidence/w-grade.txt": f"W: 538 B\nWRITE {word}: synthetic\nexit 0\n", "shots/s3-write": ""}
    admitted = "WRITE ADMITTED: synthetic\nexit 0\n"
    after = {"evidence/w-grade.txt": admitted, "evidence/r-after.txt": "AFTER RECORDED: synthetic\nexit 0\n"}
    sent = [(name, "SENT-OK") for name, _ in ORDER]
    upto = lambda label: sent[: [n for n, _ in ORDER].index(label)]
    for what, label, entries, records, allowed in (
        ("M1 first", "M1", [], {}, True),
        ("I1 after M1, its state met", "I1", upto("I1"), {"evidence/m1-state.txt": met}, True),
        ("I1 after M1, its state a STOP", "I1", upto("I1"), {"evidence/m1-state.txt": "STOP-BOOK-STATE: x\nexit 1\n"}, False),
        ("I1 after M1, the state line not last before exit 0", "I1", upto("I1"),
         {"evidence/m1-state.txt": "BOOK STATE MET\nexit 0\nexit 1\n"}, False),
        ("G1 before I1", "G1", upto("I1"), {}, False),
        ("W after U1, the bind and the precheck met", "W", upto("W"), bind, True),
        ("W, the bind a STOP", "W", upto("W"), {**bind, "evidence/bind.txt": "STOP-BOOK-STATE: x\nexit 1\n"}, False),
        ("W, no bind record", "W", upto("W"), {k: v for k, v in bind.items() if k != "evidence/bind.txt"}, False),
        ("W, the precheck failed", "W", upto("W"), {**bind, "evidence/r-precheck.txt": "STOP-RESTORE-SOURCE: x\nexit 1\n"},
         False),
        ("W, no precheck record", "W", upto("W"), {k: v for k, v in bind.items() if k != "precheck.txt"}, False),
        ("W before U1", "W", upto("U1"), bind, False),
        ("M2 after W graded ADMITTED, the shot taken", "M2", upto("M2"), graded("ADMITTED"), True),
        ("M2 after W graded REFUSED", "M2", upto("M2"), graded("REFUSED"), True),
        ("M2 after W graded UNCLEAR (v3 C8's one state read)", "M2", upto("M2"), graded("UNCLEAR"), True),
        ("M2, the write's answer NOT READ", "M2", upto("M2"),
         {**graded("ADMITTED"), "evidence/w-grade.txt": "NOT READ: W: x\nexit 2\n"}, False),
        ("M2, no screenshot after the write", "M2", upto("M2"), {**graded("ADMITTED"), "shots/s3-write": None}, False),
        ("I2 after M2, the write admitted and M2 met", "I2", upto("I2"),
         {"evidence/w-grade.txt": admitted, "evidence/m2-state.txt": met}, True),
        ("I2, the write refused", "I2", upto("I2"),
         {"evidence/w-grade.txt": "WRITE REFUSED: x\nexit 0\n", "evidence/m2-state.txt": met}, False),
        ("I2, M2 a STOP", "I2", upto("I2"),
         {"evidence/w-grade.txt": admitted, "evidence/m2-state.txt": "STOP-AIM-STATE: x\nexit 1\n"}, False),
        ("M3 after I2, the restore recorded", "M3", upto("M3"), after, True),
        ("M3, no restore record", "M3", upto("M3"), {"evidence/w-grade.txt": admitted}, False),
        ("M3, the write unclear", "M3", upto("M3"), {**after, "evidence/w-grade.txt": "WRITE UNCLEAR: x\nexit 0\n"}, False),
        ("I3 after M3, its state met", "I3", upto("I3"), {"evidence/m3-state.txt": met}, True),
        ("I3 after M3, its state a STOP", "I3", upto("I3"), {"evidence/m3-state.txt": "STOP-RESTORED-STATE: x\nexit 1\n"},
         False),
        ("U3 last", "U3", upto("U3"), {}, True),
        ("anything after U3", "M1", sent, {}, False),
        ("W again", "W", upto("M2"), bind, False),
        ("G3 after I3 failed", "G3", upto("I3") + [("I3", "REQUEST-FAILED")], {}, False),
        ("a label not in the row", "C1", [], {}, False),
    ):
        why = admit(label, entries, records)
        case(f"order, {what}: {'admitted' if why is None else 'refused: ' + why}", (why is None) == allowed)
    # The start marker: a send whose answer never came counts as sent, and stops every later one.
    why = admit("W", upto("W"), bind, started={"W"})
    case(f"start marker, W started but not logged, W again: {why}", why is not None and "sent already" in why)
    why = admit("M2", upto("M2"), graded("ADMITTED"), started={"W"})
    case(f"start marker, W logged, M2 admitted: {why}", why is None)
    why = admit("M2", upto("W"), graded("ADMITTED"), started={"W"})
    case(f"start marker, W started but not logged, M2: {why}", why is not None and "STOP-NO-ANSWER" in why)
    # The request rules, on the row's five request files as they are.
    rows = listed(ROOT)
    for label, request in ORDER[:5]:
        body = (ROOT / request).read_bytes()
        why = check_request(label, body, rows.get(request))
        case(f"request, {label}'s file {request}: {why or 'admitted'}", why is None)
    write = (ROOT / "requests/create.utf16le.xml").read_bytes()
    marks = (ROOT / "requests/marks.utf16le.xml").read_bytes()
    line = lambda body: (hashlib.sha256(body).hexdigest(), len(body))
    why = check_request("M2", write, line(write))
    case(f"request, the write's file under a read's label: {why}", why is not None and "only W imports" in why)
    why = check_request("W", marks, line(marks))
    case(f"request, a read's file under W: {why}", why is not None and "not the pinned one" in why)
    edited = write.replace("Import Data".encode("utf-16-le"), "Export Data".encode("utf-16-le"), 1)
    why = kind(edited)
    case(f"request, the write's file with TALLYREQUEST Export: still not an export ({why})", why.startswith("neither"))
    why = check_request("W", write, ("0" * 64, len(write)))
    case(f"request, W's SHA256.txt line not its file: {why}", why is not None and "SHA256.txt" in why)
    case(f"request, a cut file: {kind(write[:-20])}", kind(write[:-20]).startswith("does not parse"))
    # The record readers, on a scratch local/ folder.
    local = scratch / "local"
    (local / "evidence").mkdir(parents=True, exist_ok=True)
    (local / "shots").mkdir(parents=True, exist_ok=True)
    for name in ("shots/s3-write.png", "shots/s3-write.jpg"):
        if (local / name).exists():
            (local / name).unlink()
    (local / "evidence/w-grade.txt").write_text("WRITE ADMITTED: x\nexit 0\n", encoding="utf-8")
    case("records, no screenshot after the write: absent", records_for("M2", local)["shots/s3-write"] is None)
    (local / "shots/s3-write.jpg").write_bytes(b"\xff\xd8")
    got = records_for("M2", local)
    case("records, the screenshot as .jpg: present; the grade read", (got["shots/s3-write"], got["evidence/w-grade.txt"])
         == ("", "WRITE ADMITTED: x\nexit 0\n"))
    # send() itself, on a scratch row folder, by a program that does not exist: nothing can be sent, and a curl that
    # cannot start is recorded as a failed request (a start marker, a transport record and a REQUEST-FAILED log line).
    row = next(scratch / f"row-{n}" for n in range(1, 1000) if not (scratch / f"row-{n}").exists())  # never reused
    (row / "requests").mkdir(parents=True, exist_ok=True)
    (row / "requests/marks.utf16le.xml").write_bytes(marks)
    (row / "SHA256.txt").write_text(f"{line(marks)[0]}  {len(marks)}  requests/marks.utf16le.xml\n", encoding="utf-8")
    try:
        code = send("M1", row, curl=str(scratch / "no-such-program-for-the-self-test.exe"))
    except (OSError, Refused) as error:
        code = f"raised {type(error).__name__}"
    shown = (row / "local/answers/a-M1.transport.txt")
    shown = shown.read_text(encoding="utf-8").splitlines() if shown.exists() else []
    case(f"send, a curl that cannot start: exit 1, curl exit -1, REQUEST-FAILED ({code})",
         (code, "curl exit -1" in shown, (shown[-1:] or [""])[0].startswith("VERDICT REQUEST-FAILED"),
          (row / "local/answers/a-M1.started.txt").is_file(),
          log_entries(row / "local/evidence/send-log.txt")) == (1, True, True, True, [("M1", "REQUEST-FAILED")]))
    # The log reader, on lines written as send() writes them, and on a line that is not a send line.
    log = scratch / "send-log.txt"
    tail = (" sha256 x | curl exit 0 status 200 CONTENT-LENGTH 4 answer 4 B sha256 y | first byte 0.01 s total 0.01 s"
            " | VERDICT SENT-OK\n")
    log.write_text("2026-11-26T00:00:00.000Z M1 requests/marks.utf16le.xml 990 B" + tail +
                   "2026-11-26T00:00:01.000Z I1 requests/items.utf16le.xml 1330 B" + tail +
                   "2026-11-26T00:00:02.000Z W requests/create.utf16le.xml 3646 B" + tail, encoding="utf-8", newline="\n")
    case("log reader, three SENT-OK lines, one with a one-letter label",
         log_entries(log) == [("M1", "SENT-OK"), ("I1", "SENT-OK"), ("W", "SENT-OK")])
    with open(log, "a", encoding="utf-8", newline="\n") as f:
        f.write("a line typed by hand\n")
    try:
        log_entries(log)
        good = False
    except Refused:
        good = True
    case("log reader, a stray line refuses", good)
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
