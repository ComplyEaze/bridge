#!/usr/bin/env python3 -I
"""Compare one ledger's vouchers between two voucher answers (#1356).

Run: python -I compare1356.py LEDGER EXPECTED_COUNT GROUND_TRUTH_ANSWER CANDIDATE_ANSWER

Both answers are Tally collection exports (UTF-16LE, with or without a BOM). Each is decoded,
stripped ONLY of the characters XML forbids (LAB-COMMON C4), and parsed as XML. Rows are the
VOUCHER children of ENVELOPE/BODY/DATA/COLLECTION.

For each voucher, keyed by its GUID, the script takes:
- the voucher's own fields (every direct child element that has no children of its own, except
  the window read's COMPUTE results, named BRIDGEVCH...), names and values exactly as sent, never
  trimmed;
- every ALLLEDGERENTRIES.LIST whose LEDGERNAME equals LEDGER exactly, each as its whole subtree
  in a canonical form (element names, attributes and text, in document order), which includes its
  AMOUNT, ISDEEMEDPOSITIVE and every BILLALLOCATIONS.LIST with its NAME, BILLTYPE, BILLDATE and
  AMOUNT.

The ground truth's vouchers that hold a LEDGER entry are the expected set. EXPECTED_COUNT is how
many there must be; a different number, or none, is an error, so a wrong ledger name or an empty
read can never compare as equal.

Exit codes:
- 0: the candidate holds exactly the expected vouchers, each equal (its own fields and its LEDGER
  entries);
- 1: they differ, each difference listed;
- 2: an error, never a comparison: a file that cannot be read or does not parse, a root other than
  ENVELOPE, a VOUCHER without a GUID or a GUID repeated in one answer, or a ground truth whose
  expected set is not EXPECTED_COUNT.
"""
import re
import sys
import xml.etree.ElementTree as ET

FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')


class Refused(Exception):
    pass


def text_of(path):
    try:
        data = open(path, 'rb').read()
    except OSError as error:
        raise Refused('%s cannot be read: %s' % (path, error))
    if data[:2] == b'\xff\xfe':
        data = data[2:]
    try:
        return data.decode('utf-16-le')
    except UnicodeDecodeError as error:
        raise Refused('%s is not UTF-16LE: %s' % (path, error))


def canonical(element):
    """The element's whole subtree: name, sorted attributes, text, then each child in order."""
    attributes = tuple(sorted(element.attrib.items()))
    return (element.tag, attributes, element.text or '', tuple(canonical(child) for child in element))


def vouchers(path, ledger):
    text = FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text_of(path)))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        raise Refused('%s does not parse: %s' % (path, error))
    if root.tag != 'ENVELOPE':
        raise Refused('%s: the root is %s, not ENVELOPE' % (path, root.tag))
    rows = root.findall('./BODY/DATA/COLLECTION/VOUCHER')
    table = {}
    for voucher in rows:
        guid_element = voucher.find('GUID')
        guid = None if guid_element is None else (guid_element.text or '').strip()
        if not guid:
            raise Refused('%s: a VOUCHER has no GUID' % path)
        if guid in table:
            raise Refused('%s: GUID %s is repeated' % (path, guid))
        # The window read's COMPUTE results (BRIDGEVCH...) are not fetched fields: the
        # candidate carries no COMPUTE lines, so they are left out on both sides.
        fields = tuple(
            (child.tag, child.text or '')
            for child in voucher
            if len(child) == 0 and not child.tag.endswith('.LIST') and not child.tag.startswith('BRIDGEVCH')
        )
        entries = tuple(
            canonical(entry)
            for entry in voucher.findall('ALLLEDGERENTRIES.LIST')
            if entry.find('LEDGERNAME') is not None and (entry.find('LEDGERNAME').text or '') == ledger
        )
        table[guid] = (fields, entries)
    return len(rows), table


def main():
    if len(sys.argv) != 5:
        print(__doc__)
        return 2
    ledger, expected_text, truth_path, candidate_path = sys.argv[1:]
    try:
        expected = int(expected_text)
        truth_rows, truth = vouchers(truth_path, ledger)
        candidate_rows, candidate = vouchers(candidate_path, ledger)
    except (ValueError, Refused) as error:
        print('ERROR: %s' % error)
        return 2
    wanted = {guid: voucher for guid, voucher in truth.items() if voucher[1]}
    print('ground truth: %d VOUCHER rows, %d with a %r entry' % (truth_rows, len(wanted), ledger))
    print('candidate:    %d VOUCHER rows, %d with a %r entry' % (candidate_rows, sum(1 for v in candidate.values() if v[1]), ledger))
    if len(wanted) != expected or expected == 0:
        print('ERROR: the ground truth holds %d vouchers with a %r entry, not the %d expected' % (len(wanted), ledger, expected))
        return 2
    ok = True
    for guid in sorted(set(wanted) | set(candidate)):
        if guid not in candidate:
            print('ONLY IN GROUND TRUTH  %s' % guid)
            ok = False
        elif guid not in wanted:
            print('NOT EXPECTED          %s  (in the ground truth without a %r entry: %s)' % (guid, ledger, guid in truth))
            ok = False
        elif candidate[guid] != wanted[guid]:
            truth_fields, truth_entries = wanted[guid]
            fields, entries = candidate[guid]
            print('DIFFERENT             %s' % guid)
            if fields != truth_fields:
                print('  fields    truth %r\n            candidate %r' % (truth_fields, fields))
            if entries != truth_entries:
                print('  entries   truth %r\n            candidate %r' % (truth_entries, entries))
            ok = False
        else:
            print('equal                 %s  %d %r entr%s' % (guid, len(wanted[guid][1]), ledger, 'y' if len(wanted[guid][1]) == 1 else 'ies'))
    print('RESULT: %s' % ('all %d equal' % len(wanted) if ok else 'differences above'))
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
