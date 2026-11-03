#!/usr/bin/env python3
# count.py: prints shapes and counts for every answer run.py wrote beside this file (a-<label>-<request>), with an XML
# parser, never a text search. It prints tags, counts, lengths, yes or no, and only the values this sheet names; any
# other value prints as its length. Copy its whole output into RESULTS.md.
# Example: python3 -I count.py
import pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
GUID = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
DISPATCH_WORDS = ["BASICSHIPDOCUMENTNO", "BASICSHIPPEDBY", "BASICFINALDESTINATION", "EICHECKPOST",
                  "BILLOFLADINGNO", "BILLOFLADINGDATE", "BASICSHIPVESSELNO"]
DELIVERY_NOTES = "INVOICEDELNOTES.LIST"
SHOWN = {"1", "2", "3", "Sales", "Payment", "Yes", "No", "20250403"}
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
# Each pair: the request without the added words, the request with them, and the words added.
PAIRS = [("G", "GD", DISPATCH_WORDS), ("EG", "EGD", DISPATCH_WORDS), ("CG", "CGD", DISPATCH_WORDS),
         ("CEG", "CEGD", DISPATCH_WORDS), ("GD", "GDN", [DELIVERY_NOTES])]
def show(text):
    text = text or ""
    return text if text in SHOWN or text == "" else f"<{len(text)} chars>"
def parse(path):
    raw = path.read_bytes()
    text = (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le")
    gstins = len(GSTIN.findall(text))
    # Tally writes control characters as numeric references XML 1.0 refuses; drop them before parsing.
    text = re.sub(r"&#(\d+);", lambda m: "" if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    return ET.fromstring(text), len(raw), gstins
def rows(root):
    return [v for v in root.iter("VOUCHER") if v.find("VOUCHERNUMBER") is not None]
def name_of(v):
    return f"{show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('VOUCHERTYPENAME'))}"
def shape(node):
    # Every element below node, in order, with its attributes and text: what "otherwise equal" compares.
    return [(e.tag, sorted(e.attrib.items()), (e.text or "").strip()) for e in node.iter()]
answers = sorted(here.glob("a-*-*.xml"))
if not answers:
    sys.exit("no answers beside count.py")
parsed = {}
for path in answers:
    try:
        parsed[path.name] = parse(path)
    except ET.ParseError as error:
        print(f"{path.name}: does not parse ({error}); stop and report it")
for name, (root, size, gstins) in parsed.items():
    found = rows(root)
    print(f"{name}: {size} bytes, GSTIN shapes {gstins}, rows with VOUCHERNUMBER {len(found)}")
    if name.endswith("M-marks.xml"):
        companies = [c for c in root.iter("COMPANY") if (c.findtext("GUID") or "").strip()]
        print(f"  COMPANY elements with a GUID {len(companies)}")
        for c in companies:
            print(f"  GUID is the book's: {'yes' if c.findtext('GUID').strip() == GUID else 'no'};"
                  f" ALTVCHID '{(c.findtext('ALTVCHID') or '').strip()}'; ALTMSTID '{(c.findtext('ALTMSTID') or '').strip()}'")
    elif name.endswith("L-list.xml"):
        for v in found:
            print(f"  {show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('DATE'))} {show(v.findtext('VOUCHERTYPENAME'))}")
    elif name.endswith("-GDN.xml"):
        for v in found:
            lists = v.findall(DELIVERY_NOTES)
            print(f"  row {name_of(v)}: {DELIVERY_NOTES} {len(lists)}"
                  + "".join(f"; one with {len(list(e))} children: "
                            + ", ".join(f"{c.tag} '{show((c.text or '').strip())}'" for c in e) for e in lists))
    elif re.search(r"-C?E?GD\.xml$", name):
        for v in found:
            print(f"  row {name_of(v)}: " + ", ".join(
                f"{tag} {len(v.findall(tag))} '{show((v.findtext(tag) or '').strip())}'" for tag in DISPATCH_WORDS))
# Each pair, from the same label: the row's three questions.
labels = sorted({m.group(1) for n in parsed if (m := re.fullmatch(r"a-(.+?)-[A-Z]+\.xml", n))})
for label in labels:
    for without_name, with_name, words in PAIRS:
        a, b = parsed.get(f"a-{label}-{with_name}.xml"), parsed.get(f"a-{label}-{without_name}.xml")
        if not a or not b:
            continue
        with_words, without = rows(a[0]), rows(b[0])
        once = bool(with_words) and all(len(v.findall(t)) == 1 for v in with_words for t in words)
        def rest(v):
            return [c for c in v if c.tag not in words]
        otherwise = bool(with_words) and len(with_words) == len(without) and all(
            [shape(c) for c in rest(x)] == [shape(c) for c in list(y)] for x, y in zip(with_words, without))
        stripped = ET.fromstring(ET.tostring(a[0]))
        for v in rows(stripped):
            for t in words:
                for e in v.findall(t):
                    v.remove(e)
        whole = shape(stripped) == shape(b[0])
        print(f"{with_name} against {without_name} ({label}): rows {len(with_words)} and {len(without)};"
              f" each added word once in every row: {'yes' if once else 'no'};"
              f" rows otherwise equal: {'yes' if otherwise else 'no'};"
              f" whole answer otherwise equal: {'yes' if whole else 'no'}")
