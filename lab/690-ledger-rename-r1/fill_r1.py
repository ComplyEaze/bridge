"""Fill an R1 template with ledger X's GUID, read from a ledgers answer:
python -I fill_r1.py TEMPLATE LEDGERS_ANSWER OUT.

The GUID is taken from the answer, never typed: exactly one ledger whose own name
(the first NAME of the first NAME.LIST under its first LANGUAGENAME.LIST) is
exactly "R1 Guard X". The answer is read as LAB-COMMON C4 says. The script
replaces the one placeholder and nothing else, keeps the byte-order mark and the
encoding, and refuses anything that does not check out. It prints the GUID, the
filled file's size and its SHA-256 for the log.
"""
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET

PLACEHOLDER = "@@GUID_OF_R1_GUARD_X@@"
TEMPLATES = {
    "r1m-guid-matched-template.utf16le.xml": ("66ece96740b96e99c32f03ed9a3a7c3801ae3f966f58cd68d68d63c90f2303ce", "R1 Guard X"),
    "r1a-guid-mismatch-template.utf16le.xml": ("078f2207b932978c9c4786263de256798ad8c825d0d9ea94928206aa8a5f3267", "R1 Guard Y"),
}
FORBIDDEN = set(range(0, 9)) | {11, 12} | set(range(14, 32))


def c4_text(raw):
    """LAB-COMMON C4: decode UTF-16LE, with or without a BOM, and drop only the
    characters XML forbids, raw or as references; keep 9, 10 and 13."""
    text = raw.decode("utf-16-le").lstrip("\ufeff")

    def reference(match):
        value = match.group(1)
        code = int(value[1:], 16) if value[:1] in "xX" else int(value)
        return "" if code in FORBIDDEN else match.group(0)

    text = re.sub(r"&#([xX][0-9a-fA-F]+|[0-9]+);", reference, text)
    return "".join(ch for ch in text if ord(ch) not in FORBIDDEN)


def guid_of_x(answer_path):
    envelope = ET.fromstring(c4_text(open(answer_path, "rb").read()))
    if (envelope.findtext("HEADER/STATUS") or "").strip() != "1":
        sys.exit("STOP: the ledgers answer does not report STATUS 1")
    found = []
    for ledger in envelope.findall("BODY/DATA/COLLECTION/LEDGER"):
        lists = ledger.findall("LANGUAGENAME.LIST")
        name_lists = lists[0].findall("NAME.LIST") if lists else []
        names = name_lists[0].findall("NAME") if name_lists else []
        if names and names[0].text == "R1 Guard X":
            found.append((ledger.findtext("GUID") or "").strip())
    if len(found) != 1:
        sys.exit(f"STOP: {len(found)} ledgers have the own name R1 Guard X, not one")
    guid = found[0]
    if not re.fullmatch(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}-[0-9a-f]{8}", guid):
        sys.exit("STOP: the GUID is not in the form a ledger's GUID takes")
    return guid


def main(template_path, answer_path, out_path):
    entry = TEMPLATES.get(os.path.basename(template_path))
    if entry is None:
        sys.exit("STOP: not one of the two R1 templates")
    expected_sha256, addressed_to = entry
    template = open(template_path, "rb").read()
    if hashlib.sha256(template).hexdigest() != expected_sha256:
        sys.exit("STOP: the template's SHA-256 is not the one in SHA256.txt")
    if template[:2] != b"\xff\xfe":
        sys.exit("STOP: the template has no UTF-16LE byte-order mark")
    guid = guid_of_x(answer_path)
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
    ledgers = root.findall("./BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/LEDGER")
    if len(ledgers) != 1 or ledgers[0].findtext("GUID") != guid or ledgers[0].get("NAME") != addressed_to:
        sys.exit("STOP: the filled request is not one Alter addressed as the template says, carrying the GUID")
    try:
        with open(out_path, "xb") as out:
            out.write(filled)
    except FileExistsError:
        sys.exit("STOP: the output file already exists; it is never overwritten")
    except OSError as error:
        sys.exit(f"STOP: the output file could not be written: {error.strerror}")
    print(guid, len(filled), hashlib.sha256(filled).hexdigest(), out_path)


if __name__ == "__main__":
    if len(sys.argv) != 4:
        sys.exit("usage: python -I fill_r1.py TEMPLATE LEDGERS_ANSWER OUT")
    main(sys.argv[1], sys.argv[2], sys.argv[3])
