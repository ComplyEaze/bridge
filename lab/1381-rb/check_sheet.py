#!/usr/bin/env python3
"""Check the sheet's file references, its command lines and their order, the four read files and the one write file,
the lines that hold the sender to the row's eleven sends, the grader to its marks and the restore helper to BK-S, and
the complete three-column manifest. Python 3.10, standard library only; reads only.

Row 1381-RB's check, from row 1381-RR's check_sheet.py (lab/1381-rb-sheet-draft at 62858add, lab/1381-rr/check_sheet.py),
with this row's lines: Steps holds exactly STEP_LINES (step 0's, the eleven sends, the state, bind, write, aim and compute
lines, the restore precheck, plan and after, and step 7's), each once and in order, and among them LINES in that order;
Self-checks holds exactly SELF_LINES; measure.py's nine pins of master's fixtures (name, sha256 and bytes) equal the
sheet's table of The pins; each read file one Export of a Collection with no SVFROMDATE, SVTODATE or import (no date
window, no write); the write file one Import Data of All Masters for the book, one TALLYMESSAGE holding one STOCKITEM with
ACTION="Create", whose NAME is measure.py's WRITE_ITEM, and no date window; send.py's WRITE_SHA and WRITE_BYTES the write
file's own, its line in the sheet's table and The write's; send.py's order, address, gates and once-only line held
exactly once each, and no client timeout in it; measure.py's two marks held; restore.py held to BK-S's and the
fallback's folder, sha256 and bytes and to the write's grade; no GUID-shaped text and no unfilled @@ token in the two
rule files. Only local/ is skipped: any other file outside SHA256.txt is an error, and a __pycache__ folder is named in
its own STOP-PYCACHE line first; nothing is deleted. SHA256.txt may not list a path under local/. With --fresh (step 0's
pre-sitting run, right after the copy): local/ may hold only step 0's own records made before it (FRESH) and one
s0-window screenshot (FRESH_SHOT); any other file there came with the copy or from an earlier sitting, and is
STOP-LOCAL-PREFILLED. Exit 0: whole. Exit 1: a finding. Exit 2: a failed read.
"""

import hashlib
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parent
DOCS = ("RUN-SHEET.md", "LAB-COMMON-RB.md")
SUFFIXES = (".py", ".md", ".txt", ".xml")
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
SKIP_DIRS = {"local"}
FRESH = {"local/evidence/s0-made.txt", "local/evidence/s0-fetch-origin.txt", "local/evidence/s0-copy.txt",
         "local/evidence/s0-fresh.txt", "local/evidence/notes.txt"}
# The step 0 screenshot, taken straight after the folder line: exactly one of these two names (the screen tool's format).
FRESH_SHOT = {"local/shots/s0-window.png", "local/shots/s0-window.jpg"}
# Names the sheet cites that are other rows' files, master's fixtures, or files the sitting writes, never this folder's.
FOREIGN = {"LAB-COMMON-RR.md", "LAB-COMMON-RA.md", "PROVENANCE.md", "kept-local.txt", "notes.txt", "RESULTS.md",
           "send-log.txt", "m1-state.txt", "m2-state.txt", "m3-state.txt", "bind.txt", "w-grade.txt", "aim.txt",
           "precheck.txt", "r-precheck.txt", "s7-listing.txt", "r-plan.txt", "r-after.txt", "measure-all.txt",
           "restore-plan.txt", "r-before-listing.txt", "r-after-listing.txt",
           "stock-create-lab-marks.utf16le.xml", "stock-create-lab-items-before.utf16le.xml",
           "stock-create-lab-groups.utf16le.xml", "stock-create-lab-units.utf16le.xml",
           "stock-create-lab-items-after-plain-create.utf16le.xml", "stock-create-lab-create-answer.utf16le.xml",
           "stock-create-lab-plain-create.request.utf16le.xml", "single-import-missing-ledger.utf16le.xml",
           "partial-import-missing-ledger.utf16le.xml", "builtin_unknown_report_refusal_live.utf16le.xml"}
READS = ("requests/marks.utf16le.xml", "requests/items.utf16le.xml", "requests/groups.utf16le.xml",
         "requests/units.utf16le.xml")
WRITE_REQUEST = "requests/create.utf16le.xml"
BOOK = "BRIDGE STOCK CREATE LAB"
CMD = 'cd /c/bridge-lab/1381-rb && if [ -e local/evidence/{f} ]; then echo "STOP: local/evidence/{f} exists"; else python -I -X utf8 {c} > local/evidence/{f} 2>&1; echo "exit $?" >> local/evidence/{f}; cat local/evidence/{f}; fi'


def send(a):
    return CMD.format(f=f"send-{a}.txt", c=f"send.py {a}")


def measure(f, c):
    return CMD.format(f=f, c=f"measure.py {c}")


def restore(c):
    return CMD.format(f=f"r-{c}.txt", c=f'restore.py {c} "<DATA>"')


COMPUTE = ('cd /c/bridge-lab/1381-rb && python -I -X utf8 measure.py all local/gen/fx > local/evidence/measure-all.txt '
           '2>&1; echo "exit $?" >> local/evidence/measure-all.txt; cat local/evidence/measure-all.txt')
# The command lines of Steps that carry the row, in the order they must stand.
LINES = ((send("M1"), "M1's send line"), (measure("m1-state.txt", "state M1"), "M1's state line"),
         (send("I1"), "I1's send line"), (send("G1"), "G1's send line"), (send("U1"), "U1's send line"),
         (measure("bind.txt", "bind"), "the bind line"), (restore("precheck"), "the restore precheck line"),
         (send("W"), "W's send line"), (measure("w-grade.txt", "write"), "the write's grade line"),
         (send("M2"), "M2's send line"), (measure("m2-state.txt", "state M2"), "M2's state line"),
         (send("I2"), "I2's send line"), (measure("aim.txt", "aim"), "the aim line"),
         (restore("plan"), "the restore plan line"), (restore("after"), "the restore after line"),
         (send("M3"), "M3's send line"), (measure("m3-state.txt", "state M3"), "M3's state line"),
         (send("I3"), "I3's send line"), (send("G3"), "G3's send line"), (send("U3"), "U3's send line"),
         (COMPUTE, "the compute line"))
STEP_LINES = (
    ('if [ -e /c/bridge-lab/1381-rb ]; then echo "STOP: /c/bridge-lab/1381-rb exists"; else mkdir /c/bri'
     'dge-lab/1381-rb /c/bridge-lab/1381-rb/local /c/bridge-lab/1381-rb/local/evidence /c/bridge-lab/138'
     '1-rb/local/gen /c/bridge-lab/1381-rb/local/shots && echo "made" > /c/bridge-lab/1381-rb/local/evid'
     'ence/s0-made.txt; echo "exit $?" >> /c/bridge-lab/1381-rb/local/evidence/s0-made.txt; cat /c/bridg'
     'e-lab/1381-rb/local/evidence/s0-made.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && git -C <CLONE> fetch origin > local/evidence/s0-fetch-origin.txt 2>&1;'
     ' echo "exit $?" >> local/evidence/s0-fetch-origin.txt; cat local/evidence/s0-fetch-origin.txt'),
    ('cd /c/bridge-lab/1381-rb && { git -C <CLONE> archive <SHEET> lab/1381-rb | tar -x -C /c/bridge-lab'
     '/1381-rb --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"; } > local/evidence/s'
     '0-copy.txt 2>&1; cat local/evidence/s0-copy.txt'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 check_sheet.py --fresh > local/evidence/s0-fresh.txt'
     ' 2>&1; echo "exit $?" >> local/evidence/s0-fresh.txt; cat local/evidence/s0-fresh.txt'),
    ('cd /c/bridge-lab/1381-rb && mkdir local/gen/fx && { n=0; for f in agent/stock-create-lab/stock-cre'
     'ate-lab-marks.utf16le.xml agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml agent/s'
     'tock-create-lab/stock-create-lab-groups.utf16le.xml agent/stock-create-lab/stock-create-lab-units.'
     'utf16le.xml agent/stock-create-lab/stock-create-lab-items-after-plain-create.utf16le.xml agent/sto'
     'ck-create-lab/stock-create-lab-create-answer.utf16le.xml agent/single-import-missing-ledger.utf16l'
     'e.xml agent/partial-import-missing-ledger.utf16le.xml builtin_unknown_report_refusal_live.utf16le.'
     'xml; do git -C <CLONE> cat-file blob "<SHEET>:src-tauri/crates/bridge-tally-protocol/tests/fixture'
     's/$f" > "local/gen/fx/${f##*/}" || n=$((n+1)); done; echo "failed $n"; } > local/evidence/s0-fx.tx'
     't 2>&1; cat local/evidence/s0-fx.txt'),
    ('cd /c/bridge-lab/1381-rb && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evid'
     'ence/s0-bridge.txt 2>&1; echo "exit $?" >> local/evidence/s0-bridge.txt; cat local/evidence/s0-bri'
     'dge.txt'),
    ('cd /c/bridge-lab/1381-rb && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s0-tally.'
     'txt 2>&1; echo "exit $?" >> local/s0-tally.txt; cat local/s0-tally.txt'),
    ('cd /c/bridge-lab/1381-rb && netstat -ano > local/s0-netstat.txt 2>&1; echo "exit $?" >> local/s0-n'
     "etstat.txt; grep -c LISTENING local/s0-netstat.txt; grep -E ':9001 .*LISTENING' local/s0-netstat.t"
     'xt'),
    ('cd /c/bridge-lab/1381-rb && curl.exe -sS -w "%{http_code} %{size_download}\\n" -D local/evidence/s0'
     '-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s0-status.txt > local/evidence/'
     's0-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s0-status-curl.txt; cat local/evidence/s'
     '0-status-curl.txt'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-M1.txt ]; then echo "STOP: local/evidence/'
     'send-M1.txt exists"; else python -I -X utf8 send.py M1 > local/evidence/send-M1.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-M1.txt; cat local/evidence/send-M1.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/m1-state.txt ]; then echo "STOP: local/evidence'
     '/m1-state.txt exists"; else python -I -X utf8 measure.py state M1 > local/evidence/m1-state.txt 2>'
     '&1; echo "exit $?" >> local/evidence/m1-state.txt; cat local/evidence/m1-state.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-I1.txt ]; then echo "STOP: local/evidence/'
     'send-I1.txt exists"; else python -I -X utf8 send.py I1 > local/evidence/send-I1.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-I1.txt; cat local/evidence/send-I1.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-G1.txt ]; then echo "STOP: local/evidence/'
     'send-G1.txt exists"; else python -I -X utf8 send.py G1 > local/evidence/send-G1.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-G1.txt; cat local/evidence/send-G1.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-U1.txt ]; then echo "STOP: local/evidence/'
     'send-U1.txt exists"; else python -I -X utf8 send.py U1 > local/evidence/send-U1.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-U1.txt; cat local/evidence/send-U1.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/bind.txt ]; then echo "STOP: local/evidence/bin'
     'd.txt exists"; else python -I -X utf8 measure.py bind > local/evidence/bind.txt 2>&1; echo "exit $'
     '?" >> local/evidence/bind.txt; cat local/evidence/bind.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/r-precheck.txt ]; then echo "STOP: local/eviden'
     'ce/r-precheck.txt exists"; else python -I -X utf8 restore.py precheck "<DATA>" > local/evidence/r-'
     'precheck.txt 2>&1; echo "exit $?" >> local/evidence/r-precheck.txt; cat local/evidence/r-precheck.'
     'txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-W.txt ]; then echo "STOP: local/evidence/s'
     'end-W.txt exists"; else python -I -X utf8 send.py W > local/evidence/send-W.txt 2>&1; echo "exit $'
     '?" >> local/evidence/send-W.txt; cat local/evidence/send-W.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/w-grade.txt ]; then echo "STOP: local/evidence/'
     'w-grade.txt exists"; else python -I -X utf8 measure.py write > local/evidence/w-grade.txt 2>&1; ec'
     'ho "exit $?" >> local/evidence/w-grade.txt; cat local/evidence/w-grade.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-M2.txt ]; then echo "STOP: local/evidence/'
     'send-M2.txt exists"; else python -I -X utf8 send.py M2 > local/evidence/send-M2.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-M2.txt; cat local/evidence/send-M2.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/m2-state.txt ]; then echo "STOP: local/evidence'
     '/m2-state.txt exists"; else python -I -X utf8 measure.py state M2 > local/evidence/m2-state.txt 2>'
     '&1; echo "exit $?" >> local/evidence/m2-state.txt; cat local/evidence/m2-state.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-I2.txt ]; then echo "STOP: local/evidence/'
     'send-I2.txt exists"; else python -I -X utf8 send.py I2 > local/evidence/send-I2.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-I2.txt; cat local/evidence/send-I2.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/aim.txt ]; then echo "STOP: local/evidence/aim.'
     'txt exists"; else python -I -X utf8 measure.py aim > local/evidence/aim.txt 2>&1; echo "exit $?" >'
     '> local/evidence/aim.txt; cat local/evidence/aim.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/r-plan.txt ]; then echo "STOP: local/evidence/r'
     '-plan.txt exists"; else python -I -X utf8 restore.py plan "<DATA>" > local/evidence/r-plan.txt 2>&'
     '1; echo "exit $?" >> local/evidence/r-plan.txt; cat local/evidence/r-plan.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/r-after.txt ]; then echo "STOP: local/evidence/'
     'r-after.txt exists"; else python -I -X utf8 restore.py after "<DATA>" > local/evidence/r-after.txt'
     ' 2>&1; echo "exit $?" >> local/evidence/r-after.txt; cat local/evidence/r-after.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-M3.txt ]; then echo "STOP: local/evidence/'
     'send-M3.txt exists"; else python -I -X utf8 send.py M3 > local/evidence/send-M3.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-M3.txt; cat local/evidence/send-M3.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/m3-state.txt ]; then echo "STOP: local/evidence'
     '/m3-state.txt exists"; else python -I -X utf8 measure.py state M3 > local/evidence/m3-state.txt 2>'
     '&1; echo "exit $?" >> local/evidence/m3-state.txt; cat local/evidence/m3-state.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-I3.txt ]; then echo "STOP: local/evidence/'
     'send-I3.txt exists"; else python -I -X utf8 send.py I3 > local/evidence/send-I3.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-I3.txt; cat local/evidence/send-I3.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-G3.txt ]; then echo "STOP: local/evidence/'
     'send-G3.txt exists"; else python -I -X utf8 send.py G3 > local/evidence/send-G3.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-G3.txt; cat local/evidence/send-G3.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && if [ -e local/evidence/send-U3.txt ]; then echo "STOP: local/evidence/'
     'send-U3.txt exists"; else python -I -X utf8 send.py U3 > local/evidence/send-U3.txt 2>&1; echo "ex'
     'it $?" >> local/evidence/send-U3.txt; cat local/evidence/send-U3.txt; fi'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 measure.py all local/gen/fx > local/evidence/measure'
     '-all.txt 2>&1; echo "exit $?" >> local/evidence/measure-all.txt; cat local/evidence/measure-all.tx'
     't'),
    ('cd /c/bridge-lab/1381-rb && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s7-tally.'
     'txt 2>&1; echo "exit $?" >> local/s7-tally.txt; cat local/s7-tally.txt'),
    ('cd /c/bridge-lab/1381-rb && netstat -ano > local/s7-netstat.txt 2>&1; echo "exit $?" >> local/s7-n'
     "etstat.txt; grep -c LISTENING local/s7-netstat.txt; grep -E ':9001 .*LISTENING' local/s7-netstat.t"
     'xt'),
    ('cd /c/bridge-lab/1381-rb && curl.exe -sS -w "%{http_code} %{size_download}\\n" -D local/evidence/s7'
     '-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s7-status.txt > local/evidence/'
     's7-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s7-status-curl.txt; cat local/evidence/s'
     '7-status-curl.txt'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 check_sheet.py > local/evidence/s7-sheet.txt 2>&1; e'
     'cho "exit $?" >> local/evidence/s7-sheet.txt; cat local/evidence/s7-sheet.txt'),
    ("cd /c/bridge-lab/1381-rb && find local -type f ! -name s7-listing.txt | sort | xargs -d '\\n' sha25"
     '6sum > local/evidence/s7-listing.txt 2>&1; echo "exit $?" >> local/evidence/s7-listing.txt; wc -l '
     'local/evidence/s7-listing.txt'),
)
SELF_LINES = (
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 send.py --selftest local/selftest/send > local/evide'
     'nce/selfcheck-send.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-send.txt; tail -n 2 local/'
     'evidence/selfcheck-send.txt'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 measure.py --selftest local/gen/fx local/selftest/me'
     'asure > local/evidence/selfcheck-measure.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-meas'
     'ure.txt; tail -n 2 local/evidence/selfcheck-measure.txt'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 restore.py --selftest local/selftest/restore > local'
     '/evidence/selfcheck-restore.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-restore.txt; tail'
     ' -n 2 local/evidence/selfcheck-restore.txt'),
    ('cd /c/bridge-lab/1381-rb && python -I -X utf8 check_sheet.py > local/evidence/selfcheck-sheet.txt '
     '2>&1; echo "exit $?" >> local/evidence/selfcheck-sheet.txt; cat local/evidence/selfcheck-sheet.txt'),
)
# measure.py's pins of master's fixtures (name, sha256, bytes), and the sheet's table of them (The pins).
PIN = re.compile(r'(?m)^    "[a-z-]+": \("([^"]+\.xml)", "([0-9a-f]{64})", ([0-9]+)\),$')
TABLE = re.compile(r"(?m)^\| [^|]+ \| `([^`]+\.xml)` \| ([0-9]+) \| `([0-9a-f]{64})` \| [0-9a-f]{8} \|$")
# The sheet's table of the requests (The requests): file, bytes, sha256.
REQ_TABLE = re.compile(r"(?m)^\| [^|]+ \| `(requests/[^`]+\.xml)` \| ([0-9]+) \| `([0-9a-f]{64})` \| [0-9a-f]{8} \|$")
HELD = (
    ("send.py", 'URL = "http://127.0.0.1:9001"\n'),
    ("send.py", 'ORDER = (("M1", "requests/marks.utf16le.xml"), ("I1", "requests/items.utf16le.xml"),\n'
                '         ("G1", "requests/groups.utf16le.xml"), ("U1", "requests/units.utf16le.xml"),\n'
                '         ("W", "requests/create.utf16le.xml"), ("M2", "requests/marks.utf16le.xml"),\n'
                '         ("I2", "requests/items.utf16le.xml"), ("M3", "requests/marks.utf16le.xml"),\n'
                '         ("I3", "requests/items.utf16le.xml"), ("G3", "requests/groups.utf16le.xml"),\n'
                '         ("U3", "requests/units.utf16le.xml"))\n'),
    ("send.py", 'WRITE = "W"\n'),
    ("send.py", 'GATES = {"I1": (("evidence/m1-state.txt", ("BOOK STATE MET",)),),\n'
                '         "W": (("evidence/bind.txt", ("BIND MET",)), ("evidence/r-precheck.txt", ("PRECHECK MET",)),\n'
                '               ("precheck.txt", None)),\n'
                '         "M2": (("evidence/w-grade.txt", ("WRITE ADMITTED", "WRITE REFUSED", "WRITE UNCLEAR")), '
                '("shots/s3-write", None)),\n'
                '         "I2": (("evidence/w-grade.txt", ("WRITE ADMITTED",)), ("evidence/m2-state.txt", '
                '("BOOK STATE MET",))),\n'
                '         "M3": (("evidence/w-grade.txt", ("WRITE ADMITTED",)), ("evidence/r-after.txt", '
                '("AFTER RECORDED",))),\n'
                '         "I3": (("evidence/m3-state.txt", ("BOOK STATE MET",)),)}\n'),
    ("send.py", '        return f"{label} was sent already: a request is sent once in the row\'s life"\n'),
    ("measure.py", 'BOOK_MARK = "210"'),
    ("measure.py", 'AIM_MARK = "211"'),
    ("restore.py", 'SOURCE = "C:/bridge-lab/1108r3/backups/bks"\n'),
    ("restore.py", 'FALLBACK = "C:/lab/1381-o/backup-1"\n'),
    ("restore.py", '    if any(line.strip().startswith("WRITE REFUSED: ") for line in '
                   'graded.read_text(encoding="utf-8").splitlines()):\n'),
)
GONE = (("send.py", "--max-time"), ("send.py", "-m "), ("send.py", "--connect-timeout"))
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
TOKEN = re.compile(r"@@[A-Za-z0-9_-]+@@")
HEADER = re.compile(r"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>([A-Za-z]+)</TALLYREQUEST>"
                    r"<TYPE>([A-Za-z]+)</TYPE><ID>[^<]+</ID></HEADER><BODY><DESC>.*</DESC></BODY></ENVELOPE>", re.S)


def section(text, title):
    start = text.index(f"\n## {title}\n")
    end = text.find("\n## ", start + 1)
    return text[start:] if end < 0 else text[start:end]


def commands(sec):
    """The command lines of a section's fenced blocks, in order (a line beginning # is an instruction)."""
    out, inside = [], False
    for line in sec.splitlines():
        if line.strip().startswith("```"):
            inside = not inside
        elif inside and not line.strip().startswith("#"):
            out.append(line.strip())
    return out


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


def held_value(text, pattern):
    found = re.findall(pattern, text)
    return found[0] if len(found) == 1 else None


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
    for i, line in enumerate(STEP_LINES, 1):
        if text.count(line) != 1 or steps.count(line) != 1:
            errors.append(f"Steps command line {i} ({line[:50]}...) is not held exactly once, in Steps")
    found = commands(steps)
    if found != list(STEP_LINES):
        errors.append(f"Steps holds {len(found)} command lines; they are not exactly STEP_LINES ({len(STEP_LINES)}), "
                      "in that order")
    selfs = section(text, "Self-checks before the sitting")
    for i, line in enumerate(SELF_LINES, 1):
        if text.count(line) != 1 or selfs.count(line) != 1:
            errors.append(f"self-check line {i} is not held exactly once, in Self-checks")
    if commands(selfs) != list(SELF_LINES):
        errors.append("Self-checks does not hold exactly SELF_LINES, in that order")
    measure = (ROOT / "measure.py").read_text(encoding="utf-8")
    pins = sorted(PIN.findall(measure))
    table = sorted((name, sha, size) for name, size, sha in TABLE.findall(section(text, "The pins")))
    if len(pins) != 9 or pins != table:
        errors.append("measure.py's nine pins of master's fixtures (name, sha256, bytes) are not the sheet's The pins")
    send = (ROOT / "send.py").read_text(encoding="utf-8")
    wsha = held_value(send, r'(?m)^WRITE_SHA = "([0-9a-f]{64})"$')
    wsize = held_value(send, r"(?m)^WRITE_BYTES = ([0-9]+)$")
    body = (ROOT / WRITE_REQUEST).read_bytes()
    rows = {name: (int(size), sha) for name, size, sha in REQ_TABLE.findall(section(text, "The requests"))}
    if wsha is None or wsize is None or (int(wsize), wsha) != (len(body), hashlib.sha256(body).hexdigest()):
        errors.append("send.py's WRITE_SHA and WRITE_BYTES are not once each, or not the write file's own")
    elif rows.get(WRITE_REQUEST) != (int(wsize), wsha) or (
            f"{int(wsize):,} bytes, sha256 `{wsha}`" not in section(text, "The write")):
        errors.append("send.py's WRITE_SHA and WRITE_BYTES are not the sheet's The requests row for W and The write's")
    for name, line in HELD:
        try:
            held = (ROOT / name).read_text(encoding="utf-8").count(line)
        except OSError as e:
            held = f"unreadable ({e})"
        if held != 1:
            errors.append(f"{name} does not hold exactly once {line.strip()[:60]!r} (found {held})")
    restore = (ROOT / "restore.py").read_text(encoding="utf-8")
    sha = re.findall(r'(?m)^WANT_SHA = "([0-9a-f]{64})"$', restore)
    size = re.findall(r"(?m)^WANT_BYTES = ([0-9]+)$", restore)
    if len(sha) != 1 or len(size) != 1 or f"{int(size[0]):,} bytes, sha256 `{sha[0]}`" not in section(text, "The backup"):
        errors.append("restore.py's WANT_SHA and WANT_BYTES are not once each, or not the bytes and sha256 of one line "
                      "of The backup")
    fsha = re.findall(r'(?m)^FALLBACK_SHA = "([0-9a-f]{64})"$', restore)
    fsize = re.findall(r"(?m)^FALLBACK_BYTES = ([0-9]+)$", restore)
    if len(fsha) != 1 or len(fsize) != 1 or fsha == sha or (
            f"backup-1\\TDBK1800_<company number>.001`, {int(fsize[0]):,} bytes, sha256 `{fsha[0]}`"
            not in section(text, "The backup")):
        errors.append("restore.py's FALLBACK_SHA and FALLBACK_BYTES are not once each, or not the sheet's backup-1 line")
    for name, line in GONE:
        if line in (ROOT / name).read_text(encoding="utf-8"):
            errors.append(f"{name} holds {line!r}: a client timeout")
    for doc in DOCS:
        doc_text = (ROOT / doc).read_text(encoding="utf-8")
        for match in GUID_SHAPE.finditer(doc_text):
            errors.append(f"GUID-shaped text in {doc}: {match.group(0)[:8]}...")
        for match in TOKEN.finditer(doc_text):
            errors.append(f"an unfilled token in {doc}: {match.group(0)}")
    listed = dict((row.split("  ")[2], row.split("  ")[0]) for row in
                  (ROOT / "SHA256.txt").read_text(encoding="utf-8").splitlines() if row.count("  ") == 2)
    for name in READS + (WRITE_REQUEST,):
        if rows.get(name, (None, None))[1] != listed.get(name, "-"):
            errors.append(f"{name}: its SHA256.txt sha256 is not its row in the sheet's The requests")
    return errors


def unbom(name):
    body = (ROOT / name).read_bytes()
    if body[:2] != b"\xff\xfe":
        raise ValueError(f"{name}: no byte-order mark")
    try:
        return body[2:].decode("utf-16-le")
    except UnicodeDecodeError:
        raise ValueError(f"{name}: not UTF-16LE") from None


def requests():
    errors = []
    for name in READS:
        try:
            text = unbom(name)
        except ValueError as e:
            errors.append(str(e))
            continue
        header = HEADER.fullmatch(text)
        if not header or header.groups() != ("Export", "Collection"):
            errors.append(f"{name}: not one Export of a Collection")
        for word in ("SVFROMDATE", "SVTODATE", "IMPORT", "TALLYMESSAGE", "ACTION="):
            if word in text.upper():
                errors.append(f"{name}: carries {word}")
    try:
        text = unbom(WRITE_REQUEST)
        root = ET.fromstring(text.split("?>", 1)[1] if text.startswith("<?xml") else text)
    except (ValueError, ET.ParseError) as e:
        return errors + [f"{WRITE_REQUEST}: does not parse ({e})"]
    name = held_value((ROOT / "measure.py").read_text(encoding="utf-8"), r'(?m)^WRITE_ITEM = "([^"]+)"')
    items = root.findall(".//STOCKITEM")
    if (root.tag != "ENVELOPE" or root.findtext("HEADER/TALLYREQUEST") != "Import Data"
            or len(root.findall("BODY/IMPORTDATA")) != 1
            or root.findtext("BODY/IMPORTDATA/REQUESTDESC/REPORTNAME") != "All Masters"
            or root.findtext("BODY/IMPORTDATA/REQUESTDESC/STATICVARIABLES/SVCURRENTCOMPANY") != BOOK):
        errors.append(f"{WRITE_REQUEST}: not one Import Data of All Masters for {BOOK}")
    messages = root.findall("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE")
    if len(messages) != 1 or len(list(messages[0])) != 1 or len(items) != 1 or list(messages[0])[0] is not items[0]:
        errors.append(f"{WRITE_REQUEST}: not one TALLYMESSAGE holding one STOCKITEM and nothing else")
    elif items[0].get("ACTION") != "Create" or name is None or items[0].get("NAME") != name or (
            items[0].findtext("NAME") != name):
        errors.append(f"{WRITE_REQUEST}: its STOCKITEM is not ACTION=\"Create\" with measure.py's WRITE_ITEM as its NAME")
    if [e for e in root.iter() if e.get("ACTION") not in (None, "Create")] or len(
            [e for e in root.iter() if e.get("ACTION")]) != 1:
        errors.append(f"{WRITE_REQUEST}: an ACTION other than the one Create")
    for word in ("SVFROMDATE", "SVTODATE"):
        if word in text.upper():
            errors.append(f"{WRITE_REQUEST}: carries {word}")
    return errors


def pycache_hits(paths):
    hits = set()
    for name in paths:
        parts = pathlib.PurePosixPath(name).parts
        if "__pycache__" in parts:
            hits.add("/".join(parts[: parts.index("__pycache__") + 1]))
    return sorted(hits)


def prefilled(paths):
    early = sorted(p for p in paths if p.startswith("local/") and p not in FRESH | FRESH_SHOT)
    shots = sorted(p for p in paths if p in FRESH_SHOT)
    return early + (shots if len(shots) > 1 else [])


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
            print(f"OK: local/ holds only step 0's own records ({len(FRESH)} names and one s0-window screenshot allowed)")
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
    print(f"OK: Steps holds exactly its {len(STEP_LINES)} command lines, once each, in order (among them the {len(LINES)} "
          f"that carry the row); the {len(SELF_LINES)} self-check lines; measure.py's nine pins the sheet's; send.py's "
          f"write pin the write file's and the sheet's; send.py, measure.py and restore.py held ({len(HELD)} lines once "
          "each, BK-S's and the fallback's sha256 and bytes the sheet's, no timeout)")
    print(f"OK: {len(READS)} reads, each one Export of a Collection, no SVFROMDATE or SVTODATE, no import; the one write "
          "one Create of WRITE_ITEM by Import Data; each sha256 in the sheet; no GUID-shaped text or unfilled token in the "
          "rule files")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] not in ([], ["--fresh"]):
        print("usage: check_sheet.py [--fresh]")
        sys.exit(2)
    sys.exit(main(sys.argv[1:] == ["--fresh"]))
