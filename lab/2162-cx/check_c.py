#!/usr/bin/env python3
"""Reads row 2162-L2b's answers as LAB-COMMON C4 says. lab/1381-r1/check_r1.py (at c76e23d6) with two modes added,
`types` and `vouchers`, VOUCHERTYPE added to the row tags, and `marks` failing unless it finds exactly one
COMPANY row; its other modes are unchanged.

  check_c.py marks ANSWER          COMPANY rows: GUID (first 8 characters), ALTVCHID, ALTMSTID
  check_c.py import ANSWER         every counter, and each LINEERROR
  check_c.py diff BEFORE AFTER     rows added, removed or changed, matched by GUID
  check_c.py types ANSWER          each VOUCHERTYPE row: NAME, PARENT and NUMBERINGMETHOD as received, MASTERID, ALTERID
  check_c.py vouchers ANSWER       each VOUCHER row with a VOUCHERNUMBER: number, type name, ISCANCELLED, BRIDGEVCHISSALES

Rows are read ONLY at ./BODY/DATA/COLLECTION/<ROW> (CMPINFO's bare count lines are never rows). An import answer's
counters are read only at ./<COUNTER> of its RESPONSE. An answer that does not parse, a counter that is missing, or
a row without its key field is said as such and the command exits 1: a read that failed is never printed as
"nothing" or "no change". The book is synthetic and these sheet-named values print as received, with their length.
"""
import re, sys
import xml.etree.ElementTree as ET

COUNTERS = ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS',
            'LASTVCHID', 'LASTMID')
ROW_TAGS = ('STOCKITEM', 'STOCKGROUP', 'UNIT', 'COMPANY', 'VOUCHERTYPE')
IDS = ('GUID', 'MASTERID', 'ALTERID', 'BRIDGECOMPANYGUID')
GST_FIELDS = ('PARENT', 'BASEUNITS', 'GSTAPPLICABLE', 'GSTTYPEOFSUPPLY', 'GSTDETAILS.LIST', 'HSNDETAILS.LIST')
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)


def fail(why):
    print(f'NOT READ: {why}')
    sys.exit(1)


def parse(path):
    raw = open(path, 'rb').read()
    if not raw:
        fail(f'{path} is empty')
    text = raw[2:].decode('utf-16-le') if raw[:2] == b'\xff\xfe' else raw.decode('utf-16-le')
    try:
        return ET.fromstring(FORBIDDEN.sub('', text))
    except ET.ParseError as e:
        fail(f'{path} does not parse: {e}')


def canon(el):
    """An element as a comparable string: tag, attributes, trimmed text, children in order."""
    attrs = ''.join(f' {k}={v!r}' for k, v in sorted(el.attrib.items()))
    return f'<{el.tag}{attrs}>{(el.text or "").strip()}{"".join(canon(c) for c in el)}</{el.tag}>'


def rows(path):
    root = parse(path)
    if root.tag != 'ENVELOPE':
        fail(f'{path} is not an ENVELOPE')
    out = {}
    for tag in ROW_TAGS:
        for r in root.findall(f'./BODY/DATA/COLLECTION/{tag}'):
            guid = (r.findtext('GUID') or '').strip()
            if not guid:
                fail(f'a {tag} row named {r.get("NAME")!r} has no GUID')
            if guid in out:
                fail(f'a GUID appears twice ({guid[:8]}...)')
            fields = {}
            for c in r:
                fields.setdefault(c.tag, []).append(c)
            out[guid] = (tag, r.get('NAME'), fields)
    return out


def show_name(name):
    return 'None' if name is None else f'{name!r} ({len(name)} characters)'


def field_text(fields, tag):
    els = fields.get(tag, [])
    if not els:
        return 'absent'
    return ' | '.join(canon(e) for e in els)


def shape(fields, tag):
    els = fields.get(tag, [])
    if not els:
        return 'absent'
    if len(els) == 1 and not len(els[0]):
        return 'empty' if not (els[0].text or '').strip() else 'a value'
    return f'{len(els)} element(s), {sum(1 for e in els for _ in e.iter()) - len(els)} descendants'


def item(path, name):
    hits = [f for _, (tag, n, f) in rows(path).items() if tag == 'STOCKITEM' and n == name]
    if len(hits) != 1:
        fail(f'{len(hits)} STOCKITEM rows named {name!r} in {path}')
    return hits[0]


def cmd_marks(path):
    found = [(f, n) for _, (tag, n, f) in rows(path).items() if tag == 'COMPANY']
    print('COMPANY rows:', len(found))
    if len(found) != 1:
        fail(f'{len(found)} COMPANY rows: a marks read without its one COMPANY row is no answer')
    for f, _ in found:
        g = lambda k: (f[k][0].text or '').strip() if k in f else 'absent'
        print(f'  GUID {g("GUID")[:8]}... ALTVCHID {g("ALTVCHID")} ALTMSTID {g("ALTMSTID")}')


def cmd_import(path):
    root = parse(path)
    if root.tag != 'RESPONSE':
        fail(f'{path} is not a RESPONSE')
    missing = [k for k in COUNTERS if root.find(f'./{k}') is None]
    for k in COUNTERS:
        el = root.find(f'./{k}')
        print(f'{k} {"MISSING" if el is None else (el.text or "").strip()}')
    for e in root.findall('./LINEERROR'):
        print('LINEERROR', repr(e.text))
    if missing:
        fail(f'counters missing: {missing}')


def cmd_rows(path):
    found = rows(path)
    print('rows:', len(found))
    for guid, (tag, name, fields) in sorted(found.items(), key=lambda kv: (kv[1][0], kv[1][1] or '')):
        print(f'{tag} {show_name(name)}')
        for k in sorted(set(fields) - {'NAME', 'GUID'}):
            print(f'  {k}: {shape(fields, k)}')


def cmd_diff(before, after):
    a, b = rows(before), rows(after)
    print('rows before:', len(a), ' after:', len(b))
    for guid in sorted(set(b) - set(a)):
        print('ADDED', b[guid][0], show_name(b[guid][1]))
    for guid in sorted(set(a) - set(b)):
        print('REMOVED', a[guid][0], show_name(a[guid][1]))
    unchanged = 0
    for guid in sorted(set(a) & set(b)):
        (_, na, fa), (_, nb, fb) = a[guid], b[guid]
        changed = [k for k in sorted(set(fa) | set(fb)) if field_text(fa, k) != field_text(fb, k)]
        if na != nb:
            changed.insert(0, 'NAME attribute')
        if not changed:
            unchanged += 1
            continue
        print('CHANGED', show_name(nb), ':', ', '.join(changed))
        for k in changed:
            if k != 'NAME attribute':
                print(f'  {k}: {field_text(fa, k)}  ->  {field_text(fb, k)}')
    print('unchanged rows:', unchanged)


def cmd_fetch(a0, a1):
    r0, r1 = rows(a0), rows(a1)
    print('rows a0:', len(r0), ' a1:', len(r1), ' same GUIDs:', set(r0) == set(r1))
    for guid in sorted(set(r0) & set(r1)):
        (_, n0, f0), (_, n1, f1) = r0[guid], r1[guid]
        lost = [k for k in f0 if field_text(f0, k) != field_text(f1, k)]
        added = sorted(set(f1) - set(f0))
        print(f'{show_name(n1)}: NAME attribute {"same" if n0 == n1 else "DIFFERENT"}; a0 fields not equal in a1: '
              f'{lost or "none"}; a1 adds: {", ".join(f"{k} ({shape(f1, k)})" for k in added) or "none"}')


def cmd_same(path_a, name_a, path_b, name_b):
    fa, fb = item(path_a, name_a), item(path_b, name_b)
    for k in GST_FIELDS:
        ta, tb = field_text(fa, k), field_text(fb, k)
        print(f'{k}: {"same" if ta == tb else "DIFFERENT"} ({shape(fa, k)} / {shape(fb, k)})')


def cmd_names(path, name):
    f = item(path, name)
    entries = [n for l in f.get('LANGUAGENAME.LIST', []) for nl in l.findall('./NAME.LIST') for n in nl.findall('./NAME')]
    print(f'LANGUAGENAME.LIST elements: {len(f.get("LANGUAGENAME.LIST", []))}; name entries: {len(entries)}')
    for n in entries:
        print('  ', show_name(n.text))


def field(row, tag):
    els = row.findall(tag)
    return 'absent' if not els else ' | '.join(repr(e.text or '') for e in els)


def cmd_types(path):
    found = [(n, f) for _, (tag, n, f) in rows(path).items() if tag == 'VOUCHERTYPE']
    print('VOUCHERTYPE rows:', len(found))
    for name, f in sorted(found, key=lambda x: x[0] or ''):
        g = lambda k: ' | '.join(repr(e.text or '') for e in f[k]) if k in f else 'absent'
        print(f'  {show_name(name)} PARENT {g("PARENT")} NUMBERINGMETHOD {g("NUMBERINGMETHOD")} '
              f'MASTERID {g("MASTERID")} ALTERID {g("ALTERID")}')


def cmd_vouchers(path):
    root = parse(path)
    if root.tag != 'ENVELOPE':
        fail(f'{path} is not an ENVELOPE')
    found = root.findall('./BODY/DATA/COLLECTION/VOUCHER')
    if any(r.find('VOUCHERNUMBER') is None for r in found):
        fail('a VOUCHER row has no VOUCHERNUMBER')
    print('VOUCHER rows:', len(found))
    for r in found:
        print(f'  {field(r, "VOUCHERNUMBER")} type {field(r, "VOUCHERTYPENAME")} ISCANCELLED {field(r, "ISCANCELLED")} '
              f'BRIDGEVCHISSALES {field(r, "BRIDGEVCHISSALES")}')


if __name__ == '__main__':
    mode, args = (sys.argv[1], sys.argv[2:]) if len(sys.argv) > 1 else ('', [])
    commands = {'marks': (cmd_marks, 1), 'import': (cmd_import, 1), 'rows': (cmd_rows, 1), 'diff': (cmd_diff, 2),
                'fetch': (cmd_fetch, 2), 'same': (cmd_same, 4), 'names': (cmd_names, 2),
                'types': (cmd_types, 1), 'vouchers': (cmd_vouchers, 1)}
    if mode not in commands or len(args) != commands[mode][1]:
        sys.exit(__doc__)
    commands[mode][0](*args)
