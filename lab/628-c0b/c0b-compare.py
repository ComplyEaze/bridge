#!/usr/bin/env python3 -I
"""Tables and comparisons for item 628-C0b. Reads answers; sends nothing.

Run: python -I c0b-compare.py EVIDENCE_DIR
It reads ans-r1-narrow-dec.xml, ans-r2-agent-dec.xml, ans-r3-native-dec.xml, ans-r4-narrow-year.xml
and ans-r5-agent-year.xml from EVIDENCE_DIR (a missing file is reported and that part is skipped;
it is never read as an empty answer), writes one table per answer as <answer>.table.txt beside it,
and prints the comparisons. Exit 0 when every answer present parsed; 1 when a comparison found a
difference; 2 when an answer could not be read or parsed, or no answer was found.

Decoding and rows follow LAB-COMMON C4: UTF-16LE with or without a BOM (UTF-8 only with its BOM or
when the bytes hold no NUL); the characters XML forbids are dropped (raw or as references), 9, 10
and 13 are kept; rows are the VOUCHER children of ENVELOPE/BODY/DATA/COLLECTION. Names are never
trimmed and print with repr() so a trailing line break shows. An element that is absent prints as
<absent>; one present and empty prints as ''. The two are never merged.
"""
import hashlib, os, re, sys
import xml.etree.ElementTree as ET

ANSWERS = ['r1-narrow-dec', 'r2-agent-dec', 'r3-native-dec', 'r4-narrow-year', 'r5-agent-year']
SCALARS = ['GUID', 'MASTERID', 'ALTERID', 'DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'REFERENCE',
           'REFERENCEDATE', 'PARTYLEDGERNAME', 'PARTYGSTIN', 'ISOPTIONAL', 'ISCANCELLED', 'ISINVOICE',
           'ISPOSTDATED', 'NARRATION', 'BRIDGEVCHRESERVEDNAME', 'BRIDGEVCHISPURCHASE',
           'BRIDGEVCHISJOURNAL', 'BRIDGEVCHISPAYMENT']
NAMES = {'PARTYLEDGERNAME', 'NARRATION', 'VOUCHERTYPENAME', 'LEDGERNAME', 'NAME'}
ABSENT = '<absent>'
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


def child_text(el, tag):
    found = el.findall(tag)
    if not found:
        return ABSENT
    if len(found) > 1:
        return 'REPEATED x%d' % len(found)
    text = found[0].text or ''
    return text if tag in NAMES else text.strip()


def entries(voucher, list_tag):
    out = []
    for e in voucher.findall(list_tag):
        bills = []
        for b in e.findall('BILLALLOCATIONS.LIST'):
            bills.append((child_text(b, 'NAME'), child_text(b, 'BILLTYPE'), child_text(b, 'AMOUNT')))
        out.append((child_text(e, 'LEDGERNAME'), child_text(e, 'AMOUNT'),
                    child_text(e, 'ISDEEMEDPOSITIVE'), tuple(sorted(bills))))
    return out


def load(path):
    data = open(path, 'rb').read()
    root = ET.fromstring(clean(decode(data)))
    if root.tag != 'ENVELOPE':
        raise ValueError('root is %s, not ENVELOPE' % root.tag)
    rows = root.findall('./BODY/DATA/COLLECTION/VOUCHER')
    vouchers = {}
    for v in rows:
        rec = {tag: child_text(v, tag) for tag in SCALARS}
        rec['@VCHTYPE'] = v.get('VCHTYPE', ABSENT)
        rec['ALL'] = entries(v, 'ALLLEDGERENTRIES.LIST')
        rec['LEDGER'] = entries(v, 'LEDGERENTRIES.LIST')
        guid = rec['GUID']
        if guid in vouchers or guid in (ABSENT, ''):
            raise ValueError('a VOUCHER row with GUID %r repeated or missing' % guid)
        vouchers[guid] = rec
    return data, vouchers


def number(text):
    try:
        return int(text)
    except ValueError:
        return -1


def table(name, data, vouchers):
    lines = ['%s: %d bytes, sha256 %s, %d VOUCHER rows, %s bytes per row' % (
        name, len(data), hashlib.sha256(data).hexdigest(), len(vouchers),
        (len(data) // len(vouchers)) if vouchers else 'n/a')]
    for rec in sorted(vouchers.values(), key=lambda r: number(r['MASTERID'])):
        lines.append('VOUCHER ' + ' | '.join(
            '%s=%s' % (t, rec[t] if rec[t] == ABSENT else repr(rec[t])) for t in SCALARS + ['@VCHTYPE']))
        for label in ('ALL', 'LEDGER'):
            for led, amt, pos, bills in rec[label]:
                lines.append('  %s entry %r amount=%s deemed_positive=%s bills=%s' % (
                    label, led, amt, pos, list(bills) if bills else 'none'))
    return '\n'.join(lines) + '\n'


def compare(a_name, a, b_name, b, fields, entry_label):
    diffs = []
    for guid in sorted(set(a) | set(b)):
        if guid not in a or guid not in b:
            diffs.append('GUID %s only in %s' % (guid, a_name if guid in a else b_name))
            continue
        for f in fields:
            if a[guid][f] != b[guid][f]:
                diffs.append('GUID %s %s: %s=%r %s=%r' % (guid, f, a_name, a[guid][f], b_name, b[guid][f]))
        if sorted(a[guid][entry_label]) != sorted(b[guid][entry_label]):
            diffs.append('GUID %s ledger entries differ:\n  %s %r\n  %s %r' % (
                guid, a_name, sorted(a[guid][entry_label]), b_name, sorted(b[guid][entry_label])))
    print('%s vs %s: %d GUIDs in %s, %d in %s, %d differences' % (
        a_name, b_name, len(a), a_name, len(b), b_name, len(diffs)))
    for d in diffs:
        print('  DIFF ' + d)
    return len(diffs)


def main(folder):
    loaded = {}
    for name in ANSWERS:
        path = os.path.join(folder, 'ans-%s.xml' % name)
        if not os.path.exists(path):
            print('MISSING %s: not compared (a missing answer is never an empty one)' % path)
            continue
        try:
            data, vouchers = load(path)
        except Exception as e:
            print('UNREADABLE %s: %s' % (path, e))
            return 2
        open(path + '.table.txt', 'w', encoding='utf-8').write(table(name, data, vouchers))
        print('read %s: %d bytes, %d VOUCHER rows' % (name, len(data), len(vouchers)))
        loaded[name] = vouchers
    if not loaded:
        return 2
    found = 0
    same = SCALARS + ['@VCHTYPE']
    for a, b in (('r1-narrow-dec', 'r2-agent-dec'), ('r4-narrow-year', 'r5-agent-year')):
        if a in loaded and b in loaded:
            found += compare(a, loaded[a], b, loaded[b], same, 'ALL')
    if 'r2-agent-dec' in loaded and 'r3-native-dec' in loaded:
        # The native read carries no class COMPUTEs, so only Tally's own fields are compared, and its
        # ALLLEDGERENTRIES.LIST against the agent read's. LEDGERENTRIES.LIST is printed in the table.
        native = [f for f in same if not f.startswith('BRIDGE')]
        found += compare('r2-agent-dec', loaded['r2-agent-dec'], 'r3-native-dec', loaded['r3-native-dec'],
                         native, 'ALL')
        for guid, rec in sorted(loaded['r3-native-dec'].items()):
            only_ledger = sorted(set(rec['LEDGER']) - set(rec['ALL']))
            if only_ledger:
                print('  NOTE GUID %s: %d LEDGERENTRIES.LIST entries not in its ALLLEDGERENTRIES.LIST: %r'
                      % (guid, len(only_ledger), only_ledger))
    return 1 if found else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1]) if len(sys.argv) == 2 else 2)
