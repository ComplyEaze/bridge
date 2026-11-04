#!/usr/bin/env python3
"""Row 2119-B's requests and its ties, for BRIDGE BILLTRAIL LAB and its party Lab Party P1.

  python -I tie.py fixed --out <dir>
      Writes the seven requests, UTF-16LE with a byte-order mark, the way the transport sends them. Three must be
      byte for byte the requests sent on 27 Oct 2026 (lab/1356-party-term-and-ledger-outstandings-run-sheet), and the
      command fails unless they are:
        marks.utf16le.xml          r01, 984 B: the company's GUID, ALTVCHID and ALTMSTID
        lo-0331.utf16le.xml        r04, 1198 B: Ledger Outstandings for the party to 31 March 2026
        lo-0731.utf16le.xml        r05, 1198 B: the same to 31 July 2025
      The other four are master's own requests (1400175e), rendered as their Rust renderers render them:
        receivable-0331.utf16le.xml  render_native_bills_request (Receivable), 1 April 2025 to 31 March 2026
        payable-0331.utf16le.xml     render_native_bills_request (Payable), the same dates
        ledgers-0331.utf16le.xml     render_native_ledger_snapshot_request, the same dates
        ledgers-0731.utf16le.xml     render_native_ledger_snapshot_request, 1 April 2025 to 31 July 2025
  python -I tie.py marks <answer>
  python -I tie.py report <answer>
  python -I tie.py lo <answer>
  python -I tie.py ledgers <answer> <marks answer>
      Parses one answer by LAB-COMMON C4 and prints what the sheet records of it.
  python -I tie.py tie0331 <marks> <receivable> <payable> <ledgers-0331> <lo-0331>
  python -I tie.py tie0731 <marks> <ledgers-0731> <lo-0731>
      The ties of steps 6 and 9: each prints one line per tie, `tie <name> equal` or `tie <name> NOT EQUAL ...`.

Nothing from Tally enters a request: the company, the party, the dates and every request's text are fixed here.

Exit 0 = done (a tie that is NOT EQUAL is a result, not a failure), 1 = failed (the line says why; the sheet treats it
as STOP), 2 = the command line is wrong.
"""
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET
from decimal import Decimal, InvalidOperation

COMPANY = "BRIDGE BILLTRAIL LAB"
PARTY = "Lab Party P1"
BOOKS_FROM = "20250401"

MARKS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
    "<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES>"
    "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES>"
    '<TDL><TDLMESSAGE><COLLECTION NAME="Bridge Agent Company High Water" ISMODIFY="No"><TYPE>Company</TYPE>'
    "<FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
# native_outstandings/request.rs:131-144.
BILLS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>{id}</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE>'
    "</STATICVARIABLES></DESC></BODY></ENVELOPE>"
)
# native_outstandings/request.rs:197-208, unfiltered (parent_filter_parts(None) is two empty strings).
LEDGERS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
    "<ID>List of Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE></STATICVARIABLES>'
    '<TDL><TDLMESSAGE><COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">'
    "<FETCH>NAME, PARENT, CLOSINGBALANCE, OPENINGBALANCE, ISBILLWISEON, CURRENCYNAME</FETCH>"
    "<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION></TDLMESSAGE></TDL>"
    "</DESC></BODY></ENVELOPE>"
)
# r04's text (lab/1356-party-term-and-ledger-outstandings-run-sheet), its layout and line breaks its own.
LEDGER_OUTSTANDINGS = (
    "\n<ENVELOPE>\n    <HEADER>\n        <VERSION>1</VERSION>\n        <TALLYREQUEST>Export</TALLYREQUEST>\n"
    "        <TYPE>Data</TYPE>\n        <ID>Ledger Outstandings</ID>\n    </HEADER>\n    <BODY>\n        <DESC>\n"
    "            <STATICVARIABLES>\n                <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>\n"
    "                <LedgerName>{ledger}</LedgerName>\n"
    '                <SVTODATE TYPE="Date">{day}</SVTODATE>\n'
    "                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>\n                <EXPLODEFLAG>Yes</EXPLODEFLAG>\n"
    "            </STATICVARIABLES>\n        </DESC>\n    </BODY>\n</ENVELOPE>\n"
)
SENT_27_OCT = {
    "marks.utf16le.xml": (984, "501c3094407612a082bd1518518276bcdd97af77ddcc358d4b7da2387fc84910"),
    "lo-0331.utf16le.xml": (1198, "f560dba1651ca4b4d89359feffd8e4f6058ee15ae380c05929cca072313d95dc"),
    "lo-0731.utf16le.xml": (1198, "7c093656c55f14dd9f0164d6b9cd4806aafb3e5139a7d315f79794a3263942ad"),
}
MONTHS = {m: i for i, m in enumerate(("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"), 1)}
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
LO_ONCE = ("BILLOP", "BILLCL", "BILLDUE", "BILLOVERDUE")
# One voucher line is these four in this order, then a BILLVCHQUALIFIER on some (r04's answer, 27 Oct).
LO_VOUCHER = ("BILLVCHDATE", "BILLVCHTYPE", "BILLVCHNUMBER", "BILLVCHAMOUNT")
LO_QUALIFIER = "BILLVCHQUALIFIER"
LEDGER_ONCE = ("PARENT", "CLOSINGBALANCE", "OPENINGBALANCE", "ISBILLWISEON", "CURRENCYNAME", "BRIDGECOMPANYGUID")


class Failed(Exception):
    pass


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def escape(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def requests():
    company = escape(COMPANY)
    return [
        ("marks.utf16le.xml", MARKS.format(company=company)),
        ("receivable-0331.utf16le.xml", BILLS.format(id="Bills Receivable", company=company, start=BOOKS_FROM, day="20260331")),
        ("payable-0331.utf16le.xml", BILLS.format(id="Bills Payable", company=company, start=BOOKS_FROM, day="20260331")),
        ("ledgers-0331.utf16le.xml", LEDGERS.format(company=company, start=BOOKS_FROM, day="20260331")),
        ("lo-0331.utf16le.xml", LEDGER_OUTSTANDINGS.format(company=company, ledger=escape(PARTY), day="20260331")),
        ("ledgers-0731.utf16le.xml", LEDGERS.format(company=company, start=BOOKS_FROM, day="20250731")),
        ("lo-0731.utf16le.xml", LEDGER_OUTSTANDINGS.format(company=company, ledger=escape(PARTY), day="20250731")),
    ]


def fixed(folder):
    rendered = [(name, encode(text)) for name, text in requests()]
    for name, data in rendered:
        if name in SENT_27_OCT and (len(data), hashlib.sha256(data).hexdigest()) != SENT_27_OCT[name]:
            raise Failed("{} does not render the bytes sent on 27 Oct ({} B, {})".format(
                name, len(data), hashlib.sha256(data).hexdigest()))
    os.makedirs(folder, exist_ok=True)
    lines = []
    for name, data in rendered:
        with open(os.path.join(folder, name), "xb") as handle:
            handle.write(data)
        lines.append("{} {} {}".format(name, len(data), hashlib.sha256(data).hexdigest()))
    return "\n".join(lines)


def forbidden(code):
    return code <= 8 or code in (11, 12) or 14 <= code <= 31


def clean(text):
    return REFERENCE.sub(lambda m: "" if forbidden(int(m.group(1)[1:], 16) if m.group(1)[0] == "x" else int(m.group(1))) else m.group(0),
                         FORBIDDEN_RAW.sub("", text))


def root(path):
    with open(path, "rb") as handle:
        data = handle.read()
    if not data:
        raise Failed("the answer is empty")
    try:
        text = data.decode("utf-16-le")
    except UnicodeDecodeError as error:
        raise Failed("the answer is not UTF-16LE: {}".format(error))
    text = text[1:] if text.startswith("﻿") else text
    try:
        element = ET.fromstring(clean(text))
    except ET.ParseError as error:
        raise Failed("the answer does not parse: {}".format(error))
    if element.tag != "ENVELOPE":
        raise Failed("the root is {}, not ENVELOPE".format(element.tag))
    root.size = len(data)
    return element


def day(text):
    """YYYYMMDD from Tally's `1-Jul-26`; None if it is not one."""
    match = re.fullmatch(r"(\d{1,2})-([A-Z][a-z]{2})-(\d{2})", text.strip())
    if not match or match.group(2) not in MONTHS:
        return None
    return "20{}{:02d}{:02d}".format(match.group(3), MONTHS[match.group(2)], int(match.group(1)))


def amount(text, what):
    try:
        return Decimal(text.strip())
    except InvalidOperation:
        raise Failed("{} is not a number: {!r}".format(what, text))


def flat_rows(path, fixed_fields, once, repeated):
    """A flat report under ENVELOPE: a BILLFIXED group, then its scalars, with no row wrapper (wire.rs)."""
    rows, current = [], None
    for element in root(path):
        if element.tag == "STATUS":
            raise Failed("a STATUS element is present: {!r}".format(element.text or ""))
        if element.tag == "BILLFIXED":
            fields = {}
            for child in element:
                if child.tag not in fixed_fields or child.tag in fields or len(child):
                    raise Failed("a BILLFIXED with an unexpected or repeated {}".format(child.tag))
                fields[child.tag] = child.text or ""
            if set(fields) != set(fixed_fields):
                raise Failed("a BILLFIXED without its fields {}".format(",".join(fixed_fields)))
            current = {"fields": fields, "once": {}, "repeated": []}
            rows.append(current)
            continue
        if len(element):
            raise Failed("an element nested deeper than the grammar allows: {}".format(element.tag))
        if element.tag in once:
            if current is None or element.tag in current["once"]:
                raise Failed("a {} without its BILLFIXED, or a second one".format(element.tag))
            current["once"][element.tag] = element.text or ""
        elif element.tag in repeated:
            if current is None:
                raise Failed("a {} before any BILLFIXED".format(element.tag))
            current["repeated"].append(element.tag)
        else:
            raise Failed("an unexpected element under ENVELOPE: {}".format(element.tag))
    for row in rows:
        if set(row["once"]) != set(once):
            raise Failed("a bill without {}".format(",".join(sorted(set(once) - set(row["once"])))))
    return rows


def bill_date(row):
    date = day(row["fields"]["BILLDATE"])
    if date is None:
        raise Failed("a BILLDATE not read as a date: {!r}".format(row["fields"]["BILLDATE"]))
    return date


def report_rows(path):
    return [{"party": row["fields"]["BILLPARTY"], "ref": row["fields"]["BILLREF"], "date": bill_date(row),
             "amount": amount(row["once"]["BILLCL"], "a BILLCL")}
            for row in flat_rows(path, ("BILLDATE", "BILLREF", "BILLPARTY"), ("BILLCL", "BILLDUE", "BILLOVERDUE"), ())]


def voucher_lines(tags):
    lines, at = 0, 0
    while at < len(tags):
        if tuple(tags[at:at + len(LO_VOUCHER)]) != LO_VOUCHER:
            raise Failed("a bill whose voucher lines are not BILLVCHDATE, TYPE, NUMBER, AMOUNT in order")
        at += len(LO_VOUCHER)
        if at < len(tags) and tags[at] == LO_QUALIFIER:
            at += 1
        lines += 1
    return lines


def lo_rows(path):
    out = []
    for row in flat_rows(path, ("BILLDATE", "BILLREF"), LO_ONCE, LO_VOUCHER + (LO_QUALIFIER,)):
        out.append({"ref": row["fields"]["BILLREF"], "date": bill_date(row), "amount": amount(row["once"]["BILLCL"], "a BILLCL"),
                    "on_account": row["fields"]["BILLREF"] == "On Account" and row["once"]["BILLDUE"].strip() == "",
                    "vouchers": voucher_lines(row["repeated"])})
    return out


def status_one(element):
    status = element.findall("./HEADER/STATUS")
    if len(status) != 1 or (status[0].text or "").strip() != "1":
        raise Failed("the HEADER does not hold exactly one STATUS of 1")


def marks(path):
    element = root(path)
    status_one(element)
    rows = element.findall("./BODY/DATA/COLLECTION/COMPANY")
    if len(rows) != 1:
        raise Failed("{} COMPANY rows, not one".format(len(rows)))
    row = rows[0]
    values = {}
    for tag in ("GUID", "ALTVCHID", "ALTMSTID"):
        found = row.findall(tag)
        if len(found) != 1 or not (found[0].text or "").strip():
            raise Failed("the COMPANY row has no single {}".format(tag))
        values[tag] = found[0].text.strip()
    return row.get("NAME"), values


def ledger_rows(path, guid):
    element = root(path)
    status_one(element)
    collections = element.findall("./BODY/DATA/COLLECTION")
    if len(collections) != 1:
        raise Failed("{} COLLECTION elements, not one".format(len(collections)))
    rows = {}
    for row in collections[0]:
        if row.tag != "LEDGER":
            continue
        name = row.get("NAME")
        if not name:
            raise Failed("a LEDGER row without a NAME")
        if name in rows:
            raise Failed("two LEDGER rows named alike")
        values = {}
        for tag in LEDGER_ONCE:
            found = row.findall(tag)
            if len(found) > 1:
                raise Failed("a LEDGER row with two {}".format(tag))
            values[tag] = (found[0].text or "").strip() if found else None
        if values["BRIDGECOMPANYGUID"] != guid:
            raise Failed("a LEDGER row whose BRIDGECOMPANYGUID is not the marks answer's GUID")
        rows[name] = values
    return rows


def total(rows):
    return sum((row["amount"] for row in rows), Decimal(0))


def tie(name, left, right):
    if left == right:
        return "tie {} equal ({})".format(name, left)
    return "tie {} NOT EQUAL ({} against {})".format(name, left, right)


def by_key(rows):
    keyed = {}
    for row in rows:
        keyed.setdefault((row["ref"], row["date"]), []).append(row["amount"])
    return keyed


def match(name, report, lo):
    """Matches by reference and date, and says how the amounts relate: equal, of opposite sign, or neither."""
    left, right = by_key(report), by_key(lo)
    both = sorted(set(left) & set(right))
    equal = sum(1 for k in both if sorted(left[k]) == sorted(right[k]))
    opposite = sum(1 for k in both if sorted(left[k]) != sorted(right[k]) and sorted(-a for a in left[k]) == sorted(right[k]))
    repeated = sum(1 for v in list(left.values()) + list(right.values()) if len(v) > 1)
    return "{} matched {} equal {} opposite_sign {} differ {} only_report {} only_lo {} repeated_keys {}".format(
        name, len(both), equal, opposite, len(both) - equal - opposite, len(set(left) - set(right)),
        len(set(right) - set(left)), repeated)


def closing(path, guid):
    rows = ledger_rows(path, guid)
    if PARTY not in rows:
        raise Failed("the party has no LEDGER row ({} rows)".format(len(rows)))
    text = rows[PARTY]["CLOSINGBALANCE"]
    if text is None:
        raise Failed("the party's LEDGER row has no CLOSINGBALANCE")
    return rows, (Decimal(0) if text == "" else amount(text, "the party's CLOSINGBALANCE"))


def lo_split(rows):
    bills = [r for r in rows if not r["on_account"]]
    return ([r for r in bills if r["amount"] < 0], [r for r in bills if r["amount"] >= 0],
            [r for r in rows if r["on_account"]])


def tie0331(marks_path, receivable_path, payable_path, ledgers_path, lo_path):
    guid = marks(marks_path)[1]["GUID"]
    receivable = [r for r in report_rows(receivable_path) if r["party"] == PARTY]
    payable = [r for r in report_rows(payable_path) if r["party"] == PARTY]
    rows, balance = closing(ledgers_path, guid)
    lo = lo_rows(lo_path)
    debit, credit, on_account = lo_split(lo)
    named = total(receivable) + total(payable)
    return "\n".join([
        "party rows: receivable {} (sum {}), payable {} (sum {}); lo debit {} (sum {}), credit {} (sum {}), on_account {} (sum {})".format(
            len(receivable), total(receivable), len(payable), total(payable), len(debit), total(debit), len(credit),
            total(credit), len(on_account), total(on_account)),
        "ledgers rows {}; the party: CLOSINGBALANCE {} ISBILLWISEON {}".format(len(rows), balance, rows[PARTY]["ISBILLWISEON"]),
        match("debit_vs_receivable", receivable, debit),
        match("credit_vs_payable", payable, credit),
        tie("closing_vs_lo_total", balance, total(lo)),
        tie("residual_vs_on_account", balance - named, total(on_account)),
    ])


def tie0731(marks_path, ledgers_path, lo_path):
    guid = marks(marks_path)[1]["GUID"]
    rows, balance = closing(ledgers_path, guid)
    lo = lo_rows(lo_path)
    debit, credit, on_account = lo_split(lo)
    return "\n".join([
        "lo debit {} (sum {}), credit {} (sum {}), on_account {} (sum {})".format(
            len(debit), total(debit), len(credit), total(credit), len(on_account), total(on_account)),
        "ledgers rows {}; the party: CLOSINGBALANCE {}".format(len(rows), balance),
        tie("closing_vs_lo_total", balance, total(lo)),
        tie("residual_vs_on_account", balance - total(debit) - total(credit), total(on_account)),
    ])


def main(argv):
    if len(argv) == 3 and argv[0] == "fixed" and argv[1] == "--out":
        return fixed(argv[2])
    if len(argv) == 2 and argv[0] == "marks":
        name, values = marks(argv[1])
        return "marks ok bytes {} company {} guid {}... ALTVCHID {} ALTMSTID {}".format(
            root.size, name, values["GUID"][:8], values["ALTVCHID"], values["ALTMSTID"])
    if len(argv) == 2 and argv[0] == "report":
        rows = report_rows(argv[1])
        mine = [r for r in rows if r["party"] == PARTY]
        return "report ok bytes {} rows {} sum {}; the party's rows {} sum {}".format(
            root.size, len(rows), total(rows), len(mine), total(mine))
    if len(argv) == 2 and argv[0] == "lo":
        rows = lo_rows(argv[1])
        return "lo ok bytes {} rows {} sum {} voucher_lines {}".format(
            root.size, len(rows), total(rows), sum(r["vouchers"] for r in rows))
    if len(argv) == 3 and argv[0] == "ledgers":
        guid = marks(argv[2])[1]["GUID"]
        rows, balance = closing(argv[1], guid)
        return "ledgers ok bytes {} rows {}; the party's CLOSINGBALANCE {}".format(root.size, len(rows), balance)
    if len(argv) == 6 and argv[0] == "tie0331":
        return tie0331(*argv[1:])
    if len(argv) == 4 and argv[0] == "tie0731":
        return tie0731(*argv[1:])
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
