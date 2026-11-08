#!/usr/bin/env python3
# count7.py: count6.py of row 2162-S6 (lab/2162-s6-sheet) with the file names of row 2162-S7 and its comparisons.
# count6.py was count.py of row 2162-S5 (lab/2162-s5-sheet), reshaped for answers of 30,000 rows. It reads every answer
# timed.py wrote beside this file (a-<label>-<request>) with an XML parser, never a text search, and its output stays
# bounded: counts, sizes and lengths, the first 3 rows of each answer, and at most 20 lines of names. A value prints
# only if this sheet names it or the lab's own series uses it; any other value prints as its length. Copy its whole
# output into RESULTS.md.
# A row is a VOUCHER element whose parent is COLLECTION and which has a VOUCHERNUMBER child. A row's bytes are the
# bytes of the answer as sent, from the start of its VOUCHER start tag to the end of its end tag, found by expat's byte
# index over the UTF-16LE answer.
# An answer still marked .pending is a send that never ended, and is reported as no answer, never as 0 rows.
# Example: python3 -I count7.py
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
        if name == "VOUCHER" and stack and stack[-1] == "COLLECTION":
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
    sys.exit("no answers beside count7.py")
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
    if request in ("M-marks.xml", "A-M-marks.xml"):
        # The marks answer is one small COMPANY row; its three fields are read with expat, as above.
        marks = {"GUID": [], "ALTVCHID": [], "ALTMSTID": []}
        q, path_ = xml.parsers.expat.ParserCreate("UTF-16LE"), []
        def s(name, attrs): path_.append([name, ""])
        def e(name):
            tag, value = path_.pop()
            if tag in marks and path_ and path_[-1][0] == "COMPANY":
                marks[tag].append(value.strip())
        def c(data):
            if path_: path_[-1][1] += data
        q.StartElementHandler, q.EndElementHandler, q.CharacterDataHandler = s, e, c
        q.Parse(text.encode("utf-16-le"), True)
        print(f"  COMPANY GUIDs {len(marks['GUID'])}; the book's: {'yes' if marks['GUID'] == [GUID] else 'no'};"
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
# Y1 (the filter) against Y0 (as built) over the same window and book: Y1's rows are Y0's Sales-class rows? If not,
# the Sales-class rows Y1 lacks and the rows Y1 holds that are not Sales class, at most SHOWN lines each.
def key(r):
    return (r.first("DATE"), r.first("VOUCHERNUMBER"), r.first("VOUCHERTYPENAME"))
for a in names:
    m = re.fullmatch(r"([AB])-Y1-(\w+)\.xml", parsed[a][1])
    if not m:
        continue
    for b in names:
        if parsed[b][1] == f"{m.group(1)}-Y0-{m.group(2)}.xml":
            mine, every = parsed[a][2], parsed[b][2]
            theirs = [r for r in every if r.first("BRIDGEVCHISSALES") == "Yes"]
            same = shapes(mine) == shapes(theirs)
            print(f"{a} against the Sales-class rows of {b}: rows {len(mine)} and {len(theirs)} (of {len(every)});"
                  f" element for element equal: {'yes' if same else 'no'}")
            if not same:
                kept = {repr(r.cells) for r in mine}
                lost = [r for r in theirs if repr(r.cells) not in kept]
                extra = [r for r in mine if r.first("BRIDGEVCHISSALES") != "Yes"]
                print(f"  Sales-class rows of {b} not in {a}: {len(lost)}")
                for r in lost[:SHOWN]:
                    print(f"    {tuple(show(t, v) for t, v in zip(('DATE', 'VOUCHERNUMBER', 'VOUCHERTYPENAME'), key(r)))}")
                print(f"  rows of {a} that are not Sales class: {len(extra)}")
                for r in extra[:SHOWN]:
                    print(f"    {tuple(show(t, v) for t, v in zip(('DATE', 'VOUCHERNUMBER', 'VOUCHERTYPENAME'), key(r)))}")
# N1 (the number read for "1" over April 2025) against Y0 over the same month: N1's rows are Y0's rows numbered "1"?
for a in names:
    if parsed[a][1] != "C-N1-2025-04.xml":
        continue
    for b in names:
        if parsed[b][1] == "C-Y0-2025-04.xml":
            mine, theirs = parsed[a][2], [r for r in parsed[b][2] if r.first("VOUCHERNUMBER") == "1"]
            same = shapes(mine) == shapes(theirs)
            print(f"{a} against the rows of {b} numbered '1': rows {len(mine)} and {len(theirs)}; element for element equal: {'yes' if same else 'no'}")
# The month ladder: each month's first send (label c<nn>a). Every row dated in its month, and the months' sum of rows.
months, total = [], 0
for a in names:
    label, request, rows = parsed[a]
    m = re.fullmatch(r"C-Y0-(\d{4})-(\d{2})\.xml", request)
    if not m or not re.fullmatch(r"c\d{2}a", label):
        continue
    prefix = m.group(1) + m.group(2)
    outside = sum(1 for r in rows if not (r.first("DATE") or "").startswith(prefix))
    months.append(prefix)
    total += len(rows)
    print(f"{a}: month {prefix}, rows {len(rows)}, rows dated outside the month {outside}")
if months:
    print(f"months answered {len(months)} of 12, distinct {len(set(months))}; rows over them {total};"
          f" S6's Y0-year rows 29900; equal: {'yes' if len(set(months)) == 12 and total == 29900 else 'no'}")
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
