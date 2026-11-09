#!/usr/bin/env python3
# gen9.py: writes the requests of row 2162-KC beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2162-RS's and row 2162-KD's (ROW.md names each), and changes only what each line
# below names. Every change is asserted to occur exactly as often as expected.
#   I-0062.xml    the control: template-import-0010 (the build's request that created BP/26-27/0010) with the number
#                 BP/26-27/0062 (VOUCHERNUMBER and REFERENCE) and the REMOTEID tail 0811. Party, figures, date and
#                 type (BRIDGE Sales) unchanged.
#   I-0063.xml    the second invoice: the same with BP/26-27/0063 and the tail 0812.
#   I-0064K.xml   a number held by another Sales type: the same with BP/26-27/0064, the voucher type BRIDGE Sales Keyed
#                 (VCHTYPE and VOUCHERTYPENAME, as row 2162-KD's gen.py made I-0061) and the tail 0813.
#   I-0064.xml    that number on BRIDGE Sales: the same with BP/26-27/0064 and the tail 0814.
#   R-0062.xml, R-0063.xml   the build's read-back (R-0010, row 2162-BC's) for each number.
#   V-0062.xml, V-0063.xml, V-0064.xml   every native field of the vouchers of each number in 2026-27, as gen8.py made
#                 V-0057: template-full-1 with the company, the two dates and its filter changed.
#   W-2026-08.xml the build's number read of one month (render_invoice_window_numbers_request) for 2026-08:
#                 template-y-pilot (row 2162-S7's year window on this book) with its two dates changed, twice each.
#                 It is asserted equal to that function's text for BRIDGE PILOT LAB, 20260801 to 20260831.
# Example: python3 -I gen9.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    print(f"{name}: {len(text.encode('utf-16-le')) + 2} bytes")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

TAIL = "00000000-0000-4000-8000-000000000777"
template = load("template-import-0010.utf16le.xml")
def invoice(number, tail, keyed=False):
    text = swap(template, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    if keyed:
        text = swap(text, 'VCHTYPE="BRIDGE Sales"', 'VCHTYPE="BRIDGE Sales Keyed"', 1)
        text = swap(text, "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>",
                    "<VOUCHERTYPENAME>BRIDGE Sales Keyed</VOUCHERTYPENAME>", 1)
        assert text.count("BRIDGE Sales Keyed") == 2
    return swap(text, TAIL, TAIL[:-4] + tail, 1)
save("I-0062.xml", invoice("0062", "0811"))
save("I-0063.xml", invoice("0063", "0812"))
save("I-0064K.xml", invoice("0064", "0813", keyed=True))
save("I-0064.xml", invoice("0064", "0814"))

for number in ("0062", "0063"):
    save(f"R-{number}.xml", swap(load("R-0010.xml"), '"BP/26-27/0010"', f'"BP/26-27/{number}"', 1))

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
for number in ("0062", "0063", "0064"):
    save(f"V-{number}.xml", swap(full, "$MasterId = 1", f'$VoucherNumber = "BP/26-27/{number}"', 1))

window = load("template-y-pilot.utf16le.xml")
window = swap(window, "20260401", "20260801", 2)
window = swap(window, "20270331", "20260831", 2)
# render_invoice_window_numbers_request (agent_import_invoice_wire.rs), with company, from and to filled in.
BUILD = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
         '<ID>Bridge Invoice Number</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML'
         '</SVEXPORTFORMAT><SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">20260801'
         '</SVFROMDATE><SVTODATE TYPE="Date">20260831</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE>'
         '<SYSTEM TYPE="Formulae" NAME="BridgeInvoiceNumber">$Date &gt;= $$Date:"20260801" AND $Date &lt;= '
         '$$Date:"20260831"</SYSTEM><COLLECTION NAME="Bridge Invoice Number" ISMODIFY="No"><TYPE>Voucher</TYPE>'
         '<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE</FETCH>'
         '<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE><FILTERS>BridgeInvoiceNumber</FILTERS>'
         '</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')
assert window == BUILD, "W-2026-08 is not the build's month read"
save("W-2026-08.xml", window)
