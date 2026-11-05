#!/usr/bin/env python3
"""The relations identities.py does not check, on Tally's own answers as received (UTF-16LE).

  relations.py window PL.xml STOCKSUMMARY.xml NEGATIVESTOCK.xml
  relations.py carry PL-YEAR1.xml LEDGERS-YEAR1.xml PL-YEAR2.xml LEDGERS-YEAR2.xml

Prints verdicts and counts only: no names, no amounts. Signs as Tally prints them (a debit is
negative). An answer that cannot be read, or a line missing or repeated, is said as such and the
verdicts that need it are not printed: a missing line is never read as zero.
"""
import re, sys
from decimal import Decimal


def text(path):
    raw = open(path, 'rb').read()
    return raw.decode('utf-16' if raw[:2] == b'\xff\xfe' else 'utf-16-le')


def amount(s):
    s = s.strip()
    return Decimal(s) if s else None


def statement(path):
    """The (name, sub, main) rows of a Profit and Loss answer, in order."""
    t = text(path)
    rows = re.findall(
        r'<DSPDISPNAME>(.*?)</DSPDISPNAME>\s*</DSPACCNAME>\s*<PLAMT>\s*'
        r'<PLSUBAMT>(.*?)</PLSUBAMT>\s*<BSMAINAMT>(.*?)</BSMAINAMT>\s*</PLAMT>', t, re.S)
    if len(rows) != t.count('<DSPACCNAME>'):
        sys.exit(f'{path}: {t.count("<DSPACCNAME>")} names but {len(rows)} name-amount pairs: not read')
    return [(n.replace('&amp;', '&'), amount(s), amount(m)) for n, s, m in rows]


def one(rows, name):
    hits = [r for r in rows if r[0] == name]
    print(f'  "{name}" lines: {len(hits)}')
    return hits[0] if len(hits) == 1 else None


def ledgers(path):
    t = text(path)
    out = {}
    for m in re.finditer(r'<LEDGER NAME="([^"]*)".*?</LEDGER>', t, re.S):
        b = m.group(0)
        def field(k):
            f = re.search(r'<%s[^>]*>(.*?)</%s>' % (k, k), b, re.S)
            return f.group(1) if f else None
        out[m.group(1).replace('&amp;', '&')] = (field('PARENT'), field('TBALOPENING'), field('TBALCLOSING'))
    if len(out) != t.count('<LEDGER NAME='):
        sys.exit(f'{path}: a ledger name repeats: not read')
    return out


def stock_rows(path):
    t = text(path)
    rows = re.findall(r'<DSPCLQTY>(.*?)</DSPCLQTY>\s*<DSPCLRATE>(.*?)</DSPCLRATE>\s*<DSPCLAMTA>(.*?)</DSPCLAMTA>', t, re.S)
    if len(rows) != t.count('<DSPSTKCL>'):
        sys.exit(f'{path}: {t.count("<DSPSTKCL>")} stock rows but {len(rows)} read: not read')
    return rows


def window(pl_path, ss_path, ns_path):
    rows = statement(pl_path)
    print('Profit and Loss lines:', len(rows))
    cos, o, p, c = (one(rows, n) for n in ('Cost of Sales :', 'Opening Stock', 'Add: Purchase Accounts', 'Less: Closing Stock'))
    direct = [r for r in rows if 'Direct Expenses' in r[0]]
    print('  lines naming Direct Expenses:', len(direct),
          ' with a sub amount:', sum(r[1] is not None for r in direct),
          ' with a main amount:', sum(r[2] is not None for r in direct))
    if None in (cos, o, p, c) or cos[2] is None:
        print('relation 4 (Cost of Sales heading): not checked, a line is missing, repeated or without its amount')
    else:
        parts = [x[1] or Decimal(0) for x in (o, p)] + [r[1] for r in direct if r[1] is not None]
        print('relation 4 (Cost of Sales heading == opening + purchases + direct expenses in the sub column - closing):',
              cos[2] == sum(parts) - (c[1] or Decimal(0)))
    ss = stock_rows(ss_path)
    values = [amount(a) for _, _, a in ss]
    print('stock summary rows:', len(ss), ' rows without a value:', sum(v is None for v in values))
    if c is not None and c[1] is not None and all(v is not None for v in values):
        print('stock summary total == closing stock line:', sum(values) == c[1])
    else:
        print('stock summary total == closing stock line: not checked')
    ns = stock_rows(ns_path)
    print('negative stock rows:', len(ns), ' of them without a value:', sum(amount(a) is None for _, _, a in ns))


def carry(pl1, led1, pl2, led2):
    rows1, rows2 = statement(pl1), statement(pl2)
    print('year 1:'); c1 = one(rows1, 'Less: Closing Stock')
    print('year 2:'); o2 = one(rows2, 'Opening Stock')
    if c1 and o2 and c1[1] is not None and o2[1] is not None:
        print("year 2's opening stock == year 1's closing stock:", o2[1] == c1[1])
    else:
        print("year 2's opening stock == year 1's closing stock: not checked")
    l1, l2 = ledgers(led1), ledgers(led2)
    pl_ledgers = [n for n, (par, _, _) in l2.items() if par and 'Primary' in par and n.startswith('Profit')]
    print('Profit & Loss A/c ledgers in year 2:', len(pl_ledgers))
    if len(pl_ledgers) != 1 or amount(l2[pl_ledgers[0]][1] or '') is None:
        print('carried result: not checked'); return
    opening = amount(l2[pl_ledgers[0]][1])
    tally_nett = sum(r[2] for r in rows1 if r[2] is not None)
    nominal = {'Sales Accounts', 'Purchase Accounts', 'Direct Incomes', 'Direct Expenses', 'Indirect Incomes', 'Indirect Expenses'}
    led_result = sum(amount(v[2] or '') or Decimal(0) for v in l1.values()
                     if v[0] and v[0].replace('&amp;', '&') in nominal)
    print('ledgers of year 1 directly under a nominal primary group:',
          sum(1 for v in l1.values() if v[0] and v[0].replace('&amp;', '&') in nominal))
    for label, value in (("Tally's year-1 nett (sum of the main column)", tally_nett),
                         ("the year-1 ledgers' result (nominal closings)", led_result)):
        print(f"year 2's Profit & Loss A/c opening == {label}:", opening == value,
              ' with the sign reversed:', opening == -value)


if __name__ == '__main__':
    mode, args = sys.argv[1], sys.argv[2:]
    if mode == 'window' and len(args) == 3:
        window(*args)
    elif mode == 'carry' and len(args) == 4:
        carry(*args)
    else:
        sys.exit(__doc__)
