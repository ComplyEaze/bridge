#!/usr/bin/env python3
# sv.py: row 2480-SV's reader for where a voucher landed. New for this row; it is the "new sc.py line" of the
# proposal, kept in a file of its own so that sc.py stays SC's, byte for byte. It parses no answer itself: it loads
# sk.py (SK's, byte for byte, as sc.py does) and, through it, wx.py, cv.py and nm.py, and calls their own parse, so a
# shape they STOP on is a STOP here too (exit 1). It prints never more than sk.py's CAP lines a call (AGENTS.md P5),
# and never prints a VOUCHERKEY's or a GUID's value.
#
# Each id's TARGET is the ledger its voucher names: its own, but for kb, whose name the altered ledger's name list
# holds, so its TARGET is that ledger, whose NAME attribute is ka's (row 2480-SC, Part B). A ledger is told by its
# NAME attribute exactly, as every voucher row's LEDGERNAME carries it (SC's k8). The new numbers are gen-sv.py's.
# Modes:
#   sv.py --landed ID BEFORE AFTER
#                          BEFORE and AFTER are the book-wide reads (Vb-book answers) sent before and after ID's
#                          voucher J-<ID>.xml. ONE line:
#                            LANDED <id>: ON <id2>                   the one new voucher, numbered as ID's, carries an
#                                                                    entry on one ledger of names.tsv, <id2>'s
#                          or, in place of ON <id2>: ON NONE OF THE ELEVEN, ON <n> OF THE ELEVEN, NO NEW VOUCHER,
#                          <n> NEW VOUCHERS, NOT ITS NUMBER, or BOOK CHANGED (a voucher of BEFORE is missing from
#                          AFTER, or differs in its number or its entry lines).
#                          Exit 0 only for ON <ID's TARGET>; exit 3 for every other line, each a result, not a STOP.
#                          The verdict is printed after it, on a line of its own:
#                            VERDICT <id>: ON ITS LEDGER          (exit 0)
#                            VERDICT <id>: ON ITS SIBLING         (ON <id2>, id2 the other of ID's pair q or u)
#                            VERDICT <id>: NOT ON ITS LEDGER      (any other ON line)
#                            VERDICT <id>: NOT JUDGED             (every other line)
#   sv.py --judge2 ID FORM ANSWER BOOK
#                          ONE line for a read after the vouchers, by ID's name; FORM as sc.py's (outstandings, census,
#                          movement, window); BOOK the book-wide read after the last voucher. Its vouchers on ID's
#                          TARGET are those of BOOK with an entry on that ledger's name exactly (by VOUCHERKEY for
#                          movement and window, by GUID for the census, by their bill numbers for outstandings):
#                            JUDGE2 <id> <form>: OWN <k> OF <n>, OTHERS <m>
#                          Exit 0 only for k = n, n above 0 and m = 0; exit 3 for any other line, and for
#                            JUDGE2 <id> <form>: EMPTY ANSWER        (as wx.py decides it; checked first)
#                            JUDGE2 <id> <form>: <the answer's kind> (a STATUS, or what nm.py makes of it)
#                          A voucher (or bill) held twice is a STOP.
# Example: python3 -I sv.py --landed qb a-v2-Vb-book.xml a-v3-Vb-book.xml
import pathlib, sys
here = pathlib.Path(__file__).resolve().parent
NEW_NUMBER = {"qa": "0303", "qb": "0304", "ua": "0305", "ub": "0306", "ka": "0307", "kb": "0308"}
TARGET = {"qa": "qa", "qb": "qb", "ua": "ua", "ub": "ub", "ka": "ka", "kb": "ka"}
SIBLING = {"qa": "qb", "qb": "qa", "ua": "ub", "ub": "ua"}

def load(name):
    """The functions of a script of this folder: its text up to the line where it reads its arguments, run once."""
    source = (here / name).read_text("utf-8")
    marker = "\nargs = sys.argv[1:]\n"
    if source.count(marker) != 1:
        print(f"STOP: {name}: not the script this one expects (its argument line is not there once)")
        raise SystemExit(1)
    space = {"__file__": str(here / name), "__name__": f"sv_{name[:-3]}"}
    exec(compile(source.split(marker, 1)[0], name, "exec"), space)
    return space

sk = load("sk.py")
say, stop = sk["say"], sk["stop"]

def vouchers(wx, path, what):
    kind, rows = wx["read"](path)
    if kind != "vouchers":
        stop(f"{path}: the {what} is not a read with vouchers ({kind})")
    return rows

def landed(ident, before, after):
    if ident not in NEW_NUMBER:
        stop("usage: --landed ID BEFORE AFTER, ID one of " + ", ".join(NEW_NUMBER))
    table = sk["names_tsv"]()
    id_of = {name: i for i, (_, name) in table.items()}
    wx = sk["reader"]("wx.py")
    old, new = vouchers(wx, before, "BEFORE read"), vouchers(wx, after, "AFTER read")
    head = f"LANDED {ident}"
    by_key = {r["key"]: r for r in new}
    same = lambda a, b: a["number"] == b["number"] and [e["line"] for e in a["entries"]] == [e["line"] for e in b["entries"]]
    if any(r["key"] not in by_key or not same(r, by_key[r["key"]]) for r in old):
        return f"{head}: BOOK CHANGED", None
    added = [r for r in new if r["key"] not in {o["key"] for o in old}]
    if not added:
        return f"{head}: NO NEW VOUCHER", None
    if len(added) > 1:
        return f"{head}: {len(added)} NEW VOUCHERS", None
    if added[0]["number"] != f"BP/26-27/{NEW_NUMBER[ident]}":
        return f"{head}: NOT ITS NUMBER", None
    on = sorted({id_of[e["line"][0]] for e in added[0]["entries"] if e["line"][0] in id_of})
    if not on:
        return f"{head}: ON NONE OF THE ELEVEN", None
    if len(on) > 1:
        return f"{head}: ON {len(on)} OF THE ELEVEN", None
    return f"{head}: ON {on[0]}", on[0]

def verdict(ident, where):
    if where is None:
        return "NOT JUDGED", 3
    if where == TARGET[ident]:
        return "ON ITS LEDGER", 0
    if SIBLING.get(TARGET[ident]) == where:
        return "ON ITS SIBLING", 3
    return "NOT ON ITS LEDGER", 3

def judge2(ident, form, answer, book):
    if ident not in TARGET or form not in ("outstandings", "census", "movement", "window"):
        stop("usage: --judge2 ID FORM ANSWER BOOK, ID one of " + ", ".join(TARGET) +
             ", FORM outstandings, census, movement or window")
    head = f"JUDGE2 {ident} {form}"
    table = sk["names_tsv"]()
    wx = sk["reader"]("wx.py")
    if wx["bytes_or_empty"](answer) is None:
        return f"{head}: EMPTY ANSWER", 3
    target = table[TARGET[ident]][1]
    mine = [r for r in vouchers(wx, book, "book-wide read") if any(e["line"][0] == target for e in r["entries"])]
    if form == "outstandings":
        kind, rows = sk["reader"]("nm.py")["answer"](answer, show=False)
        if rows is None:
            return f"{head}: {kind}", 3
        held = [r["BILLREF"] for r in rows]
        want = {b[0] for r in mine for e in r["entries"] if e["line"][0] == target for b in e["bills"] if b[0]}
    elif form == "census":
        kind, rows = sk["reader"]("cv.py")["read"](answer)
        if kind != "vouchers":
            return f"{head}: {'NO BYTES' if kind == 'none' else 'STATUS ' + rows[0]}", 3
        held, want = [g for g, _, _ in rows], {r["own"]["GUID"] for r in mine}
    else:
        kind, rows = wx["read"](answer)
        if kind != "vouchers":
            return f"{head}: STATUS {rows[0]}", 3
        held, want = [r["key"] for r in rows], {r["key"] for r in mine}
    if len(set(held)) != len(held):
        stop(f"{answer}: a voucher or bill is held more than once")
    own = sum(1 for h in held if h in want)
    others = len(held) - own
    return f"{head}: OWN {own} OF {len(want)}, OTHERS {others}", 0 if own == len(want) > 0 and others == 0 else 3

args = sys.argv[1:]
if len(args) == 4 and args[0] == "--landed":
    line, where = landed(*args[1:])
    say(line)
    text, code = verdict(args[1], where)
    say(f"VERDICT {args[1]}: {text}")
    raise SystemExit(code)
elif len(args) == 5 and args[0] == "--judge2":
    text, code = judge2(*args[1:])
    say(text)
    raise SystemExit(code)
else:
    stop("usage: sv.py --landed ID BEFORE AFTER | --judge2 ID FORM ANSWER BOOK")
