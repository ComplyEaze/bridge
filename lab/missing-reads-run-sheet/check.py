#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [--census FILE] [--company FILE] : checks one answer of the missing-reads sheet.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold; keep the file, write the
printed line down, go on. Exit 3 = STOP: a bound, a completeness rule or an identity rule broke; stop and send
nothing more. Exit 2 = the check could not run (any error at all, a missing file, a missing argument): never a pass.
One line is printed. `check.py same A B`: 0 when the two files are byte-identical, 1 when not, 2 on an error.
--census FILE: the census answer of the same day (its row count is the voucher count the read may hold).
--company FILE: this book's book-extent answer (step a02); its GUID must prefix every voucher, stock item and stock group."""
import re, sys
from xml.etree import ElementTree as ET

BOOK = {"a01": "BRIDGE SHAPE LAB", "a02": "BRIDGE SHAPE LAB", "a12": "BRIDGE SHAPE LAB",
        "b01": "BRIDGE CORPUS FOREX", "b02": "BRIDGE CORPUS FOREX", "b05": "BRIDGE CORPUS FOREX"}
CAPS = {"a01": 8000, "b01": 8000, "a02": 6000, "a12": 6000, "b02": 6000, "b05": 6000, "a04": 24000, "a06": 24000,
        "a09": 60000, "a10": 20000, "a11": 60000, "a13": 40000, "b03": 40000, "b04": 60000}
PER_VOUCHER = {"a05": (80000, 240000), "a07": (80000, 240000), "a08": (160000, 320000)}
NEEDS_CENSUS = {"a05", "a07", "a08"}
NEEDS_COMPANY = {"a05", "a07", "a08", "a09", "a10"}

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
    collection(root)
    return data, root

def census_rows(path):
    _, root = common("census", path)
    return rows(root, "VOUCHER")

def company_guid(path):
    _, root = common("a02", path)
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop(f"{len(rs)} COMPANY rows in the book-extent answer, expected 1")
    g = txt(rs[0], "GUID")
    if not g: raise Err("no GUID in the book-extent answer")
    return g

def run(argv):
    if argv[1] == "same":
        a, b = open(argv[2], "rb").read(), open(argv[3], "rb").read()
        return 0 if a == b else 1, f"same: {len(a)} B vs {len(b)} B, {'identical' if a == b else 'DIFFERENT'}"
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
    if cap is None: raise Err(f"unknown step {step}")
    if size > cap: raise Stop(f"{size} B is above the cap {cap} B")
    notes.append(f"{size} B (cap {cap})")
    guid = None
    if step in NEEDS_COMPANY:
        cf = opt("--company")
        if not cf: raise Err("needs --company (this book's book-extent answer)")
        guid = company_guid(cf)

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
        vals = {k: txt(rs[0], k) for k in ("NAME", "GUID", "ALTVCHID", "ALTMSTID", "LASTVOUCHERDATE", "BOOKSFROM")}
        notes.append(str({k: (v[:13] if k == "GUID" else v) for k, v in vals.items()}))
        if vals["NAME"] != BOOK[step]: raise Stop("; ".join(notes) + f" - the answer is not for {BOOK[step]}")
        if not (vals["GUID"] and vals["ALTVCHID"] and vals["ALTMSTID"]): bad += 1; notes.append("a mark or the GUID is missing")
    elif step in ("a04", "a06"):
        vr = rows(root, "VOUCHER")
        want = 1 if step == "a04" else 2
        notes.append(f"{len(vr)} vouchers (EXPECT {want})")
        if len(vr) > 8: raise Stop("; ".join(notes) + " - more than 8 vouchers on the day: do not send the data read")
        if len(vr) < 1: raise Stop("; ".join(notes) + " - no voucher on the day: do not send the data read, tell us")
        if len(vr) != want: bad += 1
    elif step in ("a05", "a07", "a08"):
        vs = rows(root, "VOUCHER")
        notes.append(f"{len(vs)} vouchers (census {n})")
        if len(vs) != n: bad += 1; notes.append("count differs from the census")
        if any(not txt(v, "GUID").startswith(guid) for v in vs):
            raise Stop("; ".join(notes) + " - a voucher GUID does not start with this book's GUID: wrong company")
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
        if any(not (txt(r, "GUID") or "").startswith(guid) for r in rs if txt(r, "GUID")):
            raise Stop("; ".join(notes) + " - a GUID does not start with this book's GUID: wrong company")
        if step == "a10":
            marks = {txt(r, "BRIDGECOMPANYGUID") for r in rs}
            if marks != {guid}: raise Stop("; ".join(notes) + f" - BRIDGECOMPANYGUID {sorted(marks)} is not this book's GUID")
    elif step in ("a11", "b04"):
        rs = rows(root, "TAXUNIT")
        marker = collection(root).get("MSTDEPTYPE")
        notes.append(f"{len(rs)} tax units; type marker {marker}")
        if not rs and not marker:
            bad += 1; notes.append("an empty answer without the type marker is not read as none: tell us")
    elif step in ("a13", "b03"):
        cs = rows(root, "CURRENCY")
        names = [name_of(c) for c in cs]
        rate_tags = sorted({e.tag for c in cs for e in c.iter() if "RATE" in e.tag})
        notes.append(f"{len(cs)} currencies {names}; rate elements {rate_tags}")
        if len(cs) < 2 or "$" not in names: bad += 1; notes.append("EXPECT the base and the $ currency master")
    return (1 if bad else 0), f"{step}: " + "; ".join(notes)

def main(argv):
    try:
        code, msg = run(argv)
    except Stop as e:
        code, msg = 3, f"{argv[1] if len(argv) > 1 else '?'}: {e}"
    except BaseException as e:                       # any other failure is "could not check", never a pass or a finding
        code, msg = 2, f"{argv[1] if len(argv) > 1 else '?'}: could not run the check ({type(e).__name__}: {e})"
    print(f"{['PASS', 'FINDING', 'CHECK-ERROR', 'STOP'][code]} {msg}")
    return code

if __name__ == "__main__":
    sys.exit(main(sys.argv))
