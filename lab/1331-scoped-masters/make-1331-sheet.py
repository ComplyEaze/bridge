#!/usr/bin/env python3 -I
"""Writes the request files of lab sheet 1331 (scoped invoice masters) into OUT/requests.

Run: python3 -I make-1331-sheet.py BASE_DIR OUT_DIR [AVOID_LIST]
AVOID_LIST, if given, is a file of words (one per line) no ledger name may contain, in any case or
spacing; a code that would contain one is skipped, so the names stay 20,000 and distinct.
BASE_DIR holds the captured request files this sheet changes one filter of:
  r02-marks.xml, r05-ledgers-compliance.xml, r06-ledgers-paired.xml, r07-groups.xml,
  r13-ledger-catalogue-v2.xml
  (lab/1342-capture-requests), rates-scoped-hsn.xml (the pilot-lab-ledger-rates-scoped-hsn
  request fixture) and extent-stock.xml (lab/1233-large-stock-book m-marks-S).
Every request is UTF-16LE with a BOM, as the base files are. Only the named parts change.
"""
import hashlib, os, sys

BASE, OUT = sys.argv[1], sys.argv[2]
AVOID = []
if len(sys.argv) > 3:
    AVOID = [w.strip().lower().replace(' ', '') for w in open(sys.argv[3], encoding='utf-8') if w.strip()]
REQ = os.path.join(OUT, 'requests')
os.makedirs(os.path.join(REQ, 'imports'), exist_ok=True)

COMPANY = 'BRIDGE PILOT LAB'
CGUID = '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
# The five ledgers of the lab invoice, by catalogue spelling and GUID (catalogue of 30 Oct 2026).
FIVE = [
    ('BRIDGE Walk-in', CGUID + '-000000d0'),
    ('BRIDGE Goods 18%', CGUID + '-000000d3'),
    ('BRIDGE CGST 9%', CGUID + '-000000d5'),
    ('BRIDGE SGST 9%', CGUID + '-000000d7'),
    ('BRIDGE Round Off', CGUID + '-000000d8'),
]
ABSENT_GUID = CGUID + '-0000ffff'
ABSENT_NAME = 'BRIDGE No Such Ledger'
FORMULA = 'BridgeInvoiceLedgers'


def read(name):
    data = open(os.path.join(BASE, name), 'rb').read()
    assert data[:2] == b'\xff\xfe', name
    return data[2:].decode('utf-16-le')


def write(rel, text):
    path = os.path.join(REQ, rel)
    data = b'\xff\xfe' + text.encode('utf-16-le')
    with open(path, 'wb') as f:
        f.write(data)
    return path


def once(text, old, new):
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new)


def esc(value):
    return value.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('"', '&quot;')


def or_of(field, values):
    return ' OR '.join('$%s = "%s"' % (field, esc(v)) for v in values)


rates = read('rates-scoped-hsn.xml')
PARENT_SYSTEM = rates[rates.index('<SYSTEM '):rates.index('</SYSTEM>') + len('</SYSTEM>')]
PARENT_FILTERS = '<FILTERS>BridgeParentPart</FILTERS>'


def rates_with(formula_text):
    t = once(rates, PARENT_SYSTEM, '<SYSTEM TYPE="Formulae" NAME="%s">%s</SYSTEM>' % (FORMULA, formula_text))
    return once(t, PARENT_FILTERS, '<FILTERS>%s</FILTERS>' % FORMULA)


def rates_parents(parents):
    return once(rates, PARENT_SYSTEM, '<SYSTEM TYPE="Formulae" NAME="BridgeParentPart">%s</SYSTEM>' % or_of('Parent', parents))


master = read('r05-ledgers-compliance.xml')
balance = read('r06-ledgers-paired.xml')


def master_with(formula_text, base=None):
    t = once(master if base is None else base, '<COLLECTION NAME="List of Ledgers"',
             '<SYSTEM TYPE="Formulae" NAME="%s">%s</SYSTEM><COLLECTION NAME="List of Ledgers"' % (FORMULA, formula_text))
    return once(t, '</COMPUTE></COLLECTION>', '</COMPUTE><FILTERS>%s</FILTERS></COLLECTION>' % FORMULA)


extent = read('extent-stock.xml')
extent = once(extent, '<SVCURRENTCOMPANY>BRIDGE STOCK LAB</SVCURRENTCOMPANY>', '<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>' % COMPANY)
extent = once(extent, '1edb9b05-8d35-4c0c-9959-dce655730463', CGUID)

guids = [g for _, g in FIVE]
names = [n for n, _ in FIVE]
files = {
    'm-marks.xml': read('r02-marks.xml'),
    'e-extent.xml': extent,
    'k-catalogue-v2.xml': read('r13-ledger-catalogue-v2.xml'),
    'g-groups.xml': read('r07-groups.xml'),
    'g1-rates-guid-five.xml': rates_with(or_of('GUID', guids)),
    'g2-rates-guid-four-and-absent.xml': rates_with(or_of('GUID', guids[:4] + [ABSENT_GUID])),
    'g3-rates-guid-absent-only.xml': rates_with(or_of('GUID', [ABSENT_GUID])),
    'g4-master-guid-five.xml': master_with(or_of('GUID', guids)),
    'g5-balance-guid-five.xml': master_with(or_of('GUID', guids), balance),
    'n1-rates-name-five.xml': rates_with(or_of('Name', names)),
    'n2-rates-name-case-changed.xml': rates_with(or_of('Name', ['BRIDGE WALK-IN'])),
    'n3-rates-name-absent-only.xml': rates_with(or_of('Name', [ABSENT_NAME])),
    'p1-rates-parents-four.xml': rates,
    'c-master-whole.xml': master,
    'c-balance-whole.xml': balance,
    'p2-rates-parents-three.xml': rates_parents(['Duties & Taxes', 'Indirect Expenses', 'Sales Accounts']),
}
assert files['p2-rates-parents-three.xml'] != rates
for rel, text in files.items():
    write(rel, text)

# Imports: 40 files of 500 Sundry Debtors ledgers each, in the captured Create shape
# (crates/bridge-tally-protocol/src/ledger_create.rs) inside the All Masters import envelope
# that lab/1233-large-stock-book used. Names carry no digit: a six-letter code over
# consonants that cannot spell a word, so 1..20,000 map to distinct names.
LETTERS = 'BCDFGHJKLMNPQRSTVWXZ'


def code(n):
    out = ''
    for _ in range(6):
        n, r = divmod(n, len(LETTERS))
        out = LETTERS[r] + out
    assert n == 0
    return out


HEAD = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER>'
        '<BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>%s'
        '</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>' % COMPANY)
TAIL = '</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
def avoided(name):
    flat = name.lower().replace(' ', '')
    return any(w in flat or w in name.lower() for w in AVOID)


def names_from(start):
    i = start
    while True:
        name = 'Lab Scale Debtor ' + code(i)
        i += 1
        if not avoided(name):
            yield name


names_iter = names_from(0)
seen = set()
for chunk in range(40):
    msgs = []
    for _ in range(500):
        name = next(names_iter)
        assert name not in seen
        seen.add(name)
        msgs.append('<TALLYMESSAGE><LEDGER NAME="%s" ACTION="Create"><NAME>%s</NAME><PARENT>Sundry Debtors</PARENT>'
                    '<ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE>' % (name, name))
    write('imports/i%03d-debtors.xml' % (chunk + 1), HEAD + ''.join(msgs) + TAIL)
assert len(seen) == 20000

lines = []
for root, _, fs in os.walk(REQ):
    for f in sorted(fs):
        p = os.path.join(root, f)
        data = open(p, 'rb').read()
        lines.append('%s  %s  (%d bytes)' % (hashlib.sha256(data).hexdigest(), os.path.relpath(p, OUT), len(data)))
with open(os.path.join(OUT, 'SHA256.txt'), 'w') as f:
    f.write('\n'.join(sorted(lines, key=lambda l: l.split('  ')[1])) + '\n')
print('\n'.join(sorted(lines, key=lambda l: l.split('  ')[1])[:20]))
print(len(lines), 'files')
