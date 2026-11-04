#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   L-seed.xml          setup: one ledger Create, BRIDGE Customer RJ under Sundry Debtors, not bill-wise, with one
#                       dated GST registration row (Regular, Rajasthan, 08ZZZZZ0000Z1ZQ from 1 Apr 2026). From
#                       template-ledger.utf16le.xml (lab/2162-s3a-sheet L-ledger.xml, byte-equal).
#   I-0053.xml          sub-row a, the registered customer: the build's request that created BP/26-27/0010
#                       (template-import-0010.utf16le.xml, lab/2162-s4's, byte-equal) with the number, the REMOTEID,
#                       the party in its seven places, PARTYGSTIN added where render_sales_invoice_xml writes it, and
#                       GSTREGISTRATIONTYPE Regular. Nothing else changes.
#   I-0054.xml          sub-row b, the round off on the debit side: the same template with the number, the REMOTEID,
#                       the party's amount (1,050.00 in place of 1,051.00) and the round-off leg (debit 0.84 in place
#                       of credit 0.16). Nothing else changes.
#   R-0053.xml, R-0054.xml  the read-back (template-r-0010.utf16le.xml, lab/2162-s4's, byte-equal) with the number.
#   R-0010.xml          the read-back for BP/26-27/0010, the control: the template itself.
#   N-0053.xml, N-0054.xml  the number read with the e-invoice words, as lab/2162-s5's gen.py makes it.
# M-marks.xml, run.py, Q-rates.xml and C-catalogue.xml are not generated; ROW.md says where each is from.
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old))
    return text.replace(old, new)
GSTIN = "08ZZZZZ0000Z1ZQ"
CUSTOMER = "BRIDGE Customer RJ"

ledger = load("template-ledger.utf16le.xml")
ledger = swap(ledger, "BRIDGE Walk-in BW", CUSTOMER, 2)
ledger = swap(ledger, "<ISBILLWISEON>Yes</ISBILLWISEON>",
              "<ISBILLWISEON>No</ISBILLWISEON><LEDGSTREGDETAILS.LIST><APPLICABLEFROM>20260401</APPLICABLEFROM>"
              "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>"
              f"<GSTIN>{GSTIN}</GSTIN></LEDGSTREGDETAILS.LIST>", 1)
save("L-seed.xml", ledger)

base = load("template-import-0010.utf16le.xml")
def invoice(number, remote_tail):
    text = swap(base, "BP/26-27/0010", number, 2)
    return swap(text, "000000000777", remote_tail, 1)

a = invoice("BP/26-27/0053", "000000000788")
a = swap(a, "BRIDGE Walk-in", CUSTOMER, 7)
a = swap(a, "</CONSIGNEEMAILINGNAME><STATENAME>",
         f"</CONSIGNEEMAILINGNAME><PARTYGSTIN>{GSTIN}</PARTYGSTIN><STATENAME>", 1)
a = swap(a, "<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>",
         "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE>", 1)
save("I-0053.xml", a)

b = invoice("BP/26-27/0054", "000000000789")
b = swap(b, "<AMOUNT>-1051.00</AMOUNT>", "<AMOUNT>-1050.00</AMOUNT>", 1)
b = swap(b, "<LEDGERNAME>BRIDGE Round Off</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>0.16</AMOUNT>",
         "<LEDGERNAME>BRIDGE Round Off</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-0.84</AMOUNT>", 1)
save("I-0054.xml", b)

r = load("template-r-0010.utf16le.xml")
save("R-0010.xml", r)
for number in ("0053", "0054"):
    save(f"R-{number}.xml", swap(r, '"BP/26-27/0010"', f'"BP/26-27/{number}"', 1))

n = load("template-n-0001.utf16le.xml")
n = swap(n, "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH>",
         "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE</FETCH>", 1)
for number in ("0053", "0054"):
    save(f"N-{number}.xml", swap(n, '$VoucherNumber = "BP/26-27/0001"', f'$VoucherNumber = "BP/26-27/{number}"', 1))
