#!/usr/bin/env python3 -I
"""Tables and comparisons for item 628-C0b. Reads answers; sends nothing.

Run: python -I c0b-compare.py EVIDENCE_DIR COMPANY_GUID
It reads ans-r1-narrow-dec.xml, ans-r2-agent-dec.xml, ans-r3-native-dec.xml, ans-r4-narrow-year.xml
and ans-r5-agent-year.xml from EVIDENCE_DIR, writes one table per answer it could read as
<answer>.table.txt beside it, and prints the comparisons and checks. A missing or unreadable answer
is reported and its comparisons are reported as not run; it is never read as an empty answer.
Exit 0 only when all five answers were read, every comparison ran with 0 differences and every
check held; 1 when every answer was read but a comparison or check failed; 2 when an answer was
missing or could not be read, or the arguments are wrong.

Decoding and rows follow LAB-COMMON C4: UTF-16LE with or without a BOM (UTF-8 only with its BOM or
when the bytes hold no NUL); the characters XML forbids are dropped (raw or as references), 9, 10
and 13 are kept; rows are the VOUCHER children of ENVELOPE/BODY/DATA/COLLECTION, and every row's
GUID must begin with COMPANY_GUID. Names and references are never trimmed and print with repr() and
their length, so a trailing space or line break shows. An element that is absent prints as
<absent>; one present and empty prints as ''. A repeated element is kept as all its values in
order. Absent, empty and repeated are never merged.
"""
import hashlib, os, re, sys
import xml.etree.ElementTree as ET

ANSWERS = ['r1-narrow-dec', 'r2-agent-dec', 'r3-native-dec', 'r4-narrow-year', 'r5-agent-year']
SCALARS = ['GUID', 'MASTERID', 'ALTERID', 'DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'REFERENCE',
           'REFERENCEDATE', 'PARTYLEDGERNAME', 'PARTYGSTIN', 'ISOPTIONAL', 'ISCANCELLED', 'ISINVOICE',
           'ISPOSTDATED', 'NARRATION', 'BRIDGEVCHRESERVEDNAME', 'BRIDGEVCHISPURCHASE',
           'BRIDGEVCHISJOURNAL', 'BRIDGEVCHISPAYMENT']
UNTRIMMED = {'PARTYLEDGERNAME', 'NARRATION', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'REFERENCE',
             'LEDGERNAME', 'NAME'}
ABSENT = '<absent>'
DECEMBER_MIN_ROWS = 3  # C0's Purchase, K4 and K5 are certain
MIN_WILDCARD_RATIO = 2  # protocol reference §8: the entry wildcard is about 6x a named list
FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*([0-9a-fA-F]+)|0*([0-9]+));')


def decode(data):
    if data[:2] == b'\xff\xfe':
        return data[2:].decode('utf-16-le')
    if data[:3] == b'\xef\xbb\xbf':
        return data[3:].decode('utf-8')
    if b'\x00' in data[:200]:
        return data.decode('utf-16-le')
    return data.decode('utf-8')


def clean(text):
    def ref(m):
        code = int(m.group(1), 16) if m.group(1) else int(m.group(2))
        return '' if code < 32 and code not in (9, 10, 13) else m.group(0)
    return FORBIDDEN_RAW.sub('', FORBIDDEN_REF.sub(ref, text))


def text_of(el):
    text = el.text or ''
    return text if el.tag in UNTRIMMED else text.strip()


def child_text(el, tag):
    found = el.findall(tag)
    if not found:
        return ABSENT
    if len(found) > 1:
        return tuple(text_of(f) for f in found)
    return text_of(found[0])


def shown(value):
    if value == ABSENT:
        return ABSENT
    if isinstance(value, tuple):
        return 'REPEATED%r' % (value,)
    return '%r(len %d)' % (value, len(value))


def allocation(b):
    # every child of the allocation, by tag and text, so an extra or missing child is a difference
    return tuple(sorted((c.tag, text_of(c)) for c in b))


def entries(voucher, list_tag):
    out = []
    for e in voucher.findall(list_tag):
        bills = tuple(sorted(allocation(b) for b in e.findall('BILLALLOCATIONS.LIST')))
        out.append((child_text(e, 'LEDGERNAME'), child_text(e, 'AMOUNT'),
                    child_text(e, 'ISDEEMEDPOSITIVE'), bills))
    return out


def load(path, company_guid):
    data = open(path, 'rb').read()
    root = ET.fromstring(clean(decode(data)))
    if root.tag != 'ENVELOPE':
        raise ValueError('root is %s, not ENVELOPE (a refusal or an error document)' % root.tag)
    if root.find('./BODY/DATA/COLLECTION') is None:
        raise ValueError('no ENVELOPE/BODY/DATA/COLLECTION (a refusal or an error document)')
    vouchers = {}
    for v in root.findall('./BODY/DATA/COLLECTION/VOUCHER'):
        rec = {tag: child_text(v, tag) for tag in SCALARS}
        rec['@VCHTYPE'] = v.get('VCHTYPE', ABSENT)
        rec['ALL'] = entries(v, 'ALLLEDGERENTRIES.LIST')
        rec['LEDGER'] = entries(v, 'LEDGERENTRIES.LIST')
        guid = rec['GUID']
        if not isinstance(guid, str) or guid in (ABSENT, '') or guid in vouchers:
            raise ValueError('a VOUCHER row with GUID %s repeated or missing' % shown(guid))
        if not guid.startswith(company_guid):
            raise ValueError('a VOUCHER row with GUID %r not of THE COMPANY %r' % (guid, company_guid))
        vouchers[guid] = rec
    return data, vouchers


def number(text):
    try:
        return int(text)
    except (TypeError, ValueError):
        return -1


def per_row(data, vouchers):
    return len(data) // len(vouchers) if vouchers else 0


def table(name, data, vouchers):
    lines = ['%s: %d bytes, sha256 %s, %d VOUCHER rows, %d bytes per row' % (
        name, len(data), hashlib.sha256(data).hexdigest(), len(vouchers), per_row(data, vouchers))]
    for rec in sorted(vouchers.values(), key=lambda r: number(r['MASTERID'])):
        lines.append('VOUCHER ' + ' | '.join('%s=%s' % (t, shown(rec[t])) for t in SCALARS + ['@VCHTYPE']))
        for label in ('ALL', 'LEDGER'):
            for led, amt, pos, bills in rec[label]:
                lines.append('  %s entry %s amount=%s deemed_positive=%s bills=%s' % (
                    label, shown(led), shown(amt), shown(pos), list(bills) if bills else 'none'))
    return '\n'.join(lines) + '\n'


def compare(a_name, a, b_name, b, fields, entry_label):
    diffs = []
    for guid in sorted(set(a) | set(b)):
        if guid not in a or guid not in b:
            diffs.append('GUID %s only in %s' % (guid, a_name if guid in a else b_name))
            continue
        for f in fields:
            if a[guid][f] != b[guid][f]:
                diffs.append('GUID %s %s: %s=%s %s=%s' % (
                    guid, f, a_name, shown(a[guid][f]), b_name, shown(b[guid][f])))
        ea, eb = sorted(a[guid][entry_label], key=repr), sorted(b[guid][entry_label], key=repr)
        if ea != eb:
            diffs.append('GUID %s ledger entries differ:\n  %s %r\n  %s %r' % (guid, a_name, ea, b_name, eb))
    print('%s vs %s: %d rows in %s, %d in %s, %d differences' % (
        a_name, b_name, len(a), a_name, len(b), b_name, len(diffs)))
    for d in diffs:
        print('  DIFF ' + d)
    return len(diffs)


def check(ok, text):
    print('CHECK %s: %s' % ('ok' if ok else 'FAILED', text))
    return 0 if ok else 1


def main(folder, company_guid):
    loaded, sizes, unread = {}, {}, 0
    for name in ANSWERS:
        path = os.path.join(folder, 'ans-%s.xml' % name)
        if not os.path.exists(path):
            print('MISSING %s: its comparisons are not run (a missing answer is never an empty one)' % path)
            unread += 1
            continue
        try:
            data, vouchers = load(path, company_guid)
        except Exception as e:
            print('UNREADABLE %s: %s: its comparisons are not run' % (path, e))
            unread += 1
            continue
        open(path + '.table.txt', 'w', encoding='utf-8').write(table(name, data, vouchers))
        print('read %s: %d bytes, %d VOUCHER rows, %d bytes per row' % (
            name, len(data), len(vouchers), per_row(data, vouchers)))
        loaded[name], sizes[name] = vouchers, per_row(data, vouchers)
    failed = 0
    same = SCALARS + ['@VCHTYPE']
    for narrow, agent, min_rows in (('r1-narrow-dec', 'r2-agent-dec', DECEMBER_MIN_ROWS),
                                    ('r4-narrow-year', 'r5-agent-year', DECEMBER_MIN_ROWS)):
        if narrow not in loaded or agent not in loaded:
            print('NOT RUN %s vs %s: an answer is missing or unreadable' % (narrow, agent))
            continue
        failed += compare(narrow, loaded[narrow], agent, loaded[agent], same, 'ALL')
        failed += check(len(loaded[narrow]) >= min_rows and len(loaded[agent]) >= min_rows,
                        '%s and %s each hold at least %d rows' % (narrow, agent, min_rows))
        failed += check(sizes[agent] >= MIN_WILDCARD_RATIO * sizes[narrow] > 0,
                        '%s bytes per row (%d) at least %d times %s (%d)' % (
                            agent, sizes[agent], MIN_WILDCARD_RATIO, narrow, sizes[narrow]))
    if 'r2-agent-dec' in loaded and 'r3-native-dec' in loaded:
        # The native read carries no class COMPUTEs, so only Tally's own fields are compared, and its
        # ALLLEDGERENTRIES.LIST against the agent read's. LEDGERENTRIES.LIST is printed in the table.
        native = [f for f in same if not f.startswith('BRIDGE')]
        failed += compare('r2-agent-dec', loaded['r2-agent-dec'], 'r3-native-dec', loaded['r3-native-dec'],
                          native, 'ALL')
        notes = 0
        for guid, rec in sorted(loaded['r3-native-dec'].items()):
            only_ledger = sorted(set(rec['LEDGER']) - set(rec['ALL']), key=repr)
            if only_ledger:
                notes += 1
                print('  NOTE GUID %s: %d LEDGERENTRIES.LIST entries not in its ALLLEDGERENTRIES.LIST: %r'
                      % (guid, len(only_ledger), only_ledger))
        print('r3-native-dec: %d vouchers with LEDGERENTRIES.LIST entries outside ALLLEDGERENTRIES.LIST' % notes)
    else:
        print('NOT RUN r2-agent-dec vs r3-native-dec and the LEDGERENTRIES check: an answer is missing or unreadable')
    if unread:
        return 2
    return 1 if failed else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1], sys.argv[2]) if len(sys.argv) == 3 else 2)
