#!/usr/bin/env python3
"""Render the build's GUID-scoped ledger-rates request, or prove its pinned hash."""

import argparse
import datetime
import hashlib
import pathlib
import sys

PINNED = "96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812"
FETCH = (
    "NAME, GUID, REMOTEID, MASTERID, ALTERID, PARENT, PARTYGSTIN, INCOMETAXNUMBER, "
    "NAMEONPAN, LEDPINCODE, LEDGSTPINCODE, MSMEREGNUMBER, LEDUDYAMREGNUMBER, "
    "BANKACCHOLDERNAME, BANKDETAILS, IFSCODE, EMAIL, LEDGERPHONE, STATENAME, "
    "LEDADDRESS.LIST, TAXTYPE, GSTDUTYHEAD, OPENINGBALANCE, LEDGSTREGDETAILS.LIST, "
    "GSTDETAILS.LIST, RATEOFTAXCALCULATION, ROUNDINGMETHOD, ROUNDINGLIMIT, "
    "HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE"
)


def xml_text(value):
    # src-tauri/crates/bridge-tally-protocol/src/xml_text.rs:41-49.
    return (
        value.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
        .replace("'", "&apos;")
        .replace("\r", "&#13;")
        .replace("\n", "&#10;")
    )


def ledger_date(value):
    try:
        parsed = datetime.datetime.strptime(value, "%Y%m%d").date()
    except ValueError as error:
        raise argparse.ArgumentTypeError("want a calendar date as YYYYMMDD") from error
    if parsed.strftime("%Y%m%d") != value:
        raise argparse.ArgumentTypeError("want a calendar date as YYYYMMDD")
    return value


def ledger_guid(value):
    if not value:
        raise argparse.ArgumentTypeError("a ledger GUID is empty")
    if len(value) > 64:
        raise argparse.ArgumentTypeError("a ledger GUID is over 64 characters")
    if any(c not in "0123456789abcdefABCDEF-" for c in value):
        raise argparse.ArgumentTypeError("a ledger GUID admits only ASCII hexadecimal digits and hyphens")
    return value


def render_ledger_rates_request_for_ledgers(company, from_date, to_date, guids):
    # GUID de-duplication and ordering: ledger_guid_part.rs:95-131.
    ordered = sorted(set(guids))
    if not ordered:
        raise ValueError("at least one ledger GUID is required")
    if len(ordered) > 6:
        raise ValueError("at most six ledger GUIDs are admitted")
    formula = " OR ".join(f'$GUID = "{guid}"' for guid in ordered)
    system = f'<SYSTEM TYPE="Formulae" NAME="BridgeInvoiceLedgers">{formula}</SYSTEM>'
    static = (
        "<STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
        f"<SVCURRENTCOMPANY>{xml_text(company)}</SVCURRENTCOMPANY>"
        f'<SVFROMDATE TYPE="Date">{from_date}</SVFROMDATE>'
        f'<SVTODATE TYPE="Date">{to_date}</SVTODATE></STATICVARIABLES>'
    )
    # agent_import_invoice_wire.rs:1242-1258; native_outstandings/request.rs:235-254.
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST>"
        "<TYPE>Collection</TYPE><ID>List of Ledgers</ID></HEADER><BODY><DESC>"
        f"{static}<TDL><TDLMESSAGE>{system}"
        '<COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">'
        f"<FETCH>{FETCH}</FETCH>"
        "<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>"
        "<FILTERS>BridgeInvoiceLedgers</FILTERS></COLLECTION></TDLMESSAGE></TDL>"
        "</DESC></BODY></ENVELOPE>"
    )


def selftest():
    # Fixed inputs: agent_import_request_bytes_tests.rs:72-75, 291-294, 447-450, 477-482.
    request = render_ledger_rates_request_for_ledgers(
        "Synthetic & Co <Lab> \"Q\" Ünïcode", "20250401", "20260331", ["abc-b", "abc-a"]
    )
    # The pin hashes request.as_bytes(), the Rust string's UTF-8 bytes: lines 627-630.
    digest = hashlib.sha256(request.encode("utf-8")).hexdigest()
    print(digest)
    if digest != PINNED:
        print(f"NOT equal to the pinned hash {PINNED}")
        return 1
    print("equal to the pinned hash")
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--company")
    parser.add_argument("--from-date", type=ledger_date)
    parser.add_argument("--to-date", type=ledger_date)
    parser.add_argument("--output", type=pathlib.Path)
    parser.add_argument("guids", metavar="GUID", nargs="*", type=ledger_guid)
    args = parser.parse_args(argv)
    if args.selftest:
        return selftest()
    missing = [name for name in ("company", "from_date", "to_date") if getattr(args, name) is None]
    if missing or not args.guids:
        parser.error("company, both dates, and at least one GUID are required")
    try:
        request = render_ledger_rates_request_for_ledgers(
            args.company, args.from_date, args.to_date, args.guids
        )
    except ValueError as error:
        parser.error(str(error))
    if args.output is None:
        sys.stdout.write(request)
        return 0
    # Same body as text_encoding.rs:40-48: BOM-prefixed UTF-16LE, with no added newline.
    body = b"\xff\xfe" + request.encode("utf-16-le")
    args.output.write_bytes(body)
    print(f"{args.output}: {len(body)} bytes, sha256 {hashlib.sha256(body).hexdigest()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
