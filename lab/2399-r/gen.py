#!/usr/bin/env python3
# gen.py I-0105.xml N-0105.xml: writes this row's three request files from row 2464-RS's two (I-0105.xml,
# sha256 b333331a..., N-0105.xml, sha256 588f998d...), by replacing exact strings and nothing else, and writes
# the ledger import and the ledger read by the same plain templates. Every replacement is counted and must
# occur exactly as many times as stated, else it stops. Output is UTF-16LE with a BOM, as the lab sends.
# Run: python3 -I gen.py <RS's I-0105.xml> <RS's N-0105.xml>   (then check SHA256.txt)
import pathlib, sys
here = pathlib.Path(__file__).resolve().parent
def load(path):
    raw = pathlib.Path(path).read_bytes()
    if raw[:2] != b"\xff\xfe":
        raise SystemExit(f"{path}: no UTF-16LE byte-order mark")
    return raw[2:].decode("utf-16-le")
def swap(text, old, new, times):
    if text.count(old) != times:
        raise SystemExit(f"{old!r} occurs {text.count(old)} time(s), expected {times}: stop")
    return text.replace(old, new)
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
NUMBER_OLD, NUMBER_NEW = "BP/26-27/0105", "BP/26-27/0399"
REMOTE_OLD, REMOTE_NEW = "00000000-0000-4000-8000-000000001105", "00000000-0000-4000-8000-000000002399"
PROBE = "BRIDGE Census Probe 2399"
i = load(sys.argv[1])
i = swap(i, NUMBER_OLD, NUMBER_NEW, 2)   # REFERENCE and VOUCHERNUMBER
i = swap(i, REMOTE_OLD, REMOTE_NEW, 1)
save("I-0399.xml", i)
n = load(sys.argv[2])
n = swap(n, NUMBER_OLD, NUMBER_NEW, 1)
save("N-0399.xml", n)
# A ledger read by name: the same collection form as N-0399, filtered on the ledger's name.
l = load(sys.argv[2])
l = swap(l, "Bridge Invoice Number", "Bridge Ledger Probe", 2)
l = swap(l, "BridgeInvoiceNumber", "BridgeLedgerProbe", 2)
l = swap(l, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
l = swap(l, "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH>", "<FETCH>NAME, PARENT</FETCH>", 1)
l = swap(l, "<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE>", "", 1)
l = swap(l, '$Date &gt;= $$Date:"20260401" AND $Date &lt;= $$Date:"20270331" AND $VoucherNumber = "BP/26-27/0105"',
         f'$Name = "{PROBE}"', 1)
save("L-probe.xml", l)
# The master import: one plain ledger under a primary group, no GST detail.
m = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY>'
     '<IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>BRIDGE PILOT LAB'
     '</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF">'
     f'<LEDGER NAME="{PROBE}" ACTION="Create"><NAME.LIST><NAME>{PROBE}</NAME></NAME.LIST><PARENT>Indirect Expenses</PARENT>'
     '</LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>')
save("I-L2399.xml", m)
print("wrote I-0399.xml N-0399.xml L-probe.xml I-L2399.xml")
