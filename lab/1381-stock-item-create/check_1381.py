#!/usr/bin/env python3
"""Reads row 1381-M's answers as LAB-COMMON C4 says, and prints what the run sheet records.

  check_1381.py marks ANSWER               the COMPANY rows: GUID, ALTVCHID, ALTMSTID
  check_1381.py import ANSWER              every counter, and each LINEERROR
  check_1381.py rows ANSWER                each STOCKITEM, STOCKGROUP or UNIT row, field by field
  check_1381.py diff BEFORE AFTER          rows added, removed or changed, matched by GUID
  check_1381.py same ANSWER NAME-A NAME-B  whether two items hold the same GST fields

Names are printed as received, never trimmed, with their length. An answer that does not parse, a
counter that is missing, or a row without a GUID is said as such and the command exits 1: a read that
failed is never printed as "nothing" or "no change".
"""
import re, sys
import xml.etree.ElementTree as ET

COUNTERS = ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS',
            'LASTVCHID', 'LASTMID')
ROW_TAGS = ('STOCKITEM', 'STOCKGROUP', 'UNIT')
IDS = ('GUID', 'MASTERID', 'ALTERID', 'BRIDGECOMPANYGUID')
GST_FIELDS = ('PARENT', 'BASEUNITS', 'GSTAPPLICABLE', 'GSTTYPEOFSUPPLY', 'GSTDETAILS.LIST', 'HSNDETAILS.LIST')
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)


def fail(why):
    print(f'NOT READ: {why}')
    sys.exit(1)


def raw_text(path):
    raw = open(path, 'rb').read()
    if not raw:
        fail(f'{path} is empty')
    return raw[2:].decode('utf-16-le') if raw[:2] == b'\xff\xfe' else raw.decode('utf-16-le')


def parse(path):
    try:
        return ET.fromstring(FORBIDDEN.sub('', raw_text(path)))
    except ET.ParseError as e:
        fail(f'{path} does not parse: {e}')


def canon(el):
    """An element as a comparable string: tag, attributes, trimmed text, children in order."""
    attrs = ''.join(f' {k}={v!r}' for k, v in sorted(el.attrib.items()))
    inner = ''.join(canon(c) for c in el)
    return f'<{el.tag}{attrs}>{(el.text or "").strip()}{inner}</{el.tag}>'


def rows(path):
    root = parse(path)
    found = [r for c in root.iter('COLLECTION') for r in c if r.tag in ROW_TAGS]
    out = {}
    for r in found:
        guid = r.findtext('GUID')
        if guid is None or not guid.strip():
            fail(f'a {r.tag} row named {r.get("NAME")!r} has no GUID')
        guid = guid.strip()
        if guid in out:
            fail(f'GUID {guid} appears twice')
        fields = {}
        for c in r:
            fields.setdefault(c.tag, []).append(c)
        out[guid] = (r.tag, r.get('NAME'), fields)
    return out


def show_name(name):
    return 'None' if name is None else f'{name!r} ({len(name)} characters)'


def field_text(fields, tag):
    els = fields.get(tag, [])
    if not els:
        return 'absent'
    if len(els) > 1 or len(els[0]):
        return f'{len(els)} element(s): ' + ' | '.join(canon(e) for e in els)
    return repr((els[0].text or '').strip())


def cmd_marks(path):
    companies = [c for c in parse(path).iter('COMPANY') if c.find('GUID') is not None]
    print('COMPANY rows:', len(companies))
    for c in companies:
        print(' ', ' '.join(f'{k} {(c.findtext(k) or "").strip()}' for k in ('GUID', 'ALTVCHID', 'ALTMSTID')))


def cmd_import(path):
    root = parse(path)
    missing = [k for k in COUNTERS if root.find(f'.//{k}') is None]
    for k in COUNTERS:
        el = root.find(f'.//{k}')
        print(f'{k} {"MISSING" if el is None else (el.text or "").strip()}')
    for e in root.iter('LINEERROR'):
        print('LINEERROR', repr(e.text))
    if missing:
        fail(f'counters missing: {missing}')


def cmd_rows(path):
    found = rows(path)
    print('rows:', len(found))
    for guid, (tag, name, fields) in sorted(found.items(), key=lambda kv: kv[1][1] or ''):
        print(f'{tag} {show_name(name)} GUID {guid}')
        for k in IDS[1:3]:
            print(f'  {k} {field_text(fields, k)}')
        for k in sorted(set(fields) - set(IDS) - {'NAME'}):
            print(f'  {k} {field_text(fields, k)}')


def cmd_diff(before, after):
    a, b = rows(before), rows(after)
    print('rows before:', len(a), ' after:', len(b))
    for guid in sorted(set(b) - set(a)):
        print('ADDED', b[guid][0], show_name(b[guid][1]), guid)
    for guid in sorted(set(a) - set(b)):
        print('REMOVED', a[guid][0], show_name(a[guid][1]), guid)
    for guid in sorted(set(a) & set(b)):
        (_, na, fa), (_, nb, fb) = a[guid], b[guid]
        changed = [k for k in sorted(set(fa) | set(fb)) if field_text(fa, k) != field_text(fb, k)]
        if na != nb:
            changed.insert(0, 'NAME attribute')
        if changed:
            print('CHANGED', show_name(nb), guid, ':', ', '.join(changed))
            for k in changed:
                if k != 'NAME attribute':
                    print(f'  {k}: {field_text(fa, k)}  ->  {field_text(fb, k)}')
    print('unchanged rows:', sum(1 for g in set(a) & set(b)
                                 if a[g][1] == b[g][1] and all(field_text(a[g][2], k) == field_text(b[g][2], k)
                                                               for k in set(a[g][2]) | set(b[g][2]))))


def cmd_same(path, name_a, name_b):
    found = rows(path)
    pick = {}
    for want in (name_a, name_b):
        hits = [f for _, (tag, n, f) in found.items() if tag == 'STOCKITEM' and n == want]
        if len(hits) != 1:
            fail(f'{len(hits)} STOCKITEM rows named {want!r}')
        pick[want] = hits[0]
    for k in GST_FIELDS:
        ta, tb = field_text(pick[name_a], k), field_text(pick[name_b], k)
        print(f'{k}: {"same" if ta == tb else "DIFFERENT"}')
        if ta != tb:
            print(f'  {name_a}: {ta}\n  {name_b}: {tb}')


if __name__ == '__main__':
    mode, args = (sys.argv[1], sys.argv[2:]) if len(sys.argv) > 1 else ('', [])
    commands = {'marks': (cmd_marks, 1), 'import': (cmd_import, 1), 'rows': (cmd_rows, 1),
                'diff': (cmd_diff, 2), 'same': (cmd_same, 3)}
    if mode not in commands or len(args) != commands[mode][1]:
        sys.exit(__doc__)
    commands[mode][0](*args)
