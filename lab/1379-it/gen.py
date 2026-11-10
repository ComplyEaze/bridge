#!/usr/bin/env python3
# gen.py: writes the requests of row 1379-IT beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' or to master's (ROW.md says which), and changes only what each line
# below names. Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked
# first. I-0093.xml is authored for shape discovery: no build renders it (ROW.md, "Code admits the trigger").
#   I-0093.xml   template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010), with:
#                1. the number BP/26-27/0093 in REFERENCE and VOUCHERNUMBER, where the build writes the number twice;
#                2. the REMOTEID tail 000000000e01;
#                3. no VCHENTRYMODE element: the 2016 item request has none (ITEM_REQUEST's elements 16 to 23);
#                4. the two service legs (BRIDGE Svc 998313 5% 600.40 and BRIDGE Svc 998314 5% 400.40) replaced, in
#                   their place after the party leg, by one ALLINVENTORYENTRIES.LIST cut from ITEM_REQUEST (its
#                   elements 30 to 50) with RATE, ACTUALQTY, BILLEDQTY and BATCHALLOCATIONS.LIST left out, the item
#                   BRIDGE Item A, and in the item line and its one ACCOUNTINGALLOCATIONS.LIST the ledger BRIDGE Svc
#                   998313 5%, side No and amount 1000.80, the two legs' sum. The party, tax and round-off legs are
#                   the invoice's, so the total stays 1051.00 and the tax stays 2.5% of 1000.80 on each head.
#   V-0093.xml   template-full-0057.utf16le.xml (row 2162-RS's native read by number) for the number BP/26-27/0093.
#   R-0093.xml   R-0010.xml (the build's read-back for BP/26-27/0010) for the number BP/26-27/0093.
#   S-items.xml  template-stock-items.utf16le.xml (master's masters_stock_items_request fixture, the build's
#                stock-item master read) with the company BRIDGE PILOT LAB in place of BRIDGE SHAPE LAB.
#   G-godowns.xml  template-godowns.utf16le.xml (master's masters_godowns_request fixture, the build's godown master
#                read), with the same company change.
# "Element n" of ITEM_REQUEST counts its text split before every '<' that follows a '>', from 1 (ROW.md lists them).
# Example: python3 -I gen.py
import hashlib
import pathlib
from decimal import Decimal
here = pathlib.Path(__file__).resolve().parent
ITEM_REQUEST = "template-v-sales-2016.utf16le.xml"  # lab/2016-stock-lines-answers at 186c39d6, reqs/v-sales.xml
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "template-full-0057.utf16le.xml": "cd8e91b441040b742f575955f10d57e187f93e618139e2a6325064c935b09020",
    "template-stock-items.utf16le.xml": "c90cfcc238cf308b58fc160e6ecd78affa691729a3dbc2ff963f4d777e82d075",
    "template-godowns.utf16le.xml": "a2a63164752bd2c438de7a27ca4cb854cb0bf5b1a60db3723986370140f051f3",
    ITEM_REQUEST: "c749d38acd45289d3f27a872e087699c8075972fd5c691fd24c9cba594346c74",
}
ITEM, LEDGER, AMOUNT = "BRIDGE Item A", "BRIDGE Svc 998313 5%", "1000.80"
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)
def leg(name, side, amount):
    return (f"<LEDGERENTRIES.LIST><LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{side}</ISDEEMEDPOSITIVE>"
            f"<AMOUNT>{amount}</AMOUNT></LEDGERENTRIES.LIST>")
def elements(text):
    out, start = [], 0
    for i in range(1, len(text)):
        if text[i] == "<" and text[i - 1] == ">":
            out.append(text[start:i]); start = i
    return out + [text[start:]]

# The item line, cut from the 2016 request element by element; each element kept is asserted to be what ROW.md says.
item = elements(load(ITEM_REQUEST))
assert not any("VCHENTRYMODE" in e for e in item[15:23]), "ITEM_REQUEST's voucher head names VCHENTRYMODE"
assert item[29] == "<ALLINVENTORYENTRIES.LIST>" and item[49] == "</ALLINVENTORYENTRIES.LIST>", (item[29], item[49])
assert item[30] == "<STOCKITEMNAME>Hydrochloric Acid 33pc</STOCKITEMNAME>", item[30]
assert item[31] == "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>", item[31]
assert item[32].startswith("<RATE>") and item[34].startswith("<ACTUALQTY>") and item[35].startswith("<BILLEDQTY>")
assert item[33] == "<AMOUNT>30.00</AMOUNT>", item[33]
assert item[36] == "<BATCHALLOCATIONS.LIST>" and item[43] == "</BATCHALLOCATIONS.LIST>", (item[36], item[43])
assert item[44:49] == ["<ACCOUNTINGALLOCATIONS.LIST>", "<LEDGERNAME>Sales - Exempt</LEDGERNAME>",
                       "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>", "<AMOUNT>30.00</AMOUNT>",
                       "</ACCOUNTINGALLOCATIONS.LIST>"], item[44:49]
kept = [item[29], item[30], item[31], item[33]] + item[44:50]  # elements 30, 31, 32, 34 and 45 to 50
line = "".join(kept)
line = swap(line, "<STOCKITEMNAME>Hydrochloric Acid 33pc</STOCKITEMNAME>", f"<STOCKITEMNAME>{ITEM}</STOCKITEMNAME>", 1)
line = swap(line, "<LEDGERNAME>Sales - Exempt</LEDGERNAME>", f"<LEDGERNAME>{LEDGER}</LEDGERNAME>", 1)
line = swap(line, "<AMOUNT>30.00</AMOUNT>", f"<AMOUNT>{AMOUNT}</AMOUNT>", 2)
assert line == (f"<ALLINVENTORYENTRIES.LIST><STOCKITEMNAME>{ITEM}</STOCKITEMNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>"
                f"<AMOUNT>{AMOUNT}</AMOUNT><ACCOUNTINGALLOCATIONS.LIST><LEDGERNAME>{LEDGER}</LEDGERNAME>"
                f"<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{AMOUNT}</AMOUNT></ACCOUNTINGALLOCATIONS.LIST>"
                f"</ALLINVENTORYENTRIES.LIST>"), line

invoice = load("template-import-0010.utf16le.xml")
text = swap(invoice, "BP/26-27/0010", "BP/26-27/0093", 2)
assert "<REFERENCE>BP/26-27/0093</REFERENCE>" in text and "<VOUCHERNUMBER>BP/26-27/0093</VOUCHERNUMBER>" in text
text = swap(text, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000e01", 1)
text = swap(text, "<VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE>", "", 1)
services = leg("BRIDGE Svc 998313 5%", "No", "600.40") + leg("BRIDGE Svc 998314 5%", "No", "400.40")
assert Decimal("600.40") + Decimal("400.40") == Decimal(AMOUNT)
text = swap(text, leg("BRIDGE Walk-in", "Yes", "-1051.00") + services, leg("BRIDGE Walk-in", "Yes", "-1051.00") + line, 1)
assert text.count("<LEDGERENTRIES.LIST>") == 4 and text.count("<ALLINVENTORYENTRIES.LIST>") == 1
for word in ("VCHENTRYMODE", "RATE>", "ACTUALQTY", "BILLEDQTY", "BATCHALLOCATIONS", "GODOWN", "BILLALLOCATIONS",
             "BP/26-27/0010", "998314"):
    assert word not in text, f"the item request names {word}"
save("I-0093.xml", text)

full = load("template-full-0057.utf16le.xml")
save("V-0093.xml", swap(full, '"BP/26-27/0057"', '"BP/26-27/0093"', 1))
readback = load("R-0010.xml")
save("R-0093.xml", swap(readback, '"BP/26-27/0010"', '"BP/26-27/0093"', 1))
items = load("template-stock-items.utf16le.xml")
save("S-items.xml", swap(items, "<SVCURRENTCOMPANY>BRIDGE SHAPE LAB</SVCURRENTCOMPANY>",
                         "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1))
godowns = load("template-godowns.utf16le.xml")
save("G-godowns.xml", swap(godowns, "<SVCURRENTCOMPANY>BRIDGE SHAPE LAB</SVCURRENTCOMPANY>",
                           "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1))
