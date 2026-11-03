#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   N-<tag>.xml  the number read (render_invoice_number_request), lab/1342-capture-requests r10-number-control-k0.xml
#                with BP/26-27/0001 changed to the sub-row's number. Window 1 Apr 2026 to 31 Mar 2027 unchanged.
#   I-<tag>.xml  the request that created BP/26-27/0010 (the build's native_post_request), with the number in
#                VOUCHERNUMBER and REFERENCE and a new REMOTEID. Party, figures and date unchanged.
#   R-<tag>.xml  the read-back (render_invoice_readback_request, S1's R), number changed.
# M-marks.xml is not generated: it is copied unchanged (ROW.md says from where).
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
# tag -> number
NUMBERS = {"0051": "BP/26-27/0051", "lc0051": "bp/26-27/0051", "z051": "BP/26-27/051", "0052": "BP/26-27/0052"}
# I files: (file tag, number tag, last digits of the REMOTEID; 0777 is 0010's, 0778 to 0782 S2 to S3b's)
IMPORTS = [("0051", "0051", "0783"), ("0051b", "0051", "0784"), ("lc0051", "lc0051", "0785"),
           ("z051", "z051", "0786"), ("0052", "0052", "0787")]
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
template_n = load("template-n-0001.utf16le.xml")
template_i = load("template-import-0010.utf16le.xml")
template_r = load("template-r-0010.utf16le.xml")
for tag, number in NUMBERS.items():
    assert template_n.count('$VoucherNumber = "BP/26-27/0001"') == 1
    save(f"N-{tag}.xml", template_n.replace('"BP/26-27/0001"', f'"{number}"'))
    assert template_r.count('"BP/26-27/0010"') == 1
    save(f"R-{tag}.xml", template_r.replace('"BP/26-27/0010"', f'"{number}"'))
for tag, number_tag, digits in IMPORTS:
    number = NUMBERS[number_tag]
    i = template_i
    assert i.count(">BP/26-27/0010<") == 2
    i = i.replace(">BP/26-27/0010<", f">{number}<")
    old_id = 'REMOTEID="00000000-0000-4000-8000-000000000777"'
    assert i.count(old_id) == 1
    i = i.replace(old_id, old_id.replace('0777"', f'{digits}"'))
    save(f"I-{tag}.xml", i)
