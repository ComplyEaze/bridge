#!/usr/bin/env python3
# gen5.py: writes the requests of row 2162-S5b beside this file, UTF-16LE with a BOM, as sent. It starts from S13a2's
# files (lab/2162-s13a2-sheet, folder lab/2162-s13a2/), which are beside it byte-equal, and changes only what each line
# below names. Every change is asserted to occur exactly as often as expected.
#   I-0053.xml   S13a2's, unchanged: the registered customer BRIDGE Customer RJ, WITH the buyer and consignee block.
#   I-0054.xml   I-0053 with the number BP/26-27/0054, the REMOTEID tail 0789, and the six block elements removed.
#   I-0055.xml   the unregistered customer BRIDGE Walk-in WITH the block: template-import-0010 (the build's request
#                that created BP/26-27/0010) with the number BP/26-27/0055 and the REMOTEID tail 0790.
#   I-0056.xml   I-0055 with the number BP/26-27/0056, the REMOTEID tail 0791, and the six block elements removed.
#   R-<n>.xml    the read-back (template-r-0010) for each number; R-0010 and R-0053 are S13a2's, unchanged.
#   N-<n>.xml    the number read with the e-invoice words for each number; N-0053 is S13a2's, unchanged.
#   V-<n>.xml    every native field of the vouchers numbered <n> in 2026-27: the native read of row 2162-IRN's
#                template-full-1 with the company, the two dates and its filter changed.
#   K-walkin.xml, K-customer.xml  every native field of one ledger by name: the same read with TYPE Ledger.
# The six block elements are the ones the rehearsal's posted invoices lacked and its keyed ones carried (protocol
# reference, Voucher writes, section 9.16, "Posted beside keyed"): BASICBUYERNAME, PARTYMAILINGNAME,
# CONSIGNEEMAILINGNAME, CONSIGNEESTATENAME, CONSIGNEECOUNTRYNAME and VATDEALERTYPE. COUNTRYOFRESIDENCE stays.
# Example: python3 -I gen5.py
import pathlib, re
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
BLOCK = ["BASICBUYERNAME", "PARTYMAILINGNAME", "CONSIGNEEMAILINGNAME", "CONSIGNEESTATENAME", "CONSIGNEECOUNTRYNAME",
         "VATDEALERTYPE"]
def without_block(text):
    for tag in BLOCK:
        found = re.findall(rf"<{tag}>[^<]*</{tag}>", text)
        assert len(found) == 1, (tag, found)
        text = text.replace(found[0], "")
    assert "COUNTRYOFRESIDENCE" in text
    return text
def renumber(text, old, new, old_tail, new_tail):
    text = swap(text, f"BP/26-27/{old}", f"BP/26-27/{new}", 2)
    return swap(text, f"00000000{old_tail}", f"00000000{new_tail}", 1)

registered = load("I-0053.xml")
assert all(f"<{tag}>" in registered for tag in BLOCK)
save("I-0054.xml", without_block(renumber(registered, "0053", "0054", "0788", "0789")))
walk_in = renumber(load("template-import-0010.utf16le.xml"), "0010", "0055", "0777", "0790")
save("I-0055.xml", walk_in)
save("I-0056.xml", without_block(renumber(walk_in, "0055", "0056", "0790", "0791")))

readback, number = load("R-0010.xml"), load("N-0053.xml")
for n in ["0054", "0055", "0056"]:
    save(f"R-{n}.xml", swap(readback, '"BP/26-27/0010"', f'"BP/26-27/{n}"', 1))
    save(f"N-{n}.xml", swap(number, '"BP/26-27/0053"', f'"BP/26-27/{n}"', 1))

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
for n in ["0053", "0054", "0055", "0056"]:
    save(f"V-{n}.xml", swap(full, "$MasterId = 1", f'$VoucherNumber = "BP/26-27/{n}"', 1))
ledger = swap(full, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
for name, party in [("K-walkin.xml", "BRIDGE Walk-in"), ("K-customer.xml", "BRIDGE Customer RJ")]:
    save(name, swap(ledger, "$MasterId = 1", f'$Name = "{party}"', 1))
