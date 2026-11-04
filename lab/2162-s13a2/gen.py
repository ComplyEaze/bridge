#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent. It is lab/2162-s13's gen.py with GSTIN changed and
# sub-row b's files (I-0054, R-0054, N-0054) left out; every file it writes is byte-equal to S13's of the same name
# except L-seed.xml and I-0053.xml, which differ from S13's in the GSTIN alone.
#   L-seed.xml          setup: one ledger Create, BRIDGE Customer RJ under Sundry Debtors, not bill-wise, with one
#                       dated GST registration row (Regular, Rajasthan, 08ZZZZZ0000Z3ZO from 1 Apr 2026). From
#                       template-ledger.utf16le.xml (lab/2162-s3a-sheet L-ledger.xml, byte-equal).
#   I-0053.xml          the registered customer: the build's request that created BP/26-27/0010
#                       (template-import-0010.utf16le.xml, lab/2162-s4's, byte-equal) with the number, the REMOTEID,
#                       the party in its seven places, PARTYGSTIN added where render_sales_invoice_xml writes it, and
#                       GSTREGISTRATIONTYPE Regular. Nothing else changes.
#   R-0053.xml          the read-back (template-r-0010.utf16le.xml, lab/2162-s4's, byte-equal) with the number.
#   R-0010.xml          the read-back for BP/26-27/0010, the control: the template itself.
#   N-0053.xml          the number read with the e-invoice words, as lab/2162-s5's gen.py makes it.
# The GSTIN is checked here, before any file is written: its check character (mod 36), and that it is neither the
# company's own (COMPANY_GSTIN, the book's tax unit) nor the one other GSTIN the book's ledgers held before S13
# (BOOK_GSTINS, from S13's a-s2-Q-rates answer).
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
def check_character(first14):
    chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    total = 0
    for index, char in enumerate(first14):
        value = chars.index(char) * (2 if index % 2 else 1)
        total += value // 36 + value % 36
    return chars[(36 - total % 36) % 36]
GSTIN = "08ZZZZZ0000Z3ZO"
COMPANY_GSTIN = "08ZZZZZ0000Z1ZQ"
BOOK_GSTINS = {"08ZZZZZ0000Z2ZP"}
CUSTOMER = "BRIDGE Customer RJ"
assert len(GSTIN) == 15 and GSTIN[-1] == check_character(GSTIN[:14]), GSTIN
assert COMPANY_GSTIN[-1] == check_character(COMPANY_GSTIN[:14])
assert GSTIN != COMPANY_GSTIN and GSTIN not in BOOK_GSTINS, GSTIN

ledger = load("template-ledger.utf16le.xml")
ledger = swap(ledger, "BRIDGE Walk-in BW", CUSTOMER, 2)
ledger = swap(ledger, "<ISBILLWISEON>Yes</ISBILLWISEON>",
              "<ISBILLWISEON>No</ISBILLWISEON><LEDGSTREGDETAILS.LIST><APPLICABLEFROM>20260401</APPLICABLEFROM>"
              "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>"
              f"<GSTIN>{GSTIN}</GSTIN></LEDGSTREGDETAILS.LIST>", 1)
save("L-seed.xml", ledger)

base = load("template-import-0010.utf16le.xml")
a = swap(base, "BP/26-27/0010", "BP/26-27/0053", 2)
a = swap(a, "000000000777", "000000000788", 1)
a = swap(a, "BRIDGE Walk-in", CUSTOMER, 7)
a = swap(a, "</CONSIGNEEMAILINGNAME><STATENAME>",
         f"</CONSIGNEEMAILINGNAME><PARTYGSTIN>{GSTIN}</PARTYGSTIN><STATENAME>", 1)
a = swap(a, "<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>",
         "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE>", 1)
save("I-0053.xml", a)

r = load("template-r-0010.utf16le.xml")
save("R-0010.xml", r)
save("R-0053.xml", swap(r, '"BP/26-27/0010"', '"BP/26-27/0053"', 1))

n = load("template-n-0001.utf16le.xml")
n = swap(n, "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH>",
         "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE</FETCH>", 1)
save("N-0053.xml", swap(n, '$VoucherNumber = "BP/26-27/0001"', '$VoucherNumber = "BP/26-27/0053"', 1))
