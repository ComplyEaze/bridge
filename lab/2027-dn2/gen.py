#!/usr/bin/env python3
# gen.py: makes no request. It asserts that each request beside this file taken from an earlier sheet is that sheet's
# file, byte for byte by its sha256, and that EGDN.xml, CGDN.xml and CEGDN.xml are EGD.xml, CGD.xml and CEGD.xml with
# the Delivery Note list named after the seven dispatch words and nothing else changed (see ROW.md, Files). Run it
# before step 0; any failure is a stop.
# Example: python3 -I gen.py
import hashlib, pathlib
here = pathlib.Path(__file__).resolve().parent
EARLIER = {
    # lab/2027-d2-sheet at 31c51237, folder lab/2027-d2/
    "run.py": "4c192af54123dc84bc55485bde4402b9bfed6e237a34524a3bb26a370e4fcddc",
    "M-marks.xml": "f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918",
    "L-list.xml": "0566b424bbfd017e91013382d7c14d98965afe89f47ad466ed07ebc11b44706a",
    "EGD.xml": "e182bb2450fd050cb6c486f91420540c4c080f1ae67e68a42c5514e1cfa1a7b5",
    "CGD.xml": "05b10dca1a214fb30aec4da09f83e1c45202df2306f256eda953d134b9858f4a",
    "CEGD.xml": "1b72a961c6164acaeacbe8571fb90769190af9286898ebf3ba26b9fa65ff9b5e",
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
for name in ["EGD", "CGD", "CEGD"]:
    without, with_list = load(f"{name}.xml"), load(f"{name}N.xml")
    assert without.count(DISPATCH + "</FETCH>") == 1 and without.count(DELIVERY_NOTES) == 0, name
    assert with_list == without.replace(DISPATCH + "</FETCH>", DISPATCH + DELIVERY_NOTES + "</FETCH>"), name
    assert "BRIDGE EINVOICE LAB" in without and '<SVFROMDATE TYPE="Date">20250403</SVFROMDATE>' in without, name
print("checked: run.py M-marks.xml L-list.xml EGD.xml EGDN.xml CGD.xml CGDN.xml CEGD.xml CEGDN.xml")
