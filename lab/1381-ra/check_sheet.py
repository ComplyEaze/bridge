#!/usr/bin/env python3
"""Check the sheet's file references, its send lines and their order, the five request files, the lines that hold the
sender to the row's reads, and the complete three-column manifest. Python 3.10, standard library only; reads only.

Row 1381-RA's check, in the shape of row 1381-BX's check_sheet.py (lab/1381-bx at 92f48553), with this row's own
checks: each of the ten send lines exactly once, in the row's order, inside Steps; the state line once, between M1's
and C1's send lines; the compute line once, after M2's; each request file one Export of a Collection with no
SVFROMDATE, SVTODATE or import (the Educational-mode date check, and no write); send.py's order, address and C1 gate
held exactly once each, and no client timeout in it; no GUID-shaped text in the two rule files. Only local/ is
skipped: any other file outside SHA256.txt is an error, and a __pycache__ folder is named in its own STOP-PYCACHE line
first; nothing is deleted. SHA256.txt may not list a path under local/.
With --fresh (step 0's pre-sitting run, right after the copy): local/ may hold only step 0's own records made before
it (FRESH); any other file there came with the copy or from an earlier sitting, and is STOP-LOCAL-PREFILLED.
Exit 0: whole. Exit 1: a finding. Exit 2: a failed read.
"""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".md", ".txt", ".xml")
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
SKIP_DIRS = {"local"}
FRESH = {"local/evidence/s0-made.txt", "local/evidence/s0-fetch-origin.txt", "local/evidence/s0-copy.txt",
         "local/evidence/s0-fresh.txt"}
# Names the sheet cites that are other rows' files or files the sitting writes, never this folder's.
FOREIGN = {"rbreq.py", "run.py", "gen_r1.py", "check_r1.py", "pa.py", "kept-local.txt", "notes.txt", "RESULTS.md",
           "PROVENANCE.md", "send-log.txt", "m1-state.txt", "measure-all.txt"}
LABELS = ("M1", "C1", "C2", "I1", "I2", "G1", "G2", "U1", "U2", "M2")
REQUESTS = ("requests/marks.utf16le.xml", "requests/count.utf16le.xml", "requests/items.utf16le.xml",
            "requests/groups.utf16le.xml", "requests/units.utf16le.xml")
SEND = ('cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-{a}.txt ]; then echo "STOP: local/evidence/send-{a}.txt '
        'exists"; else python -I -X utf8 send.py {a} > local/evidence/send-{a}.txt 2>&1; echo "exit $?" >> '
        'local/evidence/send-{a}.txt; cat local/evidence/send-{a}.txt; fi')
STATE = ('cd /c/bridge-lab/1381-ra && python -I -X utf8 measure.py state > local/evidence/m1-state.txt 2>&1; '
         'echo "exit $?" >> local/evidence/m1-state.txt; cat local/evidence/m1-state.txt')
COMPUTE = ('cd /c/bridge-lab/1381-ra && python -I -X utf8 measure.py all local/gen/fixtures/agent/stock-create-lab > '
           'local/evidence/measure-all.txt 2>&1; echo "exit $?" >> local/evidence/measure-all.txt; '
           'cat local/evidence/measure-all.txt')
HELD = (
    ("send.py", 'URL = "http://127.0.0.1:9001"\n'),
    ("send.py", 'ORDER = (("M1", "requests/marks.utf16le.xml"), ("C1", "requests/count.utf16le.xml"),\n'
                '         ("C2", "requests/count.utf16le.xml"), ("I1", "requests/items.utf16le.xml"),\n'
                '         ("I2", "requests/items.utf16le.xml"), ("G1", "requests/groups.utf16le.xml"),\n'
                '         ("G2", "requests/groups.utf16le.xml"), ("U1", "requests/units.utf16le.xml"),\n'
                '         ("U2", "requests/units.utf16le.xml"), ("M2", "requests/marks.utf16le.xml"))\n'),
    ("send.py", '        if "BOOK STATE MET" not in lines or "exit 0" not in lines:\n'),
    ("send.py", '        return f"{label} is in the send log already: a read is sent once in the row\'s life"\n'),
    ("measure.py", 'BOOK_MARK = "210"'),
)
GONE = (("send.py", "--max-time"), ("send.py", "-m "), ("send.py", "--connect-timeout"))
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
HEADER = re.compile(r"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>([A-Za-z]+)</TALLYREQUEST>"
                    r"<TYPE>([A-Za-z]+)</TYPE><ID>[^<]+</ID></HEADER><BODY><DESC>.*</DESC></BODY></ENVELOPE>", re.S)


def section(text, title):
    start = text.index(f"\n## {title}\n")
    end = text.find("\n## ", start + 1)
    return text[start:] if end < 0 else text[start:end]


def references():
    missing, checked = [], set()
    for doc in ("RUN-SHEET.md", "LAB-COMMON-RA.md"):
        # Inline code only, line by line, fenced blocks left out (a fence's backticks would pair with the next one).
        text = re.sub(r"(?ms)^\s*```.*?^\s*```[ \t]*$", "", (ROOT / doc).read_text(encoding="utf-8"))
        for token in (t for line in text.splitlines() for t in re.findall(r"`([^`]+)`", line)):
            if any(mark in token for mark in ("*", "<", ">", ":", "…")) or any(c.isspace() for c in token):
                continue
            name = token.replace("\\", "/")
            if name.startswith(("/", "../", "src-tauri/", "lab/", "local/", "scripts/", "docs/")) or name in FOREIGN:
                continue
            if not (name.endswith(SUFFIXES) or name in (".gitattributes", ".gitignore")):
                continue
            checked.add(name)
            if not (ROOT / name).is_file():
                missing.append(name)
    return sorted(checked), sorted(set(missing))


def sheet():
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    errors = []
    steps = section(text, "Steps")
    at = []
    for label in LABELS:
        line = SEND.format(a=label)
        if text.count(line) != 1 or steps.count(line) != 1:
            errors.append(f"{label}: the sheet does not hold its send line exactly once, in Steps")
        at.append(steps.find(line))
    if all(i >= 0 for i in at) and at != sorted(at):
        errors.append("the send lines are not in the row's order " + " ".join(LABELS))
    if text.count(STATE) != 1 or not (0 <= at[0] < steps.find(STATE) < at[1]):
        errors.append("the state line is not exactly once, between M1's and C1's send lines")
    if text.count(COMPUTE) != 1 or not (at[-1] >= 0 and steps.find(COMPUTE) > at[-1]):
        errors.append("the compute line is not exactly once, after M2's send line")
    for name, line in HELD:
        try:
            held = (ROOT / name).read_text(encoding="utf-8").count(line)
        except OSError as e:
            held = f"unreadable ({e})"
        if held != 1:
            errors.append(f"{name} does not hold exactly once {line.strip()[:60]!r} (found {held})")
    for name, line in GONE:
        if line in (ROOT / name).read_text(encoding="utf-8"):
            errors.append(f"{name} holds {line!r}: a client timeout")
    for doc in ("RUN-SHEET.md", "LAB-COMMON-RA.md"):
        for match in GUID_SHAPE.finditer((ROOT / doc).read_text(encoding="utf-8")):
            errors.append(f"GUID-shaped text in {doc}: {match.group(0)[:8]}...")
    return errors


def requests():
    errors = []
    for name in REQUESTS:
        body = (ROOT / name).read_bytes()
        if body[:2] != b"\xff\xfe":
            errors.append(f"{name}: no byte-order mark")
            continue
        try:
            text = body[2:].decode("utf-16-le")
        except UnicodeDecodeError:
            errors.append(f"{name}: not UTF-16LE")
            continue
        header = HEADER.fullmatch(text)
        if not header or header.groups() != ("Export", "Collection"):
            errors.append(f"{name}: not one Export of a Collection")
        for word in ("SVFROMDATE", "SVTODATE", "IMPORT", "TALLYMESSAGE", "ACTION="):
            if word in text.upper():
                errors.append(f"{name}: carries {word}")
    return errors


def pycache_hits(paths):
    hits = set()
    for name in paths:
        parts = pathlib.PurePosixPath(name).parts
        if "__pycache__" in parts:
            hits.add("/".join(parts[: parts.index("__pycache__") + 1]))
    return sorted(hits)


def prefilled(paths):
    return sorted(p for p in paths if p.startswith("local/") and p not in FRESH)


def manifest():
    entries, errors = {}, []
    for line in (ROOT / "SHA256.txt").read_text(encoding="utf-8").splitlines():
        match = ROW.fullmatch(line)
        if not match:
            errors.append(f"bad manifest row: {line!r}")
            continue
        digest, size, name = match.groups()
        parts = pathlib.PurePosixPath(name)
        if name in entries or parts.is_absolute() or ".." in parts.parts or parts.parts[0] in SKIP_DIRS:
            errors.append(f"duplicate or unsafe path: {name}")
            continue
        entries[name] = (digest, int(size))
    actual = {}
    for path in ROOT.rglob("*"):
        rel = path.relative_to(ROOT)
        if not path.is_file() or rel.as_posix() == "SHA256.txt" or rel.parts[0] in SKIP_DIRS:
            continue
        actual[rel.as_posix()] = path
    for name in sorted(set(entries) - set(actual)):
        errors.append(f"manifest names missing file: {name}")
    for name in sorted(set(actual) - set(entries)):
        errors.append(f"file absent from manifest: {name}")
    for name, path in actual.items():
        if name in entries:
            raw = path.read_bytes()
            if entries[name] != (hashlib.sha256(raw).hexdigest(), len(raw)):
                errors.append(f"hash or size differs: {name}")
    return sorted(actual), errors


def main(fresh=False):
    try:
        if fresh:
            early = prefilled([p.relative_to(ROOT).as_posix() for p in (ROOT / "local").rglob("*") if p.is_file()])
            if early:
                print(f"STOP-LOCAL-PREFILLED: local/ already holds {len(early)} file(s) step 0 did not make: "
                      + ", ".join(early[:10]) + (" ..." if len(early) > 10 else "") + "; nothing is deleted")
                return 1
            print(f"OK: local/ holds only step 0's own records ({len(FRESH)} names allowed)")
        found = [p.relative_to(ROOT).as_posix() for p in ROOT.rglob("*")
                 if p.relative_to(ROOT).parts[0] not in SKIP_DIRS]
        hits = pycache_hits(found)
        if hits:
            print(f"STOP-PYCACHE: {', '.join(hits)} in the row folder: bytecode a helper wrote there; nothing is deleted, "
                  "record it and stop")
        checked, missing = references()
        files, errors = manifest()
        errors.extend(sheet())
        errors.extend(requests())
    except (OSError, ValueError) as e:
        print(f"FAILED READ: {type(e).__name__}: {e}")
        return 2
    errors.extend(f"a rule file names a missing file: {name}" for name in missing)
    if hits and not errors:
        errors.append("a __pycache__ folder is present (STOP-PYCACHE above)")
    if errors:
        for error in errors[:100]:
            print("ERROR", error)
        if len(errors) > 100:
            print(f"... {len(errors) - 100} errors not printed")
        return 1
    print(f"OK: {len(checked)} file references exist; SHA256.txt matches all {len(files)} other files; only local/ skipped")
    print(f"OK: the ten send lines once each in Steps, in the order {' '.join(LABELS)}; the state line between M1 and "
          f"C1; the compute line after M2; send.py held to the row's reads ({len(HELD)} lines once each, no timeout)")
    print(f"OK: {len(REQUESTS)} requests, each one Export of a Collection, no SVFROMDATE or SVTODATE, no import; "
          "no GUID-shaped text in the rule files")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] not in ([], ["--fresh"]):
        print("usage: check_sheet.py [--fresh]")
        sys.exit(2)
    sys.exit(main(sys.argv[1:] == ["--fresh"]))
