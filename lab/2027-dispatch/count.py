#!/usr/bin/env python3
# count.py: prints shapes and counts for every answer run.py wrote beside this file (a-<label>-<request>), with an XML
# parser, never a text search. It prints tags, counts, lengths, yes or no, and only the values this sheet invented
# or names; any other value prints as its length. Copy its whole output into RESULTS.md.
# Example: python3 -I count.py
import pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
GUID = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
DISPATCH_WORDS = ["BASICSHIPDOCUMENTNO", "BASICSHIPPEDBY", "BASICFINALDESTINATION", "EICHECKPOST",
                  "BILLOFLADINGNO", "BILLOFLADINGDATE", "BASICSHIPVESSELNO"]
KEYED = ["DN-71", "DD-72", "ROADWAY", "Mysuru", "CARRIER-73", "LR-74", "KA-75"]
SHOWN = set(KEYED) | {"1", "2", "3", "Sales", "Payment", "Credit Note", "Yes", "No", "20250403"}
DISPATCH = re.compile(r"SHIP|DISPATCH|DESTINATION|CARRIER|LADING|VEHICLE|DELIVERY|TRANSPORT|CHECKPOST")
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
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
def keyed_paths(v, value):
    paths = {}
    def walk(node, path):
        for child in node:
            here_path = f"{path}/{child.tag}" if path else child.tag
            if (child.text or "").strip() == value:
                paths[here_path] = paths.get(here_path, 0) + 1
            walk(child, here_path)
    walk(v, "")
    return paths
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
    elif name.endswith("-W.xml") or name.endswith("-WD.xml"):
        for v in found:
            print(f"  row {name_of(v)}: {len(list(v))} direct children")
            for tag in DISPATCH_WORDS:
                elements = v.findall(tag)
                if not elements:
                    print(f"    {tag}: 0")
                for e in elements:
                    print(f"    {tag}: {len(elements)}, TYPE '{e.get('TYPE', '')}', '{show((e.text or '').strip())}'")
    elif "-F-" in name:
        for v in found:
            print(f"  voucher {name_of(v)}: {len(list(v))} direct children, {sum(1 for _ in v.iter())} elements")
            for value in KEYED:
                paths = keyed_paths(v, value)
                print(f"    {value}: {sum(paths.values())} elements" + "".join(f"; {p} x{n}" for p, n in sorted(paths.items())))
            filled = [(c.tag, (c.text or "").strip()) for c in v if DISPATCH.search(c.tag) and (c.text or "").strip()
                      and not c.tag.endswith(".LIST")]
            print(f"    direct children matching the dispatch names, non-empty: {len(filled)}")
            for tag, value in filled:
                print(f"      {tag} '{show(value)}'")
            lists = [c for c in v if DISPATCH.search(c.tag) and c.tag.endswith(".LIST")]
            print(f"    direct .LIST children matching the dispatch names: {len(lists)}"
                  + "".join(f"; {c.tag} with {sum(1 for _ in c.iter()) - 1} elements" for c in lists))
# Each WD answer against the W answer of the same label: the row's three questions.
for name, (root, _, _) in parsed.items():
    m = re.fullmatch(r"a-(.+)-WD\.xml", name)
    if not m:
        continue
    plain = parsed.get(f"a-{m.group(1)}-W.xml")
    with_words, without = rows(root), rows(plain[0]) if plain else []
    once = bool(with_words) and all(len(v.findall(t)) == 1 for v in with_words for t in DISPATCH_WORDS)
    def rest(v):
        return [c for c in v if c.tag not in DISPATCH_WORDS]
    otherwise = bool(with_words) and len(with_words) == len(without) and all(
        [shape(c) for c in rest(a)] == [shape(c) for c in list(b)] for a, b in zip(with_words, without))
    whole = None
    if plain:
        stripped = ET.fromstring(ET.tostring(root))
        for v in rows(stripped):
            for t in DISPATCH_WORDS:
                for e in v.findall(t):
                    v.remove(e)
        whole = shape(stripped) == shape(plain[0])
    print(f"WD ({m.group(1)}) against W: rows {len(with_words)} and {len(without)};"
          f" each dispatch word once in every row: {'yes' if once else 'no'};"
          f" rows otherwise equal: {'yes' if otherwise else 'no'};"
          f" whole answer otherwise equal: {'yes' if whole else 'no'}")
