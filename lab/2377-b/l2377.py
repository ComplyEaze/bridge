#!/usr/bin/env python3
"""Row 2377-L's requests and checks, for BRIDGE BILLTRAIL LAB (Part A of the #2377 design note's rows).

  python -I l2377.py fixed --out <dir>
      Writes the fixed requests, UTF-16LE with a byte-order mark, the way the transport sends them:
        marks.utf16le.xml     r01 of lab/1356-party-term-and-ledger-outstandings-run-sheet, byte for byte (checked)
        books.utf16le.xml     the same collection with FETCH GUID,BOOKSFROM (BOOKSFROM is one of master's
                              AUDIT_COMPANY_FETCH fields, xml_read_profiles.rs:490); no sender on master in this form
        lo-p1-0331.utf16le.xml  r04 of that sheet, byte for byte (checked): Lab Party P1 to 20260331
        lo-p1-0731.utf16le.xml  r05 of that sheet, byte for byte (checked): Lab Party P1 to 20250731
        lo-unknown.utf16le.xml  the same shape with the name "Lab Party Q9", to 20260331
        lo-case.utf16le.xml     the same shape with "lab party p1", to 20260331
        lo-space.utf16le.xml    the same shape with "Lab  Party P1" (two spaces), to 20260331
  python -I l2377.py books <books answer>
      Prints BOOKSFROM as YYYYMMDD.
  python -I l2377.py snapshot <from> --out <dir>
      Writes snap-0331 and snap-0731: master's unfiltered ledger snapshot (render_native_ledger_snapshot,
      native_outstandings/request.rs:197-210, no parent or GUID part) from <from> to each date.
  python -I l2377.py pick <snap-0331 answer> --out <dir>
      From the snapshot: STOP if "Lab Party Q9" is a ledger. Picks Z, the first bill-wise ledger (in the order of its
      name's UTF-8 bytes) other than Lab Party P1 whose closing balance is zero, and C, the first bill-wise ledger whose
      closing balance is a credit (positive). Each name must pass bills.py's admitted alphabet. Writes picks.txt and
      lo-z.utf16le.xml and lo-c.utf16le.xml to 20260331. A pick with no candidate is written as "none" and its
      request is not written.
  python -I l2377.py lo <lo answer>
      Counts BILLFIXED groups and On Account rows, and sums every BILLCL. An empty or unparsable BILLCL fails.
  python -I l2377.py tie <lo answer> <snap answer> <ledger name>
      The design note's tie: the sum of BILLCL against the ledger's CLOSINGBALANCE. Prints both and "equal",
      "negated" or "differ". It decides nothing by itself; the sheet records the word.

Exit 0 = done, 1 = failed (the line says why; the sheet treats it as STOP), 2 = the command line is wrong.
"""
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET
from decimal import Decimal, InvalidOperation

COMPANY = "BRIDGE BILLTRAIL LAB"
P1 = "Lab Party P1"
UNKNOWN = "Lab Party Q9"
DATES = (("0331", "20260331"), ("0731", "20250731"))
MARKS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
    "<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES>"
    "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES>"
    '<TDL><TDLMESSAGE><COLLECTION NAME="Bridge Agent Company High Water" ISMODIFY="No"><TYPE>Company</TYPE>'
    "<FETCH>{fetch}</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
SNAPSHOT = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
    "<ID>List of Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{start}</SVFROMDATE>"
    "<SVTODATE TYPE=\"Date\">{day}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE>"
    "<COLLECTION NAME=\"List of Ledgers\" ISMODIFY=\"Yes\"><FETCH>NAME, PARENT, CLOSINGBALANCE, OPENINGBALANCE, "
    "ISBILLWISEON, CURRENCYNAME</FETCH><COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>"
    "</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
# r04's text, as in bills.py: the layout, the variable order and the line breaks are its own.
LEDGER_OUTSTANDINGS = (
    "\n<ENVELOPE>\n    <HEADER>\n        <VERSION>1</VERSION>\n        <TALLYREQUEST>Export</TALLYREQUEST>\n"
    "        <TYPE>Data</TYPE>\n        <ID>Ledger Outstandings</ID>\n    </HEADER>\n    <BODY>\n        <DESC>\n"
    "            <STATICVARIABLES>\n                <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>\n"
    "                <LedgerName>{ledger}</LedgerName>\n"
    '                <SVTODATE TYPE="Date">{day}</SVTODATE>\n'
    "                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>\n                <EXPLODEFLAG>Yes</EXPLODEFLAG>\n"
    "            </STATICVARIABLES>\n        </DESC>\n    </BODY>\n</ENVELOPE>\n"
)
# The 27 Oct requests, by size and sha256, from lab/1356-party-term-and-ledger-outstandings-run-sheet.
R01 = (None, 0, "501c3094407612a082bd1518518276bcdd97af77ddcc358d4b7da2387fc84910")
R04 = ("lo-p1-0331", 1198, "f560dba1651ca4b4d89359feffd8e4f6058ee15ae380c05929cca072313d95dc")
R05 = ("lo-p1-0731", 0, "7c093656c55f14dd9f0164d6b9cd4806aafb3e5139a7d315f79794a3263942ad")
NAME = re.compile(r"\A[A-Za-z0-9]+( [A-Za-z0-9]+)*\Z")
FORMULA_WORDS = ("and", "or", "not", "is", "in", "contains", "like", "between", "starting", "ending", "with",
                 "equal", "greater", "less", "than")
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
DAY = re.compile(r"\A\d{8}\Z")


class Failed(Exception):
    pass


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def escape(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def write(folder, name, text, expect=None):
    data = encode(text)
    digest = hashlib.sha256(data).hexdigest()
    if expect is not None and digest != expect:
        raise Failed("{} does not render the 27 Oct bytes ({} B, {})".format(name, len(data), digest))
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, name + ".utf16le.xml"), "xb") as handle:
        handle.write(data)
    return "{}.utf16le.xml {} {}".format(name, len(data), digest)


def lo(ledger, day):
    return LEDGER_OUTSTANDINGS.format(company=escape(COMPANY), ledger=escape(ledger), day=day)


def fixed(folder):
    return "\n".join([
        write(folder, "marks", MARKS.format(company=escape(COMPANY), fetch="GUID,ALTVCHID,ALTMSTID"), R01[2]),
        write(folder, "books", MARKS.format(company=escape(COMPANY), fetch="GUID,BOOKSFROM")),
        write(folder, "lo-p1-0331", lo(P1, "20260331"), R04[2]),
        write(folder, "lo-p1-0731", lo(P1, "20250731"), R05[2]),
        write(folder, "lo-unknown", lo(UNKNOWN, "20260331")),
        write(folder, "lo-case", lo("lab party p1", "20260331")),
        write(folder, "lo-space", lo("Lab  Party P1", "20260331")),
    ])


def text_of(path):
    with open(path, "rb") as handle:
        data = handle.read()
    if data.startswith(b"\xff\xfe"):
        text = data[2:].decode("utf-16-le")
    elif len(data) % 2 == 0 and len(data) >= 2 and data[1] == 0:
        # Tally's answers come as UTF-16LE with no byte-order mark (CONTENT-TYPE text/xml; charset=utf-16).
        text = data.decode("utf-16-le")
    else:
        raise Failed("{}: not UTF-16LE".format(path))
    return REFERENCE.sub(keep_reference, FORBIDDEN_RAW.sub("", text))


def keep_reference(match):
    # Tally writes control characters as references XML forbids; bills.py's clean() drops the same set.
    body = match.group(1)
    code = int(body[1:], 16) if body[0] == "x" else int(body)
    forbidden = code <= 8 or code in (11, 12) or 14 <= code <= 31
    return "" if forbidden else match.group(0)


def tree(path):
    try:
        return ET.fromstring(text_of(path))
    except ET.ParseError as error:
        raise Failed("{}: does not parse ({})".format(path, error))


def amount(text, what):
    value = (text or "").strip()
    if not value:
        raise Failed("{}: empty".format(what))
    try:
        return Decimal(value)
    except InvalidOperation:
        raise Failed("{}: not an amount ({!r})".format(what, value))


def books(path):
    found = [e.text.strip() for e in tree(path).iter("BOOKSFROM") if e.text and e.text.strip()]
    if len(found) != 1 or not DAY.match(found[0]):
        raise Failed("BOOKSFROM: expected one YYYYMMDD value, found {!r}".format(found))
    return found[0]


def snapshot(start, folder):
    if not DAY.match(start):
        raise Failed("from: not YYYYMMDD")
    return "\n".join(write(folder, "snap-" + tag, SNAPSHOT.format(company=escape(COMPANY), start=start, day=day))
                     for tag, day in DATES)


def ledgers(path):
    rows = {}
    for element in tree(path).iter("LEDGER"):
        name = element.get("NAME") or (element.findtext("NAME") or "")
        if not name:
            raise Failed("a LEDGER with no name")
        if name in rows:
            raise Failed("ledger {!r} twice".format(name))
        closing = (element.findtext("CLOSINGBALANCE") or "").strip()
        rows[name] = {
            "billwise": (element.findtext("ISBILLWISEON") or "").strip(),
            # Master's parser reads an empty closing balance as zero (compute.rs:436); recorded as "empty" here.
            "closing": Decimal(closing) if closing else None,
        }
    if not rows:
        raise Failed("no LEDGER in the snapshot")
    return rows


def admitted(name):
    return NAME.match(name) is not None and not any(w.lower() in FORMULA_WORDS for w in name.split(" "))


def pick(path, folder):
    rows = ledgers(path)
    if UNKNOWN in rows:
        raise Failed("{!r} is a ledger in the book: L3 cannot run".format(UNKNOWN))
    if P1 not in rows:
        raise Failed("{!r} is not a ledger in the book".format(P1))
    order = sorted(rows, key=lambda n: n.encode("utf-8"))
    billwise = [n for n in order if rows[n]["billwise"] == "Yes" and n != P1 and admitted(n)]
    zero = [n for n in billwise if rows[n]["closing"] is None or rows[n]["closing"] == 0]
    credit = [n for n in billwise if rows[n]["closing"] is not None and rows[n]["closing"] > 0]
    out = ["ledgers {}; bill-wise and admitted, other than P1: {}".format(len(rows), len(billwise))]
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "picks.txt"), "x", encoding="utf-8") as handle:
        for tag, names in (("z", zero), ("c", credit)):
            name = names[0] if names else "none"
            handle.write("{}\t{}\n".format(tag, name))
            out.append("{} {}".format(tag, "none" if not names else write(folder, "lo-" + tag, lo(name, "20260331"))))
    return "\n".join(out)


def lo_rows(path):
    root = tree(path)
    if root.tag != "ENVELOPE":
        raise Failed("{}: the root is {!r}, not ENVELOPE".format(path, root.tag))
    fixed_groups = root.findall("BILLFIXED")
    closings = root.findall("BILLCL")
    if len(fixed_groups) != len(closings):
        raise Failed("{}: {} BILLFIXED but {} BILLCL".format(path, len(fixed_groups), len(closings)))
    total = sum((amount(e.text, "BILLCL") for e in closings), Decimal(0))
    on_account = sum(1 for g in fixed_groups if (g.findtext("BILLREF") or "").strip() == "On Account")
    other = sorted({child.tag for child in root} - {"BILLFIXED", "BILLOP", "BILLCL", "BILLDUE", "BILLOVERDUE",
                                                    "BILLVCHDATE", "BILLVCHTYPE", "BILLVCHNUMBER",
                                                    "BILLVCHAMOUNT", "BILLVCHQUALIFIER"})
    return len(fixed_groups), on_account, total, other


def lo_summary(path):
    count, on_account, total, other = lo_rows(path)
    return "bills {} on_account {} sum_billcl {} other_tags {}".format(count, on_account, total, ",".join(other) or "-")


def tie(lo_path, snap_path, name):
    rows = ledgers(snap_path)
    if name not in rows:
        raise Failed("{!r} is not in the snapshot".format(name))
    _, _, total, _ = lo_rows(lo_path)
    closing = rows[name]["closing"]
    shown = "empty" if closing is None else str(closing)
    value = closing if closing is not None else Decimal(0)
    word = "equal" if total == value else "negated" if total == -value else "differ"
    return "sum_billcl {} ledger_closing {} {}".format(total, shown, word)


def main(argv):
    if len(argv) == 3 and argv[0] == "fixed" and argv[1] == "--out":
        return fixed(argv[2])
    if len(argv) == 2 and argv[0] == "books":
        return books(argv[1])
    if len(argv) == 4 and argv[0] == "snapshot" and argv[2] == "--out":
        return snapshot(argv[1], argv[3])
    if len(argv) == 4 and argv[0] == "pick" and argv[2] == "--out":
        return pick(argv[1], argv[3])
    if len(argv) == 2 and argv[0] == "lo":
        return lo_summary(argv[1])
    if len(argv) == 4 and argv[0] == "tie":
        return tie(*argv[1:])
    return None


if __name__ == "__main__":
    try:
        result = main(sys.argv[1:])
    except (Failed, OSError, ValueError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    if result is None:
        print(__doc__)
        sys.exit(2)
    print(result)
    sys.exit(0)
