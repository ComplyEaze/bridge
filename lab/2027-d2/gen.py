#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent, the requests this row makes from the four a build
# renders (GD.xml, EGD.xml, CGD.xml and CEGD.xml, see ROW.md), and asserts each change:
#   G.xml, EG.xml, CG.xml, CEG.xml  each of the four with the seven dispatch words removed: the request master sends
#                                   today for the same call;
#   GDN.xml                         GD with the Delivery Note list named after the seven words.
# It also asserts GD.xml is lab/2027-dispatch-sheet's WD.xml, byte-equal, by its sha256.
# Example: python3 -I gen.py
import hashlib, pathlib
here = pathlib.Path(__file__).resolve().parent
DISPATCH = (",BASICSHIPDOCUMENTNO,BASICSHIPPEDBY,BASICFINALDESTINATION,EICHECKPOST,"
            "BILLOFLADINGNO,BILLOFLADINGDATE,BASICSHIPVESSELNO")
DELIVERY_NOTES = ",INVOICEDELNOTES.*"
WD_2027D_SHA256 = "6bda8952ed78cd584fe01ef66a824b95bbc2da24f18a1810882d6f32c8c45341"
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
assert hashlib.sha256((here / "GD.xml").read_bytes()).hexdigest() == WD_2027D_SHA256
for name in ["GD", "EGD", "CGD", "CEGD"]:
    text = load(f"{name}.xml")
    assert text.count(DISPATCH + "</FETCH>") == 1 and text.count(DISPATCH) == 1, name
    save(f"{name[:-1]}.xml", text.replace(DISPATCH, ""))
gd = load("GD.xml")
save("GDN.xml", gd.replace(DISPATCH + "</FETCH>", DISPATCH + DELIVERY_NOTES + "</FETCH>"))
assert load("GDN.xml").count(DELIVERY_NOTES) == 1
print("written: G.xml EG.xml CG.xml CEG.xml GDN.xml")
