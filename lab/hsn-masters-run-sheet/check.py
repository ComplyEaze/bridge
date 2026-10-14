#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [--company FILE] [options] : checks one answer of the masters-shape HSN sheet.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold: keep the file, write the
printed line down, go on unless the step says STOP. Exit 3 = STOP: a bound, a completeness rule or an identity rule broke;
stop and send nothing more. Exit 2 = the check could not run (any error at all, a missing file, a missing argument):
never a pass. One line is printed (ASCII only).
`check.py same A B`: 0 when the two files are byte-identical or equal leaf by leaf under COLLECTION (header counters
may differ and are recorded), 1 when not, 2 on an error (the same file twice, an empty file).

--marks-from FILE (h02): prints how ALTMSTID and ALTVCHID moved against FILE (an earlier h02 answer); the voucher mark
  ALTVCHID moving is a FINDING; the company GUID or number differing is a STOP.
Steps: h01 company list, h02 book extent, m03 stock items (masters shape), m04 stock groups (masters shape).
--company FILE (m03, m04: required): this book's book-extent answer (h02). Every row must carry BRIDGECOMPANYGUID equal to
  its GUID and a GUID starting with it (STOP otherwise); every row must carry MASTERID and ALTERID (FINDING otherwise).
m03 and m04 need at least one expectation, so that a check never compares nothing:
--expect-coded N : exactly N rows carry an HSN code (an HSNCODE element with text; a description alone is not a code).
--expect-gst NAME[,NAME] | none : the rows holding GST details content are exactly these (none = no row).
--before FILE : every row equals its row in FILE, and the set of row names is the same.
--show NAME : also print the HSN entries and, per GST entry, the rate rows of that row.
--created NAME (m03 only; needs --before, --parent, --hsn, --gst-from): the row NAME is NEW (absent in --before) and
  holds the stated parent; every other row equals its row in --before.
--hsn "CODE:YYYYMMDD[,CODE:YYYYMMDD]" | none | auto-or-none : the created row holds exactly these HSN entries (code and
  applicable-from, any description, source "Specify Details Here"). auto-or-none = no entry, or only entries Tally made
  by itself (no HSN code, source "As per Company/Stock Group"); an entry with a code or another source is a FINDING.
--gst-from "YYYYMMDD[,YYYYMMDD]" | none | observe | auto-or-none | "YYYYMMDD[,YYYYMMDD]+own" : the created row holds exactly
  that many GSTDETAILS.LIST entries with content, with these applicable-from dates; each must read taxability Taxable and
  source "Specify Details Here". observe = print the entries and assert nothing about them (the row-level checks still apply).
  auto-or-none = no entry, or only entries Tally made by itself (source "As per Company/Stock Group", no rate anywhere).
  DATES+own = the listed dates exactly as above, and in addition only Tally's own entries (as in auto-or-none).
--own-dates "YYYYMMDD[,YYYYMMDD]" (default 20250401): the dates Tally's own entries may carry, at most one per date and
  list, in auto-or-none and DATES+own (an own entry of another date, or two of one date, is a FINDING).
--prev FILE (with --created): the created row is compared leaf by leaf with the same row of the previous step's answer and
  the line says "unchanged" or "changed: ..."; a leaf that existed before and changed or went away is a FINDING unless
  its list or entry is named in --may-change (TAG, or TAG@YYYYMMDD for one dated entry, comma separated). Dated entries are
  matched by applicable-from date, never by position. A WHOLE NEW dated entry is an addition and allowed; a leaf added to
  or removed from an entry that already existed is a change of that entry (FINDING unless named); a leaf added outside
  HSNDETAILS.LIST, GSTDETAILS.LIST and LANGUAGENAME.LIST is a FINDING; ALTERID may always move; only the empty list
  placeholder turning into a list is not a loss.
--igst "YYYYMMDD:RATE[,YYYYMMDD:RATE]" (optional): the IGST rate of the GST entry of that date.
Caps (bytes): h01 8,000; h02 6,000; m03 30,000 (40,000 with --created: the created row adds about 8,000); m04 12,000."""
import os, re, sys
from xml.etree import ElementTree as ET

BOOK = "BRIDGE SHAPE LAB"
CAPS = {"h01": 8000, "h02": 6000, "m03": 30000, "m04": 12000}
CAP_CREATED = 40000
SRC = "Specify Details Here"
ROWS = {"m03": ("STOCKITEM", 11), "m04": ("STOCKGROUP", 3)}

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
    t = re.sub(r"&#([0-8]|1[1-2]|1[4-9]|2[0-9]|3[0-1]);", r"[#\1]", t)   # numeric references XML 1.0 refuses (Tally's reserved &#4;): kept as a visible token
    t = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f]", "", t)                      # raw control bytes some names carry
    return data, t, ET.fromstring(t)

def collection(root):
    col = root.find(".//COLLECTION")
    if col is None: raise Stop("no COLLECTION in the answer")
    return col

def rows(root, tag):
    """Rows are the children of the COLLECTION. The header carries counter elements of the same names (COMPANY,
    STOCKITEM ...) holding a number; they are not rows."""
    return [e for e in collection(root) if e.tag == tag]

def name_of(e): return e.get("NAME") or (e.findtext("NAME") or "").strip()
def txt(e, tag): return (e.findtext(tag) or "").strip()
def has_text(e): return any((c.text or "").strip() for c in e.iter() if c is not e)

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
    g = txt(rs[0], "GUID")
    if not g: raise Err("no GUID in the book-extent answer")
    return g

def hsn_entries(r):
    return [(txt(h, "HSNCODE"), txt(h, "APPLICABLEFROM"), txt(h, "SRCOFHSNDETAILS")) for h in r.iter("HSNDETAILS.LIST") if has_text(h)]

def gst_entries(r):
    return [g for g in r.iter("GSTDETAILS.LIST") if has_text(g)]

def rate_rows(g):
    """Per GST entry: (state, duty head, valuation type, rate) for every rate row; the rate reads 'absent' when the
    element is missing and 'empty' when it is there without text."""
    out = []
    for s in g.iter("STATEWISEDETAILS.LIST"):
        for rd in s.iter("RATEDETAILS.LIST"):
            e = rd.find("GSTRATE")
            rate = "absent" if e is None else ((e.text or "").strip() or "empty")
            out.append((txt(s, "STATENAME"), txt(rd, "GSTRATEDUTYHEAD"), txt(rd, "GSTRATEVALUATIONTYPE"), rate))
    return out

def describe(r):
    parts = []
    for h in hsn_entries(r): parts.append(f"HSN {h[1]} {h[0]} [{h[2]}]")
    for g in gst_entries(r):
        rr = rate_rows(g)
        parts.append(f"GST {txt(g, 'APPLICABLEFROM')} {txt(g, 'TAXABILITY')} [{txt(g, 'SRCOFGSTDETAILS')}]: " +
                     ", ".join(f"{'' if not s.strip() else s.strip() + ' '}{d} {v} {rt}" for s, d, v, rt in rr))
    return parts

AUTO = "As per Company/Stock Group"
def is_auto_hsn(h): return h[0] == "" and h[2] == AUTO
def is_auto_gst(g): return txt(g, "SRCOFGSTDETAILS") == AUTO and not any((e.text or "").strip() for e in g.iter("GSTRATE"))
LISTS_MAY_GROW = {"HSNDETAILS.LIST", "GSTDETAILS.LIST", "LANGUAGENAME.LIST"}

DATED = ("HSNDETAILS.LIST", "GSTDETAILS.LIST")
def leaves(e, prefix="", dated=True):
    """Leaf paths of a row. A dated entry of HSNDETAILS.LIST / GSTDETAILS.LIST is identified by its applicable-from date
    (/GSTDETAILS.LIST@20250401/...), never by its position: the export order of dated entries is not assumed. A list without
    a date (the empty placeholder) and everything else is identified by tag and index."""
    out, seen, dseen = {}, {}, {}
    for c in e:
        d = (c.findtext("APPLICABLEFROM") or "").strip() if (dated and c.tag in DATED) else ""
        if d:
            n = dseen.get((c.tag, d), 0); dseen[(c.tag, d)] = n + 1
            p = f"{prefix}/{c.tag}@{d}" + (f"#{n + 1}" if n else "")
        else:
            i = seen.get(c.tag, 0); seen[c.tag] = i + 1
            p = f"{prefix}/{c.tag}[{i}]"
        if len(c): out.update(leaves(c, p, False))
        else: out[p] = (c.text or "").strip()
    return out
def ident(path): return path.split("/")[1]                       # the entry or element a leaf belongs to
def top(path): return re.match(r"[^\[@#]+", ident(path)).group()
PLACEHOLDERS = ("HSNDETAILS.LIST[0]", "GSTDETAILS.LIST[0]")      # identities of the undated empty lists

def igst_of(g):
    v = [rt for s, d, vt, rt in rate_rows(g) if d == "IGST"]
    return v[0] if len(v) == 1 else None

def book_marks(path):
    _, root = common(path)
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows in {os.path.basename(path)}, expected 1")
    return {k: txt(rs[0], k) for k in ("GUID", "ALTVCHID", "ALTMSTID", "COMPANYNUMBER")}

def same_rate(a, b):
    try: return float(a) == float(b)
    except (TypeError, ValueError): return False

def run(argv):
    if len(argv) < 3: raise Err("usage: check.py STEP ANSWER [options]  or  check.py same A B")
    if argv[1] == "same":
        if len(argv) != 4: raise Err("same needs exactly two files")
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
    if step not in CAPS: raise Err(f"unknown step {step}")
    opt = lambda k: argv[argv.index(k) + 1] if k in argv else None
    for k in ("--company", "--expect-coded", "--expect-gst", "--before", "--show", "--created", "--parent", "--hsn", "--gst-from", "--igst", "--prev", "--may-change", "--marks-from", "--own-dates"):
        if k in argv and argv.index(k) + 1 >= len(argv): raise Err(f"{k} needs a value")
    data, root = common(path)
    size, notes, bad = len(data), [], 0
    created = opt("--created")
    cap = CAP_CREATED if (step == "m03" and created is not None) else CAPS[step]
    if size > cap: raise Stop(f"{size} B is above the cap {cap} B")
    notes.append(f"{size} B (cap {cap})")

    if step == "h01":
        rs = rows(root, "COMPANY")
        names = sorted({name_of(c) for c in rs})
        edu = sorted({txt(c, "EDUMODE") for c in rs})
        notes.append(f"companies {names}; EDUMODE {edu}")
        if names != [BOOK]: raise Stop("; ".join(notes) + f" - the only loaded company must be {BOOK}")
        if edu != ["No"]: raise Stop("; ".join(notes) + " - Education mode (or no mode reading): one-day windows may be refused")
    elif step == "h02":
        rs = rows(root, "COMPANY")
        if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows, expected exactly 1")
        vals = {k: txt(rs[0], k) for k in ("NAME", "GUID", "ALTVCHID", "ALTMSTID", "COMPANYNUMBER", "LASTVOUCHERDATE", "BOOKSFROM")}
        notes.append(str({k: (v[:13] if k == "GUID" else v) for k, v in vals.items()}))
        if vals["NAME"] != BOOK: raise Stop("; ".join(notes) + f" - the answer is not for {BOOK}")
        if not (vals["GUID"] and vals["ALTVCHID"] and vals["ALTMSTID"]): bad += 1; notes.append("a mark or the GUID is missing")
        mf = opt("--marks-from")
        if mf:
            old = book_marks(mf)
            if old["GUID"] != vals["GUID"] or old["COMPANYNUMBER"] != vals["COMPANYNUMBER"]: raise Stop("; ".join(notes) + f" - the company GUID or number differs from {os.path.basename(mf)}: wrong company")
            notes.append(f"vs {os.path.basename(mf)}: ALTMSTID {old['ALTMSTID']} -> {vals['ALTMSTID']}; ALTVCHID {old['ALTVCHID']} -> {vals['ALTVCHID']}")
            if old["ALTVCHID"] != vals["ALTVCHID"]: bad += 1; notes.append("EXPECT the voucher mark ALTVCHID unchanged (nothing here touches a voucher)")
    else:
        tag, want_rows = ROWS[step]
        cf = opt("--company")
        if not cf: raise Err(f"{step} needs --company (this book's book-extent answer)")
        guid = company_guid(cf)
        rs = rows(root, tag)
        col = collection(root)
        notes.append("COLLECTION attributes " + str(dict(col.attrib)))
        # identity first: a row of another book, a row without a GUID, a row without the computed book mark is a STOP
        for r in rs:
            g = txt(r, "GUID")
            if not g: raise Stop("; ".join(notes) + " - a row has no GUID: its book cannot be proved")
            if not g.startswith(guid): raise Stop("; ".join(notes) + " - a GUID does not start with this book's GUID: wrong company")
            m = txt(r, "BRIDGECOMPANYGUID")
            if not m: raise Stop("; ".join(notes) + " - a row has no BRIDGECOMPANYGUID: the book mark was not computed")
            if m != guid: raise Stop("; ".join(notes) + " - a BRIDGECOMPANYGUID is not this book's GUID: wrong company")
        if not rs: raise Stop("; ".join(notes) + " - no row in the answer: nothing found is not proof, the book binding cannot be shown")
        # the expectation the step must state (a check that compares nothing is not a check)
        ec, eg, bf, show = opt("--expect-coded"), opt("--expect-gst"), opt("--before"), opt("--show")
        if created is not None:
            if step != "m03": raise Err("--created is for m03 only")
            if not (bf and opt("--parent") and opt("--hsn") and opt("--gst-from")):
                raise Err("--created needs --before, --parent, --hsn and --gst-from")
        elif any(opt(k) is not None for k in ("--hsn", "--gst-from", "--parent", "--igst", "--prev", "--may-change", "--own-dates")):
            raise Err("--hsn, --gst-from, --parent, --igst, --prev and --may-change are for --created only")
        if opt("--may-change") is not None and not opt("--prev"): raise Err("--may-change needs --prev")
        if created is None and ec is None and eg is None and bf is None:
            raise Err(f"{step} needs an expectation (--expect-coded, --expect-gst or --before): a check that compares nothing is not a check")
        want = want_rows + (1 if created is not None else 0)
        coded = sum(1 for r in rs if any((e.text or "").strip() for e in r.iter() if e.tag == "HSNCODE"))
        gst_names = sorted(name_of(r) for r in rs if gst_entries(r))
        names = [name_of(r) for r in rs]
        empties = {}
        for r in rs:
            for e in r:
                if e.tag in ("HSNDETAILS.LIST", "GSTDETAILS.LIST") and not has_text(e):
                    k = f"{e.tag} " + ("children without text" if len(e) else ("whitespace-only" if e.text else "no text"))
                    empties[k] = empties.get(k, 0) + 1
        notes.append(f"{len(rs)} rows (EXPECT {want}); {coded} with an HSN code; rows with GST content: {gst_names}; empty lists: {dict(sorted(empties.items()))}")
        if len(rs) != want: bad += 1; notes.append(f"EXPECT {want} rows")
        if len(set(names)) != len(names): bad += 1; notes.append("EXPECT row names to be unique")
        lacking = sorted(name_of(r) for r in rs if not (txt(r, "MASTERID") and txt(r, "ALTERID")))
        if lacking: bad += 1; notes.append(f"EXPECT MASTERID and ALTERID on every row, missing on {lacking}")
        if ec is not None and coded != int(ec): bad += 1; notes.append(f"EXPECT {ec} rows with an HSN code, found {coded}")
        if eg is not None:
            wantg = [] if eg == "none" else sorted(x for x in eg.split(",") if x)
            if not wantg and eg != "none": raise Err("--expect-gst needs names or none")
            if gst_names != wantg: bad += 1; notes.append(f"EXPECT GST content on {wantg}, found {gst_names}")
        wasrows = None
        if bf:
            _, _, broot = tree(bf)
            wasrows = {name_of(r): r for r in rows(broot, tag)}
            if not wasrows: raise Err("the --before file holds no rows: it proves nothing")
            nowrows = {name_of(r): r for r in rs}
            if created is not None and created in wasrows:
                bad += 1; notes.append(f"EXPECT {created} to be absent in {os.path.basename(bf)}, it is there")
            if created is None:
                if set(wasrows) != set(nowrows): bad += 1; notes.append(f"EXPECT the same row names as {os.path.basename(bf)}, differing: {sorted(set(wasrows) ^ set(nowrows))}")
            else:
                if set(nowrows) - {created} != set(wasrows): bad += 1; notes.append(f"EXPECT the rows of {os.path.basename(bf)} plus {created}, differing: {sorted((set(nowrows) - {created}) ^ set(wasrows))}")
            moved = sorted(n for n in set(wasrows) & set(nowrows) if n != created and canon(wasrows[n]) != canon(nowrows[n]))
            if moved: bad += 1; notes.append(f"EXPECT every other row unchanged from {os.path.basename(bf)}, these differ: {moved}")
            else: notes.append(f"every row common with {os.path.basename(bf)} unchanged")
        if show is not None:
            sh = [r for r in rs if name_of(r) == show]
            if len(sh) != 1: bad += 1; notes.append(f"EXPECT exactly one row named {show}, found {len(sh)}")
            else: notes.append(f"{show}: " + " | ".join(describe(sh[0])))
        if created is not None:
            tgt = [r for r in rs if name_of(r) == created]
            if len(tgt) != 1:
                bad += 1; notes.append(f"EXPECT exactly one row named {created}, found {len(tgt)}")
            else:
                r = tgt[0]
                par = txt(r, "PARENT")
                notes.append(f"{created}: PARENT {par!r}; GUID {txt(r, 'GUID')}; MASTERID {txt(r, 'MASTERID')}; ALTERID {txt(r, 'ALTERID')}")
                if par != opt("--parent"): bad += 1; notes.append(f"EXPECT PARENT {opt('--parent')!r}")
                others = [x for x in rs if x is not r]
                for k in ("GUID", "MASTERID"):
                    if txt(r, k) in {txt(x, k) for x in others}: bad += 1; notes.append(f"EXPECT a {k} of its own, it repeats another row's")
                rest = sorted(f"{e.tag}={(e.text or '').strip()}" for e in r if not len(e) and e.tag not in ("GUID", "PARENT", "MASTERID", "ALTERID", "BRIDGECOMPANYGUID"))
                notes.append(f"other leaf elements of {created}: {rest}")
                hs = opt("--hsn")
                he = hsn_entries(r)
                owndates = set(x for x in (opt("--own-dates") or "20250401").split(",") if x)
                def own_ok(dates, what):
                    nonlocal bad
                    if any(d not in owndates for d in dates) or len(set(dates)) != len(dates):
                        bad += 1; notes.append(f"EXPECT at most one {what} entry of Tally's own per date, dated {sorted(owndates)}; found dates {sorted(dates)}")
                notes.append(f"{created} HSN entries {he}")
                if hs == "auto-or-none":
                    if any(not is_auto_hsn(h) for h in he): bad += 1; notes.append(f"EXPECT no HSN entry, or only Tally's own (no code, source {AUTO})")
                    else: own_ok([h[1] for h in he], "HSN")
                else:
                    wanth = [] if hs == "none" else sorted((c, d, SRC) for c, d in (p.split(":", 1) for p in hs.split(",")))
                    if sorted(he) != wanth: bad += 1; notes.append(f"EXPECT exactly the HSN entries {wanth}")
                ge = gst_entries(r)
                gf = opt("--gst-from")
                notes.append(f"{created} GST entries with content: {len(ge)}; " + " | ".join(describe(r)[len(he):]))
                notes.append(f"Tally's own entries: HSN {sum(1 for h in he if is_auto_hsn(h))} {sorted(h[1] for h in he if is_auto_hsn(h))}, GST {sum(1 for g in ge if is_auto_gst(g))} {sorted(txt(g, 'APPLICABLEFROM') for g in ge if is_auto_gst(g))}")
                if gf == "auto-or-none":
                    if any(not is_auto_gst(g) for g in ge): bad += 1; notes.append(f"EXPECT no GST entry, or only Tally's own (source {AUTO}, no rate)")
                    else: own_ok([txt(g, "APPLICABLEFROM") for g in ge], "GST")
                elif gf != "observe":
                    extra_ok = gf.endswith("+own")
                    spec = gf[:-4] if extra_ok else gf
                    wantd = [] if spec == "none" else sorted(spec.split(","))
                    listed = [g for g in ge if txt(g, "APPLICABLEFROM") in wantd] if extra_ok else ge
                    extras = [g for g in ge if not any(g is x for x in listed)]
                    gotd = sorted(txt(g, "APPLICABLEFROM") for g in listed)
                    if gotd != wantd: bad += 1; notes.append(f"EXPECT {len(wantd)} GST entries applicable from {wantd}, found {gotd}")
                    for g in listed:
                        if txt(g, "TAXABILITY") != "Taxable" or txt(g, "SRCOFGSTDETAILS") != SRC:
                            bad += 1; notes.append(f"EXPECT taxability Taxable and source {SRC} on the entry of {txt(g, 'APPLICABLEFROM')}")
                    if extras:
                        if any(not is_auto_gst(g) for g in extras): bad += 1; notes.append(f"EXPECT any further GST entry to be Tally's own (source {AUTO}, no rate)")
                        else: own_ok([txt(g, "APPLICABLEFROM") for g in extras], "further GST")
                pv = opt("--prev")
                if pv:
                    _, _, proot = tree(pv)
                    pr = [x for x in rows(proot, tag) if name_of(x) == created]
                    if len(pr) != 1: bad += 1; notes.append(f"EXPECT exactly one row named {created} in {os.path.basename(pv)}, found {len(pr)}")
                    else:
                        a, b = leaves(pr[0]), leaves(r)
                        mayc = {x for x in (opt("--may-change") or "").split(",") if x}
                        allowed = lambda k: top(k) == "ALTERID" or top(k) in mayc or ident(k) in mayc
                        modified = sorted(k for k in a if k in b and a[k] != b[k])
                        # only an undated empty list (a placeholder, with or without empty children) that became a list is not a loss
                        empty_ph = {i for i in {ident(k) for k in a} if i in PLACEHOLDERS and all(v == "" for kk, v in a.items() if ident(kk) == i)}
                        removed = sorted(k for k in a if k not in b and ident(k) not in empty_ph)
                        added = sorted(k for k in b if k not in a)
                        was = {ident(k) for k in a}
                        badleaf = [k for k in modified + removed if not allowed(k)]
                        for k in added:
                            if ident(k) in was:                    # a leaf added to an entry or element that already existed: a change of it
                                if not allowed(k) and top(k) != "LANGUAGENAME.LIST": badleaf.append(k)
                            elif top(k) not in LISTS_MAY_GROW and top(k) != "ALTERID": badleaf.append(k)    # a whole new entry or list is an addition
                        shown = lambda ks: ", ".join(ks[:6]) + (f" (+{len(ks) - 6} more)" if len(ks) > 6 else "")
                        if not (modified or removed or added): notes.append(f"{created} vs {os.path.basename(pv)}: unchanged")
                        else: notes.append(f"{created} vs {os.path.basename(pv)}: changed: modified [{shown(modified)}]; removed [{shown(removed)}]; added [{shown(added)}]")
                        if badleaf: bad += 1; notes.append(f"EXPECT only these to change against {os.path.basename(pv)}: ALTERID, whole new dated entries, and changes inside {sorted(mayc)}; others: [{shown(sorted(badleaf))}]")
                ig = opt("--igst")
                if ig:
                    for p in ig.split(","):
                        d, v = p.split(":", 1)
                        hit = [g for g in ge if txt(g, "APPLICABLEFROM") == d]
                        if len(hit) != 1 or not same_rate(igst_of(hit[0]), v):
                            bad += 1; notes.append(f"EXPECT IGST {v} in the entry of {d}, found {[igst_of(g) for g in hit]}")
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
