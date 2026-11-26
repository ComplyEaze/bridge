#!/usr/bin/env python3
"""Read row 1381-RR's answers as LAB-COMMON-RR R6 says, and print what the sheet computes from them.

Row 1381-RR, from row 1381-RA's measure.py (lab/1381-ra-sheet at 30f5c820, lab/1381-ra/measure.py): its reader
(FORBIDDEN pattern, parse, row path, canonical rows) and its state check, unchanged; the count read and its
NUMSTOCKITEMS reading cut (this row sends no count read); `state` takes the label it checks; `all` compares M1 with M2
and I, G and U with row 1381-RA's pushed answers. Not product code. Python 3.10, standard library only. Reads only;
writes nothing but standard output (the self-test writes only in its SCRATCH).

  measure.py state M1|M2
      one marks answer: exactly one COMPANY row, the book's GUID, ALTMSTID 210 and no ALTVCHID element (the state
      rows 1381-O, 1108-R3 and 1381-RA recorded). Prints `BOOK STATE MET` (exit 0), or for M1
      `STOP-BOOK-STATE: <what differs>` and for M2 `STOP-RESTORED-STATE: <what differs>` (exit 1).
  measure.py all RA
      the five answers: for each, bytes and sha256 and its transport verdict, its rows and how many are bound to the
      book; M1's and M2's state; M1 against M2, byte-equal and rows equal; I, G and U against row 1381-RA's answers
      a-I1.xml, a-G1.xml and a-U1.xml in the folder RA (each checked first against the sha256 pinned here), byte-equal
      and rows equal. Every line is a measurement, never a pass or fail of the code.
  measure.py --selftest RA SCRATCH
      the reader on row 1381-RA's answers (RA as above, with a-M1.xml), each negative case an answer with one edit,
      written to SCRATCH (under local/, or outside the row folder).

Answers are read from local/answers/a-<label>.xml, and only when a-<label>.transport.txt ends `VERDICT SENT-OK`
(send.py). An answer whose send failed, that is empty, does not parse, or is not a success ENVELOPE holding
BODY/DATA/COLLECTION is `NOT READ` with the reason: never zero rows. A reference file that is missing or whose sha256
is not the pinned one is `NOT READ` as well, never "differs". Exit 0: every answer and reference read (state: met).
Exit 1 (state only): the state differs. Exit 2: an answer or reference not read, or a file missing.

Rows are read only at ./BODY/DATA/COLLECTION/<ROW>: the CMPINFO block above holds bare count lines with the same tag
names, which are not rows. GUIDs are printed by their first 8 characters.
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
LABELS = ("M1", "M2", "I", "G", "U")
ROW_TAG = {"M": "COMPANY", "I": "STOCKITEM", "G": "STOCKGROUP", "U": "UNIT"}
# Row 1381-RA's pushed answers (lab/1381-ra-answers at 851e254586024e757f0bbe538d3ea7d49ed1f754, answers/): name and
# sha256, each computed from that commit's blob by git. I, G and U are compared with them; M1 is the self-test's marks.
RA = {
    "M1": ("a-M1.xml", "ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1"),
    "I": ("a-I1.xml", "92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4"),
    "G": ("a-G1.xml", "b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a"),
    "U": ("a-U1.xml", "cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39"),
}
# check_r1.py's pattern (row 1381-RA's): the characters XML forbids, raw or as references, except 9, 10 and 13.
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


def marks(root):
    """The COMPANY rows of a marks answer: [(guid, ALTMSTID or None, ALTVCHID or None)]."""
    found = []
    for row in rows(root, "COMPANY"):
        g = (text_of(row, "GUID") or "").strip()
        m, v = text_of(row, "ALTMSTID"), text_of(row, "ALTVCHID")
        found.append((g, None if m is None else m.strip(), None if v is None else v.strip()))
    return found


def state_of(root, guid=BOOK_GUID, mark=BOOK_MARK):
    """What differs from the recorded state in a marks answer ([] when it is met)."""
    found = marks(root)
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


def stop_name(label):
    return "STOP-BOOK-STATE" if label == "M1" else "STOP-RESTORED-STATE"


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


def reference(label, ra):
    """Row 1381-RA's answer for LABEL from the folder RA, its sha256 checked against the pin, or NotRead."""
    name, digest = RA[label]
    try:
        raw = (pathlib.Path(ra) / name).read_bytes()
    except OSError as error:
        raise NotRead(f"{name}: {type(error).__name__}")
    if hashlib.sha256(raw).hexdigest() != digest:
        raise NotRead(f"{name} is not row 1381-RA's answer (sha256 differs from the pin)")
    return raw, parse(raw)


def cmd_state(label, folder=ANSWERS, guid=BOOK_GUID, mark=BOOK_MARK):
    try:
        raw, root = load(label, folder)
    except NotRead as error:
        print(f"NOT READ: {label}: {error}")
        return 2
    print(f"{label}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}")
    for g, m, v in marks(root):
        print(f"  COMPANY row: GUID {g[:8]}... (the book's: {'yes' if g == guid else 'no'}), "
              f"ALTMSTID {m if m is not None else 'absent'}, ALTVCHID {v if v is not None else 'absent'}")
    problems = state_of(root, guid, mark)
    if problems:
        print(f"{stop_name(label)}: " + "; ".join(problems))
        return 1
    print("BOOK STATE MET")
    return 0


def describe(label, raw, root, guid):
    """The lines one answer gives."""
    tag = ROW_TAG[label[0]]
    found = rows(root, tag)
    lines = [f"{label}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; {len(found)} {tag} row(s) at "
             f"BODY/DATA/COLLECTION"]
    extra = others(root, tag)
    if extra:
        lines.append(f"  other children of COLLECTION: {extra}")
    if label[0] == "M":
        for g, m, v in marks(root):
            lines.append(f"  GUID {g[:8]}... (the book's: {'yes' if g == guid else 'no'}), ALTMSTID "
                         f"{m if m is not None else 'absent'}, ALTVCHID {v if v is not None else 'absent'}")
        return lines
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
    return lines


def cmd_all(folder, ra, guid=BOOK_GUID):
    read, failed = {}, []
    for label in LABELS:
        try:
            raw, root = load(label, folder)
            lines = describe(label, raw, root, guid)
        except NotRead as error:
            print(f"NOT READ: {label}: {error}")
            failed.append(label)
            continue
        read[label] = (raw, root)
        print("\n".join(lines))
    print("-- the state, before the restore (M1) and after it (M2)")
    for label in ("M1", "M2"):
        if label not in read:
            print(f"{label} state: not measured (not read)")
            continue
        problems = state_of(read[label][1], guid)
        print(f"{label} state: " + (f"{stop_name(label)}: " + "; ".join(problems) if problems else "BOOK STATE MET"))
    print("-- M1 against M2")
    if "M1" in read and "M2" in read:
        same = read["M1"][0] == read["M2"][0]
        rows_same = row_canon(read["M1"][1], "COMPANY") == row_canon(read["M2"][1], "COMPANY")
        print(f"M1/M2: byte-equal {'yes' if same else 'no'}; rows equal {'yes' if rows_same else 'no'}")
    else:
        print("M1/M2: not compared (" + "/".join(x for x in ("M1", "M2") if x not in read) + " not read)")
    print("-- against row 1381-RA's answers (lab/1381-ra-answers at 851e2545)")
    for label in ("I", "G", "U"):
        name = RA[label][0]
        if label not in read:
            print(f"{label} against {name}: not compared ({label} not read)")
            continue
        try:
            kept, kept_root = reference(label, ra)
        except NotRead as error:
            print(f"NOT READ: {error}")
            failed.append(name)
            continue
        tag = ROW_TAG[label]
        print(f"{label} against {name}: byte-equal {'yes' if read[label][0] == kept else 'no'}; rows equal "
              f"{'yes' if row_canon(read[label][1], tag) == row_canon(kept_root, tag) else 'no'}")
    if failed:
        print(f"NOT ALL READ: {', '.join(failed)}")
        return 2
    print(f"ALL {len(LABELS)} ANSWERS READ")
    return 0


def selftest(ra, scratch):
    ra, scratch = pathlib.Path(ra), pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    try:
        cap = {label: reference(label, ra)[0] for label in RA}
    except NotRead as error:
        print(f"FAILED READ: row 1381-RA's answer {error}")
        return 2
    enc = lambda text: text.encode("utf-16-le")
    dec = lambda raw: raw.decode("utf-16-le")
    one = lambda raw, a, b: enc(dec(raw).replace(a, b, 1))
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

    def doubled(raw, start):
        """RAW with its first row, from START to the end of COLLECTION, written twice."""
        text = dec(raw)
        a, b = text.index(start), text.index("</COLLECTION>")
        return enc(text[:b] + text[a:b] + text[b:])

    m = cap["M1"]
    case("state, the recorded marks", problems_of(m), [])
    case("state, ALTMSTID 211", problems_of(one(m, "> 210</ALTMSTID>", "> 211</ALTMSTID>")),
         ["ALTMSTID '211', not '210'"])
    case("state, a voucher mark", problems_of(one(m, "<ALTMSTID", '<ALTVCHID TYPE="Number"> 1</ALTVCHID><ALTMSTID')),
         ["an ALTVCHID element ('1'); the recorded state has none"])
    case("state, a second company loaded", problems_of(doubled(m, "<COMPANY NAME=")),
         ["2 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    text = dec(m)
    a, b = text.index("<COMPANY NAME="), text.index("</COLLECTION>")
    case("state, no company row (CMPINFO's COMPANY line is not a row)",
         problems_of(enc(text[:a] + text[b:])), ["0 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    case("state, another GUID", problems_of(one(m, BOOK_GUID, "ffffffff" + BOOK_GUID[8:])),
         ["the row's GUID is ffffffff..., not the book's"])
    case("state, an answer cut short", problems_of(m[:-200]).startswith("NOT READ: does not parse"), True)
    case("state, an empty answer", problems_of(b""), "NOT READ: empty answer")
    case("state, Tally's error envelope",
         problems_of(one(m, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>")), "NOT READ: HEADER/STATUS is not 1")
    # The whole computation on a scratch answers folder made from row 1381-RA's answers.
    folder = scratch / "answers"
    folder.mkdir(parents=True, exist_ok=True)

    def lay(answers, failed=()):
        for label, raw in answers.items():
            (folder / f"a-{label}.xml").write_bytes(raw)
            verdict = "VERDICT REQUEST-FAILED: curl exit 7" if label in failed else "VERDICT SENT-OK"
            (folder / f"a-{label}.transport.txt").write_text(f"label {label}\n{verdict}\n", encoding="utf-8")

    def run(fn, *args):
        out = scratch / "out.txt"
        saved = sys.stdout
        with open(out, "w", encoding="utf-8") as f:
            sys.stdout = f
            try:
                code = fn(*args)
            finally:
                sys.stdout = saved
        return code, out.read_text(encoding="utf-8").splitlines()

    lay({"M1": m, "M2": m, "I": cap["I"], "G": cap["G"], "U": cap["U"]})
    code, lines = run(cmd_all, folder, ra)
    case("all, RA's answers: exit", code, 0)
    for want in ("M1 state: BOOK STATE MET", "M2 state: BOOK STATE MET", "M1/M2: byte-equal yes; rows equal yes",
                 "I against a-I1.xml: byte-equal yes; rows equal yes",
                 "G against a-G1.xml: byte-equal yes; rows equal yes",
                 "U against a-U1.xml: byte-equal yes; rows equal yes",
                 "  rows bound to the book (GUID prefix and BRIDGECOMPANYGUID): 1 of 1", "ALL 5 ANSWERS READ"):
        case(f"all, RA's answers: the line {want.strip()[:40]}...", want in lines, True)
    code, lines = run(cmd_state, "M2", folder)
    case("state M2 on RA's marks: exit", code, 0)
    lay({"I": doubled(cap["I"], "<STOCKITEM NAME="), "M2": one(m, "> 210</ALTMSTID>", "> 211</ALTMSTID>")})
    code, lines = run(cmd_all, folder, ra)
    case("all, I two rows and M2 moved: exit (still a measurement)", code, 0)
    for want in ("I against a-I1.xml: byte-equal no; rows equal no", "M1/M2: byte-equal no; rows equal no",
                 "M2 state: STOP-RESTORED-STATE: ALTMSTID '211', not '210'"):
        case(f"all, a changed answer: the line {want[:40]}...", want in lines, True)
    code, lines = run(cmd_state, "M2", folder)
    case("state M2 moved: exit and STOP name", (code, lines[-1]), (1, "STOP-RESTORED-STATE: ALTMSTID '211', not '210'"))
    lay({"I": cap["I"], "M2": m, "G": enc(dec(cap["G"]).replace("<DESC>", "<DESC> ", 1))})
    code, lines = run(cmd_all, folder, ra)
    case("all, G differing outside its rows only", "G against a-G1.xml: byte-equal no; rows equal yes" in lines, True)
    lay({"G": cap["G"], "U": cap["U"]}, failed=("U",))
    code, lines = run(cmd_all, folder, ra)
    case("all, U's send failed: exit", code, 2)
    case("all, U's send failed: never zero rows",
         "NOT READ: U: the request failed: VERDICT REQUEST-FAILED: curl exit 7" in lines, True)
    lay({"U": cap["U"]})
    other = scratch / "ra-edited"
    other.mkdir(parents=True, exist_ok=True)
    for label, (name, _) in RA.items():
        (other / name).write_bytes(cap[label] if label != "U" else cap["U"] + b"\x00\x00")
    code, lines = run(cmd_all, folder, other)
    case("all, RA's units answer not the pinned one: exit", code, 2)
    case("all, RA's units answer not the pinned one: NOT READ, not 'differs'",
         "NOT READ: a-U1.xml is not row 1381-RA's answer (sha256 differs from the pin)" in lines, True)
    (other / "a-U1.xml").unlink()
    code, lines = run(cmd_all, folder, other)
    case("all, RA's units answer missing: NOT READ", ("NOT READ: a-U1.xml: FileNotFoundError" in lines, code), (True, 2))
    print(f"{total} cases, {right} right")
    return 0 if right == total else 1


def main(argv):
    try:
        if len(argv) == 2 and argv[0] == "state" and argv[1] in ("M1", "M2"):
            return cmd_state(argv[1])
        if len(argv) == 2 and argv[0] == "all":
            return cmd_all(ANSWERS, argv[1])
        if len(argv) == 3 and argv[0] == "--selftest":
            return selftest(argv[1], argv[2])
    except OSError as error:
        print(f"FAILED READ: {type(error).__name__}: {error}")
        return 2
    print("usage: measure.py state M1|M2 | all RA | --selftest RA SCRATCH")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
