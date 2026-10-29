"""Fill the R0h template with ledger D's GUID: python -I fill_r0h.py TEMPLATE GUID OUT.

Replaces the one placeholder and nothing else, keeps the byte-order mark and the
encoding, and refuses anything that does not check out. Prints the filled file's
size and SHA-256 for the log.
"""
import hashlib
import re
import sys
import xml.etree.ElementTree as ET

PLACEHOLDER = "@@GUID_OF_R0_CASE_D@@"
TEMPLATE_SHA256 = "0ee8279a56bdfe66557bbdec8c49e276fd8bd6358b5772bc8e29897423e9e3ee"


def main(template_path, guid, out_path):
    template = open(template_path, "rb").read()
    if hashlib.sha256(template).hexdigest() != TEMPLATE_SHA256:
        sys.exit("STOP: the template's SHA-256 is not the one in SHA256.txt")
    if not re.fullmatch(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}-[0-9a-f]{8}", guid):
        sys.exit("STOP: the GUID is not in the form a ledger's GUID takes")
    if template[:2] != b"\xff\xfe":
        sys.exit("STOP: the template has no UTF-16LE byte-order mark")
    text = template[2:].decode("utf-16-le")
    if text.count(PLACEHOLDER) != 1:
        sys.exit("STOP: the template does not hold the placeholder exactly once")
    filled_text = text.replace(PLACEHOLDER, guid)
    if PLACEHOLDER in filled_text or filled_text.count(guid) != 1:
        sys.exit("STOP: the filled text is not the template with one GUID in place")
    filled = b"\xff\xfe" + filled_text.encode("utf-16-le")
    if len(filled) != len(template) + 2 * (len(guid) - len(PLACEHOLDER)):
        sys.exit("STOP: the filled file's length is not the template's plus the GUID's")
    root = ET.fromstring(filled_text.split("?>", 1)[1])
    ledger = root.find("./BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/LEDGER")
    if ledger is None or ledger.findtext("GUID") != guid:
        sys.exit("STOP: the filled request does not carry the GUID where the template put it")
    try:
        with open(out_path, "xb") as out:
            out.write(filled)
    except FileExistsError:
        sys.exit("STOP: the output file already exists; it is never overwritten")
    print(len(filled), hashlib.sha256(filled).hexdigest(), out_path)


if __name__ == "__main__":
    if len(sys.argv) != 4:
        sys.exit("usage: python -I fill_r0h.py TEMPLATE GUID OUT")
    main(sys.argv[1], sys.argv[2], sys.argv[3])
