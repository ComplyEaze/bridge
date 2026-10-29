#!/usr/bin/env python3 -I
"""Check this sheet's census requests before anything is sent (#2145).

Run: python -I check_requests.py requests

For every window in WINDOWS below it checks, by bytes, that:
- the unspanned request (the control) and the spanned one decode as UTF-16LE with a byte-order mark and parse as XML;
- each names the window's company once in SVCURRENTCOMPANY and its from and to dates in SVFROMDATE, SVTODATE and the
  two $$Date literals;
- the spanned request is the control with ` AND $AlterID &gt; 0 AND $AlterID &lt;= M` inserted once, at the end of the
  census formula, and nothing else changed.
It prints `census: ok N, FAIL F` and exits 0 only when F is 0. It prints no company figure beyond the names and dates
already in this sheet.
"""
import os
import sys
import xml.etree.ElementTree as ET

# key, company, from, to, M, control file key (two windows of one day share one control)
WINDOWS = [
    ("a-wa1", "BRIDGE AMEND LAB", "20260403", "20260403", 1676, "a-wa1"),
    ("a-d3batch", "BRIDGE AMEND LAB", "20260401", "20260401", 1469, "a-d3-0401"),
    ("a-d3cancel", "BRIDGE AMEND LAB", "20260401", "20260401", 1685, "a-d3-0401"),
    ("a-l1", "BRIDGE AMEND LAB", "20260617", "20260617", 1790, "a-l1"),
    ("p-audit", "BRIDGE PILOT LAB", "20260401", "20270331", 68, "p-audit"),
    ("s-0421", "BRIDGE STOCK LAB", "20250421", "20250421", 3, "s-0421"),
] + [
    ("g-%snov" % day, "BRIDGE GST LEDGER LAB", "202511" + day, "202511" + day, 64, "g-%snov" % day)
    for day in ["03", "04", "05", "06", "07", "08", "10", "11", "12", "13", "14", "15", "16", "17"]
]


def text(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise ValueError("no UTF-16LE byte-order mark")
    body = data[2:].decode("utf-16-le")
    ET.fromstring(body)
    return body


def check(folder, key, company, frm, to, mark, control):
    unspanned = text(os.path.join(folder, "%s_c1_unspanned.request.utf16le.xml" % control))
    spanned = text(os.path.join(folder, "%s_c2_spanned_0_%d.request.utf16le.xml" % (key, mark)))
    for body in (unspanned, spanned):
        if body.count("<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % company) != 1:
            return "company"
        if body.count('<SVFROMDATE TYPE="Date">%s</SVFROMDATE>' % frm) != 1 or body.count('<SVTODATE TYPE="Date">%s</SVTODATE>' % to) != 1:
            return "dates"
        if body.count('$$Date:"%s"' % frm) < 1 or body.count('$$Date:"%s"' % to) < 1:
            return "date literals"
    clause = " AND $AlterID &gt; 0 AND $AlterID &lt;= %d" % mark
    end = '$$Date:"%s"</SYSTEM>' % to
    if unspanned.count(end) != 1 or "$AlterID" in unspanned:
        return "control shape"
    if spanned != unspanned.replace(end, '$$Date:"%s"%s</SYSTEM>' % (to, clause)):
        return "span not the control plus one clause"
    return None


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        return 2
    ok = fail = 0
    for window in WINDOWS:
        try:
            problem = check(sys.argv[1], *window)
        except (OSError, ValueError, ET.ParseError) as error:
            problem = "unreadable (%s)" % type(error).__name__
        if problem:
            fail += 1
            print("FAIL %s: %s" % (window[0], problem))
        else:
            ok += 1
    print("census: ok %d, FAIL %d" % (ok, fail))
    return 0 if fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
