#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [--company FILE] [--committed FILE] : checks one answer of the ledger ITC look-only sheet. Nothing here sends anything.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold; keep the file, write the
printed line down, go on unless the step says STOP. Exit 3 = STOP: a bound, a completeness rule or an identity rule
broke; stop and send nothing more. Exit 2 = the check could not run (any error at all, a missing file, a missing
argument): never a pass. The last line printed starts with PASS, FINDING, CHECK-ERROR or STOP; lines before it (step itc
only) are the table of ledgers. Output is ASCII only.
Steps:
  a01 ANSWER                      company list: the only loaded company is BRIDGE PILOT LAB, Education mode No
  a02 ANSWER                      book extent: one COMPANY row, BRIDGE PILOT LAB, with its GUID and marks
  itc ANSWER --company a02-FILE [--extra NAME]...   the ledger GST read, parsed as XML. The ledgers must be exactly the
                                  committed 14 plus the declared --extra names (an undeclared or missing one is a STOP; each
                                  extra holds exactly one GSTDETAILS entry with content); `--list-extras` instead prints
                                  the names found beyond the 14 as copy-ready --extra text and declares nothing. Parsed: per ledger with a GSTDETAILS entry, name, parent,
                                  source of the details (INHERITED when it reads As per ...), applicable-from and the
                                  GSTINELIGIBLEITC state (Yes / No / absent), and counts; the book's GUID must equal the
                                  committed capture's
  vs-committed ANSWER [--extra NAME]...   the committed capture (committed/ledger_gst_details_live.utf16le.xml, size and sha256
                                  pinned below) against a new answer: byte identity by sha256 first; else the GSTDETAILS
                                  entries of the ledger rows leaf by leaf (a difference is a FINDING)
  same A B                        two answers of one request: byte-identical, or equal leaf by leaf under COLLECTION
                                  (the header counters may differ and are recorded); both must be full envelopes
"""
import hashlib, os, re, sys
from xml.etree import ElementTree as ET

BOOK = "BRIDGE PILOT LAB"
CAPS = {"a01": 8000, "a02": 6000, "itc": 110000, "vs-committed": 110000}
COMMITTED_SHA256 = "597f2b11728c39d6643ced43682b89143fce29481a803ad5f23433ee896c9847"
COMMITTED_SIZE = 51434
COMMITTED_DEFAULT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "committed", "ledger_gst_details_live.utf16le.xml")

class Stop(Exception): pass
class Err(Exception): pass

def text_of(data):
    if not data: raise Stop("the answer is empty (0 bytes)")
    if data[:2] == b"\xff\xfe": return data[2:].decode("utf-16-le")
    if data[:2] == b"\xfe\xff": return data[2:].decode("utf-16-be")
    if len(data) > 1 and data[1:2] == b"\x00": return data.decode("utf-16-le")
    return data.decode("utf-8")

def tree(path):
    data = open(path, "rb").read()
    t = text_of(data)
    t = re.sub(r"&#(?:[0-8]|1[1-2]|1[4-9]|2[0-9]|3[0-1]);", "", t)        # numeric references XML 1.0 refuses
    t = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f]", "", t)                      # raw control bytes some names carry
    try:
        root = ET.fromstring(t)
    except ET.ParseError as e:
        raise Stop(f"the answer is not a complete XML envelope ({e}); a truncated answer is never read")
    return data, t, root

def collection(root):
    col = root.find(".//COLLECTION")
    if col is None: raise Stop("no COLLECTION in the answer")
    return col

def rows(root, tag):
    return [e for e in collection(root) if e.tag == tag]

def name_of(e): return e.get("NAME") or (e.findtext("NAME") or "").strip()
def txt(e, tag): return (e.findtext(tag) or "").strip()
def has_text(e): return any((c.text or "").strip() for c in e.iter())

def common(path):
    data, t, root = tree(path)
    if root.tag != "ENVELOPE": raise Stop(f"root is {root.tag}, not ENVELOPE")
    if "<LINEERROR" in t: raise Stop("the answer carries a LINEERROR")
    st = root.findtext(".//HEADER/STATUS")
    if st is not None and st.strip() == "0": raise Stop("the answer header STATUS is 0")
    collection(root)
    return data, root

def canon(e):
    return re.sub(r"\s+", " ", ET.tostring(e, encoding="unicode")).strip()

def company_guid(path):
    _, root = common(path)
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows in the book-extent answer, expected 1")
    if name_of(rs[0]) != BOOK: raise Stop(f"the book-extent answer is for {name_of(rs[0])!r}, not {BOOK}")
    g = txt(rs[0], "GUID")
    if not g: raise Err("no GUID in the book-extent answer")
    return g

def committed_path(opt):
    p = opt("--committed") or COMMITTED_DEFAULT
    data = open(p, "rb").read()
    if len(data) != COMMITTED_SIZE or hashlib.sha256(data).hexdigest() != COMMITTED_SHA256:
        raise Err(f"{p} is not the committed capture (size {len(data)}, sha256 {hashlib.sha256(data).hexdigest()}): restore the folder from SHA256.txt")
    return p

def book_guids(led):
    return sorted({txt(r, "BRIDGECOMPANYGUID") for r in led.values()})

def ledgers(root, what):
    rs = rows(root, "LEDGER")
    names = [name_of(r) for r in rs]
    if not rs: raise Stop(f"{what}: no LEDGER row: nothing was captured")
    if len(set(names)) != len(names): raise Stop(f"{what}: a ledger name repeats")
    return {name_of(r): r for r in rs}

def entries_of(row, name):
    """The GSTDETAILS.LIST elements of a ledger row that hold content (an empty placeholder holds only whitespace)."""
    return [g for g in row.iter("GSTDETAILS.LIST") if has_text(g)]

def flat(e, path=""):
    """Leaf map of an element: path (with child index) -> text."""
    out, seen = {}, {}
    for c in e:
        i = seen.get(c.tag, 0); seen[c.tag] = i + 1
        p = f"{path}/{c.tag}[{i}]"
        if len(c): out.update(flat(c, p))
        else: out[p] = (c.text or "").strip()
    return out

def declared_extras(argv, ref):
    """The ledgers the coordinator declares beyond the committed capture: every --extra NAME."""
    ex = [argv[i + 1] for i, a in enumerate(argv[:-1]) if a == "--extra"]
    if "--extra" in argv[-1:]: raise Err("--extra needs a name")
    if len(set(ex)) != len(ex): raise Err("an --extra name is declared twice")
    if [n for n in ex if n in ref]: raise Err(f"--extra names a committed ledger: {[n for n in ex if n in ref]}")
    return set(ex)

def check_sets(led, ref, extras):
    """Exactly the committed ledgers plus the declared extras; each extra holds exactly one GSTDETAILS entry with content."""
    und, miss, miss_x = sorted(set(led) - set(ref) - extras), sorted(set(ref) - set(led)), sorted(extras - set(led))
    if und or miss or miss_x:
        raise Stop(f"the ledgers are not the committed {len(ref)} plus the declared extras: undeclared extra ledger(s) in the answer {und}; committed ledger(s) missing {miss}; declared extra(s) not in the answer {miss_x}. Do not substitute: report")
    for n in sorted(extras):
        k = len(entries_of(led[n], n))
        if k != 1: raise Stop(f"extra ledger {n!r} holds {k} GSTDETAILS entries with content, expected exactly 1" + (": an empty list gives nothing to look at" if k == 0 else ""))

def run(argv):
    if len(argv) < 3: raise Err("usage: check.py STEP ANSWER ...  (see the top of check.py)")
    opt = lambda k: argv[argv.index(k) + 1] if k in argv else None
    if argv[1] == "same":
        if len(argv) < 4: raise Err("same needs two files")
        if os.path.realpath(argv[2]) == os.path.realpath(argv[3]): raise Err("the same file given twice proves nothing")
        a, b = open(argv[2], "rb").read(), open(argv[3], "rb").read()
        if not a or not b: raise Err("an empty file proves nothing")
        common(argv[2]); common(argv[3])             # both must be full envelopes, no LINEERROR, STATUS not 0
        if a == b: return 0, f"same: {len(a)} B vs {len(b)} B, identical"
        _, _, ra = tree(argv[2]); _, _, rb = tree(argv[3])
        ca, cb = ra.find(".//COLLECTION"), rb.find(".//COLLECTION")
        if canon(ca) != canon(cb): return 1, f"same: {len(a)} B vs {len(b)} B, DIFFERENT under COLLECTION"
        def counters(r):
            c = r.find(".//CMPINFO")
            return {} if c is None else {e.tag: (e.text or "").strip() for e in c}
        ka, kb = counters(ra), counters(rb)
        moved = sorted(f"{k} {ka.get(k)}->{kb.get(k)}" for k in set(ka) | set(kb) if ka.get(k) != kb.get(k))
        return 0, f"same: {len(a)} B vs {len(b)} B, equal leaf by leaf under COLLECTION; header counters differ (recorded, not a stop): {moved}"
    step, path = argv[1], argv[2]
    cap = CAPS.get(step)
    if cap is None: raise Err(f"unknown step {step}")
    data, root = common(path)
    size, notes, bad, table = len(data), [], 0, []
    if size > cap: raise Stop(f"{size} B is above the cap {cap} B")
    notes.append(f"{size} B (cap {cap})")

    if step == "a01":
        rs = rows(root, "COMPANY")
        names = sorted({name_of(c) for c in rs})
        edu = sorted({txt(c, "EDUMODE") for c in rs})
        notes.append(f"companies {names}; EDUMODE {edu}")
        if names != [BOOK]: raise Stop("; ".join(notes) + f" - the only loaded company must be {BOOK}")
        if edu != ["No"]: raise Stop("; ".join(notes) + " - Education mode (or no mode reading)")
    elif step == "a02":
        rs = rows(root, "COMPANY")
        if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows, expected exactly 1")
        vals = {k: txt(rs[0], k) for k in ("NAME", "GUID", "ALTVCHID", "ALTMSTID", "COMPANYNUMBER", "LASTVOUCHERDATE", "BOOKSFROM")}
        notes.append(str({k: (v[:13] if k == "GUID" else v) for k, v in vals.items()}))
        if vals["NAME"] != BOOK: raise Stop("; ".join(notes) + f" - the answer is not for {BOOK}")
        if not (vals["GUID"] and vals["ALTVCHID"] and vals["ALTMSTID"]): bad += 1; notes.append("a mark or the GUID is missing")
    elif step == "itc":
        cf = opt("--company")
        if not cf: raise Err("itc needs --company (this book's book-extent answer): a check that cannot name the book is not a check")
        guid = company_guid(cf)
        led = ledgers(root, "the answer")
        ref = ledgers(common(committed_path(opt))[1], "the committed capture")
        extras = declared_extras(argv, ref)
        for n, r in led.items():
            g, b = txt(r, "GUID"), txt(r, "BRIDGECOMPANYGUID")
            if not g.startswith(guid) or b != guid: raise Stop(f"ledger {n!r} does not carry this book's GUID: wrong company")
        if "--list-extras" in argv:
            if extras: raise Err("--list-extras takes no --extra")
            if set(ref) - set(led): raise Stop(f"committed ledgers missing from the answer: {sorted(set(ref) - set(led))}. Do not substitute: report")
            found = sorted(set(led) - set(ref))
            return 0, "\n".join([f'--extra "{n}"' for n in found] + [f"itc: {size} B; {len(found)} ledger(s) beyond the committed {len(ref)}; nothing is declared or checked by this listing: the coordinator confirms the spelling, then step 3 declares them"])
        check_sets(led, ref, extras)
        if book_guids(led) != book_guids(ref):
            raise Stop(f"the book GUID {book_guids(led)} differs from the committed capture's {book_guids(ref)}: another book with the same ledger names? Report, do not substitute")
        counts = {k: {"Yes": 0, "No": 0, "absent": 0} for k in ("committed", "extra")}
        inherited = 0
        for kind, names in (("committed", sorted(set(ref))), ("extra", sorted(extras))):
            for n in names:
                es = entries_of(led[n], n)
                if not es: continue
                if len(es) > 1: raise Stop(f"ledger {n!r} holds {len(es)} GSTDETAILS entries with content: more than one is not covered by this sheet")
                e = es[0]
                frm = txt(e, "APPLICABLEFROM")
                if not re.fullmatch(r"\d{8}", frm): raise Stop(f"ledger {n!r}: the entry has no readable APPLICABLEFROM ({frm!r})")
                direct = [c for c in e if c.tag == "GSTINELIGIBLEITC"]
                if len(list(e.iter("GSTINELIGIBLEITC"))) != len(direct): raise Stop(f"ledger {n!r}: GSTINELIGIBLEITC sits below the entry's own level: not covered")
                if len(direct) > 1: raise Stop(f"ledger {n!r}: GSTINELIGIBLEITC appears {len(direct)} times in one entry")
                st = "absent" if not direct else (direct[0].text or "").strip()
                if st not in counts[kind]: raise Stop(f"ledger {n!r}: GSTINELIGIBLEITC reads {st!r}, not Yes or No")
                counts[kind][st] += 1
                src = txt(e, "SRCOFGSTDETAILS")
                inh = src.startswith("As per")
                inherited += inh
                tag = "committed ledger" if kind == "committed" else "EXTRA ledger: wire value from this answer only"
                table.append(f"LOOK {len(table) + 1}: {n} | parent {txt(led[n], 'PARENT')} | source {src}{' (INHERITED)' if inh else ''} | applicable-from {frm[6:]}-{frm[4:6]}-{frm[:4]} | wire GSTINELIGIBLEITC {st} | {tag}")
        if not table: raise Stop("no ledger holds a GSTDETAILS entry: nothing to look at")
        tot = {k: counts["committed"][k] + counts["extra"][k] for k in ("Yes", "No", "absent")}
        notes.append(f"{len(led)} ledgers = {len(ref)} committed + {len(extras)} declared extra; {len(table)} with one GSTDETAILS entry ({inherited} inherited); GSTINELIGIBLEITC Yes {tot['Yes']}, No {tot['No']}, absent {tot['absent']} (committed ledgers {counts['committed']}, extra ledgers {counts['extra']}); committed ledger names all present; book GUID equals the 9-10 Oct capture's: yes")
        if not tot["No"]: notes.append("wire No: not in this book, not covered")
        if sum(1 for v in tot.values() if v) == 1: notes.append("all entries read the same wire value: one wire value seen")
    elif step == "vs-committed":
        cp = committed_path(opt)
        h = hashlib.sha256(data).hexdigest()
        led, ref = ledgers(root, "the answer"), ledgers(common(cp)[1], "the committed capture")
        extras = declared_extras(argv, ref)
        check_sets(led, ref, extras)
        if h == COMMITTED_SHA256 and size == COMMITTED_SIZE:
            return 0, f"vs-committed: {size} B, sha256 {h} equals the committed capture: byte-identical"
        if book_guids(led) != book_guids(ref):
            raise Stop(f"the book GUID {book_guids(led)} differs from the committed capture's {book_guids(ref)}: another book. Report, do not substitute")
        notes.append(f"not byte-identical (sha256 {h}); the {len(ref)} committed rows compared on their GSTDETAILS entries leaf by leaf; {len(extras)} declared extra ledger(s) {sorted(extras)} each with one entry, not compared (no committed value)")
        diffs, other = [], []
        for n in sorted(ref):
            fa = {}; fr = {}
            for i, g in enumerate(entries_of(led[n], n)): fa.update(flat(g, f"E{i}"))
            for i, g in enumerate(entries_of(ref[n], n)): fr.update(flat(g, f"E{i}"))
            d = sorted(k for k in set(fa) | set(fr) if fa.get(k) != fr.get(k))
            if d:
                diffs.append(n)
                notes.append(f"{n}: " + "; ".join(f"{k} {fr.get(k, 'absent')}->{fa.get(k, 'absent')}" for k in d[:4]) + ("; ..." if len(d) > 4 else ""))
            rest = lambda r: canon(ET.fromstring(re.sub(r"<GSTDETAILS\.LIST>.*?</GSTDETAILS\.LIST>|<GSTDETAILS\.LIST\s*/>", "", ET.tostring(r, encoding="unicode"), flags=re.S)))
            if rest(led[n]) != rest(ref[n]): other.append(n)
        if diffs: bad += 1; notes.append(f"EXPECT the GSTDETAILS entries as committed; they differ on {diffs}: the wire may have moved since 9-10 Oct")
        notes.append(f"other content of the committed rows (not GSTDETAILS): {'equal' if not other else 'differs on ' + str(other) + ' (recorded, not a finding)'}")
    return (1 if bad else 0), "\n".join(table + [f"{step}: " + "; ".join(notes)])

def main(argv):
    try:
        code, msg = run(argv)
    except Stop as e:
        code, msg = 3, f"{argv[1] if len(argv) > 1 else '?'}: {e}"
    except BaseException as e:                       # any other failure is "could not check", never a pass or a finding
        code, msg = 2, f"{argv[1] if len(argv) > 1 else '?'}: could not run the check ({type(e).__name__}: {e})"
    head, _, last = msg.rpartition("\n")
    out = (head + "\n" if head else "") + f"{['PASS', 'FINDING', 'CHECK-ERROR', 'STOP'][code]} {last}"
    print(out.encode("ascii", "backslashreplace").decode("ascii"))
    return code

if __name__ == "__main__":
    sys.exit(main(sys.argv))
