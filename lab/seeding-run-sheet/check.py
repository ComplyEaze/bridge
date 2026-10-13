#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [--company FILE] [--expect-coded N] [--expect K=V,K=V] : checks one answer of the seeding sheet.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold; keep the file, write the
printed line down, go on. Exit 3 = STOP: a bound, a completeness rule or an identity rule broke; stop and send
nothing more. Exit 2 = the check could not run (any error at all, a missing file, a missing argument): never a pass.
One line is printed. `check.py same A B`: 0 when the two files are byte-identical, 1 when not, 2 on an error.
--census FILE: the census answer of the same day (its row count is the voucher count the read may hold).
--company FILE: this book's book-extent answer (step a02 or b02); its GUID must prefix the GUID of every voucher, stock item,
stock group and tax unit.
--row NAME --code C --from YYYYMMDD --before FILE: steps a09 and a10: the one row NAME holds exactly one HSN entry with code C, applicable from the date, source "Specify Details Here", and every other row is identical to its row in FILE (FINDING when not).
--expect-coded N: steps a09 and a10: exactly N rows carry an HSN code (FINDING when the count differs).
--before FILE (feat): every fetched field other than ALTMSTID and ALTVCHID must read as in FILE (FINDING when not).
--expect K=V,...: step feat (required): each named flag of the company-features answer appears exactly once and must read V (FINDING when not).
Steps: a01 company list, a02 and a12 book extent, a09 stock items, a10 stock groups, feat company features, obj company object."""
import os, re, sys
from xml.etree import ElementTree as ET

BOOK = {"a01": "BRIDGE SHAPE LAB", "a02": "BRIDGE SHAPE LAB", "a12": "BRIDGE SHAPE LAB",
        "b01": "BRIDGE CORPUS FOREX", "b02": "BRIDGE CORPUS FOREX", "b05": "BRIDGE CORPUS FOREX"}
CAPS = {"feat": 8000, "obj": 130000, "a01": 8000, "b01": 8000, "a02": 6000, "a12": 6000, "b02": 6000, "b05": 6000, "a04": 24000, "a06": 24000,
        "a09": 40000, "a10": 20000, "a11": 60000, "a13": 40000, "b03": 40000, "b04": 60000}
PER_VOUCHER = {"a05": (80000, 240000), "a07": (80000, 240000)}
NEEDS_CENSUS = {"a05", "a07", "a08"}
NEEDS_COMPANY = {"a09", "a10", "feat", "obj"}
NEEDS_REFERENCE = {"a08"}                    # its cap is twice the measured answer of step a07

class Stop(Exception): pass
class Err(Exception): pass

def text_of(data):
    if data[:2] == b"\xff\xfe": return data[2:].decode("utf-16-le")
    if data[:2] == b"\xfe\xff": return data[2:].decode("utf-16-be")
    if len(data) > 1 and data[1:2] == b"\x00": return data.decode("utf-16-le")
    return data.decode("utf-8")

def tree(path):
    data = open(path, "rb").read()
    t = text_of(data)
    t = re.sub(r"&#(?:[0-8]|1[1-2]|1[4-9]|2[0-9]|3[0-1]);", "", t)        # numeric references XML 1.0 refuses
    t = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f]", "", t)                      # raw control bytes some names carry
    return data, t, ET.fromstring(t)

def collection(root):
    col = root.find(".//COLLECTION")
    if col is None: raise Stop("no COLLECTION in the answer")
    return col

def rows(root, tag):
    """Rows are the children of the COLLECTION. The header carries counter elements of the same names (COMPANY, VOUCHER,
    STOCKITEM ...) holding a number; they are not rows."""
    return [e for e in collection(root) if e.tag == tag]

def name_of(e): return e.get("NAME") or (e.findtext("NAME") or "").strip()
def txt(e, tag): return (e.findtext(tag) or "").strip()
def has_text(e): return any((c.text or "").strip() for c in e.iter() if c is not e)

def common(step, path):
    data, t, root = tree(path)
    if root.tag != "ENVELOPE": raise Stop(f"root is {root.tag}, not ENVELOPE")
    if "<LINEERROR" in t: raise Stop("the answer carries a LINEERROR")
    st = root.findtext(".//HEADER/STATUS")
    if st is not None and st.strip() == "0": raise Stop("the answer header STATUS is 0")
    if step != "obj": collection(root)       # an Object answer has a BODY, no COLLECTION
    return data, root

def prefix_check(rs, guid, what):
    """Every row must carry a GUID that starts with this book's GUID: a row of another book, or with no GUID, is a STOP."""
    for r in rs:
        g = txt(r, "GUID")
        if not g: raise Stop(f"a {what} row has no GUID: its book cannot be proved")
        if not g.startswith(guid): raise Stop(f"a {what} GUID does not start with this book's GUID: wrong company")

def census_rows(path):
    _, root = common("census", path)
    return rows(root, "VOUCHER")

def company_name(path):
    _, root = common("a02", path)
    rs = rows(root, "COMPANY")
    return name_of(rs[0]) if len(rs) == 1 else ""

def canon(e):
    return re.sub(r"\s+", " ", ET.tostring(e, encoding="unicode")).strip()

def company_guid(path):
    _, root = common("a02", path)
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows in the book-extent answer, expected 1")
    g = txt(rs[0], "GUID")
    if not g: raise Err("no GUID in the book-extent answer")
    return g

def run(argv):
    if argv[1] == "same":
        if os.path.realpath(argv[2]) == os.path.realpath(argv[3]): raise Err("the same file given twice proves nothing")
        a, b = open(argv[2], "rb").read(), open(argv[3], "rb").read()
        if not a or not b: raise Err("an empty file proves nothing")
        if a == b: return 0, f"same: {len(a)} B vs {len(b)} B, identical"
        # not byte-identical: equal when everything under COLLECTION is equal leaf by leaf; the header's own counters
        # (DESC/CMPINFO) may differ and are recorded, not a stop (they count what the last request fetched)
        _, _, ra = tree(argv[2]); _, _, rb = tree(argv[3])
        ca, cb = ra.find(".//COLLECTION"), rb.find(".//COLLECTION")
        if ca is None or cb is None: return 1, f"same: {len(a)} B vs {len(b)} B, DIFFERENT (no COLLECTION to compare leaf by leaf)"
        if canon(ca) != canon(cb): return 1, f"same: {len(a)} B vs {len(b)} B, DIFFERENT under COLLECTION"
        def counters(r):
            c = r.find(".//CMPINFO")
            return {} if c is None else {e.tag: (e.text or "").strip() for e in c}
        ka, kb = counters(ra), counters(rb)
        moved = sorted(f"{k} {ka.get(k)}->{kb.get(k)}" for k in set(ka) | set(kb) if ka.get(k) != kb.get(k))
        return 0, f"same: {len(a)} B vs {len(b)} B, equal leaf by leaf under COLLECTION; header counters differ (recorded, not a stop): {moved}"
    step, path = argv[1], argv[2]
    opt = lambda k: argv[argv.index(k) + 1] if k in argv else None
    data, root = common(step, path)
    size, notes, bad = len(data), [], 0
    n = None
    if step in NEEDS_CENSUS:
        cf = opt("--census")
        if not cf: raise Err("needs --census (the census answer of the same day)")
        n = len(census_rows(cf))
        if n < 1: raise Stop("the census of the day holds no voucher: the data read is not sent")
    cap = CAPS.get(step)
    if step in PER_VOUCHER:
        per, ceiling = PER_VOUCHER[step]
        cap = min(n * per, ceiling)
    if step in NEEDS_REFERENCE:
        rf = opt("--reference")
        if not rf: raise Err("needs --reference (the answer of step a07)")
        cap = min(2 * len(open(rf, "rb").read()), 240000)
    if cap is None: raise Err(f"unknown step {step}")
    if size > cap: raise Stop(f"{size} B is above the cap {cap} B")
    notes.append(f"{size} B (cap {cap})")
    guid = None
    cf = opt("--company")
    if step in NEEDS_COMPANY and not cf: raise Err("needs --company (this book's book-extent answer)")
    if cf: guid = company_guid(cf)

    if step in ("a01", "b01"):
        rs = rows(root, "COMPANY")
        names = sorted({name_of(c) for c in rs})
        edu = sorted({txt(c, "EDUMODE") for c in rs})
        notes.append(f"companies {names}; EDUMODE {edu}")
        if names != [BOOK[step]]: raise Stop("; ".join(notes) + f" - the only loaded company must be {BOOK[step]}")
        if edu != ["No"]: raise Stop("; ".join(notes) + " - Education mode (or no mode reading): one-day windows may be refused")
    elif step in ("a02", "a12", "b02", "b05"):
        rs = rows(root, "COMPANY")
        if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows, expected exactly 1")
        vals = {k: txt(rs[0], k) for k in ("NAME", "GUID", "ALTVCHID", "ALTMSTID", "COMPANYNUMBER", "LASTVOUCHERDATE", "BOOKSFROM")}
        notes.append(str({k: (v[:13] if k == "GUID" else v) for k, v in vals.items()}))
        if vals["NAME"] != BOOK[step]: raise Stop("; ".join(notes) + f" - the answer is not for {BOOK[step]}")
        if not (vals["GUID"] and vals["ALTVCHID"] and vals["ALTMSTID"]): bad += 1; notes.append("a mark or the GUID is missing")
    elif step in ("a04", "a06"):
        vr = rows(root, "VOUCHER")
        want = 1 if step == "a04" else 2
        notes.append(f"{len(vr)} vouchers (EXPECT {want})")
        if len(vr) > 8: raise Stop("; ".join(notes) + " - more than 8 vouchers on the day: do not send the data read")
        if len(vr) < 1: raise Stop("; ".join(notes) + " - no voucher on the day: do not send the data read, tell us")
        prefix_check(vr, guid, "voucher")
        if len(vr) != want: bad += 1
    elif step in ("a05", "a07", "a08"):
        vs = rows(root, "VOUCHER")
        notes.append(f"{len(vs)} vouchers (census {n})")
        if len(vs) != n: bad += 1; notes.append("count differs from the census")
        prefix_check(vs, guid, "voucher")
        target = "SHAPELAB-C9-costcentre" if step == "a05" else "SHAPELAB-B6-two-batches"
        hit = [v for v in vs if txt(v, "NARRATION") == target]
        if len(hit) != 1:
            bad += 1; notes.append(f"{len(hit)} vouchers with narration {target} (EXPECT 1)")
        else:
            v = hit[0]
            if step == "a05":
                al = sorted((txt(a, "NAME"), txt(a, "AMOUNT")) for a in v.iter("COSTCENTREALLOCATIONS.LIST"))
                notes.append(f"cost-centre allocations {al}")
                if txt(v, "VOUCHERTYPENAME") != "Journal": bad += 1; notes.append("EXPECT a Journal")
                if al != [("Assembly", "-400.00"), ("Trading", "-600.00")]:
                    bad += 1; notes.append("EXPECT Trading -600.00 and Assembly -400.00 (recorded in the reference, no wire bytes kept)")
            else:
                notes.append(f"type {txt(v, 'VOUCHERTYPENAME')}, number {txt(v, 'VOUCHERNUMBER')}")
                if txt(v, "VOUCHERTYPENAME") != "Purchase": bad += 1; notes.append("EXPECT a Purchase")
                bat = sorted({txt(b, "BATCHNAME") for b in v.iter("BATCHALLOCATIONS.LIST")} - {""})
                god = sorted({txt(b, "GODOWNNAME") for b in v.iter("BATCHALLOCATIONS.LIST")} - {""})
                notes.append(f"batch names {bat}; godowns {god}")
                if step == "a08":
                    inv = len(list(v.iter("ALLINVENTORYENTRIES.LIST")))
                    acc = len(list(v.iter("ACCOUNTINGALLOCATIONS.LIST")))
                    notes.append(f"inventory entries {inv}; accounting allocations {acc}")
                    if inv < 1 or len(bat) < 2:
                        bad += 1; notes.append("EXPECT at least 1 inventory entry and 2 non-empty batch names (design note: two batches)")
    elif step in ("a09", "a10"):
        tag = "STOCKITEM" if step == "a09" else "STOCKGROUP"
        rs = rows(root, tag)
        coded = sum(1 for r in rs if any((e.text or "").strip() for e in r.iter() if e.tag in ("HSNCODE", "HSN")))
        gst = sum(1 for r in rs if any(has_text(e) for e in r.iter("GSTDETAILS.LIST")))
        notes.append(f"{len(rs)} rows; {coded} with an HSN code; {gst} with GST details content")
        want = 11 if step == "a09" else 3
        if len(rs) != want: bad += 1; notes.append(f"EXPECT {want} rows (earlier capture of this book)")
        ec = opt("--expect-coded")
        if ec is not None and coded != int(ec): bad += 1; notes.append(f"EXPECT {ec} rows with an HSN code, found {coded}")
        gst_names = sorted(name_of(r) for r in rs if any(has_text(e) for e in r.iter("GSTDETAILS.LIST")))
        notes.append(f"rows with GST content: {gst_names}")
        rowname = opt("--row")
        if rowname is not None:
            code, frm, before = opt("--code"), opt("--from"), opt("--before")
            if not (code and frm and before): raise Err("--row needs --code, --from and --before")
            target = [r for r in rs if name_of(r) == rowname]
            if len(target) != 1:
                bad += 1; notes.append(f"EXPECT exactly one row named {rowname}, found {len(target)}")
            else:
                lists = [h for h in target[0].iter("HSNDETAILS.LIST") if has_text(h)]
                entries = [(txt(h, "HSNCODE"), txt(h, "APPLICABLEFROM"), txt(h, "SRCOFHSNDETAILS")) for h in lists]
                notes.append(f"{rowname} HSN lists with content {entries}")
                if entries != [(code, frm, "Specify Details Here")]:
                    bad += 1; notes.append(f"EXPECT exactly one HSN list, with {(code, frm, 'Specify Details Here')}")
            _, _, broot = tree(before)
            wasrows = {name_of(r): r for r in rows(broot, tag)}
            if len(target) == 1 and rowname in wasrows:
                def stripped(r):
                    # what Tally adds by itself when an HSN row is saved (seen live, 14 Oct): a State Cess duty row with
                    # no rate, the language id 1033, and on a group one GST row (source As per Company/Stock Group,
                    # duty heads, no rate); the HSN list and the alteration id move by design
                    c = ET.fromstring(ET.tostring(r))
                    for p in list(c.iter()):
                        for h in list(p):
                            if h.tag in ("HSNDETAILS.LIST", "ALTERID"): p.remove(h)
                            elif h.tag == "LANGUAGEID" and (h.text or "").strip() == "1033": p.remove(h)
                            elif h.tag == "RATEDETAILS.LIST" and txt(h, "GSTRATEDUTYHEAD") == "State Cess" and h.find("GSTRATE") is None: p.remove(h)
                            elif step == "a10" and h.tag == "GSTDETAILS.LIST" and (not has_text(h) or (txt(h, "SRCOFGSTDETAILS") == "As per Company/Stock Group" and not list(h.iter("GSTRATE")))): p.remove(h)
                    return canon(c)
                same_other = stripped(target[0]) == stripped(wasrows[rowname])
                notes.append(f"{rowname} other content (all but the HSN list, ALTERID and what Tally adds by itself: a State Cess row without a rate, LANGUAGEID 1033" + (", one As per Company/Stock Group GST row without a rate" if step == "a10" else "") + f"): {'unchanged' if same_other else 'CHANGED'}")
                if not same_other: bad += 1; notes.append("EXPECT nothing but the HSN entry (and Tally's own additions) on the named row")
                if step == "a10":
                    gl = [(txt(g, "APPLICABLEFROM"), txt(g, "SRCOFGSTDETAILS")) for g in target[0].iter("GSTDETAILS.LIST") if has_text(g)]
                    notes.append(f"{rowname} GST rows {gl}")
            was = {name_of(r): canon(r) for r in rows(broot, tag)}
            now = {name_of(r): canon(r) for r in rs}
            moved = sorted(n for n in set(was) | set(now) if n != rowname and was.get(n) != now.get(n))
            if moved: bad += 1; notes.append(f"EXPECT every other row unchanged, these differ: {moved}")
        prefix_check(rs, guid, "stock item" if step == "a09" else "stock group")
        if step == "a10":
            marks = {txt(r, "BRIDGECOMPANYGUID") for r in rs}
            if marks != {guid}: raise Stop("; ".join(notes) + f" - BRIDGECOMPANYGUID {sorted(marks)} is not this book's GUID")
    elif step == "feat":
        rs = rows(root, "COMPANY")
        if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows, expected exactly 1")
        if not opt("--expect"): raise Err("feat needs --expect (a check that compares nothing is not a check)")
        counts = {}
        for e in rs[0]: counts[e.tag] = counts.get(e.tag, 0) + 1
        flags = {e.tag: (e.text or "").strip() for e in rs[0] if e.tag.startswith("IS")}
        notes.append("flags " + ",".join(f"{k}={v}" for k, v in sorted(flags.items()) if k in ("ISTDSON", "ISTCSON", "ISGSTCLASSIFON", "ISGSTON", "ISINVENTORYON", "ISCOSTCENTRESON")))
        if not txt(rs[0], "GUID").startswith(guid): raise Stop("the company GUID is not this book's: wrong company")
        bf = opt("--before")
        if bf:
            _, _, froot = tree(bf)
            frs = rows(froot, "COMPANY")
            if len(frs) != 1: raise Err("--before file does not hold exactly one COMPANY row")
            def leaves(c): return {e.tag: (e.text or "").strip() for e in c if not len(e) and e.tag not in ("ALTMSTID", "ALTVCHID")}
            a, b = leaves(frs[0]), leaves(rs[0])
            moved = sorted(k for k in set(a) | set(b) if a.get(k) != b.get(k))
            notes.append(f"ALTMSTID {txt(frs[0], 'ALTMSTID')} -> {txt(rs[0], 'ALTMSTID')}; ALTVCHID {txt(frs[0], 'ALTVCHID')} -> {txt(rs[0], 'ALTVCHID')}")
            fixed = {pair.split("=", 1)[0] for pair in (opt("--expect") or "").split(",") if "=" in pair}
            # a flag named in --expect may move; every other field must not
            extra = [k for k in moved if k not in fixed]
            if extra: bad += 1; notes.append(f"EXPECT only the flags this step changes to differ from {os.path.basename(bf)}; these also differ: {extra}")
        for pair in (opt("--expect") or "").split(","):
            if pair:
                k, v = pair.split("=", 1)
                if counts.get(k) != 1: bad += 1; notes.append(f"EXPECT exactly one {k} element, found {counts.get(k, 0)}")
                elif flags.get(k) != v: bad += 1; notes.append(f"EXPECT {k}={v}, found {flags.get(k)}")
    elif step == "obj":
        if root.find(".//COLLECTION") is not None: raise Stop("a company Object answer holds no COLLECTION")
        comps = root.findall("BODY/DATA/TALLYMESSAGE/COMPANY")
        if len(comps) != 1: raise Stop(f"{len(comps)} BODY/DATA/TALLYMESSAGE/COMPANY elements, expected exactly 1: not a company Object export")
        company = comps[0]
        want_name = company_name(opt("--company"))
        if not want_name or company.get("NAME") != want_name: raise Stop(f"the object is for {company.get('NAME')!r}, not {want_name!r}: wrong company")
        guids = [(e.text or "").strip() for e in company if e.tag == "GUID"]
        if guids != [guid]: raise Stop("the object's own GUID is not this book's GUID: wrong company")
        tags = {}
        for e in company.iter():
            if any(w in e.tag for w in ("TDS", "TCS", "CLASSIF")) and not len(e):
                tags[e.tag] = (e.text or "").strip()
        hl = [(e.text or "").strip() for e in company.iter("HSNCODELENGTH")]
        notes.append(f"{sum(1 for _ in company.iter())} elements; HSNCODELENGTH {hl}; {len(tags)} TDS/TCS/classification elements: " + ",".join(f"{k}={v}" for k, v in sorted(tags.items())))
    elif step in ("a11", "b04"):
        rs = rows(root, "TAXUNIT")
        marker = collection(root).get("MSTDEPTYPE")
        notes.append(f"{len(rs)} tax units; type marker {marker}")
        prefix_check(rs, guid, "tax unit")
        if not rs:
            bad += 1; notes.append("EXPECT at least the default tax unit (every earlier answer had one); an empty answer is not read as none")
    elif step in ("a13", "b03"):
        cs = rows(root, "CURRENCY")
        names = [name_of(c) for c in cs]
        rate_tags = sorted({e.tag for c in cs for e in c.iter() if "RATE" in e.tag})
        notes.append(f"{len(cs)} currencies {names}; rate elements {rate_tags}")
        if guid:
            gs = [txt(c, "GUID") for c in cs if txt(c, "GUID")]
            if any(not g.startswith(guid) for g in gs): raise Stop("; ".join(notes) + " - a currency GUID does not start with this book's GUID: wrong company")
            notes.append(f"{len(gs)} of {len(cs)} rows carry a GUID")
        if len(cs) < 2: bad += 1; notes.append("EXPECT the base and one foreign currency master")
        if step == "b03" and "$" not in names: bad += 1; notes.append("EXPECT the $ master on this book")
    return (1 if bad else 0), f"{step}: " + "; ".join(notes)

def main(argv):
    try:
        code, msg = run(argv)
    except Stop as e:
        code, msg = 3, f"{argv[1] if len(argv) > 1 else '?'}: {e}"
    except BaseException as e:                       # any other failure is "could not check", never a pass or a finding
        code, msg = 2, f"{argv[1] if len(argv) > 1 else '?'}: could not run the check ({type(e).__name__}: {e})"
    # ASCII only: the lab console is not UTF-8, and a name such as the rupee sign must not stop the check at its print
    line = f"{['PASS', 'FINDING', 'CHECK-ERROR', 'STOP'][code]} {msg}".encode("ascii", "backslashreplace").decode("ascii")
    print(line)
    return code

if __name__ == "__main__":
    sys.exit(main(sys.argv))
