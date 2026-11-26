#!/usr/bin/env python3
"""Check the sheet's file references, its command lines and their order, the four request files, the lines that hold
the sender to the row's reads and the restore helper to BK-S, and the complete three-column manifest. Python 3.10,
standard library only; reads only.

Row 1381-RR's check, from row 1381-RA's check_sheet.py (lab/1381-ra-sheet at 30f5c820, lab/1381-ra/check_sheet.py),
with this row's lines: each of the five send lines exactly once, inside Steps, in the order M1, the M1 state line, the
restore plan line, the restore after line, M2, the M2 state line, I, G, U, the compute line; each request file one
Export of a Collection with no SVFROMDATE, SVTODATE or import (no date window, no write); send.py's order, address and
gates held exactly once each, and no client timeout in it; restore.py held to BK-S's folder, sha256 and bytes; no
GUID-shaped text in the two rule files. Only local/ is skipped: any other file outside SHA256.txt is an error, and a
__pycache__ folder is named in its own STOP-PYCACHE line first; nothing is deleted. SHA256.txt may not list a path
under local/. With --fresh (step 0's pre-sitting run, right after the copy): local/ may hold only step 0's own records
made before it (FRESH); any other file there came with the copy or from an earlier sitting, and is
STOP-LOCAL-PREFILLED. Exit 0: whole. Exit 1: a finding. Exit 2: a failed read.
"""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
DOCS = ("RUN-SHEET.md", "LAB-COMMON-RR.md")
SUFFIXES = (".py", ".md", ".txt", ".xml")
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
SKIP_DIRS = {"local"}
FRESH = {"local/evidence/s0-made.txt", "local/evidence/s0-fetch-origin.txt", "local/evidence/s0-copy.txt",
         "local/evidence/s0-fresh.txt"}
# Names the sheet cites that are other rows' files or files the sitting writes, never this folder's.
FOREIGN = {"gen_ra.py", "c6prep.py", "LAB-COMMON-RA.md", "kept-local.txt", "notes.txt", "RESULTS.md", "send-log.txt",
           "m1-state.txt", "m2-state.txt", "r-plan.txt", "r-after.txt", "measure-all.txt", "restore-plan.txt", "r-before-listing.txt",
           "r-after-listing.txt", "a-M1.xml", "a-I1.xml", "a-G1.xml", "a-U1.xml"}
LABELS = ("M1", "M2", "I", "G", "U")
REQUESTS = ("requests/marks.utf16le.xml", "requests/items.utf16le.xml", "requests/groups.utf16le.xml",
            "requests/units.utf16le.xml")
SEND = ('cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-{a}.txt ]; then echo "STOP: local/evidence/send-{a}.txt '
        'exists"; else python -I -X utf8 send.py {a} > local/evidence/send-{a}.txt 2>&1; echo "exit $?" >> '
        'local/evidence/send-{a}.txt; cat local/evidence/send-{a}.txt; fi')
STATE = ('cd /c/bridge-lab/1381-rr && if [ -e local/evidence/{s}-state.txt ]; then echo "STOP: local/evidence/{s}-state.txt '
         'exists"; else python -I -X utf8 measure.py state {u} > local/evidence/{s}-state.txt 2>&1; echo "exit $?" >> '
         'local/evidence/{s}-state.txt; cat local/evidence/{s}-state.txt; fi')
RESTORE = ('cd /c/bridge-lab/1381-rr && if [ -e local/evidence/r-{c}.txt ]; then echo "STOP: local/evidence/r-{c}.txt '
           'exists"; else python -I -X utf8 restore.py {c} <DATA> > local/evidence/r-{c}.txt 2>&1; echo "exit $?" >> '
           'local/evidence/r-{c}.txt; cat local/evidence/r-{c}.txt; fi')
COMPUTE = ('cd /c/bridge-lab/1381-rr && python -I -X utf8 measure.py all local/gen/ra > local/evidence/measure-all.txt '
           '2>&1; echo "exit $?" >> local/evidence/measure-all.txt; cat local/evidence/measure-all.txt')
# The command lines of Steps, in the order they must stand.
LINES = ((SEND.format(a="M1"), "M1's send line"), (STATE.format(s="m1", u="M1"), "M1's state line"),
         (RESTORE.format(c="plan"), "the restore plan line"), (RESTORE.format(c="after"), "the restore after line"),
         (SEND.format(a="M2"), "M2's send line"), (STATE.format(s="m2", u="M2"), "M2's state line"),
         (SEND.format(a="I"), "I's send line"), (SEND.format(a="G"), "G's send line"),
         (SEND.format(a="U"), "U's send line"), (COMPUTE, "the compute line"))
HELD = (
    ("send.py", 'URL = "http://127.0.0.1:9001"\n'),
    ("send.py", 'ORDER = (("M1", "requests/marks.utf16le.xml"), ("M2", "requests/marks.utf16le.xml"),\n'
                '         ("I", "requests/items.utf16le.xml"), ("G", "requests/groups.utf16le.xml"),\n'
                '         ("U", "requests/units.utf16le.xml"))\n'),
    ("send.py", 'GATES = {"M2": (("m1-state.txt", "BOOK STATE MET"), ("r-after.txt", "AFTER RECORDED")),\n'
                '         "I": (("m2-state.txt", "BOOK STATE MET"),)}\n'),
    ("send.py", '        return f"{label} is in the send log already: a read is sent once in the row\'s life"\n'),
    ("measure.py", 'BOOK_MARK = "210"'),
    ("restore.py", 'SOURCE = "C:/bridge-lab/1108r3/backups/bks"\n'),
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
    for doc in DOCS:
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
    for line, what in LINES:
        if text.count(line) != 1 or steps.count(line) != 1:
            errors.append(f"{what}: the sheet does not hold it exactly once, in Steps")
        at.append(steps.find(line))
    if all(i >= 0 for i in at) and at != sorted(at):
        errors.append("the command lines of Steps are not in the order " + "; ".join(what for _, what in LINES))
    for name, line in HELD:
        try:
            held = (ROOT / name).read_text(encoding="utf-8").count(line)
        except OSError as e:
            held = f"unreadable ({e})"
        if held != 1:
            errors.append(f"{name} does not hold exactly once {line.strip()[:60]!r} (found {held})")
    restore = (ROOT / "restore.py").read_text(encoding="utf-8")
    listed = dict((row.split("  ")[2], row.split("  ")[0]) for row in
                  (ROOT / "SHA256.txt").read_text(encoding="utf-8").splitlines() if row.count("  ") == 2)
    sha = re.findall(r'(?m)^WANT_SHA = "([0-9a-f]{64})"$', restore)
    size = re.findall(r"(?m)^WANT_BYTES = ([0-9]+)$", restore)
    if len(sha) != 1 or len(size) != 1 or f"`{sha[0]}`" not in text or f"{int(size[0]):,} bytes" not in text:
        errors.append("restore.py's WANT_SHA and WANT_BYTES are not once each, or not the sheet's BK-S")
    for name, line in GONE:
        if line in (ROOT / name).read_text(encoding="utf-8"):
            errors.append(f"{name} holds {line!r}: a client timeout")
    for doc in DOCS:
        for match in GUID_SHAPE.finditer((ROOT / doc).read_text(encoding="utf-8")):
            errors.append(f"GUID-shaped text in {doc}: {match.group(0)[:8]}...")
    for name in REQUESTS:
        if f"`{listed.get(name, '-')}`" not in text:
            errors.append(f"{name}: its SHA256.txt sha256 is not in the sheet")
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
    except (OSError, ValueError, IndexError) as e:
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
    print(f"OK: the {len(LINES)} command lines once each in Steps, in order (M1, its state, the restore plan and after, "
          f"M2, its state, I, G, U, the compute); send.py and restore.py held ({len(HELD)} lines once each, BK-S's "
          "sha256 and bytes the sheet's, no timeout)")
    print(f"OK: {len(REQUESTS)} requests, each one Export of a Collection, no SVFROMDATE or SVTODATE, no import, each "
          "sha256 in the sheet; no GUID-shaped text in the rule files")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] not in ([], ["--fresh"]):
        print("usage: check_sheet.py [--fresh]")
        sys.exit(2)
    sys.exit(main(sys.argv[1:] == ["--fresh"]))
