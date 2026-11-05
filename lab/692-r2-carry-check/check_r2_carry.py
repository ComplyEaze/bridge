#!/usr/bin/env python3
"""Three True/False checks on the answers row 692 kept, about R2's carried Profit & Loss A/c.

  python -I check_r2_carry.py ANSWERS-FOLDER

ANSWERS-FOLDER is the 692 run's own `answers` folder on the lab PC. Four files are read from it, and
each must first equal, in bytes and sha256, the line `lab/692-opening-stock-answers` committed for it
(09287f70, SHA256.txt). Nothing is sent to Tally. Verdicts and counts only: no name, no amount.

The parsing is relations.py's (lab/692-opening-stock at 991c3ec4): a line missing or repeated is said
as such, and a verdict that needs it prints `not checked`. A missing line is never read as zero.
"""
import hashlib, pathlib, re, sys
from decimal import Decimal

KEPT = {
    'r1-w1-pl.utf16le.xml': (1922, 'c5f34dcb0b4135af46b72f01372f063a3906db30405ae271d92696d92273efa8'),
    'r1-w2-ledgers.utf16le.xml': (13548, 'c5bd41fd0816b3dd75d3f003ee4d7900a1eba27722dd91b786034807a6fa4286'),
    'r2-w1-pl.utf16le.xml': (1922, 'feebe9af203f43d4841faf31458d73babd3db5e78ba753370cb2924cdbf22ab8'),
    'r2-w2-ledgers.utf16le.xml': (13548, 'f08f2ae27f89f29fb9df84f94a1184d6b4db3000c663da490c0a2d4bea0b75d8'),
}


def read(folder, name):
    raw = (folder / name).read_bytes()
    size, digest = KEPT[name]
    if len(raw) != size or hashlib.sha256(raw).hexdigest() != digest:
        sys.exit(f'{name}: {len(raw)} B, not the file 09287f70 recorded: nothing checked')
    print(f'{name}: {size} B, sha256 equal to the record')
    return raw.decode('utf-16' if raw[:2] == b'\xff\xfe' else 'utf-16-le')


def amount(s):
    s = s.strip()
    return Decimal(s) if s else None


def statement(name, t):
    rows = re.findall(
        r'<DSPDISPNAME>(.*?)</DSPDISPNAME>\s*</DSPACCNAME>\s*<PLAMT>\s*'
        r'<PLSUBAMT>(.*?)</PLSUBAMT>\s*<BSMAINAMT>(.*?)</BSMAINAMT>\s*</PLAMT>', t, re.S)
    if len(rows) != t.count('<DSPACCNAME>'):
        sys.exit(f'{name}: {t.count("<DSPACCNAME>")} names but {len(rows)} name-amount pairs: not read')
    return [(n.replace('&amp;', '&'), amount(s), amount(m)) for n, s, m in rows]


def nett(name, t):
    """Tally's year-1 nett: the sum of the main column, as relations.py's `carry` takes it."""
    rows = statement(name, t)
    print(f'  {name}: Profit and Loss lines {len(rows)}')
    return sum(r[2] for r in rows if r[2] is not None)


def closing(name, t):
    hits = [r for r in statement(name, t) if r[0] == 'Less: Closing Stock']
    print(f'  {name}: "Less: Closing Stock" lines {len(hits)}')
    return hits[0][1] if len(hits) == 1 else None


def pl_opening(name, t):
    out = []
    blocks = list(re.finditer(r'<LEDGER NAME="([^"]*)".*?</LEDGER>', t, re.S))
    if len(blocks) != t.count('<LEDGER NAME='):
        sys.exit(f'{name}: a ledger row does not close: not read')
    for m in blocks:
        b = m.group(0)
        parent = re.search(r'<PARENT[^>]*>(.*?)</PARENT>', b, re.S)
        if parent and 'Primary' in parent.group(1) and m.group(1).startswith('Profit'):
            opening = re.search(r'<TBALOPENING[^>]*>(.*?)</TBALOPENING>', b, re.S)
            out.append(amount(opening.group(1)) if opening else None)
    print(f'  {name}: Profit & Loss A/c ledgers {len(out)}')
    return out[0] if len(out) == 1 else None


def verdict(label, left, right, op):
    if left is None or right is None:
        print(f'{label}: not checked')
    elif op == '==':
        print(f'{label}: {left == right}  with the sign reversed: {left == -right}')
    else:
        print(f'{label}: {left != right}')


if len(sys.argv) != 2:
    sys.exit(__doc__)
folder = pathlib.Path(sys.argv[1])
text = {name: read(folder, name) for name in KEPT}
r1_nett = nett('r1-w1-pl.utf16le.xml', text['r1-w1-pl.utf16le.xml'])
r1_open = pl_opening('r1-w2-ledgers.utf16le.xml', text['r1-w2-ledgers.utf16le.xml'])
r2_open = pl_opening('r2-w2-ledgers.utf16le.xml', text['r2-w2-ledgers.utf16le.xml'])
r1_close = closing('r1-w1-pl.utf16le.xml', text['r1-w1-pl.utf16le.xml'])
r2_close = closing('r2-w1-pl.utf16le.xml', text['r2-w1-pl.utf16le.xml'])
verdict("check 1, R2's year-2 P&L A/c opening == R1's year-1 nett", r2_open, r1_nett, '==')
verdict("check 2, R2's year-2 P&L A/c opening == R1's year-2 P&L A/c opening", r2_open, r1_open, '==')
verdict("check 3, R2's year-1 closing stock != R1's", r2_close, r1_close, '!=')
