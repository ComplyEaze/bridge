#!/usr/bin/env python3 -I
"""check.py STEP ANSWER [options] : checks one answer of the ledger-twin sheet. Nothing here sends anything.

Exit 0 = PASS: every EXPECT of the step held. Exit 1 = FINDING: an EXPECT did not hold; keep the file, write the
printed line down, go on only where the step says so. Exit 3 = STOP: a completeness, identity or bound rule broke;
stop and send nothing more. Exit 2 = the check could not run (any error at all, a missing file, a bad option): never a
pass. The last line printed starts with PASS, FINDING, STOP or CHECK-ERROR. Output is ASCII only; a line break inside
a name prints as <CR><LF> and a name is never trimmed. In an option a name is written with <CRLF> where it carries
the break (the shell never sees a real line break).

Steps:
  co ANSWER                         company list: BRIDGE LINE BREAK LAB is loaded, it is the only company, Education mode No
  marks ANSWER [--prev FILE]        one COMPANY row, BRIDGE LINE BREAK LAB; prints its GUID, ALTVCHID, ALTMSTID and,
                                    with --prev, the movement of both marks
  imp ANSWER --expect created|altered|errors|record [--lineerror TEXT]
                                    an import answer: a complete RESPONSE; with `created` exactly CREATED 1 and every
                                    other counter 0; with `altered` ALTERED 1 and every other counter 0; with `errors`
                                    ERRORS 1 and every other counter 0 (the LINEERROR text is printed; with
                                    --lineerror it must equal TEXT). A clean RESPONSE with other counters is a FINDING;
                                    anything that is not a complete RESPONSE is a STOP. With `record` the
                                    outcome is what is being measured: any complete RESPONSE passes and is
                                    classified (CREATED, ALTERED, ERRORS, or OTHER with the counters printed)
  ledgers ANSWER --marks FILE --before FILE [--add "NAME|PARENT" ...] [--moved "NAME|PARENT" ...] [--absent "NAME" ...]
                                    the ledger list: every row carries the book's GUID (from the marks answer); against
                                    the --before list exactly the --add rows are new (name byte-exact, parent as given)
                                    and the --moved rows are the same ledger (same GUID) whose group is now PARENT;
                                    every other row unchanged in name, GUID and group; every --absent name is in
                                    neither list (the book does not hold it yet). Prints the table of the new and
                                    moved rows. A new ledger the step did not name, or a missing one, is a FINDING
  names ANSWER --marks FILE --before FILE
                                    prints the ledgers whose NAME or NAME-element text differs from the --before list,
                                    for the steps whose outcome is what is being measured (always PASS if complete)
  vch ANSWER --ledgers FILE --before FILE --amount AMOUNT [--allowed "NAME" ...] [--exact "NAME"]
                                    the voucher read: exactly one voucher is new against --before and has a debit
                                    entry of AMOUNT; the entry's LEDGERNAME is matched, byte-exact, to the NAME of the
                                    LEDGER rows of the --ledgers answer. Prints the voucher and the ledger it landed on
                                    (name, GUID, group). With --exact the voucher must land on the ledger of exactly
                                    that name, else FINDING. With --allowed it must land on one of the listed names,
                                    else FINDING. With neither it only reports (PASS when it matched exactly one row)
  bal ANSWER --ledgers FILE         the balance read: prints, per ledger GUID of the --ledgers answer, OPENINGBALANCE and
                                    CLOSINGBALANCE (a read whose request had never been sent before the sitting)
  same A B                          two answers of one request: byte-identical, or equal row by row under COLLECTION
"""
import os, re, sys
from xml.etree import ElementTree as ET

BOOK = "BRIDGE LINE BREAK LAB"
CAPS = {"co": 8000, "marks": 6000, "ledgers": 60000, "vch": 150000, "bal": 60000, "imp": 4000}

class Stop(Exception): pass
class Err(Exception): pass

def show(s):
    if s is None: return "(none)"
    out = []
    for ch in s:
        if ch == "\r": out.append("<CR>")
        elif ch == "\n": out.append("<LF>")
        elif ch == "\t": out.append("<TAB>")
        elif 32 <= ord(ch) < 127: out.append(ch)
        else: out.append("<U+%04X>" % ord(ch))
    return "'%s' (%d chars)" % ("".join(out), len(s))

def arg_name(s):
    return s.replace("<CRLF>", "\r\n")

def text_of(data):
    if not data: raise Stop("the answer is empty (0 bytes)")
    if data[:2] == b"\xff\xfe": return data[2:].decode("utf-16-le")
    if data[:2] == b"\xfe\xff": return data[2:].decode("utf-16-be")
    if len(data) > 1 and data[1:2] == b"\x00": return data.decode("utf-16-le")
    return data.decode("utf-8")

def tree(path, cap=None):
    data = open(path, "rb").read()
    if cap is not None and len(data) > cap: raise Stop("the answer is %d bytes, above its cap of %d" % (len(data), cap))
    t = text_of(data)
    # numeric references XML 1.0 refuses (tab, line feed and carriage return are kept)
    t = re.sub(r"&#(?:[0-8]|1[12]|1[4-9]|2[0-9]|3[01]);", "", t)
    t = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f]", "", t)
    try:
        root = ET.fromstring(t)
    except ET.ParseError as e:
        raise Stop("the answer is not a complete XML document (%s); a truncated answer is never read" % e)
    return data, t, root

def collection(root):
    col = root.find(".//COLLECTION")
    if col is None: raise Stop("no COLLECTION in the answer")
    return col

def envelope(path, cap):
    data, t, root = tree(path, cap)
    if root.tag != "ENVELOPE": raise Stop("root is %s, not ENVELOPE" % root.tag)
    if "<LINEERROR" in t: raise Stop("the answer carries a LINEERROR")
    st = root.findtext(".//HEADER/STATUS")
    if st is not None and st.strip() == "0": raise Stop("the answer header STATUS is 0")
    return data, root

def rows(root, tag): return [e for e in collection(root) if e.tag == tag]
def txt(e, tag): return (e.findtext(tag) or "").strip()

def company_of(path):
    _, root = envelope(path, CAPS["marks"])
    rs = rows(root, "COMPANY")
    if len(rs) != 1: raise Stop("%d COMPANY rows in the marks answer, expected 1" % len(rs))
    if rs[0].get("NAME") != BOOK: raise Stop("the marks answer is for %s, not %s" % (show(rs[0].get("NAME")), BOOK))
    g = txt(rs[0], "GUID")
    if not g: raise Stop("no GUID in the marks answer")
    return rs[0], g

def ledger_map(path, guid, what):
    _, root = envelope(path, CAPS["ledgers"])
    rs = rows(root, "LEDGER")
    if not rs: raise Stop("%s: no LEDGER row: nothing was captured" % what)
    out = {}
    for r in rs:
        n = r.get("NAME")
        if n is None: raise Stop("%s: a LEDGER row without a NAME attribute" % what)
        if n in out: raise Stop("%s: the NAME %s appears twice" % (what, show(n)))
        if txt(r, "BRIDGECOMPANYGUID") != guid: raise Stop("%s: row %s carries another book's GUID" % (what, show(n)))
        out[n] = r
    return out

def info(r):
    return {"guid": txt(r, "GUID"), "parent": (r.findtext("PARENT") or ""), "alterid": txt(r, "ALTERID"),
            "names": [(e.text or "") for e in r.iter("NAME")]}

def parse_pair(s):
    if "|" not in s: raise Err("expected NAME|PARENT, got %r" % s)
    n, p = s.rsplit("|", 1)
    return arg_name(n), p

# ---------------------------------------------------------------- steps
def step_co(a):
    _, root = envelope(a["answer"], CAPS["co"])
    rs = rows(root, "COMPANY")
    names = [r.get("NAME") for r in rs]
    for r in rs: print("COMPANY %s | GUID %s" % (show(r.get("NAME")), txt(r, "GUID")))
    if names != [BOOK]: raise Stop("the loaded companies are %s, expected exactly [%s]" % ([show(n) for n in names], BOOK))
    edu = [txt(r, "EDUMODE") for r in rs]
    print("EDUMODE %s" % edu)
    if edu != ["No"]: raise Stop("Education mode, or no mode reading: %s" % edu)
    return "PASS: the only loaded company is %s, Education mode No" % BOOK

def step_marks(a):
    r, g = company_of(a["answer"])
    v, m = txt(r, "ALTVCHID"), txt(r, "ALTMSTID")
    print("COMPANY %s | GUID %s | ALTVCHID %s | ALTMSTID %s" % (show(r.get("NAME")), g, v, m))
    if a.get("prev"):
        pr, pg = company_of(a["prev"])
        if pg != g: raise Stop("the GUID differs from the earlier marks answer")
        print("movement: ALTVCHID %s -> %s, ALTMSTID %s -> %s" % (txt(pr, "ALTVCHID"), v, txt(pr, "ALTMSTID"), m))
    return "PASS: marks read"

def counters(root):
    c = {}
    for k in ("CREATED", "ALTERED", "DELETED", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS"):
        v = root.findtext(k)
        if v is None: raise Stop("the RESPONSE has no %s counter" % k)
        c[k] = int(v.strip())
    return c

def step_imp(a):
    _, t, root = tree(a["answer"], CAPS["imp"])
    if root.tag != "RESPONSE": raise Stop("root is %s, not RESPONSE: not an import answer" % root.tag)
    c = counters(root)
    errs = [(e.text or "") for e in root.iter("LINEERROR")]
    print("counters: " + ", ".join("%s %d" % kv for kv in c.items()) + "; LASTVCHID %s; LASTMID %s" % (txt(root, "LASTVCHID"), txt(root, "LASTMID")))
    for e in errs: print("LINEERROR %s" % show(e))
    want = a["expect"]
    if want == "record":
        kind = [k for k in ("CREATED", "ALTERED", "ERRORS") if c[k] == 1 and all(v == 0 for kk, v in c.items() if kk != k)]
        word = kind[0] if kind else "OTHER"
        return "PASS: recorded, the answer reads as %s%s" % (word, "" if kind else " (counters not of the three named shapes: write them down)")
    key = {"created": "CREATED", "altered": "ALTERED", "errors": "ERRORS"}[want]
    ok = c[key] == 1 and all(v == 0 for k, v in c.items() if k != key)
    if want == "errors" and a.get("lineerror") is not None and errs != [a["lineerror"]]:
        return "FINDING: the LINEERROR text is not the expected %s" % show(a["lineerror"])
    if want != "errors" and errs: return "FINDING: a LINEERROR in an answer that was expected to be clean"
    if ok: return "PASS: %s 1, every other counter 0" % key
    return "FINDING: expected %s 1 and every other counter 0" % key

def step_ledgers(a):
    _, g = company_of(a["marks"])
    now = ledger_map(a["answer"], g, "this ledger list")
    before = ledger_map(a["before"], g, "the --before ledger list")
    add = [parse_pair(x) for x in a.get("add", [])]
    moved = [parse_pair(x) for x in a.get("moved", [])]
    bad = []
    for n in [arg_name(x) for x in a.get("absent", [])]:
        if n in now or n in before: bad.append("the name %s is already in the book" % show(n))
    new = [n for n in now if n not in before]
    gone = [n for n in before if n not in now]
    for n in gone: bad.append("a ledger of the before-list is gone: %s" % show(n))
    for (n, p) in add:
        if n not in now: bad.append("expected new ledger %s is not there" % show(n)); continue
        if n in before: bad.append("%s was already in the before-list" % show(n)); continue
        i = info(now[n]); print("NEW %s | GUID %s | parent %s | NAME elements %s" % (show(n), i["guid"], show(i["parent"]), [show(x) for x in i["names"]]))
        if i["parent"] != p: bad.append("%s is under %s, expected %s" % (show(n), show(i["parent"]), show(p)))
    for n in new:
        if n not in [x[0] for x in add]: bad.append("a new ledger the step did not name: %s" % show(n))
    for (n, p) in moved:
        if n not in now or n not in before: bad.append("moved ledger %s is not in both lists" % show(n)); continue
        i, j = info(now[n]), info(before[n])
        print("MOVED %s | GUID %s -> %s | parent %s -> %s | ALTERID %s -> %s" % (show(n), j["guid"], i["guid"], show(j["parent"]), show(i["parent"]), j["alterid"], i["alterid"]))
        if i["guid"] != j["guid"]: bad.append("%s changed GUID" % show(n))
        if i["parent"] != p: bad.append("%s is under %s, expected %s" % (show(n), show(i["parent"]), show(p)))
    for n in before:
        if n in now and n not in [x[0] for x in moved]:
            i, j = info(now[n]), info(before[n])
            if i["guid"] != j["guid"] or i["parent"] != j["parent"] or i["names"] != j["names"]:
                bad.append("the ledger %s changed (guid/parent/NAME elements) but the step did not name it" % show(n))
    print("ledgers: %d now, %d before" % (len(now), len(before)))
    return "PASS: the ledger list moved exactly as the step says" if not bad else "FINDING: " + "; ".join(bad)

def step_names(a):
    _, g = company_of(a["marks"])
    now = ledger_map(a["answer"], g, "this ledger list")
    before = ledger_map(a["before"], g, "the --before ledger list")
    byguid = {info(r)["guid"]: (n, r) for n, r in before.items()}
    for n, r in now.items():
        i = info(r)
        if i["guid"] in byguid:
            on, orr = byguid[i["guid"]]
            j = info(orr)
            if on != n or j["names"] != i["names"] or j["parent"] != i["parent"]:
                print("CHANGED GUID %s | NAME %s -> %s | NAME elements %s -> %s | parent %s -> %s" % (
                    i["guid"], show(on), show(n), [show(x) for x in j["names"]], [show(x) for x in i["names"]], show(j["parent"]), show(i["parent"])))
        else:
            print("NEW GUID %s | NAME %s | NAME elements %s | parent %s" % (i["guid"], show(n), [show(x) for x in i["names"]], show(i["parent"])))
    for guid, (n, r) in byguid.items():
        if guid not in {info(x)["guid"] for x in now.values()}: print("GONE GUID %s | NAME %s" % (guid, show(n)))
    return "PASS: names compared"

def vouchers(path, cap):
    _, root = envelope(path, cap)
    return rows(root, "VOUCHER")

def step_vch(a):
    # the --ledgers answer is also a ledger list: its book GUID is read from its own rows
    lg_rows = rows(envelope(a["ledgers"], CAPS["ledgers"])[1], "LEDGER")
    if not lg_rows: raise Stop("the --ledgers answer has no LEDGER row")
    guids = {txt(r, "BRIDGECOMPANYGUID") for r in lg_rows}
    if len(guids) != 1 or "" in guids: raise Stop("the --ledgers answer does not carry exactly one book GUID")
    lg = ledger_map(a["ledgers"], guids.pop(), "the --ledgers answer")
    now = vouchers(a["answer"], CAPS["vch"])
    old = {txt(v, "GUID") for v in vouchers(a["before"], CAPS["vch"])}
    new = [v for v in now if txt(v, "GUID") not in old]
    print("vouchers: %d now, %d before, %d new" % (len(now), len(old), len(new)))
    if len(new) != 1: return "FINDING: %d new vouchers, expected exactly 1" % len(new)
    v = new[0]
    amt = a["amount"].strip()
    debits = []
    for e in v.findall("ALLLEDGERENTRIES.LIST"):
        if txt(e, "ISDEEMEDPOSITIVE") == "Yes" and txt(e, "AMOUNT") == "-" + amt:
            debits.append(e.findtext("LEDGERNAME") or "")
    print("VOUCHER %s %s no %s | GUID %s | narration %s" % (txt(v, "DATE"), txt(v, "VOUCHERTYPENAME"), txt(v, "VOUCHERNUMBER"), txt(v, "GUID"), show(v.findtext("NARRATION"))))
    for e in v.findall("ALLLEDGERENTRIES.LIST"):
        print("  entry %s | %s | deemed-positive %s" % (show(e.findtext("LEDGERNAME")), txt(e, "AMOUNT"), txt(e, "ISDEEMEDPOSITIVE")))
    if len(debits) != 1: return "FINDING: %d debit entries of -%s, expected exactly 1" % (len(debits), amt)
    name = debits[0]
    hit = [n for n in lg if n == name]
    if len(hit) != 1:
        return "FINDING: the debit entry %s matches %d ledger rows byte-exact (expected 1)" % (show(name), len(hit))
    i = info(lg[name])
    print("LANDED on ledger %s | GUID %s | parent %s" % (show(name), i["guid"], show(i["parent"])))
    if a.get("exact") is not None and name != a["exact"]:
        return "FINDING: expected the ledger %s, it landed on %s" % (show(a["exact"]), show(name))
    if a.get("allowed") and name not in a["allowed"]:
        return "FINDING: it landed on %s, which is none of the allowed %s" % (show(name), [show(x) for x in a["allowed"]])
    return "PASS: landed on %s" % show(name)

def step_bal(a):
    lg_rows = rows(envelope(a["ledgers"], CAPS["ledgers"])[1], "LEDGER")
    byguid = {txt(r, "GUID"): r.get("NAME") for r in lg_rows}
    _, root = envelope(a["answer"], CAPS["bal"])
    rs = rows(root, "LEDGER")
    if not rs: raise Stop("no LEDGER row in the balance answer")
    seen = 0
    for r in rs:
        g = txt(r, "GUID")
        if g in byguid:
            seen += 1
            print("BAL GUID %s | %s | opening %s | closing %s" % (g, show(byguid[g]), show(txt(r, "OPENINGBALANCE")), show(txt(r, "CLOSINGBALANCE"))))
    if seen == 0: return "FINDING: none of the balance rows carries a GUID of the ledger list"
    return "PASS: %d balance rows matched the ledger list by GUID" % seen

def step_same(a):
    x, y = open(a["a"], "rb").read(), open(a["b"], "rb").read()
    if x == y: return "PASS: byte-identical (%d bytes)" % len(x)
    _, _, rx = tree(a["a"]); _, _, ry = tree(a["b"])
    cx, cy = collection(rx), collection(ry)
    def canon(e): return re.sub(r"\s+", " ", ET.tostring(e, encoding="unicode")).strip()
    lx, ly = [canon(e) for e in cx], [canon(e) for e in cy]
    if lx == ly:
        print("not byte-identical, equal row by row under COLLECTION (header counters may differ)")
        return "PASS: equal row by row"
    return "FINDING: %d rows against %d, %d differ" % (len(lx), len(ly), sum(1 for p, q in zip(lx, ly) if p != q))

# ---------------------------------------------------------------- command line
OPTS = {
    "co": ({}, {}),
    "marks": ({}, {"--prev": "prev"}),
    "imp": ({"--expect": "expect"}, {"--lineerror": "lineerror"}),
    "ledgers": ({"--marks": "marks", "--before": "before"}, {}),
    "names": ({"--marks": "marks", "--before": "before"}, {}),
    "vch": ({"--ledgers": "ledgers", "--before": "before", "--amount": "amount"}, {"--exact": "exact"}),
    "bal": ({"--ledgers": "ledgers"}, {}),
}
MULTI = {"ledgers": ("--add", "--moved", "--absent"), "vch": ("--allowed",)}

def parse(argv):
    if len(argv) < 2: raise Err("usage: check.py STEP ANSWER [options]")
    step = argv[0]
    if step == "same":
        if len(argv) != 3: raise Err("same takes exactly two files")
        return step, {"a": argv[1], "b": argv[2]}
    if step not in OPTS: raise Err("unknown step %r" % step)
    need, maybe = OPTS[step]
    a = {"answer": argv[1]}
    multi = MULTI.get(step, ())
    i = 2
    while i < len(argv):
        k = argv[i]
        if k in need or k in maybe:
            if i + 1 >= len(argv): raise Err("%s needs a value" % k)
            dest = need.get(k) or maybe.get(k)
            a[dest] = argv[i + 1]
        elif k in multi:
            if i + 1 >= len(argv): raise Err("%s needs a value" % k)
            a.setdefault(k.lstrip("-"), []).append(argv[i + 1])
        else:
            raise Err("unknown option %r for %s" % (k, step))
        i += 2
    for k, dest in need.items():
        if dest not in a: raise Err("%s needs %s" % (step, k))
    if step == "imp" and a["expect"] not in ("created", "altered", "errors", "record"): raise Err("--expect is created, altered, errors or record")
    if step == "vch":
        if "exact" in a: a["exact"] = arg_name(a["exact"])
        if "allowed" in a: a["allowed"] = [arg_name(x) for x in a["allowed"]]
    return step, a

def main(argv):
    try:
        step, a = parse(argv)
        fn = {"co": step_co, "marks": step_marks, "imp": step_imp, "ledgers": step_ledgers, "names": step_names,
              "vch": step_vch, "bal": step_bal, "same": step_same}[step]
        last = fn(a)
    except Stop as e:
        print("STOP: %s" % e); return 3
    except Err as e:
        print("CHECK-ERROR: %s" % e); return 2
    except Exception as e:
        print("CHECK-ERROR: %s: %s" % (type(e).__name__, e)); return 2
    print(last)
    return 0 if last.startswith("PASS") else 1

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
