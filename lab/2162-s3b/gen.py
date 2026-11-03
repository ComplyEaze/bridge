#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   L-ledgers.xml  four ledger Creates, the shape LedgerCreate::render_message renders, in the All Masters envelope
#                  of render_import_envelope: three customers under Sundry Debtors with ISBILLWISEON Yes, and
#                  BRIDGE Seed Income under Indirect Incomes with ISBILLWISEON No. Setup only.
#   I-<tail>.xml   for each sub-row: the request that created BP/26-27/0010 (the build's native_post_request), with
#                  the number in VOUCHERNUMBER and REFERENCE, a new REMOTEID, the sub-row's customer in its seven places,
#                  and the party leg carrying the New Ref that render_sales_invoice_xml writes for a bill-wise party.
#   R-<tail>.xml   the read-back at 094c3edc (S1's R), number changed.
# M-marks.xml, B-bills.xml and V-days.xml are not generated: they are copied unchanged (ROW.md says from where).
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
PARTY_OLD = "BRIDGE Walk-in"
# (sub-row, number, customer, last digits of the REMOTEID; 0777 is 0010's, 0778 S2's, 0779 S3a's)
ROWS = [("b1", "BP/26-27/0048", "BRIDGE Walk-in BJ", "0780"),
        ("b2", "BP/26-27/0049", "BRIDGE Walk-in BS", "0781"),
        ("b3", "BP/26-27/0050", "BRIDGE Walk-in BO", "0782")]
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
def ledger(name, parent, billwise):
    return (f'<TALLYMESSAGE><LEDGER NAME="{name}" ACTION="Create"><NAME>{name}</NAME>'
        f'<PARENT>{parent}</PARENT><ISBILLWISEON>{"Yes" if billwise else "No"}</ISBILLWISEON></LEDGER></TALLYMESSAGE>')
messages = "".join(ledger(party, "Sundry Debtors", True) for _, _, party, _ in ROWS)
messages += ledger("BRIDGE Seed Income", "Indirect Incomes", False)
save("L-ledgers.xml", envelope.format(report="All Masters", company=company, messages=messages))
template_i = load("template-import-0010.utf16le.xml")
template_r = load("template-r-0010.utf16le.xml")
for _, number, party, digits in ROWS:
    tail = number.rsplit("/", 1)[-1]
    i = template_i
    assert i.count(">BP/26-27/0010<") == 2
    i = i.replace(">BP/26-27/0010<", f">{number}<")
    old_id = 'REMOTEID="00000000-0000-4000-8000-000000000777"'
    assert i.count(old_id) == 1
    i = i.replace(old_id, old_id.replace('0777"', f'{digits}"'))
    assert i.count(f">{PARTY_OLD}<") == 7 and party not in i
    i = i.replace(f">{PARTY_OLD}<", f">{party}<")
    leg = f"<LEDGERNAME>{party}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-1051.00</AMOUNT>"
    assert i.count(leg + "</LEDGERENTRIES.LIST>") == 1
    i = i.replace(leg + "</LEDGERENTRIES.LIST>", leg + f"<BILLALLOCATIONS.LIST><NAME>{number}</NAME>"
        "<BILLTYPE>New Ref</BILLTYPE><AMOUNT>-1051.00</AMOUNT></BILLALLOCATIONS.LIST></LEDGERENTRIES.LIST>")
    save(f"I-{tail}.xml", i)
    assert template_r.count('"BP/26-27/0010"') == 1
    save(f"R-{tail}.xml", template_r.replace('"BP/26-27/0010"', f'"{number}"'))
