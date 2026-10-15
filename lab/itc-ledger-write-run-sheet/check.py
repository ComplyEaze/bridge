#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [options] : checks one answer of the ledger input tax credit write sheet. Nothing here sends anything.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold; keep the file, write the
printed line down, go on only where the step says so. Exit 3 = STOP: a bound, a completeness rule or an identity rule
broke; stop and send nothing more. Exit 2 = the check could not run (any error at all, a missing file, a missing
argument): never a pass. The last line printed starts with PASS, FINDING, CHECK-ERROR or STOP; the lines before it
(steps base and led) are the table of ledgers. Output is ASCII only.
Steps:
  a01 ANSWER                          company list: the only loaded company is BRIDGE SHAPE LAB, Education mode No
  a02 ANSWER [--marks-from FILE]      book extent: one COMPANY row, BRIDGE SHAPE LAB, with its GUID and marks; with
                                      --marks-from, the movement of ALTMSTID is printed (recorded) and a move of ALTVCHID
                                      (a voucher mark; nothing in this sheet touches a voucher) is a FINDING
  base ANSWER --company a02-FILE [--created NAME]
                                      the ledger GST read of the book as it is: every LEDGER row carries this book's GUID;
                                      with --created, no ledger is named like the one the sheet creates; prints the count
                                      and, per ledger with a GSTDETAILS entry, its entries (applicable-from, taxability,
                                      source, the state of GSTINELIGIBLEITC Yes / No / absent, the rate rows)
  led5 / led6 / led7 ANSWER --company a02-FILE --before BASE-FILE --created NAME --parent PARENT   (the three forms of one check, one per step)
                                      led5 (step 5) takes also exactly --own-only; led6 (step 6) also exactly --prev FILE
                                      --expect-entry DATE:TAXABILITY:SOURCE:IGST --alterid-plus-1; led7 (step 7) the same and
                                      --strict-row. Any missing or extra option is exit 2 (nothing is checked). What the check
                                      does with each option:
                                      the ledger GST read after the ledger was created or altered: exactly one more
                                      ledger than BASE, the created one once and under PARENT, every other ledger row
                                      equal to its row in BASE (a difference is a FINDING); prints the created ledger's
                                      entries and, with --prev, which leaves of them moved since that earlier answer and
                                      the state of GSTINELIGIBLEITC then and now. --own-only: every entry is one Tally added
                                      itself (source starts with As per, no rate); --expect-entry: exactly one entry with that
                                      applicable-from (YYYYMMDD), taxability, source and IGST rate (a FINDING if not). The
                                      created ledger's row, apart from GSTDETAILS and ALTERID, is compared with --prev: a move of GUID, MASTERID,
                                      PARENT or OPENINGBALANCE is always a FINDING, any other moved leaf is a note, and a
                                      FINDING with --strict-row. --own-only also allows no second entry and records
                                      whether the entry is dated the book's first day (BOOKSFROM of the --company answer). --alterid-plus-1 (needs --prev): the created
                                      ledger's ALTERID rose by exactly 1 since --prev, else a FINDING (an extra save). Any
                                      option this check does not know is exit 2
  same A B                            two answers of one request: byte-identical, or equal leaf by leaf under COLLECTION
                                      (the header counters may differ and are recorded); both must be full envelopes
"""
import os, re, sys
from xml.etree import ElementTree as ET

BOOK = "BRIDGE SHAPE LAB"
CAPS = {"a01": 8000, "a02": 6000, "base": 100000, "led": 110000}

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

def flat(e, path=""):
    """Leaf map of an element: path (with child index) -> text."""
    out, seen = {}, {}
    for c in e:
        i = seen.get(c.tag, 0); seen[c.tag] = i + 1
        p = f"{path}/{c.tag}[{i}]"
        if len(c): out.update(flat(c, p))
        else: out[p] = (c.text or "").strip()
    return out

def company_row(path):
    _, root = common(path)
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows in the book-extent answer, expected 1")
    if name_of(rs[0]) != BOOK: raise Stop(f"the book-extent answer is for {name_of(rs[0])!r}, not {BOOK}")
    return rs[0]

def company_guid(path):
    g = txt(company_row(path), "GUID")
    if not g: raise Err("no GUID in the book-extent answer")
    return g

def ledgers(root, what):
    rs = rows(root, "LEDGER")
    names = [name_of(r) for r in rs]
    if not rs: raise Stop(f"{what}: no LEDGER row: nothing was captured")
    if len(set(names)) != len(names): raise Stop(f"{what}: a ledger name repeats")
    return {name_of(r): r for r in rs}

def entries_of(row):
    """The GSTDETAILS.LIST elements of a ledger row that hold content (an empty placeholder holds only whitespace)."""
    return [g for g in row.iter("GSTDETAILS.LIST") if has_text(g)]

def itc_state(entry, name):
    direct = [c for c in entry if c.tag == "GSTINELIGIBLEITC"]
    if len(list(entry.iter("GSTINELIGIBLEITC"))) != len(direct): raise Stop(f"ledger {name!r}: GSTINELIGIBLEITC sits below the entry's own level: not covered")
    if len(direct) > 1: raise Stop(f"ledger {name!r}: GSTINELIGIBLEITC appears {len(direct)} times in one entry")
    st = "absent" if not direct else (direct[0].text or "").strip()
    if st not in ("Yes", "No", "absent"): raise Stop(f"ledger {name!r}: GSTINELIGIBLEITC reads {st!r}, not Yes or No")
    return st

def entry_line(name, parent, e):
    frm = txt(e, "APPLICABLEFROM")
    if not re.fullmatch(r"\d{8}", frm): raise Stop(f"ledger {name!r}: an entry has no readable APPLICABLEFROM ({frm!r})")
    rates = []
    for st in e.iter("STATEWISEDETAILS.LIST"):
        for r in st.iter("RATEDETAILS.LIST"):
            rates.append(f"{txt(r, 'GSTRATEDUTYHEAD')} {txt(r, 'GSTRATE') or 'no-rate'}")
    return (f"{name} | parent {parent} | applicable-from {frm[6:]}-{frm[4:6]}-{frm[:4]} | taxability {txt(e, 'TAXABILITY') or 'absent'} | "
            f"source {txt(e, 'SRCOFGSTDETAILS') or 'absent'} | wire GSTINELIGIBLEITC {itc_state(e, name)} | rates {', '.join(rates) or 'none'}")

def check_guids(led, guid):
    for n, r in led.items():
        g, b = txt(r, "GUID"), txt(r, "BRIDGECOMPANYGUID")
        if not g.startswith(guid) or b != guid: raise Stop(f"ledger {n!r} does not carry this book's GUID: wrong company")

def by_date(row, name):
    out = {}
    for e in entries_of(row):
        d = txt(e, "APPLICABLEFROM")
        if d in out: raise Stop(f"{name!r}: two entries on {d}")
        out[d] = e
    return out

def run(argv):
    if len(argv) < 3: raise Err("usage: check.py STEP ANSWER ...  (see the top of check.py)")
    opt = lambda k: argv[argv.index(k) + 1] if k in argv and argv.index(k) + 1 < len(argv) else None
    known = {"--company", "--marks-from", "--created", "--before", "--parent", "--prev", "--own-only", "--expect-entry", "--strict-row", "--alterid-plus-1"}
    unknown = [a for a in argv[3:] if re.match(r"^[-\u2010-\u2015\u2212]", a) and a not in known and not os.path.exists(a)]
    if unknown: raise Err(f"unknown option(s) {unknown}: nothing was checked")
    label = argv[1]
    base_led = {"--company", "--before", "--created", "--parent"}
    led_steps = {"led5": base_led | {"--own-only"},
                 "led6": base_led | {"--prev", "--expect-entry", "--alterid-plus-1"},
                 "led7": base_led | {"--prev", "--expect-entry", "--alterid-plus-1", "--strict-row"}}
    if label in led_steps:
        given = {a for a in argv[3:] if a in known}
        if given != led_steps[label]:
            raise Err(f"{label} takes exactly {sorted(led_steps[label])}; missing {sorted(led_steps[label] - given)}, not allowed {sorted(given - led_steps[label])}: nothing was checked")
        argv = [argv[0], "led"] + list(argv[2:])
    elif label == "led" and os.environ.get("BRIDGE_ITCW_ENGINE") != "1":
        raise Err("use led5 (step 5), led6 (step 6) or led7 (step 7): each takes exactly the options its step prints")
    allowed = {"a01": set(), "a02": {"--marks-from"}, "base": {"--company", "--created"},
               "led": {"--company", "--before", "--created", "--parent", "--prev", "--own-only", "--expect-entry", "--strict-row", "--alterid-plus-1"}}
    if argv[1] in allowed:
        ignored = [a for a in argv[3:] if a.startswith("--") and a in known and a not in allowed[argv[1]]]
        if ignored: raise Err(f"option(s) {ignored} do not belong to step {argv[1]}: nothing was checked")
    valued = {"--company", "--marks-from", "--created", "--before", "--parent", "--prev", "--expect-entry"}
    if argv[1] == "same" and (len(argv) != 4 or any(x.startswith("-") for x in argv[2:])): raise Err("same takes exactly two files and no option")
    for k in valued | known:
        n = argv[3:].count(k)
        if n > 1: raise Err(f"option {k} is given {n} times: nothing was checked")
        if k in valued and k in argv[3:]:
            i = argv.index(k, 3)
            if i + 1 >= len(argv) or argv[i + 1].startswith("--") or not argv[i + 1].strip(): raise Err(f"option {k} has no value: nothing was checked")
    stray = [] if argv[1] == "same" else [a for i, a in enumerate(argv[3:], 3) if not a.startswith("-") and argv[i - 1] not in valued]
    if stray: raise Err(f"argument(s) {stray} belong to no option (a missing dash?): nothing was checked")
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
        prev = opt("--marks-from")
        if prev and os.path.realpath(prev) == os.path.realpath(path): raise Err("--marks-from names the answer itself: that proves nothing")
        if prev:
            pr = company_row(prev)
            if txt(pr, "GUID") != vals["GUID"]: raise Stop("the earlier answer is for another GUID")
            notes.append("marks since " + os.path.basename(prev) + ": " + ", ".join(f"{k} {txt(pr, k)}->{vals[k]}" for k in ("ALTVCHID", "ALTMSTID")) + " (ALTMSTID recorded)")
            if txt(pr, "ALTVCHID") != vals["ALTVCHID"]: bad += 1; notes.append("EXPECT ALTVCHID unchanged (no voucher is touched); it moved")
    elif step == "base":
        cf = opt("--company")
        if not cf: raise Err("base needs --company (this book's book-extent answer): a check that cannot name the book is not a check")
        guid = company_guid(cf)
        led = ledgers(root, "the answer")
        check_guids(led, guid)
        created = opt("--created")
        if created and any(n.casefold() == created.casefold() for n in led): raise Stop(f"a ledger named like {created!r} (names are not case-sensitive in Tally) is already in the book: the book is not as the sheet expects")
        withg = 0
        for n in sorted(led):
            es = entries_of(led[n])
            if es: withg += 1
            for d, e in sorted(by_date(led[n], n).items()): table.append(entry_line(n, txt(led[n], "PARENT"), e))
        notes.append(f"{len(led)} ledgers, {withg} with GSTDETAILS content; book GUID {guid[:13]}")
    else:  # led
        cf = opt("--company")
        if not cf: raise Err("led needs --company (this book's book-extent answer): a check that cannot name the book is not a check")
        guid = company_guid(cf)
        led = ledgers(root, "the answer")
        check_guids(led, guid)
        bf, name, parent = opt("--before"), opt("--created"), opt("--parent")
        if not (bf and name and parent): raise Err("led needs --before, --created and --parent")
        before = ledgers(common(bf)[1], "the earlier read (--before)")
        if name in before: raise Stop(f"the --before answer already holds {name!r}: it is not the book before the ledger was created")
        if name not in led: raise Stop(f"the created ledger {name!r} is not in the answer")
        if len(led) != len(before) + 1: raise Stop(f"{len(led)} ledgers, expected {len(before)} + 1: something else was created or removed")
        if set(led) - {name} != set(before): raise Stop(f"the other ledgers are not the same set as before: new {sorted(set(led) - {name} - set(before))}, gone {sorted(set(before) - set(led))}")
        if txt(led[name], "PARENT") != parent: raise Stop(f"{name!r} is under {txt(led[name], 'PARENT')!r}, expected {parent!r}")
        changed = [n for n in sorted(before) if canon(led[n]) != canon(before[n])]
        if changed: bad += 1; notes.append(f"EXPECT every other ledger row equal to its row before; they differ on {changed} (the book moved elsewhere)")
        else: notes.append(f"the other {len(before)} ledger rows equal their rows before")
        mine = by_date(led[name], name)
        notes.append(f"{name!r} holds {len(mine)} GSTDETAILS entr{'y' if len(mine) == 1 else 'ies'}")
        for d, e in sorted(mine.items()): table.append(entry_line(name, parent, e))
        if "--prev" in argv and not opt("--prev"): raise Err("--prev needs a file")
        if ("--strict-row" in argv or "--alterid-plus-1" in argv) and "--prev" not in argv: raise Err("--strict-row and --alterid-plus-1 need --prev")
        if "--expect-entry" in argv and "--own-only" in argv: raise Err("--own-only and --expect-entry are exclusive")
        ob = txt(led[name], "OPENINGBALANCE")
        if ob and not re.fullmatch(r"-?0+(\.0+)?", ob): bad += 1; notes.append(f"EXPECT a zero opening balance; it reads {ob!r}")
        if "--own-only" in argv:
            books = txt(company_row(cf), "BOOKSFROM")
            if len(mine) > 1: bad += 1; notes.append(f"EXPECT at most one entry of Tally's own; there are {len(mine)}")
            for d, e in mine.items():
                if books and d != books: notes.append(f"Tally's own entry is dated {d}, not the book's first day {books} (recorded: whether this read exports stored dates or period dates is not known)")
                rates = [r for r in e.iter("GSTRATE") if (r.text or "").strip()]
                if not txt(e, "SRCOFGSTDETAILS").startswith("As per") or rates:
                    bad += 1; notes.append(f"EXPECT only Tally's own entries (source As per ..., no rate); the entry {d} is not one")
        ex = opt("--expect-entry")
        if "--expect-entry" in argv:
            if not ex: raise Err("--expect-entry needs DATE:TAXABILITY:SOURCE:IGST")
            try: d, tax, src, igst = ex.split(":")
            except ValueError: raise Err("--expect-entry needs DATE:TAXABILITY:SOURCE:IGST")
            e = mine.get(d)
            got = None
            if e is not None:
                ig = [txt(r, "GSTRATE") for r in e.iter("RATEDETAILS.LIST") if txt(r, "GSTRATEDUTYHEAD") == "IGST"]
                got = (txt(e, "TAXABILITY"), txt(e, "SRCOFGSTDETAILS"), ig[0] if len(ig) == 1 else None)
            def num(v):
                try: return float(v)
                except (TypeError, ValueError): return None
            if len(mine) != 1 or e is None or got[0] != tax or got[1] != src or num(got[2]) != num(igst) or num(igst) is None:
                bad += 1; notes.append(f"EXPECT exactly one entry {d} / {tax} / {src} / IGST {igst}; got {len(mine)} entr{'y' if len(mine) == 1 else 'ies'}" + (f", entry {d}: {got}" if got else f", none dated {d}"))
        prev = opt("--prev")
        if prev:
            pl = ledgers(common(prev)[1], "the earlier led answer (--prev)")
            if name not in pl: raise Stop(f"the --prev answer has no {name!r}")
            old = by_date(pl[name], name)
            def rowleaves(r):
                x = ET.fromstring(ET.tostring(r)); [x.remove(g) for g in list(x) if g.tag == "GSTDETAILS.LIST"]
                f = flat(x); f = {k: v for k, v in f.items() if not k.startswith("/ALTERID")}
                return f
            ra, rp = rowleaves(led[name]), rowleaves(pl[name])
            rd = sorted(k for k in set(ra) | set(rp) if ra.get(k) != rp.get(k))
            hard = [k for k in rd if k in ("/GUID[0]", "/MASTERID[0]", "/PARENT[0]", "/OPENINGBALANCE[0]")]
            mv = "; ".join(f"{k} {rp.get(k, 'absent')}->{ra.get(k, 'absent')}" for k in rd[:6])
            if hard or (rd and "--strict-row" in argv): bad += 1; notes.append("EXPECT the ledger row unchanged outside its GST details (GUID, MASTERID, opening balance, parent" + (", and every other leaf" if "--strict-row" in argv else "") + "); moved: " + mv)
            elif rd: notes.append("ledger row leaves moved outside its GST details (recorded, not a finding here; at step 7 this is the decision rule's `moved:`): " + mv)
            else: notes.append("the ledger row outside its GST details equals the earlier answer's (ALTERID apart)")
            notes.append(f"created ledger MASTERID {txt(led[name], 'MASTERID')}, ALTERID {txt(pl[name], 'ALTERID')}->{txt(led[name], 'ALTERID')}")
            if "--alterid-plus-1" in argv:
                try: ok = int(txt(led[name], "ALTERID")) - int(txt(pl[name], "ALTERID")) == 1
                except ValueError: ok = False
                if not ok: bad += 1; notes.append("EXPECT the ledger's ALTERID to rise by exactly 1 (one save); it did not: an extra save, or none")
            for d in sorted(set(mine) | set(old)):
                if d not in mine or d not in old:
                    notes.append(f"entry {d}: {'new' if d not in old else 'gone'} since {os.path.basename(prev)}"); continue
                fa, fp = flat(mine[d]), flat(old[d])
                diff = sorted(k for k in set(fa) | set(fp) if fa.get(k) != fp.get(k))
                other = [k for k in diff if k != "/GSTINELIGIBLEITC[0]"]
                notes.append(f"entry {d} since {os.path.basename(prev)}: GSTINELIGIBLEITC {itc_state(old[d], name)}->{itc_state(mine[d], name)}; "
                             + ("nothing else moved" if not other else "also moved: " + "; ".join(f"{k} {fp.get(k, 'absent')}->{fa.get(k, 'absent')}" for k in other[:6])))
    force = [k for k in ("--prev", "--expect-entry", "--own-only", "--strict-row", "--alterid-plus-1", "--marks-from") if k in argv]
    if step in ("led", "a02"): notes.append("checks in force: " + (" ".join(force) if force else "none of the optional ones") + " (compare with the command the step prints; a missing one is not a pass)")
    return (1 if bad else 0), "\n".join(table + [f"{label}: " + "; ".join(notes)])

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
