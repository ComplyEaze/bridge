#!/usr/bin/env python3
"""Read row 1381-RB's answers as LAB-COMMON-RB R6 says, and print what the sheet computes from them.

Row 1381-RB, from row 1381-RR's measure.py (lab/1381-rr-sheet at 62858add, lab/1381-rr/measure.py): its reader
(FORBIDDEN pattern, parse, row path, canonical rows) and its state check, unchanged. Changed: the eleven labels M1 I1
G1 U1 W M2 I2 M3 I3 G3 U3; the pins are master's fixtures of this book (stock-create-lab/, PROVENANCE.md there) in
place of row 1381-RA's answers, each pinned by sha256 and bytes; `state` takes the mark it expects from the label (M2's
from the write's grade); new are `bind` (the bind before the write), `write` (the write's answer graded by parsing,
never by a text search), `aim` (the item read after the write) and the restore's comparison in `all`. Not product
code. Python 3.10, standard library only. Reads only; writes nothing but standard output (the self-test writes only in
its SCRATCH).

  measure.py state M1|M2|M3
      one marks answer: exactly one COMPANY row, the book's GUID, no ALTVCHID element, and ALTMSTID 210 for M1 and M3
      (BK-S's state), for M2 211 after a write graded ADMITTED and 210 after one graded REFUSED. Prints
      `BOOK STATE MET: ...` (exit 0), or `STOP-BOOK-STATE` (M1), `STOP-AIM-STATE` (M2), `STOP-RESTORED-STATE` (M3)
      with what differs (exit 1); M2 after a write graded UNCLEAR prints `STOP-WRITE-ANSWER` with the mark (exit 1).
  measure.py bind
      M1's state met, and I1, G1 and U1 byte-equal to the pins (sha256 and bytes): `BIND MET: ...` (exit 0), else
      `STOP-BOOK-STATE: <what differs>` (exit 1). The book is then at BK-S's state (lab 1381-RR), so no backup is taken.
  measure.py write
      W's answer, parsed: `WRITE ADMITTED` (a RESPONSE whose ten counters are present once each as whole numbers,
      CREATED 1, every other counter but LASTVCHID and LASTMID 0, no LINEERROR); `WRITE REFUSED` (an error ENVELOPE,
      HEADER/STATUS not 1; or a RESPONSE whose counters are all present and CREATED, ALTERED, DELETED and COMBINED all
      0, whatever else it says); `WRITE UNCLEAR` (any other parsed answer: a change counter above 0 beside an error, a
      counter missing, repeated, empty or not a whole number, CREATED above 1, an ENVELOPE with STATUS 1, another
      root). Each with every counter and each LINEERROR printed whole; exit 0. NOT READ (exit 2): the send failed,
      the answer is empty, not UTF-16LE or does not parse: "the request failed", never "nothing created".
  measure.py aim
      I2: its rows; `AIM SHOWN` when a STOCKITEM row named as the write's item is there, bound to the book (exit 0),
      else `STOP-AIM-STATE` (exit 1); I2 against the pin of the licensed read after the same write, byte-equal and
      rows equal (a measurement).
  measure.py all FX
      every answer sent: bytes, sha256, rows; the bind, the grade, the three states, the aim; M3, I3, G3 and U3
      against the pins, byte-equal and rows equal (the reference files in the folder FX, each checked against its pin
      first); and `RESTORE UNDID THE WRITE: yes` only when all four are byte-equal. Every line is a measurement.
  measure.py --selftest FX SCRATCH
      the reader, the grader and the computes on the fixtures in FX, each negative case a capture with one edit,
      written to SCRATCH (under local/, or outside the row folder).

Answers are read from local/answers/a-<label>.xml, and only when a-<label>.transport.txt ends `VERDICT SENT-OK`
(send.py). A read answer that is empty, does not parse, or is not a success ENVELOPE holding BODY/DATA/COLLECTION is
`NOT READ` with the reason: never zero rows. A reference file that is missing or not its pin is `NOT READ` as well.
Exit 2: an answer or reference not read, or a file missing. Rows are read only at ./BODY/DATA/COLLECTION/<ROW>: the
CMPINFO block above holds bare count lines with the same tag names, which are not rows. GUIDs are printed by their
first 8 characters.
"""

import hashlib
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parent
ANSWERS = ROOT / "local" / "answers"
BOOK_GUID = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1"  # scripts/merge-gate-synthetic-uuids.json on master
BOOK_MARK = "210"  # ALTMSTID of the book at BK-S's state (lab 1381-RR, M1 and M2)
AIM_MARK = "211"  # ALTMSTID after the write, licensed (lab 1381-R1, S5: 210 to 211)
WRITE_ITEM = "SC Copied Goods"  # the one STOCKITEM requests/create.utf16le.xml creates
LABELS = ("M1", "I1", "G1", "U1", "W", "M2", "I2", "M3", "I3", "G3", "U3")
REFUSED_LABELS = ("M1", "I1", "G1", "U1", "W", "M2")  # all that is sent when the write is refused in-band
ROW_TAG = {"M": "COMPANY", "I": "STOCKITEM", "G": "STOCKGROUP", "U": "UNIT"}
# Master's fixtures of this book (src-tauri/crates/bridge-tally-protocol/tests/fixtures/, stock-create-lab/
# PROVENANCE.md and the files' own commits): name, sha256 and bytes, each computed from the blob by git.
FX = {
    "marks": ("stock-create-lab-marks.utf16le.xml", "ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1", 3318),
    "items-before": ("stock-create-lab-items-before.utf16le.xml", "92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4", 8270),
    "groups": ("stock-create-lab-groups.utf16le.xml", "b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a", 4170),
    "units": ("stock-create-lab-units.utf16le.xml", "cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39", 4040),
    "items-after": ("stock-create-lab-items-after-plain-create.utf16le.xml", "2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd", 10758),
    "answer": ("stock-create-lab-create-answer.utf16le.xml", "1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010", 538),
    "refused": ("single-import-missing-ledger.utf16le.xml", "1009e1b4a24b9f10db29fa9e4008e11c55a97cdce383c59a92601390031594fb", 704),
    "partial": ("partial-import-missing-ledger.utf16le.xml", "30438f1480e4601a5c236d52be7fa85a89da9cb0d1f42a6b1ad88bf02f3fba50", 710),
    "envelope": ("builtin_unknown_report_refusal_live.utf16le.xml", "010d3144c2483d9147bb576a3b88314ff0d6434645bcbea5822bfcaa8ef43dc6", 448),
}
BIND = {"I1": "items-before", "G1": "groups", "U1": "units"}
UNDO = {"M3": "marks", "I3": "items-before", "G3": "groups", "U3": "units"}
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS")
CHANGES = ("CREATED", "ALTERED", "DELETED", "COMBINED")
# check_r1.py's pattern (rows 1381-RA and 1381-RR): the characters XML forbids, raw or as references, but 9, 10, 13.
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)


class NotRead(Exception):
    pass


def tree(raw):
    """The root element of one answer's bytes (UTF-16LE, with or without a BOM), or NotRead with the reason."""
    if not raw:
        raise NotRead("empty answer")
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    try:
        text = body.decode("utf-16-le")
    except UnicodeDecodeError:
        raise NotRead("not UTF-16LE")
    try:
        return ET.fromstring(FORBIDDEN.sub("", text))
    except ET.ParseError as error:
        raise NotRead(f"does not parse ({error})")


def parse(raw):
    """The ENVELOPE of one read answer, or NotRead with the reason."""
    root = tree(raw)
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
    """What differs from the expected state in a marks answer ([] when it is met)."""
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


def grade(raw):
    """(outcome, lines) for the write's answer: ADMITTED, REFUSED or UNCLEAR; NotRead when it cannot be read."""
    root = tree(raw)
    errors = [e.text or "" for e in root.iter("LINEERROR")]
    lines = [f"LINEERROR {e!r}" for e in errors]
    if root.tag == "ENVELOPE":
        status = [(s.text or "").strip() for s in root.findall("./HEADER/STATUS")]
        lines.insert(0, f"an ENVELOPE, HEADER/STATUS {status!r}")
        if len(status) == 1 and status[0] not in ("", "1"):
            return "REFUSED", ["an error envelope: nothing imported"] + lines
        return "UNCLEAR", ["an ENVELOPE that is not an error envelope: not an import answer this row knows"] + lines
    if root.tag != "RESPONSE":
        return "UNCLEAR", [f"the root is {root.tag}, neither RESPONSE nor ENVELOPE"] + lines
    values, bad = {}, []
    for name in COUNTERS:
        found = root.findall(f"./{name}")
        text = (found[0].text or "").strip() if len(found) == 1 else None
        if len(found) != 1:
            bad.append(f"{name} {'missing' if not found else f'{len(found)} times'}")
        elif not (text.isascii() and text.isdigit()):
            bad.append(f"{name} {text!r}, not a whole number")
        else:
            values[name] = int(text)
        lines.append(f"{name} {text if text is not None else ('MISSING' if not found else 'REPEATED')}")
    if bad:
        return "UNCLEAR", ["counters not readable: " + "; ".join(bad)] + lines
    rest = [n for n in COUNTERS if n not in ("CREATED", "LASTVCHID", "LASTMID")]
    if values["CREATED"] == 1 and not any(values[n] for n in rest) and not errors:
        return "ADMITTED", ["CREATED 1, every other counter 0, no LINEERROR"] + lines
    if not any(values[n] for n in CHANGES):
        return "REFUSED", ["CREATED, ALTERED, DELETED and COMBINED all 0: nothing imported"] + lines
    return "UNCLEAR", ["a change counter above 0 that is not CREATED 1 alone, or beside an error"] + lines


def transport_ok(label, folder):
    record = folder / f"a-{label}.transport.txt"
    if not record.exists():
        raise NotRead(f"no transport record a-{label}.transport.txt: never sent, or the send did not finish")
    last = record.read_text(encoding="utf-8").splitlines()[-1:] or [""]
    if last[0] != "VERDICT SENT-OK":
        raise NotRead(f"the request failed: {last[0]}")


def raw_of(label, folder):
    transport_ok(label, folder)
    path = folder / f"a-{label}.xml"
    if not path.exists():
        raise NotRead(f"a-{label}.xml is missing although its send says SENT-OK")
    return path.read_bytes()


def load(label, folder):
    raw = raw_of(label, folder)
    return raw, parse(raw)


def pinned(raw, key):
    """Whether RAW is the pinned fixture KEY, by sha256 and bytes."""
    _, digest, size = FX[key]
    return hashlib.sha256(raw).hexdigest() == digest and len(raw) == size


def reference(key, fx):
    """The fixture KEY from the folder FX, checked against its pin, or NotRead."""
    name = FX[key][0]
    try:
        raw = (pathlib.Path(fx) / name).read_bytes()
    except OSError as error:
        raise NotRead(f"{name}: {type(error).__name__}")
    if not pinned(raw, key):
        raise NotRead(f"{name} is not master's fixture (sha256 or bytes differ from the pin)")
    return raw


def expected(label, folder):
    """(the mark label's state must hold, the STOP name) for M1, M2, M3; M2's from the write's grade."""
    if label == "M1":
        return BOOK_MARK, "STOP-BOOK-STATE"
    if label == "M3":
        return BOOK_MARK, "STOP-RESTORED-STATE"
    outcome, _ = grade(raw_of("W", folder))
    if outcome == "ADMITTED":
        return AIM_MARK, "STOP-AIM-STATE"
    if outcome == "REFUSED":
        return BOOK_MARK, "STOP-AIM-STATE"
    return None, "STOP-WRITE-ANSWER"


def cmd_state(label, folder=ANSWERS):
    try:
        mark, stop = expected(label, folder)
        raw, root = load(label, folder)
        found = marks(root)
    except NotRead as error:
        print(f"NOT READ: {label}: {error}")
        return 2
    print(f"{label}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}")
    for g, m, v in found:
        print(f"  COMPANY row: GUID {g[:8]}... (the book's: {'yes' if g == BOOK_GUID else 'no'}), "
              f"ALTMSTID {m if m is not None else 'absent'}, ALTVCHID {v if v is not None else 'absent'}")
    if mark is None:
        print(f"{stop}: the write's answer is graded UNCLEAR; M2 records the mark only")
        return 1
    problems = state_of(root, BOOK_GUID, mark)
    if problems:
        print(f"{stop}: " + "; ".join(problems))
        return 1
    print(f"BOOK STATE MET: one COMPANY row, the book's, ALTMSTID {mark}, no ALTVCHID")
    return 0


def bind_of(folder):
    """What differs from the bind ([] when it is met): M1's state, and I1, G1, U1 equal to their pins."""
    problems = state_of(load("M1", folder)[1])
    for label, key in BIND.items():
        if not pinned(load(label, folder)[0], key):
            problems.append(f"{label} is not byte-equal to {FX[key][0]}")
    return problems


def cmd_bind(folder=ANSWERS):
    try:
        problems = bind_of(folder)
        m1 = raw_of("M1", folder)
    except NotRead as error:
        print(f"NOT READ: {error}")
        return 2
    print(f"M1 byte-equal to {FX['marks'][0]}: {'yes' if pinned(m1, 'marks') else 'no'} (a measurement)")
    if problems:
        print("STOP-BOOK-STATE: " + "; ".join(problems))
        return 1
    print("BIND MET: M1 one COMPANY row, the book's, ALTMSTID 210, no ALTVCHID; I1, G1 and U1 byte-equal to the pins")
    return 0


def cmd_write(folder=ANSWERS):
    try:
        raw = raw_of("W", folder)
        outcome, lines = grade(raw)
    except NotRead as error:
        print(f"NOT READ: W: {error} (the request failed, never 'nothing created')")
        return 2
    print(f"W: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; byte-equal to the licensed answer "
          f"{FX['answer'][0]}: {'yes' if pinned(raw, 'answer') else 'no'}")
    print("\n".join(f"  {line}" for line in lines[1:]))
    print(f"WRITE {outcome}: {lines[0]}")
    return 0


def aim_of(root):
    """(the write's item bound to the book: yes or no, the lines)."""
    found, lines, shown = rows(root, "STOCKITEM"), [], False
    for row in found:
        g = (text_of(row, "GUID") or "").strip()
        b = (text_of(row, "BRIDGECOMPANYGUID") or "").strip()
        ok = b == BOOK_GUID and g.startswith(BOOK_GUID + "-")
        shown |= ok and row.get("NAME") == WRITE_ITEM
        lines.append(f"  {row.get('NAME')!r}: GUID {g[:8]}...-{g[len(BOOK_GUID) + 1:] if ok else '?'}, bound to the "
                     f"book: {'yes' if ok else 'no'}")
    return shown, [f"I2: {len(found)} STOCKITEM row(s)"] + lines


def cmd_aim(folder=ANSWERS):
    try:
        raw, root = load("I2", folder)
        shown, lines = aim_of(root)
    except NotRead as error:
        print(f"NOT READ: I2: {error}")
        return 2
    print("\n".join(lines))
    print(f"I2 byte-equal to {FX['items-after'][0]} (licensed, after the same write): "
          f"{'yes' if pinned(raw, 'items-after') else 'no'} (a measurement)")
    if not shown:
        print(f"STOP-AIM-STATE: no STOCKITEM row {WRITE_ITEM!r} bound to the book in I2: the write is not shown")
        return 1
    print(f"AIM SHOWN: {WRITE_ITEM!r} in I2, bound to the book")
    return 0


def describe(label, raw, root):
    tag = ROW_TAG[label[0]]
    found = rows(root, tag)
    lines = [f"{label}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; {len(found)} {tag} row(s)"]
    extra = others(root, tag)
    if extra:
        lines.append(f"  other children of COLLECTION: {extra}")
    for row in found:
        lines.append(f"  {row.get('NAME')!r} ({len(row.get('NAME') or '')} characters)")
    return lines


def cmd_all(folder, fx):
    try:
        outcome = grade(raw_of("W", folder))[0] if (folder / "a-W.transport.txt").exists() else None
    except NotRead:
        outcome = None
    want = REFUSED_LABELS if outcome == "REFUSED" else LABELS
    read, failed = {}, []
    for label in want:
        try:
            if label == "W":
                raw = raw_of("W", folder)
                print(f"W: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; WRITE {grade(raw)[0]}")
                read["W"] = (raw, None)
                continue
            raw, root = load(label, folder)
            print("\n".join(describe(label, raw, root)))
            read[label] = (raw, root)
        except NotRead as error:
            print(f"NOT READ: {label}: {error}")
            failed.append(label)
    print("-- the bind, the write, the aim")
    for label in ("M1", "M2", "M3"):
        if label in read:
            try:
                mark, stop = expected(label, folder)
            except NotRead as error:
                print(f"{label} state: not measured ({error})")
                continue
            problems = ["the write graded UNCLEAR"] if mark is None else state_of(read[label][1], BOOK_GUID, mark)
            print(f"{label} state: " + (f"{stop}: " + "; ".join(problems) if problems else f"met, ALTMSTID {mark}"))
    if all(x in read for x in ("M1", "I1", "G1", "U1")):
        problems = bind_of(folder)
        print("bind: " + ("met" if not problems else "STOP-BOOK-STATE: " + "; ".join(problems)))
    if "I2" in read:
        shown = aim_of(read["I2"][1])[0]
        print(f"aim: {WRITE_ITEM!r} in I2 bound to the book {'yes' if shown else 'no'}; byte-equal to "
              f"{FX['items-after'][0]} {'yes' if pinned(read['I2'][0], 'items-after') else 'no'}")
    print("-- after the restore, against the pins")
    if outcome == "REFUSED":
        print("not measured: the write was refused in-band, so no restore was made")
    else:
        equal = []
        for label, key in UNDO.items():
            if label not in read:
                print(f"{label} against {FX[key][0]}: not compared ({label} not read)")
                continue
            try:
                kept = reference(key, fx)
            except NotRead as error:
                print(f"NOT READ: {error}")
                failed.append(FX[key][0])
                continue
            tag = ROW_TAG[label[0]]
            same = read[label][0] == kept
            equal.append(same)
            print(f"{label} against {FX[key][0]}: byte-equal {'yes' if same else 'no'}; rows equal "
                  f"{'yes' if row_canon(read[label][1], tag) == row_canon(parse(kept), tag) else 'no'}")
        if len(equal) == len(UNDO):
            print(f"RESTORE UNDID THE WRITE: {'yes' if all(equal) else 'no'} (M3, I3, G3 and U3 byte-equal to the pins: "
                  f"{sum(equal)} of 4)")
        else:
            print("RESTORE UNDID THE WRITE: not measured (an answer or a pin not read)")
    if failed:
        print(f"NOT ALL READ: {', '.join(failed)}")
        return 2
    print(f"ALL {len(want)} ANSWERS READ")
    return 0


def selftest(fx, scratch):
    fx, scratch = pathlib.Path(fx), pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    try:
        cap = {key: reference(key, fx) for key in FX}
    except NotRead as error:
        print(f"FAILED READ: master's fixture {error}")
        return 2
    enc = lambda text: text.encode("utf-16-le")
    dec = lambda raw: (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le")
    one = lambda raw, a, b: enc(dec(raw).replace(a, b, 1)) if a in dec(raw) else b"edit not applied"
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

    def graded(raw):
        try:
            return grade(raw)[0]
        except NotRead as error:
            return f"NOT READ: {error}"

    m = cap["marks"]
    m211 = one(m, "> 210</ALTMSTID>", "> 211</ALTMSTID>")
    case("state, the pinned marks", problems_of(m), [])
    case("state, ALTMSTID 211", problems_of(m211), ["ALTMSTID '211', not '210'"])
    case("state, a voucher mark", problems_of(one(m, "<ALTMSTID", '<ALTVCHID TYPE="Number"> 1</ALTVCHID><ALTMSTID')),
         ["an ALTVCHID element ('1'); the recorded state has none"])
    text = dec(m)
    a, b = text.index("<COMPANY NAME="), text.index("</COLLECTION>")
    case("state, a second company loaded", problems_of(enc(text[:b] + text[a:b] + text[b:])),
         ["2 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    case("state, no company row (CMPINFO's COMPANY line is not a row)", problems_of(enc(text[:a] + text[b:])),
         ["0 COMPANY rows, not 1 (a company other than the book is loaded, or none)"])
    case("state, another GUID", problems_of(one(m, BOOK_GUID, "ffffffff" + BOOK_GUID[8:])),
         ["the row's GUID is ffffffff..., not the book's"])
    case("state, an answer cut short", problems_of(m[:-200]).startswith("NOT READ: does not parse"), True)
    case("state, an empty answer", problems_of(b""), "NOT READ: empty answer")
    case("state, Tally's error envelope", problems_of(one(m, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>")),
         "NOT READ: HEADER/STATUS is not 1")
    # The grade, on Tally's own import answers (captured, master's fixtures), each negative case one edit.
    w = cap["answer"]
    case("grade, the licensed answer (CREATED 1, the rest 0)", graded(w), "ADMITTED")
    case("grade, the licensed answer with a byte-order mark", graded(b"\xff\xfe" + w), "ADMITTED")
    case("grade, Tally's refusal (LINEERROR, CREATED 0)", graded(cap["refused"]), "REFUSED")
    case("grade, Tally's error envelope (STATUS 0, LINEERROR)", graded(cap["envelope"]), "REFUSED")
    case("grade, Tally's partial landing (LINEERROR beside CREATED 2)", graded(cap["partial"]), "UNCLEAR")
    case("grade, CREATED 0 and no LINEERROR", graded(one(w, "<CREATED>1<", "<CREATED>0<")), "REFUSED")
    case("grade, ALTERED 1 (the item existed)", graded(one(w, "<ALTERED>0<", "<ALTERED>1<")), "UNCLEAR")
    case("grade, CREATED 1 with EXCEPTIONS 1", graded(one(w, "<EXCEPTIONS>0<", "<EXCEPTIONS>1<")), "UNCLEAR")
    case("grade, CREATED 1 with a LINEERROR", graded(one(w, "<CREATED>", "<LINEERROR>x</LINEERROR><CREATED>")),
         "UNCLEAR")
    case("grade, CREATED 2", graded(one(w, "<CREATED>1<", "<CREATED>2<")), "UNCLEAR")
    case("grade, CREATED missing", graded(one(w, "<CREATED>1</CREATED>", "")), "UNCLEAR")
    case("grade, CREATED empty (not zero)", graded(one(w, "<CREATED>1</CREATED>", "<CREATED></CREATED>")), "UNCLEAR")
    case("grade, CREATED repeated", graded(one(w, "<CREATED>1</CREATED>", "<CREATED>1</CREATED><CREATED>1</CREATED>")),
         "UNCLEAR")
    case("grade, a refusal with ALTERED missing", graded(one(cap["refused"], "<ALTERED>0</ALTERED>", "")), "UNCLEAR")
    case("grade, a refusal with ALTERED 1 (CREATED 0, a change landed)",
         graded(one(cap["refused"], "<ALTERED>0<", "<ALTERED>1<")), "UNCLEAR")
    case("grade, an ENVELOPE with STATUS 1", graded(one(cap["envelope"], "<STATUS>0<", "<STATUS>1<")), "UNCLEAR")
    case("grade, a counter nested, not a child of RESPONSE",
         graded(one(w, "<CREATED>1</CREATED>", "<X><CREATED>1</CREATED></X>")), "UNCLEAR")
    case("grade, an answer cut short", graded(w[:-20]).startswith("NOT READ: does not parse"), True)
    case("grade, an empty answer", graded(b""), "NOT READ: empty answer")
    # The commands, on a scratch answers folder made from the fixtures.
    folder = scratch / "answers"

    def lay(answers, failed=()):
        if folder.exists():
            for old in folder.iterdir():
                old.unlink()
        folder.mkdir(parents=True, exist_ok=True)
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
            except Exception as error:  # an error that escapes a command is a WRONG case, never a pass
                code = f"escaped: {type(error).__name__}"
            finally:
                sys.stdout = saved
        return code, out.read_text(encoding="utf-8").splitlines()

    base = {"M1": m, "I1": cap["items-before"], "G1": cap["groups"], "U1": cap["units"]}
    lay(base)
    code, lines = run(cmd_bind, folder)
    case("bind, the pins: exit and last line", (code, lines[-1].split(":")[0]), (0, "BIND MET"))
    lay({**base, "G1": one(cap["groups"], "<DESC>", "<DESC> ")})
    code, lines = run(cmd_bind, folder)
    case("bind, G1 one byte off its pin: STOP-BOOK-STATE",
         (code, lines[-1]), (1, "STOP-BOOK-STATE: G1 is not byte-equal to stock-create-lab-groups.utf16le.xml"))
    lay({**base, "M1": m211})
    case("bind, M1 at 211: STOP-BOOK-STATE", run(cmd_bind, folder)[0], 1)
    lay(base, failed=("U1",))
    code, lines = run(cmd_bind, folder)
    case("bind, U1's send failed: NOT READ, not a STOP on the book", (code, lines[-1].split(":")[0]), (2, "NOT READ"))
    lay({**base, "W": w})
    code, lines = run(cmd_write, folder)
    case("write, the licensed answer: exit and grade", (code, lines[-1].split(":")[0]), (0, "WRITE ADMITTED"))
    case("write, the licensed answer: byte-equal line", any(x.endswith("utf16le.xml: yes") for x in lines), True)
    lay({**base, "W": w}, failed=("W",))
    code, lines = run(cmd_write, folder)
    case("write, the send failed: NOT READ, never 'nothing created'", (code, lines[-1].startswith("NOT READ: W: the "
         "request failed")), (2, True))
    for what, answer, m2, want in (("admitted, M2 at 211", w, m211, (0, "BOOK STATE MET")),
                                   ("admitted, M2 still 210", w, m, (1, "STOP-AIM-STATE")),
                                   ("refused, M2 at 210", cap["refused"], m, (0, "BOOK STATE MET")),
                                   ("refused, M2 at 211", cap["refused"], m211, (1, "STOP-AIM-STATE")),
                                   ("unclear, M2 recorded only", cap["partial"], m, (1, "STOP-WRITE-ANSWER"))):
        lay({**base, "W": answer, "M2": m2})
        code, lines = run(cmd_state, "M2", folder)
        case(f"state M2, the write {what}", (code, lines[-1].split(":")[0]), want)
    lay({**base, "W": w, "M2": m211}, failed=("W",))
    code, lines = run(cmd_state, "M2", folder)
    case("state M2, the write's send failed: NOT READ", (code, lines[-1].split(":")[0]), (2, "NOT READ"))
    lay({"I2": cap["items-after"]})
    code, lines = run(cmd_aim, folder)
    case("aim, the licensed read after the write: AIM SHOWN", (code, lines[-1].split(":")[0]), (0, "AIM SHOWN"))
    case("aim, the licensed read: byte-equal line", any(x.endswith(": yes (a measurement)") for x in lines), True)
    lay({"I2": cap["items-before"]})
    code, lines = run(cmd_aim, folder)
    case("aim, the read before the write: STOP-AIM-STATE", (code, lines[-1].split(":")[0]), (1, "STOP-AIM-STATE"))
    lay({"I2": one(cap["items-after"], BOOK_GUID + "-", "ffffffff" + BOOK_GUID[8:] + "-")})
    case("aim, the item's GUID not the book's: STOP-AIM-STATE", run(cmd_aim, folder)[0], 1)
    lay({"M3": m})
    case("state M3, the pinned marks", run(cmd_state, "M3", folder)[0], 0)
    lay({"M3": m211})
    code, lines = run(cmd_state, "M3", folder)
    case("state M3 at 211: STOP-RESTORED-STATE", (code, lines[-1].split(":")[0]), (1, "STOP-RESTORED-STATE"))
    whole = {**base, "W": w, "M2": m211, "I2": cap["items-after"], "M3": m, "I3": cap["items-before"],
             "G3": cap["groups"], "U3": cap["units"]}
    lay(whole)
    code, lines = run(cmd_all, folder, fx)
    case("all, the admitted path on the pins: exit", code, 0)
    for want in ("RESTORE UNDID THE WRITE: yes (M3, I3, G3 and U3 byte-equal to the pins: 4 of 4)", "ALL 11 ANSWERS READ",
                 "M2 state: met, ALTMSTID 211", "bind: met"):
        case(f"all, the admitted path: the line {want[:40]}...", want in lines, True)
    lay({**whole, "I3": cap["items-after"]})
    code, lines = run(cmd_all, folder, fx)
    case("all, I3 still holding the write: not undone",
         "RESTORE UNDID THE WRITE: no (M3, I3, G3 and U3 byte-equal to the pins: 3 of 4)" in lines, True)
    lay({**base, "W": cap["refused"], "M2": m})
    code, lines = run(cmd_all, folder, fx)
    case("all, the refused path: six answers, no restore",
         (code, "ALL 6 ANSWERS READ" in lines,
          "not measured: the write was refused in-band, so no restore was made" in lines), (0, True, True))
    lay({**whole, "U3": cap["units"]}, failed=("U3",))
    code, lines = run(cmd_all, folder, fx)
    case("all, U3's send failed: NOT ALL READ, never 'undone'",
         (code, "RESTORE UNDID THE WRITE: not measured (an answer or a pin not read)" in lines), (2, True))
    other = scratch / "fx-edited"
    other.mkdir(parents=True, exist_ok=True)
    for key, (name, _, _) in FX.items():
        (other / name).write_bytes(cap[key] if key != "units" else cap["units"] + b"\x00\x00")
    lay(whole)
    code, lines = run(cmd_all, folder, other)
    case("all, the units fixture not its pin: NOT READ, not 'differs'",
         (code, "NOT READ: stock-create-lab-units.utf16le.xml is not master's fixture (sha256 or bytes differ from the "
                "pin)" in lines), (2, True))
    print(f"{total} cases, {right} right")
    return 0 if right == total else 1


def main(argv):
    try:
        if len(argv) == 2 and argv[0] == "state" and argv[1] in ("M1", "M2", "M3"):
            return cmd_state(argv[1])
        if argv == ["bind"]:
            return cmd_bind()
        if argv == ["write"]:
            return cmd_write()
        if argv == ["aim"]:
            return cmd_aim()
        if len(argv) == 2 and argv[0] == "all":
            return cmd_all(ANSWERS, argv[1])
        if len(argv) == 3 and argv[0] == "--selftest":
            return selftest(argv[1], argv[2])
    except OSError as error:
        print(f"FAILED READ: {type(error).__name__}: {error}")
        return 2
    print("usage: measure.py state M1|M2|M3 | bind | write | aim | all FX | --selftest FX SCRATCH")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
