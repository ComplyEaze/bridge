#!/usr/bin/env python3
# gen.py NUMBER: writes beside this file, UTF-16LE with a BOM, as sent:
#   L-ledger.xml  one ledger Create, the shape LedgerCreate::render_message renders, in the All Masters envelope
#                 of render_import_envelope: BRIDGE Walk-in BW, Sundry Debtors, ISBILLWISEON Yes. Setup only.
#   I-<tail>.xml  the request that created BP/26-27/0010 (the build's native_post_request), with the number in
#                 VOUCHERNUMBER and REFERENCE, a new REMOTEID, the party BRIDGE Walk-in BW in its seven places,
#                 and the party leg carrying the New Ref that render_sales_invoice_xml writes for a bill-wise party.
#   R-<tail>.xml  the read-back at 094c3edc (S1's R), number changed.
#   B-bills.xml   Bills Receivable as render_native_bills_request renders it, 1 Apr 2026 to 1 Aug 2026.
# Example: python3 -I gen.py BP/26-27/0047
import pathlib, sys
number = sys.argv[1]
here = pathlib.Path(__file__).resolve().parent
tail = number.rsplit("/", 1)[-1]
PARTY_OLD, PARTY_NEW = "BRIDGE Walk-in", "BRIDGE Walk-in BW"
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
company = "BRIDGE PILOT LAB"
envelope = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER>'
    '<BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>{report}</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{company}'
    '</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{messages}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>')
ledger = (f'<TALLYMESSAGE><LEDGER NAME="{PARTY_NEW}" ACTION="Create"><NAME>{PARTY_NEW}</NAME>'
    '<PARENT>Sundry Debtors</PARENT><ISBILLWISEON>Yes</ISBILLWISEON></LEDGER></TALLYMESSAGE>')
save("L-ledger.xml", envelope.format(report="All Masters", company=company, messages=ledger))
i = load("template-import-0010.utf16le.xml")
assert i.count(">BP/26-27/0010<") == 2
i = i.replace(">BP/26-27/0010<", f">{number}<")
old_id = 'REMOTEID="00000000-0000-4000-8000-000000000777"'
assert i.count(old_id) == 1
i = i.replace(old_id, old_id.replace('0777"', '0779"'))  # a new REMOTEID: the 0010 one is in the book
assert i.count(f">{PARTY_OLD}<") == 7 and PARTY_NEW not in i
i = i.replace(f">{PARTY_OLD}<", f">{PARTY_NEW}<")
leg = f"<LEDGERNAME>{PARTY_NEW}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-1051.00</AMOUNT>"
assert i.count(leg + "</LEDGERENTRIES.LIST>") == 1
i = i.replace(leg + "</LEDGERENTRIES.LIST>", leg + f"<BILLALLOCATIONS.LIST><NAME>{number}</NAME><BILLTYPE>New Ref</BILLTYPE>"
    "<AMOUNT>-1051.00</AMOUNT></BILLALLOCATIONS.LIST></LEDGERENTRIES.LIST>")
save(f"I-{tail}.xml", i)
r = load("template-r-0010.utf16le.xml")
assert r.count('"BP/26-27/0010"') == 1
save(f"R-{tail}.xml", r.replace('"BP/26-27/0010"', f'"{number}"'))
bills = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Bills Receivable</ID>'
    '</HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>BRIDGE PILOT LAB'
    '</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">20260401</SVFROMDATE><SVTODATE TYPE="Date">20260801</SVTODATE>'
    '</STATICVARIABLES></DESC></BODY></ENVELOPE>')
save("B-bills.xml", bills)
