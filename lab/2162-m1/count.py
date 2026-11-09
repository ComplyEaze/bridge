#!/usr/bin/env python3
# count.py: count7.py of row 2162-S7 (lab/2162-s7-sheet) with the books, names and comparisons of row 2162-M1. It reads
# every answer run.py wrote beside this file (a-<label>-<request>) with an XML parser, never a text search, and its
# output stays bounded: counts, sizes, the first and last date, and at most 20 lines of rows per answer. Both books are
# synthetic, so a row's number and type name print whole when they have the invoice-number form; any other value
# prints as its length. Copy its whole output into RESULTS.md.
# A row is a VOUCHER element whose parent is COLLECTION and which has a VOUCHERNUMBER child. An answer with no
# BODY/DATA/COLLECTION, or that does not parse, prints STOP and the script exits 1 after the other answers.
# An answer still marked .pending is a send that never ended, and is reported as no answer, never as 0 rows.
# Examples: python3 -I count.py a-a1-*          (the answers named; with no names, every answer beside this file)
#           python3 -I count.py --compare A     (part A's comparisons; C for part C)
import collections, hashlib, pathlib, re, sys
import xml.parsers.expat
here = pathlib.Path(__file__).resolve().parent
GUIDS = {"A": "6b43e498-430c-4d5c-bfef-d32e2ab93c85", "C": "eb2f5190-292e-4ed8-bfd3-eace839fe699"}
MARKS = {"A-M-marks.xml": "A", "C-M-marks.xml": "C"}
ROW_TAGS = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "ISCANCELLED", "BRIDGEVCHISSALES"]
IRN_TAGS = ["IRN", "IRNACKNO", "IRNACKDATE"]
WORD = re.compile(r"[A-Za-z0-9/ -]{1,32}")
SHOWN = 20
failed = False
def show(tag, text):
    text = (text or "").strip()
    if text == "" or text in ("Yes", "No") or re.fullmatch(r"\d{8}", text):
        return text
    if tag in ("VOUCHERNUMBER", "VOUCHERTYPENAME") and WORD.fullmatch(text):
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
    # length, so every offset is still the answer's own.
    text = re.sub(r"&#(\d+);", lambda m: " " * len(m.group(0)) if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), text)
    return raw, text
class Row:
    def __init__(self):
        self.closed, self.cells = False, []
    def first(self, tag):
        for depth, name, _, text in self.cells:
            if depth == 1 and name == tag:
                return text
        return None
    def count(self, tag):
        return sum(1 for depth, name, _, _ in self.cells if depth == 1 and name == tag)
def parse(text):
    # One pass of expat: each VOUCHER under COLLECTION, with every element inside it as (depth, tag, attributes,
    # text); and whether ENVELOPE/BODY/DATA/COLLECTION was seen.
    rows, stack, texts, top, seen = [], [], [], {}, {"collection": False}
    p = xml.parsers.expat.ParserCreate("UTF-16LE")
    def start(name, attrs):
        if name == "COLLECTION" and stack[-2:] == ["BODY", "DATA"]:
            seen["collection"] = True
        if name == "VOUCHER" and stack and stack[-1] == "COLLECTION":
            rows.append(Row())
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
            row.closed = True
            top.clear()
    def chars(data):
        cell = texts[-1] if texts else None
        if isinstance(cell, list) and len(cell) == 4:
            cell[3] = (cell[3] or "") + data
    p.StartElementHandler, p.EndElementHandler, p.CharacterDataHandler = start, end, chars
    p.Parse(text.encode("utf-16-le"), True)
    for row in rows:
        row.cells = [(d, n, a, (t or "").strip()) for d, n, a, t in row.cells]
    return rows, seen["collection"]
def marks_of(text):
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
    return marks
def tally(values):
    shown = sorted(values.items(), key=lambda kv: (-kv[1], kv[0]))
    out = "; ".join(f"{k!r} {v}" for k, v in shown[:SHOWN])
    return out + (f"; and {len(shown) - SHOWN} more" if len(shown) > SHOWN else "")
def name_of(path):
    m = re.fullmatch(r"a-([A-Za-z0-9]+)-(.+\.xml)", path.name)
    return m.groups() if m else None
def read(path):
    # (label, request, rows) for a month or year answer, (label, request, marks) for a marks answer, or None.
    global failed
    named = name_of(path)
    if named is None:
        print(f"{path.name}: not a name run.py writes; skipped")
        return None
    label, request = named
    raw = path.read_bytes()
    print(f"{path.name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
    try:
        raw, text = load(path)
    except UnicodeDecodeError as error:
        print(f"  STOP: not UTF-16LE text ({error.reason} at byte {error.start})")
        failed = True
        return None
    try:
        if request in MARKS:
            marks = marks_of(text)
            book = GUIDS[MARKS[request]]
            print(f"  COMPANY GUIDs {len(marks['GUID'])}; the book's: {'yes' if marks['GUID'] == [book] else 'no'};"
                  f" ALTVCHID {marks['ALTVCHID']}; ALTMSTID {marks['ALTMSTID']}")
            return label, request, marks
        rows, collection = parse(text)
    except xml.parsers.expat.ExpatError as error:
        print(f"  STOP: does not parse ({error})")
        failed = True
        return None
    if not collection:
        print("  STOP: no BODY/DATA/COLLECTION: the request failed, never 0 rows")
        failed = True
        return None
    numbered = [r for r in rows if r.first("VOUCHERNUMBER") is not None]
    whole = [r for r in rows if r.closed]
    if len(whole) != len(rows) or len(numbered) != len(rows):
        print(f"  STOP: {len(rows)} VOUCHER elements under COLLECTION, {len(whole)} closed, {len(numbered)} with VOUCHERNUMBER")
        failed = True
        return None
    print(f"  rows {len(rows)}")
    if rows:
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
        sales = [r for r in rows if r.first("BRIDGEVCHISSALES") == "Yes"]
        shared = collections.Counter(folded(r.first("VOUCHERNUMBER")) for r in sales)
        print(f"  Sales-class rows {len(sales)}: distinct folded numbers {len(shared)},"
              f" rows whose folded number another Sales-class row also has {sum(v for v in shared.values() if v > 1)}")
        for r in sales[:SHOWN]:
            print(f"    {r.first('DATE')} {show('VOUCHERNUMBER', r.first('VOUCHERNUMBER'))!r}"
                  f" {show('VOUCHERTYPENAME', r.first('VOUCHERTYPENAME'))!r} IRN length {len(r.first('IRN') or '')}")
        if len(sales) > SHOWN:
            print(f"    and {len(sales) - SHOWN} more")
    return label, request, rows
def compare(part):
    # The part's comparisons, over every answer of that part beside this file.
    global failed
    prefix = part.lower()
    answers = {}
    # Part A's answers, and part B's closing marks (bend), which are read on the same book after the GSTR-1 look.
    labels = ("a", "b") if part == "A" else ("c",)
    for path in sorted(p for l in labels for p in here.glob(f"a-{l}*-*.xml")):
        got = read(path)
        if got is not None:
            answers[path.name] = got
    print(f"== part {part}: comparisons")
    marks = [(l, m) for l, r, m in answers.values() if r in MARKS]
    for (la, ma), (lb, mb) in zip(marks, marks[1:]):
        print(f"marks {la} against {lb}: the same COMPANY row (GUID, ALTVCHID, ALTMSTID): {'yes' if ma == mb else 'no'}")
    months, total, firsts = [], 0, []
    for name in sorted(answers):
        label, request, rows = answers[name]
        m = re.fullmatch(rf"{part}-Y0-(\d{{4}})-(\d{{2}})\.xml", request)
        if not m or not re.fullmatch(rf"{prefix}\d{{2}}a", label):
            continue
        month = m.group(1) + m.group(2)
        outside = sum(1 for r in rows if not (r.first("DATE") or "").startswith(month))
        twin = [v for v in answers.values() if v[0] == label[:-1] + "b" and v[1] == request]
        same = "no second send" if not twin else ("yes" if [r.cells for r in rows] == [r.cells for r in twin[0][2]] else "no")
        months.append(month)
        total += len(rows)
        firsts.append((month, rows))
        print(f"{label} {month}: rows {len(rows)}, dated outside the month {outside}; second send element for element equal: {same}")
    print(f"months answered {len(months)} of 12, distinct {len(set(months))}; rows over the first sends {total}")
    empty = {hashlib.sha256((here / n).read_bytes()).hexdigest() for n, (l, r, rows) in answers.items()
             if r not in MARKS and not rows and r.startswith(f"{part}-Y0-2")}
    print(f"0-row month answers: distinct sha256 {len(empty)}")
    if part == "A":
        year = [rows for l, r, rows in answers.values() if r == "A-Y0-year.xml"]
        if not year:
            print("no answer to A-Y0-year.xml: the months are not compared with the year")
        else:
            joined = [r for _, rows in sorted(firsts) for r in rows]
            in_order = [r.cells for r in joined] == [r.cells for r in year[0]]
            as_set = sorted(repr(r.cells) for r in joined) == sorted(repr(r.cells) for r in year[0])
            print(f"the months' rows, in month order, against the year's: rows {len(joined)} and {len(year[0])};"
                  f" element for element equal in order: {'yes' if in_order else 'no'}; as a multiset: {'yes' if as_set else 'no'}")
            if not as_set:
                mine = collections.Counter(repr(r.cells) for r in joined)
                theirs = collections.Counter(repr(r.cells) for r in year[0])
                for label, extra in (("in the months, not the year", mine - theirs), ("in the year, not the months", theirs - mine)):
                    rows = [r for r in joined + year[0] if repr(r.cells) in extra]
                    print(f"  rows {label}: {sum(extra.values())}")
                    for r in rows[:SHOWN]:
                        print(f"    {r.first('DATE')} {show('VOUCHERNUMBER', r.first('VOUCHERNUMBER'))!r}")
for pending in sorted(here.glob("a-*.pending")):
    print(f"{pending.name}: no answer, the send never ended (marked {pending.read_text().strip()!r})")
args = sys.argv[1:]
if args[:1] == ["--compare"]:
    if args[1:] not in (["A"], ["C"]):
        sys.exit("--compare takes A or C")
    compare(args[1])
else:
    paths = [pathlib.Path(a) for a in args] if args else sorted(here.glob("a-*-*.xml"))
    if not paths:
        sys.exit("no answers beside count.py")
    for path in paths:
        read(path if path.is_absolute() or path.exists() else here / path.name)
sys.exit(1 if failed else 0)
