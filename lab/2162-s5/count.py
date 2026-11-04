#!/usr/bin/env python3
# count.py: prints shapes and counts for every answer run.py wrote beside this file (a-<label>-<request>), with an XML
# parser, never a text search. It prints tags, counts, lengths, yes or no, and only the values this sheet names or the
# lab's own series uses; any other value prints as its length. Copy its whole output into RESULTS.md.
# A row is a VOUCHER element with a VOUCHERNUMBER child. A row's bytes are the bytes of the answer as sent, from the
# start of its VOUCHER start tag to the end of its end tag, found by expat's byte index over the UTF-16LE answer.
# Example: python3 -I count.py
import pathlib, re, sys, statistics
import xml.etree.ElementTree as ET
import xml.parsers.expat
here = pathlib.Path(__file__).resolve().parent
GUID = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"
SEED, DROPPED = "BP/26-27/0051", "BP/26-27/051"
ROW_TAGS = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "ISCANCELLED", "BRIDGEVCHISSALES"]
IRN_TAGS = ["IRN", "IRNACKNO", "IRNACKDATE"]
COUNTERS = ["CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS"]
TYPES = {"Sales", "Purchase", "Payment", "Receipt", "Journal", "Contra", "Credit Note", "Debit Note"}
NUMBER = re.compile(r"(BP/26-27/\d{1,6}|\d{1,6})")
def show(tag, text):
    text = (text or "").strip()
    if text == "" or text in ("Yes", "No") or re.fullmatch(r"\d{8}", text):
        return text
    if tag == "VOUCHERNUMBER" and NUMBER.fullmatch(text):
        return text
    if tag == "VOUCHERTYPENAME" and (text in TYPES or text.startswith("BRIDGE ")):
        return text
    return f"<{len(text)} chars>"
def load(path):
    raw = path.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    text = body.decode("utf-16-le")
    # Tally writes control characters as numeric references XML 1.0 refuses. Each is blanked with spaces of the same
    # length, so every byte offset below is still the answer's own.
    text = re.sub(r"&#(\d+);", lambda m: " " * len(m.group(0)) if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    return text, len(raw), len(raw) - len(body)
def spans(text, lead):
    # (start, end) in bytes of the answer, for each VOUCHER element whose parent is COLLECTION.
    out, stack = [], []
    p = xml.parsers.expat.ParserCreate("UTF-16LE")
    def start(name, attrs):
        stack.append((name, p.CurrentByteIndex))
    def end(name):
        tag, at = stack.pop()
        if tag == "VOUCHER" and stack and stack[-1][0] == "COLLECTION":
            out.append((at + lead, p.CurrentByteIndex + lead + 2 * len("</VOUCHER>")))
    p.StartElementHandler, p.EndElementHandler = start, end
    p.Parse(text.encode("utf-16-le"), True)
    return out
def rows(root):
    return [v for v in root.iter("VOUCHER") if v.find("VOUCHERNUMBER") is not None]
def shape(row):
    return [(c.tag, sorted(c.attrib.items()), (c.text or "").strip()) for c in row]
answers = sorted(here.glob("a-*-*.xml"))
if not answers:
    sys.exit("no answers beside count.py")
parsed = {}
for path in answers:
    try:
        text, size, lead = load(path)
        root = ET.fromstring(text)
        parsed[path.name] = (root, size, spans(text, lead) if ("-Y-" in path.name or "-N-" in path.name) else [])
    except (ET.ParseError, xml.parsers.expat.ExpatError) as error:
        print(f"{path.name}: does not parse ({error}); stop and report it")
for name, (root, size, places) in parsed.items():
    found = rows(root)
    print(f"{name}: {size} bytes, rows with VOUCHERNUMBER {len(found)}")
    if name.endswith("M-marks.xml"):
        companies = [c for c in root.iter("COMPANY") if (c.findtext("GUID") or "").strip()]
        print(f"  COMPANY elements with a GUID {len(companies)}")
        for c in companies:
            print(f"  GUID is the book's: {'yes' if c.findtext('GUID').strip() == GUID else 'no'};"
                  f" ALTVCHID '{(c.findtext('ALTVCHID') or '').strip()}'; ALTMSTID '{(c.findtext('ALTMSTID') or '').strip()}'")
    elif "-I-" in name:
        print("  " + "; ".join(f"{t} {[(e.text or '').strip() for e in root.iter(t)]}" for t in COUNTERS))
        print(f"  LINEERROR elements {sum(1 for _ in root.iter('LINEERROR'))}: {[(e.text or '').strip() for e in root.iter('LINEERROR')]}")
    elif "-Y-" in name or "-N-" in name:
        vouchers = [v for c in root.iter("COLLECTION") for v in c if v.tag == "VOUCHER"]
        if len(places) != len(vouchers) or len(vouchers) != len(found):
            print(f"  STOP: {len(vouchers)} VOUCHER elements under COLLECTION, {len(places)} spans, {len(found)} rows")
            continue
        sizes = [b - a for a, b in places]
        if sizes:
            print(f"  row bytes: total {sum(sizes)}, the rest of the answer {size - sum(sizes)};"
                  f" per row min {min(sizes)}, max {max(sizes)}, mean {statistics.mean(sizes):.0f}")
        classes = {}
        for v in found:
            key = (v.findtext("BRIDGEVCHISSALES") or "").strip() if v.find("BRIDGEVCHISSALES") is not None else "<absent>"
            classes[key] = classes.get(key, 0) + 1
        print(f"  BRIDGEVCHISSALES per row: {sorted(classes.items())}")
        print(f"  rows numbered {SEED}: {sum(1 for v in found if (v.findtext('VOUCHERNUMBER') or '').strip() == SEED)};"
              f" rows numbered {DROPPED}: {sum(1 for v in found if (v.findtext('VOUCHERNUMBER') or '').strip() == DROPPED)}")
        for n, (v, b) in enumerate(zip(found, sizes), 1):
            cells = " ".join(f"{t}={show(t, v.findtext(t))!r}" if v.find(t) is not None else f"{t}=<absent>" for t in ROW_TAGS)
            irn = " ".join(f"{t}:{len(v.findall(t))}x{len((v.findtext(t) or '').strip())}" for t in IRN_TAGS)
            print(f"  row {n}: {b} bytes, {len(list(v))} children, {cells}, {irn}")
# Each Y answer's two numbered rows against the N answer of the same label and number.
for name, (root, _, _) in parsed.items():
    m = re.fullmatch(r"a-(.+)-Y-all\.xml", name)
    if not m:
        continue
    label = m.group(1)
    for number, request in ((SEED, "N-0051.xml"), (DROPPED, "N-z051.xml")):
        other = parsed.get(f"a-{label}-{request}")
        if other is None:
            continue
        mine = [shape(v) for v in rows(root) if (v.findtext("VOUCHERNUMBER") or "").strip() == number]
        theirs = [shape(v) for v in rows(other[0])]
        print(f"Y-all ({label}) against {request}: rows {len(mine)} and {len(theirs)}; element for element equal: {'yes' if mine == theirs else 'no'}")
# Every pair of Y answers: the same rows, element for element?
ys = [n for n in parsed if n.endswith("-Y-all.xml")]
for a in ys:
    for b in ys:
        if a < b:
            same = [shape(v) for v in rows(parsed[a][0])] == [shape(v) for v in rows(parsed[b][0])]
            print(f"{a} against {b}: rows {len(rows(parsed[a][0]))} and {len(rows(parsed[b][0]))}; element for element equal: {'yes' if same else 'no'}")
