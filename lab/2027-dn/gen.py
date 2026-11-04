#!/usr/bin/env python3
# gen.py: makes no request. It asserts that each request beside this file is the earlier sheet's file it names, byte
# for byte by its sha256, and that GDN.xml is GD.xml with the Delivery Note list named after the seven dispatch words
# and nothing else changed (see ROW.md, Files). Run it before step 0; any failure is a stop.
# Example: python3 -I gen.py
import hashlib, pathlib
here = pathlib.Path(__file__).resolve().parent
EARLIER = {
    # lab/2027-d2-sheet at 31c51237, folder lab/2027-d2/
    "run.py": "4c192af54123dc84bc55485bde4402b9bfed6e237a34524a3bb26a370e4fcddc",
    "M-marks.xml": "f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918",
    "L-list.xml": "0566b424bbfd017e91013382d7c14d98965afe89f47ad466ed07ebc11b44706a",
    "GD.xml": "6bda8952ed78cd584fe01ef66a824b95bbc2da24f18a1810882d6f32c8c45341",
    "GDN.xml": "d99554fc0d566c3c0a91cbe894a7561dea31fa4a8187cffac8b2de12e553257c",
    # lab/2027-dispatch-sheet at f6039d62, folder lab/2027-dispatch/
    "F-sales.xml": "ac7f0913b48fbd5f0b778822dd6e15a4dfa001b8242480e153ab04647f8181f8",
}
DISPATCH = (",BASICSHIPDOCUMENTNO,BASICSHIPPEDBY,BASICFINALDESTINATION,EICHECKPOST,"
            "BILLOFLADINGNO,BILLOFLADINGDATE,BASICSHIPVESSELNO")
DELIVERY_NOTES = ",INVOICEDELNOTES.*"
for name, digest in EARLIER.items():
    assert hashlib.sha256((here / name).read_bytes()).hexdigest() == digest, f"{name} is not the earlier sheet's file"
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
gd, gdn = load("GD.xml"), load("GDN.xml")
assert gd.count(DISPATCH + "</FETCH>") == 1
assert gdn == gd.replace(DISPATCH + "</FETCH>", DISPATCH + DELIVERY_NOTES + "</FETCH>")
assert "BRIDGE EINVOICE LAB" in gd and '<SVFROMDATE TYPE="Date">20250403</SVFROMDATE>' in gd
print("checked: run.py M-marks.xml L-list.xml GD.xml GDN.xml F-sales.xml")
