#!/usr/bin/env python3
"""Reads rows 1108-R0 to R2's raw answers as LAB-COMMON C4 says, and prints what the run sheet records.
1108-A2's check_1108.py, made to fail closed.

  check_1108.py marks ANSWER               the COMPANY rows: GUID, ALTVCHID, ALTMSTID
  check_1108.py import ANSWER              every counter, and each LINEERROR
  check_1108.py rows ANSWER                each COMPANY, LEDGER or VOUCHER row, field by field
  check_1108.py diff BEFORE AFTER          rows added, removed or changed, matched by GUID

An export answer (marks, rows, diff) must be an ENVELOPE whose HEADER/STATUS is 1 and whose BODY/DATA
holds exactly one COLLECTION. Its rows are that COLLECTION's COMPANY, LEDGER and VOUCHER children; the
CMPINFO count lines are not rows. marks needs exactly one COMPANY row. An import answer must be a
RESPONSE holding every counter as a child, as each import answer A2 kept does.

Names are printed as received, never trimmed, with their length.

Exit 0: the answer was read and every check above held. Exit 1 and a FAIL line: it was read and is not
as above (a root, STATUS or COLLECTION not as above, other than one COMPANY row for marks, a counter
missing, a row without a GUID, a GUID twice). Exit 2 and a NOT READ line: the file is missing or empty,
is not whole UTF-16 (an odd length), or does not parse. Neither is ever "nothing" or "no change".
"""
import re, sys
import xml.etree.ElementTree as ET

COUNTERS = ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS',
            'LASTVCHID', 'LASTMID')
ROW_TAGS = ('COMPANY', 'LEDGER', 'VOUCHER')
IDS = ('GUID', 'MASTERID', 'ALTERID', 'BRIDGECOMPANYGUID')
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)


def not_read(why):
    print(f'NOT READ: {why}')
    sys.exit(2)


def failed(why):
    print(f'FAIL: {why}')
    sys.exit(1)


def raw_text(path):
    try:
        with open(path, 'rb') as source:
            raw = source.read()
    except OSError as e:
        not_read(f'{path}: {type(e).__name__}: {e.strerror}')
    if not raw:
        not_read(f'{path} is empty')
    body = raw[2:] if raw[:2] == b'\xff\xfe' else raw
    if len(body) % 2:
        not_read(f'{path}: {len(body)} bytes after any BOM, an odd length, so not whole UTF-16')
    try:
        return body.decode('utf-16-le')
    except UnicodeDecodeError as e:
        not_read(f'{path} is not UTF-16LE: {e}')


def parse(path):
    try:
        return ET.fromstring(FORBIDDEN.sub('', raw_text(path)))
    except ET.ParseError as e:
        not_read(f'{path} does not parse: {e}')


def collection(path):
    root = parse(path)
    if root.tag != 'ENVELOPE':
        failed(f'{path}: the root is {root.tag}, not ENVELOPE')
    status = root.findtext('HEADER/STATUS')
    if status is None or status.strip() != '1':
        failed(f'{path}: HEADER/STATUS is {status!r}, not 1')
    found = root.findall('BODY/DATA/COLLECTION')
    if len(found) != 1:
        failed(f'{path}: {len(found)} BODY/DATA/COLLECTION elements, not 1')
    return found[0]


def canon(el):
    """An element as a comparable string: tag, attributes, trimmed text, children in order."""
    attrs = ''.join(f' {k}={v!r}' for k, v in sorted(el.attrib.items()))
    inner = ''.join(canon(c) for c in el)
    return f'<{el.tag}{attrs}>{(el.text or "").strip()}{inner}</{el.tag}>'


def rows(path):
    found = [r for r in collection(path) if r.tag in ROW_TAGS]
    out = {}
    for r in found:
        guid = r.findtext('GUID')
        if guid is None or not guid.strip():
            failed(f'a {r.tag} row named {r.get("NAME")!r} has no GUID')
        guid = guid.strip()
        if guid in out:
            failed(f'GUID {guid} appears twice')
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
    companies = [c for c in collection(path) if c.tag == 'COMPANY']
    print('COMPANY rows:', len(companies))
    for c in companies:
        if c.find('GUID') is None or not (c.findtext('GUID') or '').strip():
            failed(f'a COMPANY row named {c.get("NAME")!r} has no GUID')
        print(' ', ' '.join(f'{k} {(c.findtext(k) or "").strip()}' for k in ('GUID', 'ALTVCHID', 'ALTMSTID')))
    if len(companies) != 1:
        failed(f'{len(companies)} COMPANY rows, not 1')


def cmd_import(path):
    root = parse(path)
    if root.tag != 'RESPONSE':
        failed(f'{path}: the root is {root.tag}, not RESPONSE')
    missing = [k for k in COUNTERS if root.find(k) is None]
    for k in COUNTERS:
        el = root.find(k)
        print(f'{k} {"MISSING" if el is None else (el.text or "").strip()}')
    for e in root.iter('LINEERROR'):
        print('LINEERROR', repr(e.text))
    if missing:
        failed(f'counters missing: {missing}')


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


if __name__ == '__main__':
    mode, args = (sys.argv[1], sys.argv[2:]) if len(sys.argv) > 1 else ('', [])
    commands = {'marks': (cmd_marks, 1), 'import': (cmd_import, 1), 'rows': (cmd_rows, 1),
                'diff': (cmd_diff, 2)}
    if mode not in commands or len(args) != commands[mode][1]:
        sys.exit(__doc__)
    commands[mode][0](*args)
