#!/usr/bin/env python3
"""Read row 1381-RA's answers as LAB-COMMON-RA R4 says, and print what the sheet computes from them.

Row 1381-RA, with the reader of row 1381-R1's check_r1.py (lab/1381-r1 at c76e23d6: its FORBIDDEN pattern, parse
and row path); not product code. Python 3.10, standard library only. Reads only; writes nothing but standard output.

  measure.py state
      M1 alone, before any other read is sent: exactly one COMPANY row, the book's GUID, ALTMSTID 210 and no
      ALTVCHID element, the state rows 1381-O and 1108-R3 recorded (lab/1381-o-answers at cffe006d;
      lab/1108-r3-equal-voucher-answers at a64e8036, the latest to load the book). Prints `BOOK STATE MET` (exit 0)
      or `STOP-BOOK-STATE: <what differs>` (exit 1).
  measure.py all [FIXTURES]
      the ten answers: for each, bytes and sha256 and its transport verdict; C1 and C2: COMPANY rows, and
      NUMSTOCKITEMS as reported or why it is unavailable; I1 and I2: STOCKITEM rows, bound to the book, and whether
      the count equals NUMSTOCKITEMS; G1, G2, U1, U2: STOCKGROUP and UNIT rows; M1 and M2: the marks and whether
      they are equal; each pair byte-equal or not, and if not whether its rows are equal. With FIXTURES (the
      stock-create-lab capture folder at 6c9e6b33), each first read is also compared with the answer the earlier
      rows recorded for this book. Every line is a measurement, never a pass or fail of the code.
  measure.py --selftest FIXROOT SCRATCH
      the reader on committed captures (FIXROOT: src-tauri/crates/bridge-tally-protocol/tests/fixtures at
      6c9e6b33), each negative case a capture with one edit, written to SCRATCH (under local/, or outside the row
      folder).

Answers are read from local/answers/a-<label>.xml, and only when a-<label>.transport.txt ends `VERDICT SENT-OK`
(send.py). An answer whose send failed, that is empty, does not parse, or is not a success ENVELOPE holding
BODY/DATA/COLLECTION is `NOT READ` with the reason: never zero rows. Exit 0: every answer read (state: met).
Exit 1 (state only): STOP-BOOK-STATE. Exit 2: an answer not read, or a file missing.

Rows are read only at ./BODY/DATA/COLLECTION/<ROW>: the CMPINFO block above holds bare count lines with the same tag
names, which are not rows. GUIDs are printed by their first 8 characters. COMPANYNUMBER is never printed.
"""

import hashlib
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parent
ANSWERS = ROOT / "local" / "answers"
BOOK_GUID = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1"  # scripts/merge-gate-synthetic-uuids.json on master
BOOK_MARK = "210"  # ALTMSTID in every read of the book at its BK0 state (lab/1381-o-answers at cffe006d)
LABELS = ("M1", "C1", "C2", "I1", "I2", "G1", "G2", "U1", "U2", "M2")
ROW_TAG = {"M": "COMPANY", "C": "COMPANY", "I": "STOCKITEM", "G": "STOCKGROUP", "U": "UNIT"}
# The earlier rows' answers for the book at BK0 (lab/1381-o-answers at cffe006d, kept-local.txt: a-m0, a-a1, a-g1,
# a-u1), committed byte-equal as the stock-create-lab captures (their PROVENANCE.md).
RECORDED = {
    "M1": ("stock-create-lab-marks.utf16le.xml", "ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1"),
    "I1": ("stock-create-lab-items-before.utf16le.xml",
           "92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4"),
    "G1": ("stock-create-lab-groups.utf16le.xml", "b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a"),
    "U1": ("stock-create-lab-units.utf16le.xml", "cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39"),
}
# check_r1.py's pattern: the characters XML forbids, raw or as references, except 9, 10 and 13.
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)


class NotRead(Exception):
    pass


def parse(raw):
    """The ENVELOPE of one answer's bytes, or NotRead with the reason."""
    if not raw:
        raise NotRead("empty answer")
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    try:
        text = body.decode("utf-16-le")
    except UnicodeDecodeError:
        raise NotRead("not UTF-16LE")
    try:
        root = ET.fromstring(FORBIDDEN.sub("", text))
    except ET.ParseError as error:
        raise NotRead(f"does not parse ({error})")
    if root.tag != "ENVELOPE":
        raise NotRead(f"the root is {root.tag}, not ENVELOPE")
    if (root.findtext("./HEADER/STATUS") or "").strip() != "1":
        raise NotRead("HEADER/STATUS is not 1")
    if root.find("./BODY/DATA/COLLECTION") is None:
        raise NotRead("no BODY/DATA/COLLECTION")
    return root


def rows(root, tag):
    return root.findall(f"./BODY/DATA/COLLECTION/{tag}")


def others(root, tag):
    """Children of COLLECTION that are not rows of TAG, by tag and count."""
    seen = {}
    for child in root.find("./BODY/DATA/COLLECTION"):
        if child.tag != tag:
            seen[child.tag] = seen.get(child.tag, 0) + 1
    return seen


def canon(el):
    """check_r1.py's canon: tag, attributes, trimmed text, children in order."""
    attrs = "".join(f" {k}={v!r}" for k, v in sorted(el.attrib.items()))
    return f'<{el.tag}{attrs}>{(el.text or "").strip()}{"".join(canon(c) for c in el)}</{el.tag}>'


def row_canon(root, tag):
    return [canon(r) for r in rows(root, tag)]


def text_of(row, tag):
    """A child's text as received, or None when the child is absent; a repeated child refuses."""
    found = row.findall(tag)
    if len(found) > 1:
        raise NotRead(f"{tag} appears {len(found)} times in one row")
    return None if not found else (found[0].text or "")


def stock_count(row):
    """NUMSTOCKITEMS read as the code's item_count reads it (native_stock_summary.rs:687-698 at 6c9e6b33)."""
    value = text_of(row, "NUMSTOCKITEMS")
    if value is None:
        return None, "unavailable: absent"
    trimmed = value.strip()
    if not trimmed:
        return None, "unavailable: empty"
    if not all("0" <= c <= "9" for c in trimmed):
        return None, f"unavailable: not plain digits ({len(trimmed)} characters)"
    return int(trimmed), f"reported {int(trimmed)} (as received {value!r})"


def marks(root, guid):
    """The COMPANY rows of a marks answer: [(guid, ALTMSTID or None, ALTVCHID or None)]."""
    found = []
    for row in rows(root, "COMPANY"):
        g = (text_of(row, "GUID") or "").strip()
        m, v = text_of(row, "ALTMSTID"), text_of(row, "ALTVCHID")
        found.append((g, None if m is None else m.strip(), None if v is None else v.strip()))
    return found


def state_of(root, guid=BOOK_GUID, mark=BOOK_MARK):
    """What differs from the recorded state in M1's answer ([] when it is met)."""
    found = marks(root, guid)
    if len(found) != 1:
        return [f"{len(found)} COMPANY rows, not 1 (a company other than the book is loaded, or none)"]
    g, m, v = found[0]
    problems = []
    if g != guid:
        problems.append(f"the row's GUID is {g[:8]}..., not the book's")
    if m != mark:
        problems.append(f"ALTMSTID {m!r}, not {mark!r}")
    if v is not None:
        problems.append(f"an ALTVCHID element ({v!r}); the recorded state has none")
    return problems


def transport_ok(label, folder):
    record = folder / f"a-{label}.transport.txt"
    if not record.exists():
        raise NotRead(f"no transport record a-{label}.transport.txt: never sent, or the send did not finish")
    last = record.read_text(encoding="utf-8").splitlines()[-1:] or [""]
    if last[0] != "VERDICT SENT-OK":
        raise NotRead(f"the request failed: {last[0]}")


def load(label, folder):
    transport_ok(label, folder)
    path = folder / f"a-{label}.xml"
    if not path.exists():
        raise NotRead(f"a-{label}.xml is missing although its send says SENT-OK")
    raw = path.read_bytes()
    return raw, parse(raw)


def cmd_state(folder=ANSWERS, guid=BOOK_GUID, mark=BOOK_MARK):
    try:
        raw, root = load("M1", folder)
    except NotRead as error:
        print(f"NOT READ: M1: {error}")
        return 2
    print(f"M1: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}")
    for g, m, v in marks(root, guid):
        print(f"  COMPANY row: GUID {g[:8]}... (the book's: {'yes' if g == guid else 'no'}), "
              f"ALTMSTID {m if m is not None else 'absent'}, ALTVCHID {v if v is not None else 'absent'}")
    problems = state_of(root, guid, mark)
    if problems:
        print("STOP-BOOK-STATE: " + "; ".join(problems))
        return 1
    print("BOOK STATE MET")
    return 0


def describe(label, raw, root, guid):
    """The lines one answer gives, and the figures the comparisons need."""
    tag = ROW_TAG[label[0]]
    found = rows(root, tag)
    lines = [f"{label}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; {len(found)} {tag} row(s) at "
             f"BODY/DATA/COLLECTION"]
    extra = others(root, tag)
    if extra:
        lines.append(f"  other children of COLLECTION: {extra}")
    facts = {"rows": len(found)}
    if label[0] == "M":
        facts["marks"] = marks(root, guid)
        for g, m, v in facts["marks"]:
            lines.append(f"  GUID {g[:8]}... (the book's: {'yes' if g == guid else 'no'}), ALTMSTID "
                         f"{m if m is not None else 'absent'}, ALTVCHID {v if v is not None else 'absent'}")
    elif label[0] == "C":
        counts = []
        for row in found:
            g = (text_of(row, "GUID") or "").strip()
            n, said = stock_count(row)
            counts.append((g, n))
            lines.append(f"  GUID {g[:8]}... (the book's: {'yes' if g == guid else 'no'}); NUMSTOCKITEMS {said}")
        facts["count"] = counts[0][1] if len(counts) == 1 and counts[0][0] == guid else None
        if facts["count"] is None:
            lines.append("  no single row of the book: NUMSTOCKITEMS not taken from this answer")
    else:
        bound = 0
        for row in found:
            g = (text_of(row, "GUID") or "").strip()
            b = (text_of(row, "BRIDGECOMPANYGUID") or "").strip()
            ok = b == guid and g.startswith(guid + "-")
            bound += ok
            lines.append(f"  {row.get('NAME')!r} ({len(row.get('NAME') or '')} characters): GUID {g[:8]}...-"
                         f"{g[len(guid) + 1:] if g.startswith(guid + '-') else '?'}, bound to the book: "
                         f"{'yes' if ok else 'no'}")
        lines.append(f"  rows bound to the book (GUID prefix and BRIDGECOMPANYGUID): {bound} of {len(found)}")
    return lines, facts


def cmd_all(folder=ANSWERS, fixtures=None, guid=BOOK_GUID):
    read, facts, failed = {}, {}, []
    for label in LABELS:
        try:
            raw, root = load(label, folder)
            lines, fact = describe(label, raw, root, guid)
        except NotRead as error:
            print(f"NOT READ: {label}: {error}")
            failed.append(label)
            continue
        read[label], facts[label] = (raw, root), fact
        print("\n".join(lines))
    print("-- pairs")
    for a, b in (("C1", "C2"), ("I1", "I2"), ("G1", "G2"), ("U1", "U2"), ("M1", "M2")):
        if a not in read or b not in read:
            print(f"{a}/{b}: not compared ({'/'.join(x for x in (a, b) if x not in read)} not read)")
            continue
        same = read[a][0] == read[b][0]
        tag = ROW_TAG[a[0]]
        rows_same = row_canon(read[a][1], tag) == row_canon(read[b][1], tag)
        print(f"{a}/{b}: byte-equal {'yes' if same else 'no'}; rows equal {'yes' if rows_same else 'no'}")
    print("-- the count against the rows")
    for c in ("C1", "C2"):
        for i in ("I1", "I2"):
            n = facts.get(c, {}).get("count")
            r = facts.get(i, {}).get("rows")
            if n is None or r is None:
                print(f"{i} rows against {c}'s NUMSTOCKITEMS: not comparable "
                      f"({'no count' if n is None else 'no rows'})")
            else:
                print(f"{i} rows against {c}'s NUMSTOCKITEMS: {r} and {n}: {'equal' if r == n else 'NOT equal'}")
    print("-- marks between M1 and M2")
    if "M1" in facts and "M2" in facts:
        m1 = [x for x in facts["M1"]["marks"] if x[0] == guid]
        m2 = [x for x in facts["M2"]["marks"] if x[0] == guid]
        if len(m1) == 1 and len(m2) == 1:
            print(f"M1 ALTMSTID {m1[0][1]}, ALTVCHID {m1[0][2] or 'absent'}; M2 ALTMSTID {m2[0][1]}, ALTVCHID "
                  f"{m2[0][2] or 'absent'}: {'equal' if m1[0][1:] == m2[0][1:] else 'NOT equal'}")
        else:
            print(f"the book's row: {len(m1)} in M1, {len(m2)} in M2: not comparable")
    else:
        print("not compared: M1 or M2 not read")
    if fixtures is not None:
        print("-- against the answers recorded for the book at BK0 (1381-O; committed as the stock-create-lab captures)")
        for label, (name, digest) in RECORDED.items():
            if label not in read:
                print(f"{label}: not compared (not read)")
                continue
            try:
                kept = (pathlib.Path(fixtures) / name).read_bytes()
            except OSError as error:
                print(f"NOT READ: {name}: {type(error).__name__}")
                failed.append(name)
                continue
            if hashlib.sha256(kept).hexdigest() != digest:
                print(f"NOT READ: {name} is not the recorded answer (sha256 differs)")
                failed.append(name)
                continue
            tag = ROW_TAG[label[0]]
            try:
                kept_rows = row_canon(parse(kept), tag)
            except NotRead as error:
                print(f"NOT READ: {name}: {error}")
                failed.append(name)
                continue
            print(f"{label} against {name}: byte-equal {'yes' if read[label][0] == kept else 'no'}; rows equal "
                  f"{'yes' if row_canon(read[label][1], tag) == kept_rows else 'no'}")
    if failed:
        print(f"NOT ALL READ: {', '.join(failed)}")
        return 2
    print(f"ALL {len(LABELS)} ANSWERS READ")
    return 0


def selftest(fixroot, scratch):
    fixroot, scratch = pathlib.Path(fixroot), pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    lab = fixroot / "agent" / "stock-create-lab"
    try:
        cap = {k: (lab / n).read_bytes() for k, (n, _) in RECORDED.items()}
        cap["I-two"] = (lab / "stock-create-lab-items-after-plain-create.utf16le.xml").read_bytes()
        shape = (fixroot / "company_inventory_flags_shape_lab_live.utf16le.xml").read_bytes()
    except OSError as error:
        print(f"FAILED READ: a capture: {type(error).__name__}: {error}")
        return 2
    shape_guid = "3a6bd6e1-b835-4bff-89dd-8a6af138c346"  # the GUID that capture's COMPANY row holds
    enc = lambda text: text.encode("utf-16-le")
    dec = lambda raw: raw.decode("utf-16-le")
    one = lambda raw, a, b: enc(dec(raw).replace(a, b, 1))
    # One capture of the book's count does not exist: the count answer of the tests below is SHAPE LAB's capture
    # with its GUID changed to the book's and its NUMSTOCKITEMS to 1 (two edits; a test input, not evidence).
    count = enc(dec(shape).replace(shape_guid, BOOK_GUID).replace("> 11</NUMSTOCKITEMS>", "> 1</NUMSTOCKITEMS>"))
    right = total = 0

    def case(what, got, want):
        nonlocal right, total
        total += 1
        right += got == want
        print(f"{'RIGHT' if got == want else 'WRONG'} {what}: {got!r}")

    def problems_of(raw):
        try:
            return state_of(parse(raw))
        except NotRead as error:
            return f"NOT READ: {error}"

    def count_of(raw, guid=BOOK_GUID):
        try:
            found = rows(parse(raw), "COMPANY")
            return [(r.findtext("GUID").strip()[:8], stock_count(r)[1]) for r in found]
        except NotRead as error:
            return f"NOT READ: {error}"

    m = cap["M1"]
    case("state, the recorded marks", problems_of(m), [])
    case("state, ALTMSTID 211", problems_of(one(m, "> 210</ALTMSTID>", "> 211</ALTMSTID>")),
         ["ALTMSTID '211', not '210'"])
    case("state, a voucher mark", problems_of(one(m, "<ALTMSTID", '<ALTVCHID TYPE="Number"> 1</ALTVCHID><ALTMSTID')),
         ["an ALTVCHID element ('1'); the recorded state has none"])
    two = dec(m)
    a, b = two.index("<COMPANY NAME="), two.index("</COLLECTION>")
    case("state, a second company loaded", problems_of(enc(two[:b] + two[a:b] + two[b:])),
         ["2 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    case("state, no company row (CMPINFO's COMPANY line is not a row)",
         problems_of(enc(two[:a] + two[b:])), ["0 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    case("state, another GUID", problems_of(one(m, BOOK_GUID, "ffffffff" + BOOK_GUID[8:])),
         ["the row's GUID is ffffffff..., not the book's"])
    case("state, an answer cut short", problems_of(m[:-200]).startswith("NOT READ: does not parse"), True)
    case("state, an empty answer", problems_of(b""), "NOT READ: empty answer")
    case("state, Tally's error envelope",
         problems_of(one(m, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>")), "NOT READ: HEADER/STATUS is not 1")
    case("count, the edited capture", count_of(count), [("bac2b5e9", "reported 1 (as received ' 1')")])
    case("count, the capture as kept, SHAPE LAB's row", count_of(shape), [("3a6bd6e1", "reported 11 (as received ' 11')")])
    case("count, NUMSTOCKITEMS removed", count_of(re_cut(count, "NUMSTOCKITEMS")), [("bac2b5e9", "unavailable: absent")])
    case("count, NUMSTOCKITEMS empty", count_of(one(count, "> 1</NUMSTOCKITEMS>", "></NUMSTOCKITEMS>")),
         [("bac2b5e9", "unavailable: empty")])
    case("count, NUMSTOCKITEMS grouped", count_of(one(count, "> 1</NUMSTOCKITEMS>", ">1,000</NUMSTOCKITEMS>")),
         [("bac2b5e9", "unavailable: not plain digits (5 characters)")])
    # The whole computation on a scratch answers folder made from the captures.
    folder = scratch / "answers"
    folder.mkdir(parents=True, exist_ok=True)
    sets = {"M1": m, "M2": m, "C1": count, "C2": count, "I1": cap["I1"], "I2": cap["I1"], "G1": cap["G1"],
            "G2": cap["G1"], "U1": cap["U1"], "U2": cap["U1"]}

    def lay(answers, failed=()):
        for label, raw in answers.items():
            (folder / f"a-{label}.xml").write_bytes(raw)
            verdict = "VERDICT REQUEST-FAILED: curl exit 7" if label in failed else "VERDICT SENT-OK"
            (folder / f"a-{label}.transport.txt").write_text(f"label {label}\n{verdict}\n", encoding="utf-8")

    def run_all(fixtures):
        out = scratch / "out.txt"
        saved = sys.stdout
        with open(out, "w", encoding="utf-8") as f:
            sys.stdout = f
            try:
                code = cmd_all(folder, fixtures)
            finally:
                sys.stdout = saved
        return code, out.read_text(encoding="utf-8")

    lay(sets)
    code, text = run_all(lab)
    case("all, the captures: exit", code, 0)
    for want in ("C1/C2: byte-equal yes; rows equal yes", "I1 rows against C1's NUMSTOCKITEMS: 1 and 1: equal",
                 "M1 ALTMSTID 210, ALTVCHID absent; M2 ALTMSTID 210, ALTVCHID absent: equal",
                 "I1 against stock-create-lab-items-before.utf16le.xml: byte-equal yes; rows equal yes",
                 "rows bound to the book (GUID prefix and BRIDGECOMPANYGUID): 1 of 1"):
        case(f"all, the captures: the line {want[:40]}...", want in text.splitlines() or any(
            l.strip() == want for l in text.splitlines()), True)
    lay({"I2": cap["I-two"], "M2": one(m, "> 210</ALTMSTID>", "> 211</ALTMSTID>")})
    code, text = run_all(None)
    case("all, I2 two rows and M2 moved: exit (still a measurement)", code, 0)
    for want in ("I1/I2: byte-equal no; rows equal no", "I2 rows against C2's NUMSTOCKITEMS: 2 and 1: NOT equal",
                 "M1 ALTMSTID 210, ALTVCHID absent; M2 ALTMSTID 211, ALTVCHID absent: NOT equal"):
        case(f"all, a changed pair: the line {want[:40]}...", want in text.splitlines(), True)
    lay({"I2": cap["I1"], "M2": m, "G2": enc(dec(cap["G1"]).replace("<DESC>", "<DESC> ", 1))})
    code, text = run_all(None)
    case("all, G2 differing outside its rows only", "G1/G2: byte-equal no; rows equal yes" in text.splitlines(), True)
    lay({"G2": cap["G1"], "C2": count}, failed=("C2",))
    code, text = run_all(None)
    case("all, C2's send failed: exit", code, 2)
    case("all, C2's send failed: never zero rows",
         "NOT READ: C2: the request failed: VERDICT REQUEST-FAILED: curl exit 7" in text.splitlines(), True)
    print(f"{total} cases, {right} right")
    return 0 if right == total else 1


def re_cut(raw, tag):
    """RAW with its one TAG element removed whole."""
    text = raw.decode("utf-16-le")
    start = text.index(f"<{tag}")
    end = text.index(f"</{tag}>", start) + len(f"</{tag}>")
    return text[:start].rstrip(" ").encode("utf-16-le") + text[end:].encode("utf-16-le")


def main(argv):
    try:
        if argv == ["state"]:
            return cmd_state()
        if argv[:1] == ["all"] and len(argv) <= 2:
            return cmd_all(ANSWERS, argv[1] if len(argv) == 2 else None)
        if argv[:1] == ["--selftest"] and len(argv) == 3:
            return selftest(argv[1], argv[2])
    except OSError as error:
        print(f"FAILED READ: {type(error).__name__}: {error}")
        return 2
    print("usage: measure.py state | all [FIXTURES] | --selftest FIXROOT SCRATCH")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
