#!/usr/bin/env python3 -I
"""Check this sheet's requests before anything is sent (#2145, item 2145-R).

Run (in this folder): python -I check_requests.py requests

First it checks every line of SHA256.txt (sha256, bytes, path) against the file it names, and that `requests/` holds
exactly the files REQUESTS names, no more and no fewer. Then it checks by bytes that every request decodes as UTF-16LE
with a byte-order mark and parses as XML, and that each census request:
- names its company once in SVCURRENTCOMPANY and its from and to dates in SVFROMDATE and SVTODATE;
- carries its span ` AND $AlterID &gt; 0 AND $AlterID &lt;= M` exactly once, with M the number its file name ends in;
- for the windows read at more than one mark, is the same request as the others of its window with M changed and
  nothing else.
It prints `files: ok N, FAIL F` and `requests: ok N, FAIL F`, and exits 0 only when both FAIL counts are 0. It prints no
company figure beyond the names, dates and marks already in this sheet.
"""
import hashlib
import os
import sys
import xml.etree.ElementTree as ET

AMEND, PILOT = "BRIDGE AMEND LAB", "BRIDGE PILOT LAB"
# file key, company, from, to, M (census requests only)
CENSUS = [
    ("a-0401_census_spanned_0_1829", AMEND, "20260401", "20260401", 1829),
    ("a-0403_census_spanned_0_1829", AMEND, "20260403", "20260403", 1829),
    ("b-0401_census_spanned_0_1469", AMEND, "20260401", "20260401", 1469),
    ("b-0401_census_spanned_0_1685", AMEND, "20260401", "20260401", 1685),
    ("b-0403_census_spanned_0_1676", AMEND, "20260403", "20260403", 1676),
    ("p-0804-0805_census_spanned_0_68", PILOT, "20260804", "20260805", 68),
]
OTHER = [
    "company-list", "a-marks", "a-0401_verification", "a-0403_verification", "p-marks", "p-book-extent",
    "p-currencies", "p-ledgers-compliance", "p-ledgers-paired", "p-groups", "p-0804-0805_window",
]
REQUESTS = [key for key, *_ in CENSUS] + OTHER


def check_files(folder):
    ok = fail = 0
    for line in open("SHA256.txt", encoding="utf-8"):
        parts = line.split()
        if len(parts) != 3:
            continue
        digest, size, path = parts
        try:
            data = open(path, "rb").read()
        except OSError:
            data = None
        if data is None or hashlib.sha256(data).hexdigest() != digest or str(len(data)) != size:
            fail += 1
            print("FAIL file %s" % path)
        else:
            ok += 1
    present = set(os.listdir(folder))
    wanted = {"%s.request.utf16le.xml" % key for key in REQUESTS}
    for name in sorted(present ^ wanted):
        fail += 1
        print("FAIL %s: %s" % ("unexpected file" if name in present else "missing file", name))
    print("files: ok %d, FAIL %d" % (ok, fail))
    return fail


def text(folder, key):
    data = open(os.path.join(folder, "%s.request.utf16le.xml" % key), "rb").read()
    if data[:2] != b"\xff\xfe":
        raise ValueError("no UTF-16LE byte-order mark")
    body = data[2:].decode("utf-16-le")
    ET.fromstring(body)
    return body


def clause(mark):
    return " AND $AlterID &gt; 0 AND $AlterID &lt;= %d" % mark


def check_census(folder, key, company, frm, to, mark, bodies):
    body = text(folder, key)
    if body.count("<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % company) != 1:
        return "company"
    if body.count('<SVFROMDATE TYPE="Date">%s</SVFROMDATE>' % frm) != 1 or body.count('<SVTODATE TYPE="Date">%s</SVTODATE>' % to) != 1:
        return "dates"
    if body.count("$AlterID") != 2 or body.count(clause(mark)) != 1:
        return "span"
    unspanned = body.replace(clause(mark), "")
    window = (company, frm, to)
    if window in bodies and bodies[window] != unspanned:
        return "not the same request as the other marks of its window"
    bodies[window] = unspanned
    return None


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        return 2
    folder = sys.argv[1]
    file_fail = check_files(folder)
    ok = fail = 0
    bodies = {}
    for key in REQUESTS:
        try:
            census = [entry for entry in CENSUS if entry[0] == key]
            problem = check_census(folder, *census[0], bodies) if census else (text(folder, key) and None)
        except (OSError, ValueError, ET.ParseError) as error:
            problem = "unreadable (%s)" % type(error).__name__
        if problem:
            fail += 1
            print("FAIL %s: %s" % (key, problem))
        else:
            ok += 1
    print("requests: ok %d, FAIL %d" % (ok, fail))
    return 0 if fail == 0 and file_fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
