#!/usr/bin/env python3
# sc.py: row 2480-SC's reader for the altered ledger of pair k. New for this row. It parses no answer itself: it loads
# sk.py (SK's, byte for byte) and, through it, the readers of this folder (wx.py, cv.py and nm.py, each byte-equal to
# its row's), and calls their own parse and sk.py's catalogue parse, so a shape they STOP on is a STOP here too
# (exit 1). It prints never more than sk.py's CAP lines a call (AGENTS.md P5), and never prints a VOUCHERKEY's or a
# GUID's value.
# Modes:
#   sc.py --altered-state G
#                          in the catalogue answer G (a G-cat answer, sk.py's parse), the state row 2480-SK recorded
#                          after its alteration of pair k: exactly one LEDGER row whose NAME attribute is ka's name, its
#                          LANGUAGENAME.LIST names kb's name alone, its GUID the book's; no other row whose NAME
#                          attribute or name list holds either name. Then ALTERED STATE k: AS SK RECORDED (exit 0), or
#                          ALTERED STATE k: NOT AS SK RECORDED (what differs) (exit 3: the sheet says what it means).
#   sc.py --judge-b SPELLING FORM ANSWER BOOK
#                          ONE line for the altered ledger read by one spelling: SPELLING ka (its NAME attribute) or kb
#                          (the name its name list now holds); FORM outstandings (an L-lo answer, nm.py's parse),
#                          census (C-v or C-u, cv.py's), movement (Vm-v or Vm-u, wx.py's) or window (Vw-v or Vw-u,
#                          wx.py's). BOOK is the book-wide read of the sitting (a Vb-book answer). The invoice is ka's,
#                          found in BOOK as sk.py --judge finds it: the one voucher with an entry on ka's name exactly,
#                          numbered BP/26-27/0292 (row SK's book-wide read showed it so after the alteration). The line is
#                            JUDGE-B <spelling> <form>: OWN YES|NO, OTHERS <n>
#                          OWN: the answer holds that invoice (by VOUCHERKEY for movement and window, by GUID for the
#                          census, by its bill number for outstandings); OTHERS: how many vouchers (or bill rows) it
#                          holds besides. The invoice held more than once is a STOP, as wx.py and cv.py stop on a
#                          repeated voucher. Exit 0 for OWN YES, OTHERS 0; exit 3 for any other line, a result and
#                          not a STOP: also
#                            JUDGE-B <spelling> <form>: EMPTY ANSWER       (no bytes, HTTP 200, CONTENT-LENGTH 0, as
#                                                                           wx.py decides it; else a STOP)
#                            JUDGE-B <spelling> <form>: <the answer's kind> (a STATUS, or what nm.py makes of it)
#                            JUDGE-B <spelling> <form>: NOT JUDGED (why)    (BOOK does not show the invoice as made)
#                          The empty answer is checked first, for every form, so no reader is given one.
# Example: python3 -I sc.py --judge-b kb census a-b3a-C-v-kb.xml a-k8-Vb-book.xml
import pathlib, sys
here = pathlib.Path(__file__).resolve().parent

def load(name):
    """The functions of a script of this folder: its text up to the line where it reads its arguments, run once."""
    source = (here / name).read_text("utf-8")
    marker = "\nargs = sys.argv[1:]\n"
    if source.count(marker) != 1:
        print(f"STOP: {name}: not the script this one expects (its argument line is not there once)")
        raise SystemExit(1)
    space = {"__file__": str(here / name), "__name__": f"sc_{name[:-3]}"}
    exec(compile(source.split(marker, 1)[0], name, "exec"), space)
    return space

sk = load("sk.py")
say, stop = sk["say"], sk["stop"]

def altered_state(path):
    table, rows = sk["names_tsv"](), sk["catalogue"](path)
    a, b = table["ka"][1], table["kb"][1]
    say(f"{path}: LEDGER rows {len(rows)}")
    by_attribute = [r for r in rows if r[0] == a]
    why = []
    if len(by_attribute) != 1:
        why.append(f"{len(by_attribute)} rows with ka's name as their NAME attribute")
    else:
        _, listed, guid = by_attribute[0]
        if listed != [b]:
            why.append(f"its names are {listed!r}")
        if not guid.startswith(sk["BOOK"] + "-"):
            why.append("its GUID is not the book's")
    if any(r[0] == b for r in rows):
        why.append("a row has kb's name as its NAME attribute")
    if any(r[0] != a and (a in r[1] or b in r[1]) for r in rows):
        why.append("another row's name list holds ka's or kb's name")
    say(f"ALTERED STATE k: {'NOT AS SK RECORDED (' + '; '.join(why) + ')' if why else 'AS SK RECORDED'}")
    raise SystemExit(3 if why else 0)

def judge_b(spelling, form, answer, book):
    if spelling not in ("ka", "kb") or form not in ("outstandings", "census", "movement", "window"):
        stop("usage: --judge-b SPELLING FORM ANSWER BOOK, SPELLING ka or kb, FORM outstandings, census, movement or "
             "window")
    head = f"JUDGE-B {spelling} {form}"
    table = sk["names_tsv"]()
    wx = sk["reader"]("wx.py")
    if wx["bytes_or_empty"](answer) is None:
        return f"{head}: EMPTY ANSWER", 3
    book_kind, book_rows = wx["read"](book)
    if book_kind != "vouchers":
        return f"{head}: NOT JUDGED (the book-wide read is not a read with vouchers)", 3
    on = lambda r, who: any(e["line"][0] == who for e in r["entries"])
    own, why = sk["invoice_in"](book_rows, table["ka"][1], table["ka"][0], on)
    if own is None:
        return f"{head}: NOT JUDGED ({why})", 3
    if form == "outstandings":
        kind, rows = sk["reader"]("nm.py")["answer"](answer, show=False)
        if rows is None:
            return f"{head}: {kind}", 3
        held, mine = [r["BILLREF"] for r in rows], f"BP/26-27/{table['ka'][0]}"
    elif form == "census":
        kind, rows = sk["reader"]("cv.py")["read"](answer)
        if kind != "vouchers":
            return f"{head}: {'NO BYTES' if kind == 'none' else 'STATUS ' + rows[0]}", 3
        held, mine = [g for g, _, _ in rows], own["own"]["GUID"]
    else:
        kind, rows = wx["read"](answer)
        if kind != "vouchers":
            return f"{head}: STATUS {rows[0]}", 3
        held, mine = [r["key"] for r in rows], own["key"]
    if held.count(mine) > 1:
        stop(f"{answer}: the invoice is held {held.count(mine)} times, not once")
    others = sum(1 for h in held if h != mine)
    has_own = mine in held
    return f"{head}: OWN {'YES' if has_own else 'NO'}, OTHERS {others}", 0 if has_own and others == 0 else 3

args = sys.argv[1:]
if len(args) == 2 and args[0] == "--altered-state":
    altered_state(args[1])
elif len(args) == 5 and args[0] == "--judge-b":
    text, code = judge_b(*args[1:])
    say(text)
    raise SystemExit(code)
else:
    stop("usage: sc.py --altered-state G | --judge-b SPELLING FORM ANSWER BOOK")
