# legs.py: prints the ledger entries of one invoice's read-back answer (R- or RC-), or compares two. It records; it
# never decides. It exits 0 whether the legs are filled, empty, equal or not. It exits 1 (STOP) only when it cannot
# read an answer.
#
#   python3 -I legs.py ANSWER          every leg: one summary line, then the leg's lines whole
#   python3 -I legs.py --diff OLD NEW  the summary lines of both, then a line diff of the legs, leg by leg
#
# The summary line of a leg names its ledger and these fields' values ('' for an empty element, <absent> for none):
# GSTSOURCETYPE, GSTLEDGERSOURCE, HSNSOURCETYPE, HSNLEDGERSOURCE, GSTHSNNAME, GSTOVRDNTAXABILITY, GSTOVRDNTYPEOFSUPPLY,
# and the number of RATEDETAILS.LIST rows that hold a GSTRATEDUTYHEAD.
#
# It STOPs when an answer is empty, does not decode, holds no BODY/DATA/COLLECTION, holds other than exactly one
# <VOUCHER row, or holds no ALLLEDGERENTRIES.LIST. Output is bounded (AGENTS.md P5): at most 1,500 lines of leg text
# (six legs of this invoice are about 1,200) or 400 lines of diff per call, with the count of lines not shown.
#
# Answers are decoded as Tally sends them: UTF-16LE (with or without a BOM) when the second byte is zero, else UTF-8.
import difflib
import hashlib
import re
import sys

FIELDS = (
    "GSTSOURCETYPE",
    "GSTLEDGERSOURCE",
    "HSNSOURCETYPE",
    "HSNLEDGERSOURCE",
    "GSTHSNNAME",
    "GSTOVRDNTAXABILITY",
    "GSTOVRDNTYPEOFSUPPLY",
)
SHOWN_WHOLE = 1500
SHOWN_DIFF = 400


def stop(message):
    print(f"STOP: {message}")
    sys.exit(1)


def read(path):
    try:
        with open(path, "rb") as handle:
            data = handle.read()
    except OSError as error:
        stop(f"{path}: {error.strerror}")
    if not data:
        stop(f"{path}: empty")
    try:
        if data.startswith(b"\xff\xfe"):
            text = data[2:].decode("utf-16-le")
        elif len(data) > 1 and data[1] == 0:
            text = data.decode("utf-16-le")
        else:
            text = data.decode("utf-8")
    except UnicodeDecodeError:
        stop(f"{path}: does not decode")
    if not re.search(r"<BODY>.*<DATA>.*<COLLECTION", text, re.S):
        stop(f"{path}: no BODY/DATA/COLLECTION")
    # A row opens with attributes; the answer's CMPINFO block holds a bare <VOUCHER> counter.
    rows = len(re.findall(r"<VOUCHER\s", text))
    if rows != 1:
        stop(f"{path}: {rows} VOUCHER rows, not 1")
    legs = re.findall(r"<ALLLEDGERENTRIES\.LIST>.*?</ALLLEDGERENTRIES\.LIST>", text, re.S)
    if not legs:
        stop(f"{path}: no ALLLEDGERENTRIES.LIST")
    print(f"{path}: {len(data)} B sha256 {hashlib.sha256(data).hexdigest()}, {len(legs)} legs")
    return legs


def value(leg, tag):
    if re.search(rf"<{tag}(\s[^>]*)?/>", leg):
        return "''"
    found = re.findall(rf"<{tag}(?:\s[^>]*)?>([^<]*)</{tag}>", leg)
    if not found:
        return "<absent>"
    return " | ".join(repr(item) for item in found)


def summary(index, leg):
    rates = len(re.findall(r"<GSTRATEDUTYHEAD>", leg))
    fields = ", ".join(f"{tag} {value(leg, tag)}" for tag in FIELDS)
    return f"  leg {index} {value(leg, 'LEDGERNAME')}: {fields}, rate rows {rates}"


def bounded(lines, shown):
    for line in lines[:shown]:
        print(line)
    if len(lines) > shown:
        print(f"  ... {len(lines) - shown} more line(s) not shown")


args = sys.argv[1:]
if len(args) == 1:
    legs = read(args[0])
    out = []
    for index, leg in enumerate(legs):
        out.append(summary(index, leg))
        out.extend(f"    {line.rstrip()}" for line in leg.splitlines())
    bounded(out, SHOWN_WHOLE)
elif len(args) == 3 and args[0] == "--diff":
    old, new = read(args[1]), read(args[2])
    for name, legs in ((args[1], old), (args[2], new)):
        print(f"{name}:")
        for index, leg in enumerate(legs):
            print(summary(index, leg))
    if len(old) != len(new):
        print(f"legs: {len(old)} -> {len(new)}; the legs are not compared line by line")
        sys.exit(0)
    out = []
    for index, (a, b) in enumerate(zip(old, new)):
        if a == b:
            out.append(f"leg {index}: EQUAL")
            continue
        lines = list(difflib.unified_diff(a.splitlines(), b.splitlines(), "old", "new", n=0, lineterm=""))
        changed = [line for line in lines if line[:1] in "+-" and not line.startswith(("+++", "---"))]
        out.append(f"leg {index}: DIFFER, {len(changed)} line(s)")
        out.extend(f"  {line.rstrip()}" for line in lines if not line.startswith(("+++", "---")))
    bounded(out, SHOWN_DIFF)
else:
    stop("usage: legs.py ANSWER | legs.py --diff OLD NEW")
