# -*- coding: ascii -*-
# Lab helper (row 2569-S; not product code). Python 3.10, standard library only.
# postcall.py BUILD_ANSWER.json OUT.json: writes ONE post_import call file for the batch that build answer names,
# in the bytes row 2569-R's post calls used: {"tool": "post_import", "arguments": {"company_guid": ..., "batch_id": ...}}
# with no line end. It refuses (exit 2, nothing written) when OUT exists, when the answer is not a build_import_xml
# answer for the pinned book with voucher_count 1, or when batch_id is not bridge-<uuid>. Prints OUT's SHA-256.
import hashlib
import json
import os
import re
import sys

BOOK = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
BATCH = re.compile(r"bridge-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")


def fail(why):
    print("STOP: " + why)
    sys.exit(2)


def main(src, out):
    if os.path.exists(out):
        fail("exists: " + out)
    try:
        with open(src, encoding="utf-8") as f:
            body = json.load(f)
    except (OSError, ValueError) as e:
        fail("unreadable build answer (%s)" % type(e).__name__)
    if body.get("tool") != "build_import_xml" or (body.get("arguments") or {}).get("company_guid") != BOOK:
        fail("not a build_import_xml answer for the pinned book")
    result = (body.get("response") or {}).get("result") or {}
    if result.get("isError") is not False:
        fail("the build answer is an error or has no isError False")
    inner = (result.get("structuredContent") or {}).get("result") or {}
    batch = inner.get("batch_id")
    if not isinstance(batch, str) or not BATCH.fullmatch(batch):
        fail("batch_id missing or not bridge-<uuid>")
    if inner.get("voucher_count") != 1:
        fail("voucher_count is not 1")
    text = json.dumps({"tool": "post_import", "arguments": {"company_guid": BOOK, "batch_id": batch}})
    with open(out, "x", encoding="ascii", newline="") as f:
        f.write(text)
    with open(out, "rb") as f:
        digest = hashlib.sha256(f.read()).hexdigest()
    print("wrote %s %d bytes sha256 %s batch_id %s" % (out, len(text), digest, batch))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        fail("usage: python -I postcall.py BUILD_ANSWER.json OUT.json")
    main(sys.argv[1], sys.argv[2])
