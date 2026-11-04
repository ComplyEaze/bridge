#!/usr/bin/env python3
"""Row 2119-L's requests, its party pick and its bill-for-bill comparison, for BRIDGE CORPUS DENSE at 31 March 2026.

  python -I bills.py fixed --out <dir>
      Writes the two fixed requests, UTF-16LE with a byte-order mark, the way the transport sends them:
        marks.utf16le.xml        the company's GUID, ALTVCHID and ALTMSTID (the marks read of earlier rows)
        ref-receivable.utf16le.xml
                                 master's Bills Receivable request (`render_native_bills_request`,
                                 native_outstandings/request.rs:131-144) from 20250401 to 20260331: the request
                                 runtime.rs:4141-4147 sends. Byte for byte the 770-byte r-4 of lab/2119-bills-read-timing.
  python -I bills.py select <ref answer> --out <dir>
      Picks one party: the first BILLPARTY name of the Receivable answer, in the order of its UTF-8 bytes, that
      `LedgerScopeName::new` would admit (ledger_scope.rs:1-21 and :65-94: ASCII letters and digits with single spaces
      between words, no operator word, the only names observed to be read as names). Writes party.txt (the name and
      its bills in the report) and
      lo-p1.utf16le.xml: the Ledger Outstandings request sent live on 27 Oct 2026 on TallyPrime 7.2 Silver
      (r04 of lab/1356-party-term-and-ledger-outstandings-run-sheet, the text of the probe removed by #1747), with only
      the company and the party changed. Before writing, the template is rendered with r04's own company and party and
      must give r04's bytes (1198 B, sha256 f560dba1...), else the command fails.
  python -I bills.py ref <answer>
  python -I bills.py lo <answer>
      Parses one answer by LAB-COMMON C4 and prints its counts and sums.
  python -I bills.py screen <lo answer>
      The Ledger Outstandings answer as Tally's screen shows it: its row count, its total pending as Dr or Cr, and its
      first and last five rows (date, reference, opening, pending, due date, overdue days), in the answer's order.
  python -I bills.py compare <ref answer> <lo answer> <party.txt>
      Bill for bill: the Ledger Outstandings rows against that party's rows in the Receivable report. The report keeps
      debit-balance bills, so only the answer's debit bills are matched; its credit bills and its On Account row (a
      reference "On Account" with no due date) are counted and summed apart. The key is reference and date.

The only value from Tally in a request is the party's name, through the check above, escaped once as XML text, in a
static variable (never in a formula). The company, date, report, variables and layout are fixed here.

Exit 0 = done, 1 = failed (the line says why; the sheet treats it as STOP), 2 = the command line is wrong.
"""
import codecs
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET
from decimal import Decimal, InvalidOperation

COMPANY = "BRIDGE CORPUS DENSE"
FROM = "20250401"
TO = "20260331"

MARKS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
    "<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES>"
    "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES>"
    '<TDL><TDLMESSAGE><COLLECTION NAME="Bridge Agent Company High Water" ISMODIFY="No"><TYPE>Company</TYPE>'
    "<FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
REF = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Bills Receivable</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE>'
    "</STATICVARIABLES></DESC></BODY></ENVELOPE>"
)
# r04's text, byte for byte apart from the two values: the layout, the variable order and the line breaks are its own.
LEDGER_OUTSTANDINGS = (
    "\n<ENVELOPE>\n    <HEADER>\n        <VERSION>1</VERSION>\n        <TALLYREQUEST>Export</TALLYREQUEST>\n"
    "        <TYPE>Data</TYPE>\n        <ID>Ledger Outstandings</ID>\n    </HEADER>\n    <BODY>\n        <DESC>\n"
    "            <STATICVARIABLES>\n                <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>\n"
    "                <LedgerName>{ledger}</LedgerName>\n"
    '                <SVTODATE TYPE="Date">{day}</SVTODATE>\n'
    "                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>\n                <EXPLODEFLAG>Yes</EXPLODEFLAG>\n"
    "            </STATICVARIABLES>\n        </DESC>\n    </BODY>\n</ENVELOPE>\n"
)
R04 = ("BRIDGE BILLTRAIL LAB", "Lab Party P1", "20260331", 1198,
       "f560dba1651ca4b4d89359feffd8e4f6058ee15ae380c05929cca072313d95dc")
# `LedgerScopeName::new` (ledger_scope.rs:65-94), the same check: 1 to 1024 characters, ASCII letters and digits with
# single spaces between words, and no word of FORMULA_WORDS (:30-33) in any case.
NAME = re.compile(r"\A[A-Za-z0-9]+( [A-Za-z0-9]+)*\Z")
NAME_MAX = 1024
FORMULA_WORDS = ("and", "or", "not", "is", "in", "contains", "like", "between", "starting", "ending", "with",
                 "equal", "greater", "less", "than")
MONTHS = {m: i for i, m in enumerate(("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"), 1)}
CHUNK = 1 << 20
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
LO_ONCE = ("BILLOP", "BILLCL", "BILLDUE", "BILLOVERDUE")
# One voucher line is these four in this order, then a BILLVCHQUALIFIER on some (r04's answer: absent on the
# opening-balance lines, present and empty on the others).
LO_VOUCHER = ("BILLVCHDATE", "BILLVCHTYPE", "BILLVCHNUMBER", "BILLVCHAMOUNT")
LO_QUALIFIER = "BILLVCHQUALIFIER"


class Failed(Exception):
    pass


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def escape(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def ledger_outstandings(company, ledger, to):
    return LEDGER_OUTSTANDINGS.format(company=escape(company), ledger=escape(ledger), day=to)


def check_template():
    company, ledger, to, size, digest = R04
    data = encode(ledger_outstandings(company, ledger, to))
    if len(data) != size or hashlib.sha256(data).hexdigest() != digest:
        raise Failed("the template does not render r04's bytes ({} B, {})".format(len(data), hashlib.sha256(data).hexdigest()))


def write(folder, name, text):
    os.makedirs(folder, exist_ok=True)
    data = encode(text)
    with open(os.path.join(folder, name), "xb") as handle:
        handle.write(data)
    return "{} {} {}".format(name, len(data), hashlib.sha256(data).hexdigest())


def fixed(folder):
    return "\n".join([write(folder, "marks.utf16le.xml", MARKS.format(company=escape(COMPANY))),
                      write(folder, "ref-receivable.utf16le.xml", REF.format(company=escape(COMPANY), start=FROM, day=TO))])


def forbidden(code):
    return code <= 8 or code in (11, 12) or 14 <= code <= 31


def clean(text):
    return REFERENCE.sub(lambda m: "" if forbidden(int(m.group(1)[1:], 16) if m.group(1)[0] == "x" else int(m.group(1))) else m.group(0),
                         FORBIDDEN_RAW.sub("", text))


def events(path):
    parser = ET.XMLPullParser(events=("start", "end"))
    decoder = codecs.getincrementaldecoder("utf-16-le")()
    stack, carry, first, size = [], "", True, 0
    with open(path, "rb") as handle:
        while True:
            data = handle.read(CHUNK)
            size += len(data)
            final = not data
            if final and size == 0:
                raise Failed("the answer is empty")
            text = carry + decoder.decode(data, final=final)
            if first and text:
                text = text[1:] if text.startswith("﻿") else text
                first = False
            carry = ""
            if not final:
                amp = text.rfind("&", max(0, len(text) - 16))
                if amp >= 0 and ";" not in text[amp:]:
                    carry, text = text[amp:], text[:amp]
            try:
                parser.feed(clean(text))
                if final:
                    parser.close()
                ready = list(parser.read_events())
            except ET.ParseError as error:
                raise Failed("the answer does not parse: {}".format(error))
            for event, element in ready:
                if event == "start":
                    stack.append(element.tag)
                yield event, element, tuple(stack)
                if event == "end":
                    stack.pop()
            if final:
                break
    events.size = size


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
    for event, element, path_now in events(path):
        if event != "end" or len(path_now) < 2:
            if event == "end" and len(path_now) == 1 and element.tag != "ENVELOPE":
                raise Failed("the root is {}, not ENVELOPE".format(element.tag))
            continue
        if element.tag == "STATUS":
            raise Failed("a STATUS element is present: {!r}".format(element.text or ""))
        if len(path_now) == 3 and path_now[1] == "BILLFIXED":
            if element.tag not in fixed_fields:
                raise Failed("an unexpected element {} under BILLFIXED".format(element.tag))
            continue
        if len(path_now) != 2:
            raise Failed("an element nested deeper than the grammar allows: {}".format("/".join(path_now)))
        if element.tag == "BILLFIXED":
            fields = {}
            for child in element:
                if child.tag in fields:
                    raise Failed("a BILLFIXED with two {}".format(child.tag))
                fields[child.tag] = child.text or ""
            if set(fields) != set(fixed_fields):
                raise Failed("a BILLFIXED without its fields {}".format(",".join(fixed_fields)))
            current = {"fields": fields, "once": {}, "repeated": []}
            rows.append(current)
        elif element.tag in once:
            if current is None or element.tag in current["once"]:
                raise Failed("a {} without its BILLFIXED, or a second one".format(element.tag))
            current["once"][element.tag] = element.text or ""
        elif element.tag in repeated:
            if current is None:
                raise Failed("a {} before any BILLFIXED".format(element.tag))
            current["repeated"].append(element.tag)
        else:
            raise Failed("an unexpected element under ENVELOPE: {}".format(element.tag))
        element.clear()
    for row in rows:
        if set(row["once"]) != set(once):
            raise Failed("a bill without {}".format(",".join(sorted(set(once) - set(row["once"])))))
    return rows


def ref_rows(path):
    out = []
    for row in flat_rows(path, ("BILLDATE", "BILLREF", "BILLPARTY"), ("BILLCL", "BILLDUE", "BILLOVERDUE"), ()):
        date = day(row["fields"]["BILLDATE"])
        if date is None:
            raise Failed("a BILLDATE not read as a date: {!r}".format(row["fields"]["BILLDATE"]))
        out.append({"party": row["fields"]["BILLPARTY"], "ref": row["fields"]["BILLREF"], "date": date,
                    "amount": amount(row["once"]["BILLCL"], "a BILLCL")})
    return out


def voucher_lines(tags):
    """The number of voucher lines in one bill's repeated tags, or a Failed if they are not whole lines."""
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
        date = day(row["fields"]["BILLDATE"])
        if date is None:
            raise Failed("a BILLDATE not read as a date: {!r}".format(row["fields"]["BILLDATE"]))
        out.append({"ref": row["fields"]["BILLREF"], "date": date, "amount": amount(row["once"]["BILLCL"], "a BILLCL"),
                    "opening": amount(row["once"]["BILLOP"], "a BILLOP"), "on_account": row["fields"]["BILLREF"] == "On Account"
                    and row["once"]["BILLDUE"].strip() == "", "vouchers": voucher_lines(row["repeated"]),
                    "due": row["once"]["BILLDUE"].strip(), "overdue": row["once"]["BILLOVERDUE"].strip()})
    return out


def drcr(value):
    """Tally's screen form: a negative amount in these answers is a debit (r04 against its screen, 27 Oct)."""
    return "{} {}".format(abs(value), "Dr" if value < 0 else "Cr")


def screen(lo_path):
    """What step 4 ties to Tally's Ledger Outstandings screen: the total, the row count, the first and last five rows."""
    rows = lo_rows(lo_path)
    lines = ["screen rows {} total pending {}".format(len(rows), drcr(sum((r["amount"] for r in rows), Decimal(0))))]
    for at, row in enumerate(rows):
        if at < 5 or at >= len(rows) - 5:
            lines.append("row {} date {} ref {} opening {} pending {} due {} overdue {}".format(
                at + 1, row["date"], row["ref"], drcr(row["opening"]), drcr(row["amount"]), row["due"] or "-",
                row["overdue"] or "-"))
    return "\n".join(lines)


def admitted_name(name):
    return (len(name) <= NAME_MAX and NAME.match(name) is not None
            and not any(word.lower() in FORMULA_WORDS for word in name.split(" ")))


def select(ref_path, folder):
    check_template()
    rows = ref_rows(ref_path)
    names = sorted({row["party"] for row in rows}, key=lambda n: n.encode("utf-8"))
    admitted = [n for n in names if admitted_name(n)]
    if not admitted:
        raise Failed("no party name of the report is in the admitted alphabet ({} parties)".format(len(names)))
    party = admitted[0]
    bills = sum(1 for row in rows if row["party"] == party)
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "party.txt"), "x", encoding="utf-8") as handle:
        handle.write("{}\t{}\n".format(party, bills))
    return "\n".join(["parties {} in the report; {} in the alphabet; the pick holds {} bills there".format(
        len(names), len(admitted), bills), write(folder, "lo-p1.utf16le.xml", ledger_outstandings(COMPANY, party, TO))])


def summary(rows, label):
    total = sum((row["amount"] for row in rows), Decimal(0))
    return "{} ok bytes {} rows {} sum {}".format(label, events.size, len(rows), total)


def compare(ref_path, lo_path, party_path):
    with open(party_path, encoding="utf-8") as handle:
        party = handle.readline().rstrip("\n").split("\t")[0]
    ref = [r for r in ref_rows(ref_path) if r["party"] == party]
    out = [summary(ref, "ref (this party)")]
    lo = lo_rows(lo_path)
    out.append(summary(lo, "lo"))
    bills = [r for r in lo if not r["on_account"]]
    debit = [r for r in bills if r["amount"] < 0]
    credit = [r for r in bills if r["amount"] >= 0]
    on_account = [r for r in lo if r["on_account"]]
    out.append("lo bills {} debit {} credit {} (sum {}) on_account {} (sum {}) voucher_lines {}".format(
        len(bills), len(debit), len(credit), sum((r["amount"] for r in credit), Decimal(0)), len(on_account),
        sum((r["amount"] for r in on_account), Decimal(0)), sum(r["vouchers"] for r in lo)))
    left, right = {}, {}
    for r in ref:
        left.setdefault((r["ref"], r["date"]), []).append(r["amount"])
    for r in debit:
        right.setdefault((r["ref"], r["date"]), []).append(r["amount"])
    both = set(left) & set(right)
    equal = sum(1 for k in both if sorted(left[k]) == sorted(right[k]))
    repeated = sum(1 for v in list(left.values()) + list(right.values()) if len(v) > 1)
    out.append("key ref+date matched {} equal {} differ {} only_ref {} only_lo_debit {} repeated_keys {}".format(
        len(both), equal, len(both) - equal, len(set(left) - set(right)), len(set(right) - set(left)), repeated))
    return "\n".join(out)


def main(argv):
    if len(argv) == 3 and argv[0] == "fixed" and argv[1] == "--out":
        return fixed(argv[2])
    if len(argv) == 4 and argv[0] == "select" and argv[2] == "--out":
        return select(argv[1], argv[3])
    if len(argv) == 2 and argv[0] == "ref":
        return summary(ref_rows(argv[1]), "ref")
    if len(argv) == 2 and argv[0] == "lo":
        return summary(lo_rows(argv[1]), "lo")
    if len(argv) == 2 and argv[0] == "screen":
        return screen(argv[1])
    if len(argv) == 4 and argv[0] == "compare":
        return compare(*argv[1:])
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
