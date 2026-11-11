#!/usr/bin/env python3
# sk.py: row 2480-SK's pair reader. New for this row. It parses no voucher, bill or census answer itself: it loads the
# readers of this folder (wx.py, cv.py, nm.py and st.py, each byte-equal to its row's) and calls their own parse, so a
# shape they STOP on is a STOP here too (exit 1). It reads the catalogue answer (G-cat) by an XML parser (LAB-COMMON
# C4), never a text search; prints never more than CAP lines a call (AGENTS.md P5); and never prints a VOUCHERKEY's or a
# GUID's value.
# Modes:
#   sk.py --pair A B G     after the second ledger of the pair A, B was created: in the catalogue answer G, for each of
#                          A and B one line NAMES <id>: EQUAL (exactly one LEDGER row whose NAME attribute is the name of
#                          names.tsv exactly, its LANGUAGENAME.LIST names that name alone, its GUID the book's) or
#                          NAMES <id>: DIFFER (what differs). Then PAIR A B: BOTH (exit 0) if both are EQUAL and the
#                          two GUIDs differ, else PAIR A B: NOT BOTH (why) (exit 3: the pair's result, not a STOP).
#   sk.py --names G        the NAMES line for every id of names.tsv, "no row" for a name with none (exit 0: a record).
#   sk.py --altered ANSWER an import answer that st.py --import did not pass: ALTERED ONE (exit 0) only if it holds one
#                          RESPONSE, every counter of st.py's once as a whole number, CREATED 0, ALTERED 1, the other
#                          counters 0 and no LINEERROR (st.py's parse). Anything else: NOT ALTERED ONE (exit 3).
#   sk.py --ref ANSWER     the control's answer against the answer an earlier row kept for the same request (REF below,
#                          by sha256 and size): REF <request>: EQUAL (exit 0) or REF <request>: DIFFER (exit 3).
#   sk.py --judge ID FORM ANSWER BOOK
#                          ONE line for one id and one form, FORM one of outstandings (an L-lo answer, nm.py's parse),
#                          census (a C-v answer, cv.py's), movement (a Vm-v answer, wx.py's) or window (a Vw-v answer,
#                          wx.py's). BOOK is the book-wide read of the day (a Vb-book answer). The id's invoice and its
#                          sibling's (the other id of its pair; the control has none) are found in BOOK as wx.py --match
#                          finds one: the one voucher with an entry on that ledger's name exactly, numbered
#                          BP/26-27/<its number>. The line is
#                            JUDGE <id> <form>: OWN YES|NO, SIBLING YES|NO|none, OTHERS <n>
#                          OWN: the answer holds the id's invoice; SIBLING: it holds the sibling's (NOT IN THE BOOK if
#                          the sibling's invoice is not in BOOK, so nothing can be matched); OTHERS: how many
#                          vouchers (or, for outstandings, bill rows) it holds that are neither. A voucher is matched by
#                          VOUCHERKEY (movement, window), by GUID (census, whose FETCH is GUID, ALTERID and DATE), and a
#                          bill row by its BILLREF, the invoice's number (outstandings). Exit 0 for OWN YES, SIBLING NO
#                          or none, OTHERS 0; exit 3 for any other line, a result and not a STOP: also
#                            JUDGE <id> <form>: <the answer's kind>         (no bytes, a STATUS, a malformed answer)
#                            JUDGE <id> <form>: NOT JUDGED (why)            (BOOK does not show the invoice as made)
# Example: python3 -I sk.py --judge kb census a-c3a-C-v-kb.xml a-k8-Vb-book.xml
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
CAP = 200
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as wx.py, nm.py, st.py and count20.py hold it
PAIRS = (("ka", "kb"), ("ra", "rb"), ("qa", "qb"), ("ua", "ub"), ("ea", "eb"))
# The answers rows 2480-MX and 2480-MW kept for the control's requests this row sends again (lab/2480-mx-answers at
# e88cd036, a-r1a-L-lo-pl-20260801.xml; lab/2480-mw-answers at c0058059, a-k1-Vm-u-pl.xml, a-k3-Vm-v-pl.xml,
# a-k4-Vw-u-pl.xml and a-k6-Vw-v-pl.xml): request file -> (size, sha256).
REF = {
    "L-lo-pl-20260801.xml": (896, "19c796774ac740b6a26490c1fd7a9fd017a7d8b709a71540a98194d1d6d58b8e"),
    "Vm-u-pl.xml": (11954, "b8eab7b26d95eccb5cd630c773efb71e8fe5dfe00c1e47b259dc12bc83a7c9c4"),
    "Vm-v-pl.xml": (11954, "b8eab7b26d95eccb5cd630c773efb71e8fe5dfe00c1e47b259dc12bc83a7c9c4"),
    "Vw-u-pl.xml": (94752, "f158857ba662a10fd3ecbdf13c479759ff6bd7f1be8427f3d94f3d05ed6113d0"),
    "Vw-v-pl.xml": (94752, "f158857ba662a10fd3ecbdf13c479759ff6bd7f1be8427f3d94f3d05ed6113d0"),
}
here = pathlib.Path(__file__).resolve().parent
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
lines = 0

def say(text):
    global lines
    lines += 1
    if lines == CAP:
        print(f"... more than {CAP} lines: the rest is not printed (STOP for this call)")
        raise SystemExit(1)
    print(text)

def stop(text):
    print(f"STOP: {text}")
    raise SystemExit(1)

def reader(name):
    """The functions of one of this folder's readers: its text up to the line where it reads its arguments, run once."""
    source = (here / name).read_text("utf-8")
    marker = "\nargs = sys.argv[1:]\n"
    if source.count(marker) != 1:
        stop(f"{name}: not the reader this script expects (its argument line is not there once)")
    space = {"__file__": str(here / name), "__name__": f"sk_{name[:-3]}"}
    exec(compile(source.split(marker, 1)[0], name, "exec"), space)
    return space

def names_tsv():
    rows = [line.split("\t") for line in (here / "names.tsv").read_text("utf-8").split("\n") if line]
    table = {ident: (number, name) for ident, number, name in rows}
    if len(table) != 11 or [r[0] for r in rows] != ["pl"] + [i for p in PAIRS for i in p]:
        stop("names.tsv is not this row's")
    return table

def sibling_of(ident):
    for a, b in PAIRS:
        if ident in (a, b):
            return b if ident == a else a
    return None

def catalogue(path):
    """[(NAME attribute, [LANGUAGENAME names], GUID)] for every LEDGER row, each of BOOK."""
    raw = pathlib.Path(path).read_bytes() if pathlib.Path(path).is_file() else b""
    if not raw:
        stop(f"{path} is missing or empty: the request failed")
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        root = ET.fromstring(FORBIDDEN.sub("�", raw.decode("utf-16-le")))
    except (UnicodeDecodeError, ET.ParseError) as error:
        stop(f"{path} does not decode as UTF-16LE or parse: {error}")
    if root.tag != "ENVELOPE" or [(s.text or "").strip() for s in root.findall("./HEADER/STATUS")] != ["1"]:
        stop(f"{path}: not one ENVELOPE with HEADER STATUS 1")
    if list(root.iter("LINEERROR")):
        stop(f"{path} carries a LINEERROR: the request failed")
    found = root.findall("./BODY/DATA/COLLECTION")
    if len(found) != 1:
        stop(f"{path}: not one DATA COLLECTION")
    out = []
    for row in found[0]:
        if row.tag != "LEDGER":
            stop(f"{path}: a {row.tag} element in the collection")
        name, guids = row.get("NAME"), row.findall("GUID")
        company = [(c.text or "").strip().lower() for c in row.findall("BRIDGECOMPANYGUID")]
        if name is None or len(guids) != 1 or company != [BOOK]:
            stop(f"{path}: a LEDGER row without a NAME attribute, one GUID and the book's BRIDGECOMPANYGUID")
        listed = [n.text or "" for n in row.findall("LANGUAGENAME.LIST/NAME.LIST/NAME")]
        out.append((name, listed, (guids[0].text or "").strip()))
    if not out:
        stop(f"{path}: no LEDGER row, so nothing can be compared")
    return out

def names_line(ident, name, rows):
    mine = [r for r in rows if r[0] == name]
    if not mine:
        return f"NAMES {ident}: no row", None
    if len(mine) > 1:
        return f"NAMES {ident}: DIFFER ({len(mine)} rows with that NAME attribute)", None
    _, listed, guid = mine[0]
    why = []
    if listed != [name]:
        why.append(f"its names are {listed!r}")
    if not guid.startswith(BOOK + "-"):
        why.append("its GUID is not the book's")
    return (f"NAMES {ident}: DIFFER ({'; '.join(why)})", None) if why else (f"NAMES {ident}: EQUAL (one row, its own GUID)", guid)

def invoice_in(book_rows, name, number, on):
    """The one voucher of BOOK with an entry on that name exactly, numbered as the invoice; else None and why."""
    found = [r for r in book_rows if on(r, name)]
    if len(found) != 1:
        return None, f"the book-wide read holds {len(found)} vouchers on that ledger, not one"
    if found[0]["number"] != f"BP/26-27/{number}":
        return None, "the book-wide read's voucher on that ledger is not its invoice"
    return found[0], None

def judge(ident, form, answer, book):
    table = names_tsv()
    if ident not in table or form not in ("outstandings", "census", "movement", "window"):
        stop("usage: --judge ID FORM ANSWER BOOK, ID in names.tsv, FORM outstandings, census, movement or window")
    head = f"JUDGE {ident} {form}"
    wx = reader("wx.py")
    book_kind, book_rows = wx["read"](book)
    if book_kind != "vouchers":
        return f"{head}: NOT JUDGED (the book-wide read is not a read with vouchers)", 3
    on = lambda r, who: any(e["line"][0] == who for e in r["entries"])
    own, why = invoice_in(book_rows, table[ident][1], table[ident][0], on)
    if own is None:
        return f"{head}: NOT JUDGED ({why})", 3
    sib = sibling_of(ident)
    other = None
    if sib is not None:
        other, _ = invoice_in(book_rows, table[sib][1], table[sib][0], on)
    if form == "outstandings":
        nm = reader("nm.py")
        kind, rows = nm["answer"](answer, show=False)
        if rows is None:
            return f"{head}: {kind}", 3
        held = [r["BILLREF"] for r in rows]
        mine, theirs = f"BP/26-27/{table[ident][0]}", f"BP/26-27/{table[sib][0]}" if sib else None
    elif form == "census":
        cv = reader("cv.py")
        kind, rows = cv["read"](answer)
        if kind != "vouchers":
            return f"{head}: {'NO BYTES' if kind == 'none' else 'STATUS ' + rows[0]}", 3
        held = [g for g, _, _ in rows]
        mine, theirs = own["own"]["GUID"], other["own"]["GUID"] if other else None
    else:
        kind, rows = wx["read"](answer)
        if kind != "vouchers":
            return f"{head}: {'EMPTY ANSWER' if kind == 'empty' else 'STATUS ' + rows[0]}", 3
        held = [r["key"] for r in rows]
        mine, theirs = own["key"], other["key"] if other else None
    has_own, has_sib = mine in held, theirs is not None and theirs in held
    others = sum(1 for h in held if h != mine and h != theirs)
    if sib is None:
        sib_text = "none"
    elif other is None and form != "outstandings":
        sib_text = "NOT IN THE BOOK"  # the sibling's invoice is not in the book-wide read: nothing to match
    else:
        sib_text = "YES" if has_sib else "NO"
    line = f"{head}: OWN {'YES' if has_own else 'NO'}, SIBLING {sib_text}, OTHERS {others}"
    return line, 0 if has_own and not has_sib and others == 0 and sib_text in ("none", "NO") else 3

args = sys.argv[1:]
if len(args) == 4 and args[0] == "--pair":
    table, rows = names_tsv(), catalogue(args[3])
    if (args[1], args[2]) not in PAIRS:
        stop("--pair takes the two ids of one pair, in table order, then the catalogue answer")
    say(f"{args[3]}: LEDGER rows {len(rows)}")
    guids = []
    for ident in args[1:3]:
        text, guid = names_line(ident, table[ident][1], rows)
        say(f"  {text}")
        guids.append(guid)
    both = all(guids) and guids[0] != guids[1]
    why = "one GUID for both" if all(guids) else "a name is not EQUAL"
    say(f"PAIR {args[1]} {args[2]}: {'BOTH' if both else 'NOT BOTH (' + why + ')'}")
    raise SystemExit(0 if both else 3)
elif len(args) == 2 and args[0] == "--names":
    table, rows = names_tsv(), catalogue(args[1])
    say(f"{args[1]}: LEDGER rows {len(rows)}")
    for ident, (_, name) in table.items():
        say(f"  {names_line(ident, name, rows)[0]}")
elif len(args) == 2 and args[0] == "--altered":
    st = reader("st.py")
    root = st["root_of"](args[1])
    responses = [root] if root.tag == "RESPONSE" else list(root.iter("RESPONSE"))
    if len(responses) != 1:
        say(f"{args[1]}: {len(responses)} RESPONSE elements, not one")
        say("NOT ALTERED ONE")
        raise SystemExit(3)
    read = {}
    for tag in st["COUNTERS"]:
        found = responses[0].findall(tag)
        text = (found[0].text or "").strip() if len(found) == 1 else None
        read[tag] = int(text) if text is not None and text.isdigit() and text.isascii() else None
    errors = len(list(root.iter("LINEERROR")))
    say(f"{args[1]}: " + ", ".join(f"{tag} {read[tag]}" for tag in st["COUNTERS"]) + f", LINEERROR {errors}")
    zero = [t for t in st["ZERO"] if t != "ALTERED"]
    ok = (all(read[t] is not None for t in st["COUNTERS"]) and read["CREATED"] == 0 and read["ALTERED"] == 1
          and all(read[t] == 0 for t in zero) and not errors)
    say("ALTERED ONE" if ok else "NOT ALTERED ONE")
    raise SystemExit(0 if ok else 3)
elif len(args) == 2 and args[0] == "--ref":
    match = re.match(r"a-[^-]+-(.+)$", pathlib.Path(args[1]).name)
    request = match.group(1) if match else None
    if request not in REF:
        stop(f"--ref: {args[1]} is not an answer to one of {sorted(REF)}")
    raw = pathlib.Path(args[1]).read_bytes() if pathlib.Path(args[1]).is_file() else b""
    if not raw:
        stop(f"{args[1]} is missing or empty")
    size, sha = REF[request]
    equal = (len(raw), hashlib.sha256(raw).hexdigest()) == (size, sha)
    say(f"REF {request}: {'EQUAL' if equal else 'DIFFER'} ({len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()[:16]}; "
        f"kept {size} B sha256 {sha[:16]})")
    raise SystemExit(0 if equal else 3)
elif len(args) == 5 and args[0] == "--judge":
    text, code = judge(*args[1:])
    say(text)
    raise SystemExit(code)
else:
    stop("usage: sk.py --pair A B G | --names G | --altered ANSWER | --ref ANSWER | --judge ID FORM ANSWER BOOK")
