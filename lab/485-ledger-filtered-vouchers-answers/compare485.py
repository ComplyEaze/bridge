# Lab helper (#485, not product code): parse saved UTF-16 voucher answers and compare a ledger-scoped answer
# with the ground-truth window. Parsing reads the text only; the saved bytes are never changed.
# Usage: python -I compare485.py <truth.xml> <candidate.xml> <ledger name> <out.json>
import json
import re
import sys
import xml.etree.ElementTree as ET


def load(path):
    b = open(path, 'rb').read()
    if b[:2] == b'\xff\xfe':
        b = b[2:]
    return b.decode('utf-16-le') if b[1:2] == b'\x00' else b.decode('utf-8')


def parse(path):
    t = load(path)
    # Tally writes &#4; (a control character) in some names; XML 1.0 refuses it, so it is replaced for parsing only.
    root = ET.fromstring(re.sub(r'&#4;', '[4]', t))
    coll = root.find('./BODY/DATA/COLLECTION')
    rows = [] if coll is None else list(coll)
    out = []
    for v in rows:
        def txt(tag):
            e = v.find(tag)
            return (e.text or '').strip() if e is not None else None
        ledgers = sorted({(e.text or '').strip() for e in v.iter('LEDGERNAME')})
        out.append({'tag': v.tag, 'guid': txt('GUID'), 'alterid': txt('ALTERID'), 'masterid': txt('MASTERID'),
                    'date': txt('DATE'), 'type': txt('VOUCHERTYPENAME'), 'number': txt('VOUCHERNUMBER'), 'ledgers': ledgers})
    return out


truth, cand, ledger, out_path = sys.argv[1:5]
rt, rc = parse(truth), parse(cand)
want = {r['guid'] for r in rt if ledger in r['ledgers']}
got = {r['guid'] for r in rc}
res = {
    'truth_rows': len(rt), 'truth_rows_with_ledger': len(want),
    'candidate_rows': len(rc), 'candidate_row_tags': sorted({r['tag'] for r in rc}),
    'candidate_rows_with_guid_alterid_masterid_all_nonempty': sum(1 for r in rc if r['guid'] and r['alterid'] and r['masterid']),
    'sets_equal': want == got,
    'in_truth_not_candidate': sorted(want - got), 'in_candidate_not_truth': sorted(got - want),
    'candidate_rows_table': [{k: r[k] for k in ('guid', 'alterid', 'masterid', 'date', 'type', 'number')} for r in rc],
}
json.dump(res, open(out_path, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(json.dumps({k: v for k, v in res.items() if k != 'candidate_rows_table'}, ensure_ascii=False)[:2000])
