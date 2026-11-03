#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   W.xml        the vouchers window read the build sends with include_gst_details, for 3 Apr 2025 on this book:
#                template-a07.utf16le.xml (lab/2027-voucher-gst-fields-fetch at 7be9ae07, templates/
#                a07-vouchers-shape-20250424.xml, byte-equal) with its company and day changed and the six GST words
#                added after REFERENCEDATE, as that sheet's gen_requests.py made its candidate (master's
#                AGENT_VOUCHER_FETCH followed by AGENT_VOUCHER_GST_WORDS, src-tauri/src/agent_read_profiles.rs:47-59).
#   WD.xml       W with the seven dispatch words added after the six. Nothing else changes.
#   F-sales.xml  template-full-1.utf16le.xml (lab/1378-irn-capture-answers reqs/full1.xml, byte-equal) with its filter
#                changed from the first voucher to the vouchers of type Sales.
#   F-note.xml   the same, filtered to the vouchers of type Credit Note (byte-equal to lab/2162-irn-read F-note.xml).
# M-marks.xml and L-list.xml are lab/2162-irn-read's, byte-equal.
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
GST_WORDS = ["PLACEOFSUPPLY", "STATENAME", "GSTREGISTRATIONTYPE",
             "GSTNATUREOFRETURN", "VATPARTYTRANSRETURNNUMBER", "VATPARTYTRANSRETURNDATE"]
# The elements R2 of lab/2162-irn-read-answers (9a20049b) found holding a Credit Note's dispatch values: a closed set.
DISPATCH_WORDS = ["BASICSHIPDOCUMENTNO", "BASICSHIPPEDBY", "BASICFINALDESTINATION", "EICHECKPOST",
                  "BILLOFLADINGNO", "BILLOFLADINGDATE", "BASICSHIPVESSELNO"]
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old))
    return text.replace(old, new)
tail = "PARTYGSTIN,REFERENCEDATE</FETCH>"
w = load("template-a07.utf16le.xml")
w = swap(w, ">BRIDGE SHAPE LAB<", ">BRIDGE EINVOICE LAB<", 1)
w = swap(w, "20250424", "20250403", 4)
w = swap(w, tail, tail.replace("</FETCH>", "".join("," + word for word in GST_WORDS) + "</FETCH>"), 1)
save("W.xml", w)
gst_tail = GST_WORDS[-1] + "</FETCH>"
save("WD.xml", swap(w, gst_tail, gst_tail.replace("</FETCH>", "".join("," + word for word in DISPATCH_WORDS) + "</FETCH>"), 1))
full = load("template-full-1.utf16le.xml")
save("F-sales.xml", swap(full, ">$MasterId = 1<", '>$VoucherTypeName = "Sales"<', 1))
save("F-note.xml", swap(full, ">$MasterId = 1<", '>$VoucherTypeName = "Credit Note"<', 1))
