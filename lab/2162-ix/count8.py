#!/usr/bin/env python3
# count8.py: count7.py of row 2162-S7 (lab/2162-s7-sheet) with the file names and comparisons of the index rows L1 to L5.
# Rows are read only at ./BODY/DATA/COLLECTION/<ROW> (ENVELOPE, BODY, DATA, COLLECTION, then the row): CMPINFO carries
# bare count lines and is never a row. count7.py was count6.py of row 2162-S6 with the names of row 2162-S7.
# count6.py was count.py of row 2162-S5 (lab/2162-s5-sheet), reshaped for answers of 30,000 rows. It reads every answer
# timed.py wrote beside this file (a-<label>-<request>) with an XML parser, never a text search, and its output stays
# bounded: counts, sizes and lengths, the first 3 rows of each answer, and at most 20 lines of names. A value prints
# only if this sheet names it or the lab's own series uses it; any other value prints as its length. Copy its whole
# output into RESULTS.md.
# A row is a VOUCHER element whose parent is COLLECTION and which has a VOUCHERNUMBER child. A row's bytes are the
# bytes of the answer as sent, from the start of its VOUCHER start tag to the end of its end tag, found by expat's byte
# index over the UTF-16LE answer.
# An answer still marked .pending is a send that never ended, and is reported as no answer, never as 0 rows.
# Example: python3 -I count8.py
import collections, datetime, hashlib, pathlib, re, statistics, sys
import xml.parsers.expat
here = pathlib.Path(__file__).resolve().parent
GUID = "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3"
ROW_TAGS = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "ISCANCELLED", "BRIDGEVCHISSALES"]
IRN_TAGS = ["IRN", "IRNACKNO", "IRNACKDATE"]
TYPES = {"Sales", "Purchase", "Payment", "Receipt", "Journal", "Contra", "Credit Note", "Debit Note"}
NUMBER = re.compile(r"\d{1,6}")
SHOWN = 20
def show(tag, text):
    text = (text or "").strip()
    if text == "" or text in ("Yes", "No") or re.fullmatch(r"\d{8}", text):
        return text
    if tag == "VOUCHERNUMBER" and NUMBER.fullmatch(text):
        return text
    if tag == "VOUCHERTYPENAME" and (text in TYPES or text.startswith("BRIDGE ")):
        return text
    return f"<{len(text)} chars>"
def folded(number):
    # The rule adopted for #2162 on 4 Nov: case ignored, and the leading zeros of each digit run.
    return re.sub(r"\d+", lambda m: m.group(0).lstrip("0") or "0", number.strip().upper())
def load(path):
    raw = path.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    text = body.decode("utf-16-le")
    # Tally writes control characters as numeric references XML 1.0 refuses. Each is blanked with spaces of the same
    # length, so every byte offset below is still the answer's own.
    text = re.sub(r"&#(\d+);", lambda m: " " * len(m.group(0)) if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    return raw, text, len(raw) - len(body)
class Row:
    def __init__(self, start):
        self.start, self.end, self.cells = start, None, []
    def first(self, tag):
        # The text of the row's first direct child named tag, or None if it has none.
        for depth, name, _, text in self.cells:
            if depth == 1 and name == tag:
                return text
        return None
    def count(self, tag):
        return sum(1 for depth, name, _, _ in self.cells if depth == 1 and name == tag)
def parse(text, lead):
    # One pass of expat: each VOUCHER under COLLECTION, with every element inside it as (depth, tag, attributes, text).
    rows, stack, texts, top = [], [], [], {}
    p = xml.parsers.expat.ParserCreate("UTF-16LE")
    def start(name, attrs):
        if name == "VOUCHER" and stack == ["ENVELOPE", "BODY", "DATA", "COLLECTION"]:
            rows.append(Row(p.CurrentByteIndex + lead))
            top["row"], top["depth"] = rows[-1], len(stack)
        stack.append(name)
        texts.append([])
        row = top.get("row")
        if row is not None and len(stack) - 1 > top["depth"]:
            row.cells.append([len(stack) - 1 - top["depth"], name, sorted(attrs.items()), None])
            texts[-1] = row.cells[-1]
    def end(name):
        cell = texts.pop()
        stack.pop()
        if isinstance(cell, list) and len(cell) == 4 and cell[3] is None:
            cell[3] = ""
        row = top.get("row")
        if row is not None and name == "VOUCHER" and len(stack) == top["depth"]:
            row.end = p.CurrentByteIndex + lead + 2 * len("</VOUCHER>")
            top.clear()
    def chars(data):
        cell = texts[-1] if texts else None
        if isinstance(cell, list) and len(cell) == 4:
            cell[3] = (cell[3] or "") + data
    p.StartElementHandler, p.EndElementHandler, p.CharacterDataHandler = start, end, chars
    p.Parse(text.encode("utf-16-le"), True)
    for row in rows:
        row.cells = [(d, n, a, (t or "").strip()) for d, n, a, t in row.cells]
    return rows
def tally(values):
    shown = sorted(values.items(), key=lambda kv: (-kv[1], kv[0]))
    out = "; ".join(f"{k!r} {v}" for k, v in shown[:SHOWN])
    return out + (f"; and {len(shown) - SHOWN} more" if len(shown) > SHOWN else "")
for pending in sorted(here.glob("a-*.pending")):
    print(f"{pending.name}: no answer, the send never ended (marked {pending.read_text().strip()!r})")
answers = sorted(p for p in here.glob("a-*-*.xml"))
if not answers:
    sys.exit("no answers beside count8.py")
parsed = {}
for path in answers:
    m = re.fullmatch(r"a-([A-Za-z0-9]+)-(.+\.xml)", path.name)
    if not m:
        print(f"{path.name}: not a name timed.py writes; skipped")
        continue
    label, request = m.groups()
    raw, text, lead = load(path)
    print(f"{path.name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
    try:
        rows = parse(text, lead)
    except xml.parsers.expat.ExpatError as error:
        print(f"  does not parse ({error}); stop and report it")
        continue
    if request in ("M-pilot.xml", "M-dense.xml", "M-billtrail.xml"):
        # The marks answer is one small COMPANY row; its three fields are read with expat, as above.
        marks = {"GUID": [], "ALTVCHID": [], "ALTMSTID": []}
        q, path_ = xml.parsers.expat.ParserCreate("UTF-16LE"), []
        def s(name, attrs): path_.append([name, ""])
        def e(name):
            tag, value = path_.pop()
            if tag in marks and [n for n, _ in path_] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "COMPANY"]:
                marks[tag].append(value.strip())
        def c(data):
            if path_: path_[-1][1] += data
        q.StartElementHandler, q.EndElementHandler, q.CharacterDataHandler = s, e, c
        q.Parse(text.encode("utf-16-le"), True)
        print(f"  COMPANY GUIDs {len(marks['GUID'])}; the dense book's: {'yes' if marks['GUID'] == [GUID] else 'no'};"
              f" ALTVCHID {marks['ALTVCHID']}; ALTMSTID {marks['ALTMSTID']}")
        continue
    numbered = [r for r in rows if r.first("VOUCHERNUMBER") is not None]
    whole = [r for r in rows if r.end is not None]
    if len(whole) != len(rows) or len(numbered) != len(rows):
        print(f"  STOP: {len(rows)} VOUCHER elements under COLLECTION, {len(whole)} closed, {len(numbered)} with VOUCHERNUMBER")
        continue
    parsed[path.name] = (label, request, rows)
    print(f"  rows {len(rows)}")
    if not rows:
        continue
    sizes = [r.end - r.start for r in rows]
    print(f"  row bytes: total {sum(sizes)}, the rest of the answer {len(raw) - sum(sizes)};"
          f" per row min {min(sizes)}, max {max(sizes)}, mean {statistics.mean(sizes):.0f}")
    classes = collections.Counter(r.first("BRIDGEVCHISSALES") if r.first("BRIDGEVCHISSALES") is not None else "<absent>" for r in rows)
    print(f"  BRIDGEVCHISSALES: {tally(classes)}")
    print(f"  VOUCHERTYPENAME: {tally(collections.Counter(show('VOUCHERTYPENAME', r.first('VOUCHERTYPENAME')) for r in rows))}")
    print(f"  ISCANCELLED: {tally(collections.Counter(r.first('ISCANCELLED') for r in rows))}")
    dates = [r.first("DATE") for r in rows]
    print(f"  DATE: first {min(dates)}, last {max(dates)}, distinct {len(set(dates))}")
    for tag in IRN_TAGS:
        once = sum(1 for r in rows if r.count(tag) == 1)
        filled = sum(1 for r in rows if (r.first(tag) or "") != "")
        print(f"  {tag}: once in {once} rows, not empty in {filled}")
    sales = [r.first("VOUCHERNUMBER") for r in rows if r.first("BRIDGEVCHISSALES") == "Yes"]
    shared = collections.Counter(folded(n) for n in sales)
    print(f"  Sales-class rows {len(sales)}: distinct numbers {len(set(sales))}, distinct folded {len(shared)},"
          f" rows whose folded number another Sales-class row also has {sum(v for v in shared.values() if v > 1)}")
    for n, (r, b) in enumerate(zip(rows[:3], sizes), 1):
        cells = " ".join(f"{t}={show(t, r.first(t))!r}" if r.first(t) is not None else f"{t}=<absent>" for t in ROW_TAGS)
        irn = " ".join(f"{t}:{r.count(t)}x{len(r.first(t) or '')}" for t in IRN_TAGS)
        print(f"  row {n}: {b} bytes, {sum(1 for c in r.cells if c[0] == 1)} children, {cells}, {irn}")
def shapes(rows):
    return [r.cells for r in rows]
names = sorted(parsed)
# The same request sent twice: the same rows, element for element?
for a in names:
    for b in names:
        if a < b and parsed[a][1] == parsed[b][1]:
            same = shapes(parsed[a][2]) == shapes(parsed[b][2])
            print(f"{a} against {b}: rows {len(parsed[a][2])} and {len(parsed[b][2])}; element for element equal: {'yes' if same else 'no'}")
def without(r, tag):
    return [c for c in r.cells if not (c[0] == 1 and c[1] == tag)]
def by_request(name):
    return [a for a in names if parsed[a][1] == name]
# L3a (Y0 with ALTERID fetched) against A-Y0-year: the same rows, each with one ALTERID more?
for a in by_request("L3a-pilot.xml"):
    for b in by_request("A-Y0-year.xml"):
        mine, theirs = parsed[a][2], parsed[b][2]
        once = sum(1 for r in mine if r.count("ALTERID") == 1 and re.fullmatch(r"[0-9]+", r.first("ALTERID") or ""))
        same = [without(r, "ALTERID") for r in mine] == shapes(theirs)
        print(f"{a} against {b}: rows {len(mine)} and {len(theirs)}; rows with one numeric ALTERID {once};"
              f" equal less ALTERID: {'yes' if same else 'no'}")
# L3b (L3a with $AlterID > K) against L3a's rows whose ALTERID exceeds K, K from k.txt.
kfile = here / "k.txt"
k = int(re.search(r"K ([0-9]+)", kfile.read_text()).group(1)) if kfile.exists() else None
for a in by_request("L3b-pilot.xml"):
    for b in by_request("L3a-pilot.xml"):
        if k is None:
            print(f"{a}: no k.txt, so no comparison")
            continue
        mine = parsed[a][2]
        theirs = [r for r in parsed[b][2] if int(r.first("ALTERID") or "-1") > k]
        same = shapes(mine) == shapes(theirs)
        print(f"{a} against the rows of {b} with ALTERID above {k}: rows {len(mine)} and {len(theirs)}; element for element equal: {'yes' if same else 'no'}")
# L3b-dense: every row's ALTERID above 29800, and each row less ALTERID is a row of S6's kept Y0-year answer.
S6_YEAR = "1db925db90f703148e76c2971f6bd9ad1fc2980cb5f81c14d0e6ca5f6e8c78b6"
for a in by_request("L3b-dense.xml"):
    mine = parsed[a][2]
    above = sum(1 for r in mine if int(r.first("ALTERID") or "-1") > 29800)
    print(f"{a}: rows {len(mine)}, with ALTERID above 29800 {above}")
    kept = here / "s6-Y0-year.xml"
    if not kept.exists():
        print("  s6-Y0-year.xml is not beside count8.py: no comparison")
    elif hashlib.sha256(kept.read_bytes()).hexdigest() != S6_YEAR:
        print("  s6-Y0-year.xml is not S6's kept answer by sha256: STOP")
    else:
        raw, text, lead = load(kept)
        year = {repr(r.cells) for r in parse(text, lead)}
        found = sum(1 for r in mine if repr(without(r, "ALTERID")) in year)
        print(f"  rows found in S6's Y0-year, less ALTERID: {found} of {len(mine)}")
# L5: every send of B-Y0-month against the first, in label order.
sends = sorted(by_request("B-Y0-month.xml"), key=lambda a: parsed[a][0])
if sends:
    first = shapes(parsed[sends[0]][2])
    equal = sum(1 for a in sends if shapes(parsed[a][2]) == first)
    print(f"B-Y0-month sends {len(sends)}; element for element equal to {sends[0]}: {equal}")
# Timings, from timed.py's log lines.
def stamp(s):
    return datetime.datetime.strptime(s, "%Y-%m-%dT%H:%M:%S.%fZ")
for log in sorted(here.glob("log-*.txt")):
    for line in log.read_text().splitlines():
        f = line.split("\t")
        if len(f) != 6:
            print(f"{log.name}: a line timed.py does not write: {len(line)} chars")
            continue
        code, first, total, size = (f[5].split() + ["-"] * 4)[:4]
        wall = (stamp(f[1]) - stamp(f[0])).total_seconds()
        print(f"{f[3]}: {f[4]}, HTTP {code}, first byte {first} s, curl total {total} s, wall {wall:.2f} s, {size} bytes")
