#!/usr/bin/env python3 -I
"""Reads the answers of this sheet by structure, never by substring.

  python -I answers.py shape FILE      what an answer is: parsed or not, its class, root, element counts
  python -I answers.py compare A B     two status answers: their voucher rows, keyed by MASTERID
  python -I answers.py --self-test     proves every verdict below fires on built-in controls

Answers are UTF-16LE as Tally sends them (with or without a byte-order mark). Tally writes character
references a strict parser refuses (for example `&#4;`); each one below 32, other than tab, line feed and
carriage return, is replaced by the text `[#n]` before parsing, the same way in both answers of a compare.

shape: the class is `refused` (a bare RESPONSE root, any LINEERROR, or a HEADER STATUS of 0), `empty` (an
ENVELOPE with no child element: it cannot be told from a report that was not rendered) or `answered`.
Exit 0 when the file parsed; 3 when it did not (the reason and the first 500 characters are printed;
the answer is still evidence, never "empty"); 2 when it could not be read.

compare: the rows are the VOUCHER elements directly under COLLECTION; each row is every attribute and
every descendant element (name, attributes, text) in document order, keyed by its MASTERID. The CMPINFO
counters are not part of a row: they move on their own between two reads of the same request, and are
printed apart. Exit 0 = the same rows, field for field; 1 = a row differs, is missing or is new (each is
printed); 4 = both answers hold no row (the compare checks nothing); 2 = a file could not be read, did not
parse, is not a status answer, or holds a row with no MASTERID or two rows with one MASTERID; the same
file given twice, or any unexpected error, is also exit 2.
"""
import os, re, sys
import xml.etree.ElementTree as ET

LOW_REF = re.compile(r'&#(x[0-9A-Fa-f]+|[0-9]+);')

def low_ref(match):
    raw = match.group(1)
    n = int(raw[1:], 16) if raw[0] in 'xX' else int(raw)
    return '[#%d]' % n if n < 32 and n not in (9, 10, 13) else match.group(0)

def decode(data):
    if data[:2] == b'\xff\xfe':
        data = data[2:]
    return data.decode('utf-16-le')

def parse(text):
    return ET.fromstring(LOW_REF.sub(low_ref, text))

def read(path):
    with open(path, 'rb') as f:
        return decode(f.read())

def classify(root):
    if root.tag.upper() == 'RESPONSE':
        return 'refused'
    if any(e.tag.upper() == 'LINEERROR' for e in root.iter()):
        return 'refused'
    for e in root.iter():
        if e.tag.upper() == 'STATUS' and (e.text or '').strip() == '0':
            return 'refused'
    if root.tag.upper() == 'ENVELOPE' and len(root) == 0:
        return 'empty'
    return 'answered'

def counts(root):
    out = {}
    for e in root.iter():
        out[e.tag] = out.get(e.tag, 0) + 1
    return out

def rows_of(root):
    if root.tag.upper() != 'ENVELOPE' or classify(root) not in ('answered', 'empty'):
        raise ValueError('not a status answer (root %s, class %s)' % (root.tag, classify(root)))
    rows = {}
    for coll in root.iter():
        if coll.tag.upper() != 'COLLECTION':
            continue
        for v in coll:
            if v.tag.upper() != 'VOUCHER':
                continue
            ids = [c for c in v if c.tag.upper() == 'MASTERID']
            if len(ids) != 1 or not (ids[0].text or '').strip():
                raise ValueError('a VOUCHER row without exactly one MASTERID')
            key = ids[0].text.strip()
            if key in rows:
                raise ValueError('two VOUCHER rows with MASTERID %s' % key)
            rows[key] = tuple([('@', tuple(sorted(v.attrib.items())))] +
                              [(d.tag, tuple(sorted(d.attrib.items())), (d.text or '').strip())
                               for d in v.iter() if d is not v])
    return rows

def cmpinfo(root):
    return [(d.tag, (d.text or '').strip()) for c in root.iter() if c.tag.upper() == 'CMPINFO' for d in c]

def shape(text):
    try:
        root = parse(text)
    except ET.ParseError as e:
        return 3, ['parsed: no (%s)' % e, 'first 500 characters: %r' % text[:500]]
    lines = ['parsed: yes', 'class: %s' % classify(root), 'root: %s' % root.tag]
    vouchers = sum(1 for c in root.iter() if c.tag.upper() == 'COLLECTION' for v in c if v.tag.upper() == 'VOUCHER')
    lines.append('COLLECTION/VOUCHER rows: %d' % vouchers)
    for e in root.iter():
        if e.tag.upper() == 'LINEERROR':
            lines.append('LINEERROR: %r' % (e.text or ''))
    for tag, n in sorted(counts(root).items(), key=lambda kv: (-kv[1], kv[0])):
        lines.append('%6d  %s' % (n, tag))
    return 0, lines

def compare(text_a, text_b):
    try:
        root_a, root_b = parse(text_a), parse(text_b)
        rows_a, rows_b = rows_of(root_a), rows_of(root_b)
    except (ET.ParseError, ValueError) as e:
        return 2, ['cannot compare: %s' % e]
    lines = ['rows: %d and %d' % (len(rows_a), len(rows_b))]
    ca, cb = cmpinfo(root_a), cmpinfo(root_b)
    lines.append('CMPINFO: %s' % ('equal' if ca == cb else 'differs (recorded, not part of the rows): %s' %
                                  ', '.join('%s %s->%s' % (x[0], x[1], y[1]) for x, y in zip(ca, cb) if x != y)))
    if not rows_a and not rows_b:
        lines.append('both answers hold no row: the compare checks nothing')
        return 4, lines
    bad = 0
    for key in sorted(set(rows_a) | set(rows_b), key=lambda k: (len(k), k)):
        if key not in rows_b:
            bad += 1; lines.append('MASTERID %s: only in the first answer' % key)
        elif key not in rows_a:
            bad += 1; lines.append('MASTERID %s: only in the second answer' % key)
        elif rows_a[key] != rows_b[key]:
            bad += 1
            diff = [('%s %r -> %r' % (x[0], x[2], y[2])) for x, y in zip(rows_a[key], rows_b[key]) if x != y]
            if len(rows_a[key]) != len(rows_b[key]):
                diff.append('field count %d -> %d' % (len(rows_a[key]), len(rows_b[key])))
            lines.append('MASTERID %s differs: %s' % (key, '; '.join(diff)))
    lines.append('verdict: %s' % ('same rows' if not bad else '%d row(s) differ' % bad))
    return (1 if bad else 0), lines

def self_test():
    info = '<CMPINFO><LEDGER>%d</LEDGER><VOUCHER>%d</VOUCHER></CMPINFO>'
    def row(mid, inc, unc, alter):
        return ('<VOUCHER VCHTYPE="Sales"><DATE>20260801</DATE><MASTERID> %d</MASTERID><ALTERID> %d</ALTERID>'
                '<VCHGSTSTATUSISINCLUDED>%s</VCHGSTSTATUSISINCLUDED><VCHGSTSTATUSISUNCERTAIN>%s</VCHGSTSTATUSISUNCERTAIN>'
                '<NARRATION>a&#4;b</NARRATION></VOUCHER>' % (mid, alter, inc, unc))
    def answer(rows, counters=(7, 2)):
        return ('<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC>' + info % counters +
                '</DESC><DATA><COLLECTION>' + rows + '</COLLECTION></DATA></BODY></ENVELOPE>')
    a = answer(row(1, 'Yes', 'No', 10) + row(2, 'No', 'Yes', 11))
    no_rows = answer('')
    same_other_info = answer(row(1, 'Yes', 'No', 10) + row(2, 'No', 'Yes', 11), (0, 40))
    flag_flip = a.replace('<VCHGSTSTATUSISINCLUDED>No</VCHGSTSTATUSISINCLUDED>', '<VCHGSTSTATUSISINCLUDED>Yes</VCHGSTSTATUSISINCLUDED>', 1)
    alter_moved = a.replace('<ALTERID> 11</ALTERID>', '<ALTERID> 12</ALTERID>')
    one_gone = a.replace(row(2, 'No', 'Yes', 11), '')
    dup = a.replace('<MASTERID> 2</MASTERID>', '<MASTERID> 1</MASTERID>')
    refused = '<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>0</STATUS></HEADER><BODY><DATA><LINEERROR>Could not find Report</LINEERROR></DATA></BODY></ENVELOPE>'
    bare = '<RESPONSE>Unknown Request, cannot be processed</RESPONSE>'
    checks = [
        ('compare: an answer with itself', compare(a, a)[0], 0),
        ('compare: only CMPINFO differs', compare(a, same_other_info)[0], 0),
        ('compare: a status flag flips', compare(a, flag_flip)[0], 1),
        ('compare: an ALTERID moves', compare(a, alter_moved)[0], 1),
        ('compare: a row is missing', compare(a, one_gone)[0], 1),
        ('compare: both without rows', compare(no_rows, no_rows)[0], 4),
        ('compare: two rows share a MASTERID', compare(a, dup)[0], 2),
        ('compare: a refusal is not a status answer', compare(a, refused)[0], 2),
        ('compare: an unparseable answer', compare(a, '<ENVELOPE><X>')[0], 2),
        ('shape: unparseable', shape('<ENVELOPE><X>')[0], 3),
        ('shape: a status answer parses', shape(a)[0], 0),
    ]
    classes = [
        ('class: answered', classify(parse(a)), 'answered'),
        ('class: STATUS 0 with LINEERROR', classify(parse(refused)), 'refused'),
        ('class: bare RESPONSE', classify(parse(bare)), 'refused'),
        ('class: empty ENVELOPE', classify(parse('<ENVELOPE></ENVELOPE>')), 'empty'),
    ]
    bad = [name for name, got, want in checks + classes if got != want]
    rows = len(rows_of(parse(a)))
    if rows != 2:
        bad.append('rows: the control holds 2 rows, read %d' % rows)
    if decode(b'\xff\xfe' + 'x'.encode('utf-16-le')) != 'x' or decode('x'.encode('utf-16-le')) != 'x':
        bad.append('decode: a byte-order mark is not handled')
    for name in bad:
        print('CONTROL DID NOT FIRE: %s' % name)
    print('self-test: %s (%d controls)' % ('ok' if not bad else 'FAIL', len(checks) + len(classes) + 2))
    return 1 if bad else 0

def main(argv):
    if argv == ['--self-test']:
        return self_test()
    try:
        if len(argv) == 2 and argv[0] == 'shape':
            code, lines = shape(read(argv[1]))
        elif len(argv) == 3 and argv[0] == 'compare':
            if os.path.realpath(argv[1]) == os.path.realpath(argv[2]):
                print('cannot compare: the same file was given twice')
                return 2
            code, lines = compare(read(argv[1]), read(argv[2]))
        else:
            print(__doc__)
            return 2
    except (OSError, UnicodeDecodeError) as e:
        print('cannot read: %s' % e)
        return 2
    print('\n'.join(lines))
    return code

if __name__ == '__main__':
    try:
        sys.exit(main(sys.argv[1:]))
    except SystemExit:
        raise
    except BaseException as e:  # any other failure is "could not compare", never exit 1 ("a row differs")
        print('error: %s: %s' % (type(e).__name__, e))
        sys.exit(2)
