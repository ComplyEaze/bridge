#!/usr/bin/env python3
"""Row 2119-L's requests, its party pick and its bill-for-bill comparison, for BRIDGE CORPUS DENSE at 31 March 2026.

  python -I bills.py fixed --out <dir>
      Writes the three fixed requests, UTF-16LE with a byte-order mark, the way the transport sends them:
        marks.utf16le.xml        the company's GUID, ALTVCHID and ALTMSTID (the marks read of earlier rows)
        ref-receivable.utf16le.xml
                                 master's Bills Receivable request (`render_native_bills_request`,
                                 native_outstandings/request.rs:131-144) from 20250401 to 20260331: the request
                                 runtime.rs:4141-4147 sends. Byte for byte the 770-byte r-4 of lab/2119-bills-read-timing.
        ledgers.utf16le.xml      master's ledger export (`render_native_ledger_export_request`, request.rs:258-269 and
                                 :387-407): NAME, GUID, ..., PARENT, for the same period
  python -I bills.py select <ref answer> <ledgers answer> --out <dir>
      Picks the parties: the distinct BILLPARTY names of the Receivable answer, ordered by their UTF-8 bytes; the first
      1, 10 and 100 of them. Each name must be the NAME of exactly one ledger in the ledger answer, whose GUID is then
      admitted as `LedgerGuid::parse` admits it (ledger_guid_part.rs:62-73: 1 to 64 characters, ASCII hex digits and
      the hyphen), else the pick fails. Writes parties.txt (count, name, GUID) and the four candidate requests:
        cand-p1.utf16le.xml, cand-p10.utf16le.xml, cand-p100.utf16le.xml   filtered to those parties' GUIDs
        cand-all.utf16le.xml                                               no filter
      Fewer than 100 parties: cand-p100 is not written, and the line says so.
  python -I bills.py ref <answer>
  python -I bills.py cand <answer>
      Parses one answer by LAB-COMMON C4 and prints its counts and sums.
  python -I bills.py compare <ref answer> <cand answer> <parties.txt> <p1|p10|p100|all>
      Bill for bill: the candidate's rows against the Receivable rows of the same parties (all parties for `all`).

The candidate (row L asks whether it ties to the report). It is not a builder of the product, and nothing in ComplyEaze
Bridge sends it. Its names come from Tally's documentation, never from trying names:
  - the object type `Bill` and the attribute `Cleared: No` (the developer reference's FAQ "a collection of bills
    where both cleared and uncleared bills should be gathered");
  - `$GUID:Ledger:$Parent`, the method-of-another-object form master already sends as
    `$GUID:Company:##SVCurrentCompany` (request.rs:203).
The GUID filter is `ledger_guid_filter_parts`'s (request.rs:225-245) with each term on the bill's ledger, not on the
row's own GUID. The only values from Tally in a request are GUIDs, through the check above; the company, dates, type,
fetch list and formula are fixed here.

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
GUID_PREFIX = "d45bc1b0-"
FROM = "20250401"
TO = "20260331"
PICKS = (1, 10, 100)
FORMULA_NAME = "BridgeCandidateBillLedgers"
CANDIDATE = "Bridge Candidate Open Bills"

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
LEDGERS = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>List of Ledgers</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE></STATICVARIABLES>'
    '<TDL><TDLMESSAGE><COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">'
    "<FETCH>NAME, GUID, REMOTEID, MASTERID, ALTERID, PARENT, PARTYGSTIN, OPENINGBALANCE</FETCH>"
    "</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
CAND = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>{name}</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE></STATICVARIABLES>'
    '<TDL><TDLMESSAGE>{system}<COLLECTION NAME="{name}" ISMODIFY="No"><TYPE>Bill</TYPE><CLEARED>No</CLEARED>'
    "<FETCH>NAME, PARENT, BILLDATE, CLOSINGBALANCE</FETCH><COMPUTE>BRIDGELEDGERGUID:$GUID:Ledger:$Parent</COMPUTE>"
    "{filters}</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
)
GUID = re.compile(r"\A[0-9A-Fa-f-]{1,64}\Z")
MONTHS = {m: i for i, m in enumerate(("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"), 1)}
CHUNK = 1 << 20
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")


class Failed(Exception):
    pass


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def escape(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def candidate(guids):
    if guids is None:
        system = filters = ""
    else:
        if not guids:
            raise Failed("a filtered candidate needs at least one GUID")
        for guid in guids:
            if not GUID.match(guid):
                raise Failed("a GUID outside the admitted alphabet or length: {!r}".format(guid))
        terms = " OR ".join('$GUID:Ledger:$Parent = "{}"'.format(guid) for guid in sorted(set(guids)))
        system = '<SYSTEM TYPE="Formulae" NAME="{}">{}</SYSTEM>'.format(FORMULA_NAME, escape(terms))
        filters = "<FILTERS>{}</FILTERS>".format(FORMULA_NAME)
    return CAND.format(name=CANDIDATE, company=escape(COMPANY), start=FROM, day=TO, system=system, filters=filters)


def write(folder, name, text):
    os.makedirs(folder, exist_ok=True)
    data = encode(text)
    path = os.path.join(folder, name)
    with open(path, "xb") as handle:
        handle.write(data)
    return "{} {} {}".format(name, len(data), hashlib.sha256(data).hexdigest())


def fixed(folder):
    lines = [write(folder, "marks.utf16le.xml", MARKS.format(company=escape(COMPANY))),
             write(folder, "ref-receivable.utf16le.xml", REF.format(company=escape(COMPANY), start=FROM, day=TO)),
             write(folder, "ledgers.utf16le.xml", LEDGERS.format(company=escape(COMPANY), start=FROM, day=TO))]
    return "\n".join(lines)


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
    """YYYYMMDD from Tally's `1-Jul-26` (the report) or `20260701` (a collection); None if neither."""
    text = text.strip()
    if re.fullmatch(r"\d{8}", text):
        return text
    match = re.fullmatch(r"(\d{1,2})-([A-Z][a-z]{2})-(\d{2}|\d{4})", text)
    if not match or match.group(2) not in MONTHS:
        return None
    year = match.group(3) if len(match.group(3)) == 4 else "20" + match.group(3)
    return "{}{:02d}{:02d}".format(year, MONTHS[match.group(2)], int(match.group(1)))


def amount(text, what):
    try:
        return Decimal(text.strip())
    except InvalidOperation:
        raise Failed("{} is not a number: {!r}".format(what, text))


def ref_rows(path):
    """The flat report: BILLFIXED (BILLDATE, BILLREF, BILLPARTY) then BILLCL, BILLDUE, BILLOVERDUE under ENVELOPE."""
    rows, current, counts = [], None, {"BILLFIXED": 0, "BILLCL": 0, "BILLDUE": 0, "BILLOVERDUE": 0}
    for event, element, path_now in events(path):
        if event != "end" or len(path_now) < 2:
            continue
        if element.tag == "STATUS":
            raise Failed("a STATUS element is present: {!r}".format(element.text or ""))
        if len(path_now) == 3 and path_now[1] == "BILLFIXED":
            if element.tag not in ("BILLDATE", "BILLREF", "BILLPARTY"):
                raise Failed("an unexpected element {} under BILLFIXED".format(element.tag))
            continue
        if len(path_now) != 2:
            raise Failed("an element nested deeper than the grammar allows: {}".format("/".join(path_now)))
        if element.tag not in counts:
            raise Failed("an unexpected element under ENVELOPE: {}".format(element.tag))
        counts[element.tag] += 1
        if element.tag == "BILLFIXED":
            fields = {child.tag: child.text or "" for child in element}
            if set(fields) != {"BILLDATE", "BILLREF", "BILLPARTY"}:
                raise Failed("a BILLFIXED without its three fields")
            current = {"party": fields["BILLPARTY"], "ref": fields["BILLREF"], "date": day(fields["BILLDATE"])}
            if current["date"] is None:
                raise Failed("a BILLDATE not read as a date: {!r}".format(fields["BILLDATE"]))
            rows.append(current)
        elif element.tag == "BILLCL":
            if current is None or "amount" in current:
                raise Failed("a BILLCL without its BILLFIXED")
            current["amount"] = amount(element.text or "", "a BILLCL")
        element.clear()
    if len(set(counts.values())) != 1:
        raise Failed("the four counts differ: {}".format(counts))
    return rows


def cand_rows(path):
    """Rows are the children of ENVELOPE/BODY/DATA/COLLECTION; each must be a BILL with the four fields fetched."""
    rows, kinds = [], {}
    for event, element, path_now in events(path):
        if event != "end":
            continue
        if element.tag in ("STATUS", "LINEERROR"):
            raise Failed("{} present: {!r}".format(element.tag, element.text or ""))
        if path_now[:4] != ("ENVELOPE", "BODY", "DATA", "COLLECTION") or len(path_now) != 5:
            continue
        kinds[element.tag] = kinds.get(element.tag, 0) + 1
        if element.tag != "BILL":
            raise Failed("a row that is not a BILL: {}".format(element.tag))
        name = element.get("NAME")
        fields = {}
        for child in element:
            fields.setdefault(child.tag, []).append(child)
        if name is None and "NAME" in fields:
            name = fields["NAME"][0].text or ""
        if name is None and "NAME.LIST" in fields:
            inner = fields["NAME.LIST"][0].find("NAME")
            name = inner.text if inner is not None and inner.text is not None else None
        missing = [tag for tag in ("PARENT", "BILLDATE", "CLOSINGBALANCE", "BRIDGELEDGERGUID") if tag not in fields]
        if name is None or missing:
            raise Failed("a BILL without {}; its children are {}".format(
                ",".join((["NAME"] if name is None else []) + missing), ",".join(sorted(fields))))
        for tag in ("PARENT", "BILLDATE", "CLOSINGBALANCE", "BRIDGELEDGERGUID"):
            if len(fields[tag]) != 1:
                raise Failed("a BILL with {} {} elements".format(len(fields[tag]), tag))
        raw_date = fields["BILLDATE"][0].text or ""
        rows.append({"party": fields["PARENT"][0].text or "", "ref": name, "date": day(raw_date), "raw_date": raw_date,
                     "amount": amount(fields["CLOSINGBALANCE"][0].text or "", "a CLOSINGBALANCE"),
                     "ledger_guid": (fields["BRIDGELEDGERGUID"][0].text or "").strip()})
        element.clear()
    return rows


def ledger_rows(path):
    rows = []
    for event, element, path_now in events(path):
        if event != "end" or path_now[:4] != ("ENVELOPE", "BODY", "DATA", "COLLECTION") or len(path_now) != 5:
            if event == "end" and element.tag in ("STATUS", "LINEERROR"):
                raise Failed("{} present: {!r}".format(element.tag, element.text or ""))
            continue
        if element.tag != "LEDGER":
            raise Failed("a row that is not a LEDGER: {}".format(element.tag))
        guid = element.find("GUID")
        name = element.get("NAME")
        if name is None or guid is None or guid.text is None:
            raise Failed("a LEDGER without its NAME attribute or GUID")
        rows.append((name, guid.text.strip()))
        element.clear()
    return rows


def select(ref_path, ledgers_path, folder):
    names = sorted({row["party"] for row in ref_rows(ref_path)}, key=lambda n: n.encode("utf-8"))
    by_name = {}
    for name, guid in ledger_rows(ledgers_path):
        by_name.setdefault(name, []).append(guid)
    lines, chosen = [], {}
    for name in names[:max(PICKS)]:
        found = by_name.get(name, [])
        if len(found) != 1:
            raise Failed("a party is the NAME of {} ledgers, not one".format(len(found)))
        if not GUID.match(found[0]) or not found[0].startswith(GUID_PREFIX):
            raise Failed("a party's GUID is outside the admitted alphabet or another company's")
        chosen[name] = found[0]
    with open(os.path.join(folder, "parties.txt"), "x", encoding="utf-8") as handle:
        for index, name in enumerate(names[:max(PICKS)], 1):
            handle.write("{}\t{}\t{}\n".format(index, name, chosen[name]))
    lines.append("parties {} distinct in the report; picked {}".format(len(names), min(len(names), max(PICKS))))
    for count in PICKS:
        if len(names) < count:
            lines.append("cand-p{} not written: {} parties".format(count, len(names)))
            continue
        lines.append(write(folder, "cand-p{}.utf16le.xml".format(count), candidate([chosen[n] for n in names[:count]])))
    lines.append(write(folder, "cand-all.utf16le.xml", candidate(None)))
    return "\n".join(lines)


def summary(rows, label):
    total = sum((row["amount"] for row in rows), Decimal(0))
    return "{} ok bytes {} rows {} parties {} sum {}".format(label, events.size, len(rows), len({r["party"] for r in rows}), total)


def compare(ref_path, cand_path, parties_path, which):
    with open(parties_path, encoding="utf-8") as handle:
        parties = [line.rstrip("\n").split("\t") for line in handle if line.strip()]
    if which == "all":
        names, guids = None, None
    else:
        count = int(which[1:])
        names = {p[1] for p in parties[:count]}
        guids = {p[2].lower() for p in parties[:count]}
        if len(names) != count:
            raise Failed("parties.txt holds {} of the {} parties".format(len(names), count))
    ref = [r for r in ref_rows(ref_path) if names is None or r["party"] in names]
    out = [summary(ref, "ref")]
    cand = cand_rows(cand_path)
    out.append(summary(cand, "cand"))
    undated = [r for r in cand if r["date"] is None]
    if undated:
        out.append("cand dates not read as a date: {} rows, first {!r}".format(len(undated), undated[0]["raw_date"]))
    if guids is not None:
        outside = [r for r in cand if r["ledger_guid"].lower() not in guids]
        out.append("cand rows whose ledger GUID is not one asked for: {}".format(len(outside)))
    def key(r, dated):
        return (r["party"], r["ref"], r["date"]) if dated else (r["party"], r["ref"])
    for dated in (True, False):
        left, right = {}, {}
        for r in ref:
            left.setdefault(key(r, dated), []).append(r["amount"])
        for r in cand:
            right.setdefault(key(r, dated), []).append(r["amount"])
        repeated = sum(1 for v in list(left.values()) + list(right.values()) if len(v) > 1)
        both = set(left) & set(right)
        equal = sum(1 for k in both if sorted(left[k]) == sorted(right[k]))
        opposite = sum(1 for k in both if sorted(left[k]) == sorted(-a for a in right[k]) and sorted(left[k]) != sorted(right[k]))
        out.append("key {} matched {} equal {} opposite_sign {} differ {} only_ref {} only_cand {} repeated_keys {}".format(
            "party+ref+date" if dated else "party+ref", len(both), equal, opposite, len(both) - equal - opposite,
            len(set(left) - set(right)), len(set(right) - set(left)), repeated))
    return "\n".join(out)


def main(argv):
    if len(argv) == 3 and argv[0] == "fixed" and argv[1] == "--out":
        return fixed(argv[2])
    if len(argv) == 5 and argv[0] == "select" and argv[3] == "--out":
        return select(argv[1], argv[2], argv[4])
    if len(argv) == 2 and argv[0] == "ref":
        return summary(ref_rows(argv[1]), "ref")
    if len(argv) == 2 and argv[0] == "cand":
        return summary(cand_rows(argv[1]), "cand")
    if len(argv) == 5 and argv[0] == "compare" and argv[4] in ("p1", "p10", "p100", "all"):
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
