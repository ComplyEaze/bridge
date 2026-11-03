#!/usr/bin/env python3
# count.py: prints shapes and counts for every answer run.py wrote beside this file (a-<label>-<request>), with an XML
# parser, never a text search. It prints tags, counts, lengths, yes or no, and only the values this sheet invented
# or names; any other value prints as its length. Copy its whole output into RESULTS.md.
# Example: python3 -I count.py
import pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
GUID = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
NUMBER_TAGS = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "ISCANCELLED", "BRIDGEVCHISSALES"]
IRN_TAGS = ["IRN", "IRNACKNO", "IRNACKDATE"]
SHOWN = {"1", "2", "Sales", "Payment", "Credit Note", "Yes", "No", "20250403", "112233",
         "DN-71", "DD-72", "ROADWAY", "Mysuru", "CARRIER-73", "LR-74", "KA-75"}
KEYED = ["DN-71", "DD-72", "ROADWAY", "Mysuru", "CARRIER-73", "LR-74", "KA-75"]
DISPATCH = re.compile(r"SHIP|DISPATCH|DESTINATION|CARRIER|LADING|VEHICLE|DELIVERY|TRANSPORT")
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
def rest(row):
    return [(c.tag, sorted(c.attrib.items()), (c.text or "").strip()) for c in row if c.tag not in IRN_TAGS]
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
    elif "-N-" in name or "-NI-" in name or name.endswith("V-named.xml"):
        for v in found:
            print(f"  row {show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('VOUCHERTYPENAME'))}: {len(list(v))} children")
            for tag in NUMBER_TAGS + IRN_TAGS:
                elements = v.findall(tag)
                if not elements:
                    print(f"    {tag}: 0")
                    continue
                for e in elements:
                    value = e.text or ""
                    shown = f"length {len(value)}" if tag == "IRN" else f"'{show(value)}'"
                    print(f"    {tag}: {len(elements)}, TYPE '{e.get('TYPE', '')}', {shown}")
    elif name.endswith("F-note.xml"):
        for v in found:
            print(f"  voucher {show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('VOUCHERTYPENAME'))}:"
                  f" {len(list(v))} direct children, {sum(1 for _ in v.iter())} elements")
            for value in KEYED:
                paths = {}
                def walk(node, path):
                    for child in node:
                        here_path = f"{path}/{child.tag}" if path else child.tag
                        if (child.text or "").strip() == value:
                            paths[here_path] = paths.get(here_path, 0) + 1
                        walk(child, here_path)
                walk(v, "")
                print(f"    {value}: {sum(paths.values())} elements" + "".join(f"; {p} x{n}" for p, n in sorted(paths.items())))
            filled = [(c.tag, (c.text or "").strip()) for c in v if DISPATCH.search(c.tag) and (c.text or "").strip()
                      and not c.tag.endswith(".LIST")]
            print(f"    direct children matching the dispatch names, non-empty: {len(filled)}")
            for tag, value in filled:
                print(f"      {tag} '{show(value)}'")
# Each NI answer against the N answer of the same number and label, and against V-named of that label.
for name, (root, _, _) in parsed.items():
    m = re.fullmatch(r"a-(.+)-NI-(\d)\.xml", name)
    if not m:
        continue
    label, number = m.groups()
    plain = parsed.get(f"a-{label}-N-{number}.xml")
    named = parsed.get(f"a-{label}-V-named.xml")
    irn_rows, plain_rows = rows(root), rows(plain[0]) if plain else []
    print(f"NI-{number} ({label}) against N-{number}: rows {len(irn_rows)} and {len(plain_rows)};"
          f" each IRN tag once in every row: {'yes' if irn_rows and all(len(r.findall(t)) == 1 for r in irn_rows for t in IRN_TAGS) else 'no'};"
          f" otherwise equal: {'yes' if irn_rows and [rest(r) for r in irn_rows] == [rest(r) for r in plain_rows] else 'no'}")
    if named:
        key = lambda r: (r.findtext("VOUCHERNUMBER"), r.findtext("VOUCHERTYPENAME"))
        by_number = {key(r): r for r in rows(named[0])}
        for r in irn_rows:
            other = by_number.get(key(r))
            same = [t for t in IRN_TAGS if other is not None and r.findtext(t) == other.findtext(t)]
            print(f"  against V-named row {show(r.findtext('VOUCHERNUMBER'))} {show(r.findtext('VOUCHERTYPENAME'))}: equal on {', '.join(same) or 'none'}")
