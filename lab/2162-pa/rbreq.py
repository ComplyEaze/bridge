#!/usr/bin/env python3
"""Render the build's invoice read-back request for one saved batch, or prove its pinned hash.

Row 2162-PA, new, in the shape of row 2162-RC's ratereq.py; not product code. The build keeps no bytes of the
read-back it sends after a post (its evidence folds request hashes together: src-tauri/src/agent.rs:4136-4154), so
this renders the same request from the same inputs: the company's name, the voucher type GUID the build recorded
for the invoice, the invoice number, and the financial year of the invoice date.

  rbreq.py --selftest
      the request for the code's own fixed inputs; its sha256 must equal the code's pin.
  rbreq.py --company NAME --book GUID --record DATADIR --build ANSWER --number NUMBER --date YYYYMMDD --output FILE
      reads the batch id from the build answer ANSWER, finds that batch's record in DATADIR/agent-import-ledger.jsonl
      (parsed as JSON, never searched as text), and takes the one voucher's recorded voucher type GUID, number and
      date. The GUID must be the book's GUID followed by -000000ce (the BRIDGE Sales type in two lab captures), and
      the number and date must be the ones given; otherwise it writes nothing and exits 1 (a finding, not a failed
      read). Writes FILE as BOM-prefixed UTF-16LE with no added newline. Prints no GUID.

Exit 0: done (or the pin matched). Exit 1: a finding (the pin differs, or the record differs from what is given).
Exit 2: a failed read (a file missing or unreadable, no record or more than one, an input of the wrong shape).
"""

import argparse
import datetime
import hashlib
import json
import pathlib
import re
import sys

PINNED = "aa8ebaaf2d0f229bea484c67eb68135afc98c77fe587d9fb2a16a4172cb479cc"
FETCH = (
    "DATE, VOUCHERNUMBER, VOUCHERTYPENAME, REFERENCE, REFERENCEDATE, PARTYLEDGERNAME, PARTYGSTIN, STATENAME, "
    "PLACEOFSUPPLY, GSTREGISTRATIONTYPE, ISINVOICE, ISCANCELLED, ISOPTIONAL, GUID, ALTERID, ALLLEDGERENTRIES.*, "
    "VCHGSTSTATUSISINCLUDED, VCHGSTSTATUSISUNCERTAIN, VCHGSTSTATUSISEXCLUDED, ISGSTOVERRIDDEN, ISPOSTDATED, "
    "GSTREGISTRATION, CMPGSTIN"
)
SALES_TYPE_SUFFIX = "-000000ce"
GUID = re.compile(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")


class FailedRead(Exception):
    pass


def xml_text(value):
    # src-tauri/crates/bridge-tally-protocol/src/xml_text.rs:41-49 (as ratereq.py).
    return (
        value.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
        .replace("'", "&apos;")
        .replace("\r", "&#13;")
        .replace("\n", "&#10;")
    )


def financial_year_window(date):
    # agent_import_invoice.rs:2232-2241: 1 April to 31 March of the year the date falls in.
    year, month = int(date[:4]), int(date[4:6])
    start = year if month >= 4 else year - 1
    return f"{start}0401", f"{start + 1}0331"


def render_invoice_readback_request(company, type_guid, number, from_date, to_date):
    # agent_import_invoice_wire.rs:750-765. The static variables in the order of
    # static_variables_for_unvalidated_company (xml_read_profiles.rs:513-519), as ratereq.py renders them.
    static = (
        "<STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
        f"<SVCURRENTCOMPANY>{xml_text(company)}</SVCURRENTCOMPANY>"
        f'<SVFROMDATE TYPE="Date">{from_date}</SVFROMDATE><SVTODATE TYPE="Date">{to_date}</SVTODATE>'
        "</STATICVARIABLES>"
    )
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>Bridge Invoice Readback</ID></HEADER><BODY><DESC>"
        f"{static}<TDL><TDLMESSAGE>"
        f'<SYSTEM TYPE="Formulae" NAME="BridgeInvoiceReadback">$Date &gt;= $$Date:"{from_date}" AND '
        f'$Date &lt;= $$Date:"{to_date}" AND $VoucherNumber = "{number}" AND '
        f'$GUID:VoucherType:$VoucherTypeName = "{type_guid}"</SYSTEM>'
        '<COLLECTION NAME="Bridge Invoice Readback" ISMODIFY="No"><TYPE>Voucher</TYPE>'
        f"<FETCH>{FETCH}</FETCH><FILTERS>BridgeInvoiceReadback</FILTERS></COLLECTION></TDLMESSAGE></TDL>"
        "</DESC></BODY></ENVELOPE>"
    )


def selftest():
    # Fixed inputs: agent_import_request_bytes_tests.rs:74 (COMPANY), 292-294 (year), 444-446 (number and type
    # GUID; sentinel_uuid(0x5) is agent_import_tests.rs:2552-2554), 463-471 (the call). Pin: lines 182-183.
    type_guid = "00000000-0000-4000-8000-000000000005-00000193"
    request = render_invoice_readback_request(
        "Synthetic & Co <Lab> \"Q\" Ünïcode", type_guid, "INV/26-27/0042", "20250401", "20260331"
    )
    # The pin hashes request.as_bytes(), the Rust string's UTF-8 bytes: line 629.
    digest = hashlib.sha256(request.encode("utf-8")).hexdigest()
    print(digest)
    if digest != PINNED:
        print(f"NOT equal to the pinned hash {PINNED}")
        return 1
    print("equal to the pinned hash")
    return 0


def batch_id_of(build):
    try:
        body = json.loads(pathlib.Path(build).read_text(encoding="utf-8"))
        sc = body["response"]["result"]["structuredContent"]
        result = sc["result"]
    except (OSError, ValueError, KeyError, TypeError) as error:
        raise FailedRead(f"{build}: not a saved answer with a result ({type(error).__name__})")
    if body.get("tool") != "build_import_xml" or not isinstance(result, dict):
        raise FailedRead(f"{build}: not a build_import_xml answer")
    if result.get("voucher_count") != 1 or not isinstance(result.get("batch_id"), str):
        raise FailedRead(f"{build}: not a build of exactly one voucher")
    return result["batch_id"]


def batches(value, batch_id):
    # Every JSON object in a record that names this batch and holds its vouchers, wherever the record nests it.
    if isinstance(value, dict):
        if value.get("batch_id") == batch_id and isinstance(value.get("vouchers"), list):
            yield value
        for child in value.values():
            yield from batches(child, batch_id)
    elif isinstance(value, list):
        for child in value:
            yield from batches(child, batch_id)


def recorded(datadir, batch_id):
    path = pathlib.Path(datadir) / "agent-import-ledger.jsonl"
    try:
        lines = path.read_bytes().decode("utf-8").splitlines()
    except (OSError, UnicodeDecodeError) as error:
        raise FailedRead(f"{path}: unreadable ({type(error).__name__})")
    seen = set()
    for number, line in enumerate(lines, 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except ValueError:
            raise FailedRead(f"{path}: line {number} is not JSON")
        for batch in batches(record, batch_id):
            vouchers = batch["vouchers"]
            if len(vouchers) != 1 or not isinstance(vouchers[0], dict):
                raise FailedRead(f"line {number}: the batch does not hold exactly one voucher")
            voucher = vouchers[0]
            observed = ((voucher.get("invoice") or {}).get("observed") or {})
            values = (observed.get("voucher_type_guid"), voucher.get("voucher_number"), voucher.get("date"))
            if not all(isinstance(v, str) for v in values):
                raise FailedRead(f"line {number}: the voucher has no recorded type GUID, number or date")
            seen.add(values)
    if len(seen) != 1:
        raise FailedRead(f"{len(seen)} distinct records of the batch's voucher (want exactly one)")
    print(f"record: {len(lines)} journal lines read; the batch's voucher recorded once (or identically)")
    return seen.pop()


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--selftest", action="store_true")
    for name in ("--company", "--book", "--record", "--build", "--number", "--date", "--output"):
        parser.add_argument(name)
    args = parser.parse_args(argv)
    if args.selftest:
        return selftest()
    try:
        if None in (args.company, args.book, args.record, args.build, args.number, args.date, args.output):
            raise FailedRead("every option but --selftest is required")
        if not GUID.fullmatch(args.book or ""):
            raise FailedRead("--book is not a lower-case GUID")
        if not re.fullmatch(r"[A-Za-z0-9/-]{1,16}", args.number):
            raise FailedRead("--number admits 1 to 16 letters, digits, / and - (InvoiceNumber::parse, agent_import_invoice.rs:661-668)")
        try:
            if datetime.datetime.strptime(args.date, "%Y%m%d").strftime("%Y%m%d") != args.date:
                raise ValueError
        except ValueError:
            raise FailedRead("--date is not a calendar date as YYYYMMDD")
        out = pathlib.Path(args.output)
        if out.exists():
            raise FailedRead(f"{out} exists: never overwritten")
        type_guid, number, date = recorded(args.record, batch_id_of(args.build))
    except FailedRead as error:
        print(f"FAILED READ: {error}")
        return 2
    findings = []
    if type_guid != args.book + SALES_TYPE_SUFFIX:
        findings.append("the recorded voucher type GUID is not the book's GUID followed by " + SALES_TYPE_SUFFIX)
    if number != args.number:
        findings.append(f"the recorded number {number!r} is not {args.number!r}")
    if date != args.date:
        findings.append(f"the recorded date {date!r} is not {args.date}")
    if findings:
        for finding in findings:
            print("FINDING:", finding)
        print("nothing written")
        return 1
    print(f"type GUID: the book's GUID followed by {SALES_TYPE_SUFFIX}: yes; number {number}; date {date}")
    from_date, to_date = financial_year_window(date)
    print(f"window {from_date} to {to_date}; company {args.company!r}")
    request = render_invoice_readback_request(args.company, type_guid, number, from_date, to_date)
    # text_encoding.rs:40-48: BOM-prefixed UTF-16LE, with no added newline (as ratereq.py).
    body = b"\xff\xfe" + request.encode("utf-16-le")
    out.write_bytes(body)
    print(f"{out.name}: {len(body)} bytes, sha256 {hashlib.sha256(body).hexdigest()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
