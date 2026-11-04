#!/usr/bin/env python3
# count.py --company-number N: prints shapes and counts for every answer run.py wrote beside this file
# (a-<label>-<request>), with an XML parser, never a text search for structure. It prints tags, counts, lengths,
# yes or no, and only the values this sheet invented or names; any other value prints as its length. N is the book's
# company number, the number in its data folder's name; it is never printed, only counted. Copy the whole output
# into RESULTS.md.
# Example: python3 -I count.py --company-number 123
import pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
if len(sys.argv) != 3 or sys.argv[1] != "--company-number" or not sys.argv[2].isdigit():
    sys.exit("usage: count.py --company-number N")
COMPANY_NUMBER = re.compile(rf"(?<!\d){re.escape(sys.argv[2])}(?!\d)")
GUID = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
DELIVERY_NOTES = "INVOICEDELNOTES.LIST"
KEYED = ["DN-81", "DN-82", "DN-83"]
DATES = ["20250401", "20250402"]
SHOWN = set(KEYED) | set(DATES) | {"1", "2", "3", "4", "Sales", "Payment", "Yes", "No", "20250403"}
DIGITS = re.compile(r"\d{10,}")
DIGITS_ALLOWED_IN = {"VOUCHERKEY"}
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
GSTIN_SYNTHETIC = re.compile(r"\d{2}ZZZZZ0000Z[A-Z\d]Z[A-Z\d]")
PAN = re.compile(r"[A-Z]{5}\d{4}[A-Z]")
PAN_SYNTHETIC = "ZZZZZ0000Z"
def show(text):
    text = text or ""
    return text if text in SHOWN or text == "" else f"<{len(text)} chars>"
def decode(raw):
    return (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le")
def parse(path):
    raw = path.read_bytes()
    text = decode(raw)
    # Tally writes control characters as numeric references XML 1.0 refuses; drop them before parsing.
    text = re.sub(r"&#(\d+);", lambda m: "" if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    return ET.fromstring(text), raw
def content_checks(root, raw):
    # The four stop rules of ROW.md, Stop. Each count is printed; any non-zero "stop" count stops the row.
    text = decode(raw)
    in_tree, outside_allowed = 0, {}
    for e in root.iter():
        for value in [e.text or ""] + list(e.attrib.values()):
            for _ in DIGITS.findall(value):
                in_tree += 1
                if e.tag not in DIGITS_ALLOWED_IN:
                    outside_allowed[e.tag] = outside_allowed.get(e.tag, 0) + 1
    in_text = len(DIGITS.findall(text))
    gstins = GSTIN.findall(text)
    other_gstins = [g for g in gstins if not GSTIN_SYNTHETIC.fullmatch(g)]
    pans = PAN.findall(text)
    other_pans = [p for p in pans if p != PAN_SYNTHETIC]
    company_utf16 = len(COMPANY_NUMBER.findall(text))
    company_utf8 = len(COMPANY_NUMBER.findall(raw.decode("utf-8", "replace")))
    stops = []
    if outside_allowed:
        stops.append("runs of 10 or more digits outside VOUCHERKEY")
    if in_text != in_tree:
        stops.append("runs of 10 or more digits the parser did not account for")
    if other_gstins:
        stops.append("GSTIN shapes outside the all-Z synthetic family")
    if other_pans:
        stops.append("PAN shapes outside the all-Z synthetic family")
    if company_utf16 or company_utf8:
        stops.append("the company number")
    print(f"  10+ digit runs: {in_text} in the text, {in_tree} by the parser,"
          f" in VOUCHERKEY {in_tree - sum(outside_allowed.values())}, elsewhere {sum(outside_allowed.values())}"
          + "".join(f"; {tag} x{n}" for tag, n in sorted(outside_allowed.items())))
    print(f"  GSTIN shapes {len(gstins)}, outside the synthetic family {len(other_gstins)};"
          f" PAN shapes {len(pans)}, outside the synthetic family {len(other_pans)};"
          f" company number in UTF-16 {company_utf16}, in UTF-8 {company_utf8}")
    print(f"  content checks: {'pass' if not stops else 'STOP: ' + '; '.join(stops)}")
    return not stops
def rows(root):
    return [v for v in root.iter("VOUCHER") if v.find("VOUCHERNUMBER") is not None]
def name_of(v):
    return f"{show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('VOUCHERTYPENAME'))}"
def shape(node):
    # Every element below node, in order, with its attributes and text: what "otherwise equal" compares.
    return [(e.tag, sorted(e.attrib.items()), (e.text or "").strip()) for e in node.iter()]
def value_paths(v, value):
    paths = {}
    def walk(node, path):
        for child in node:
            child_path = f"{path}/{child.tag}" if path else child.tag
            if (child.text or "").strip() == value:
                paths[child_path] = paths.get(child_path, 0) + 1
            walk(child, child_path)
    walk(v, "")
    return paths
def delivery_note_lists(v):
    for i, e in enumerate(v.findall(DELIVERY_NOTES), 1):
        print(f"    {DELIVERY_NOTES} {i}: {len(list(e))} children, {sum(1 for _ in e.iter()) - 1} elements")
        for d in e.iter():
            if d is not e:
                print(f"      {d.tag} TYPE '{d.get('TYPE', '')}' '{show((d.text or '').strip())}'")
answers = sorted(here.glob("a-*-*.xml"))
if not answers:
    sys.exit("no answers beside count.py")
parsed, clean = {}, True
for path in answers:
    try:
        parsed[path.name] = parse(path)
    except ET.ParseError as error:
        print(f"{path.name}: does not parse ({error}); stop and report it")
        clean = False
for name, (root, raw) in parsed.items():
    found = rows(root)
    print(f"{name}: {len(raw)} bytes, rows with VOUCHERNUMBER {len(found)}")
    clean = content_checks(root, raw) and clean
    if name.endswith("M-marks.xml"):
        companies = [c for c in root.iter("COMPANY") if (c.findtext("GUID") or "").strip()]
        print(f"  COMPANY elements with a GUID {len(companies)}")
        for c in companies:
            print(f"  GUID is the book's: {'yes' if c.findtext('GUID').strip() == GUID else 'no'};"
                  f" ALTVCHID '{(c.findtext('ALTVCHID') or '').strip()}'; ALTMSTID '{(c.findtext('ALTMSTID') or '').strip()}'")
    elif name.endswith("L-list.xml"):
        for v in found:
            print(f"  {show(v.findtext('VOUCHERNUMBER'))} {show(v.findtext('DATE'))} {show(v.findtext('VOUCHERTYPENAME'))}")
    else:
        for v in found:
            print(f"  voucher {name_of(v)}: {len(list(v))} direct children, {sum(1 for _ in v.iter())} elements,"
                  f" {DELIVERY_NOTES} {len(v.findall(DELIVERY_NOTES))}")
            for value in KEYED + DATES:
                paths = value_paths(v, value)
                print(f"    {value}: {sum(paths.values())} elements" + "".join(f"; {p} x{n}" for p, n in sorted(paths.items())))
            delivery_note_lists(v)
# GDN against GD, from the same label.
labels = sorted({m.group(1) for n in parsed if (m := re.fullmatch(r"a-(.+?)-[A-Z]+(?:-[a-z]+)?\.xml", n))})
for label in labels:
    a, b = parsed.get(f"a-{label}-GDN.xml"), parsed.get(f"a-{label}-GD.xml")
    if not a or not b:
        continue
    with_list, without = rows(a[0]), rows(b[0])
    every = bool(with_list) and all(v.findall(DELIVERY_NOTES) for v in with_list)
    counts = ", ".join(f"{name_of(v)} {len(v.findall(DELIVERY_NOTES))}" for v in with_list)
    def rest(v):
        return [c for c in v if c.tag != DELIVERY_NOTES]
    otherwise = bool(with_list) and len(with_list) == len(without) and all(
        [shape(c) for c in rest(x)] == [shape(c) for c in list(y)] for x, y in zip(with_list, without))
    stripped = ET.fromstring(ET.tostring(a[0]))
    for v in rows(stripped):
        for e in v.findall(DELIVERY_NOTES):
            v.remove(e)
    whole = shape(stripped) == shape(b[0])
    in_gd = sum(sum(value_paths(v, k).values()) for v in without for k in KEYED)
    print(f"GDN against GD ({label}): rows {len(with_list)} and {len(without)};"
          f" {DELIVERY_NOTES} in every row: {'yes' if every else 'no'} ({counts});"
          f" rows otherwise equal: {'yes' if otherwise else 'no'};"
          f" whole answer otherwise equal: {'yes' if whole else 'no'};"
          f" keyed numbers anywhere in GD: {in_gd}")
print(f"all answers parse and pass the content checks: {'yes' if clean else 'no: STOP'}")
