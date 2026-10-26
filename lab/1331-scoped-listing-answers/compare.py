# Lab helper (#1331, not product code): parse LEDGER rows from saved UTF-16 listing answers and compare them.
# Parsing reads the text only; the saved bytes are never changed. &#4; references are kept as text.
import json
import re
import sys

FOUR = ['duties & taxes', 'indirect expenses', 'sales accounts', 'sundry debtors']


def load(path):
    b = open(path, 'rb').read()
    if b[:2] == b'\xff\xfe':
        b = b[2:]
    return b.decode('utf-16-le') if b[1:2] == b'\x00' else b.decode('utf-8')


def unesc(s):
    return s.replace('&amp;', '&').replace('&lt;', '<').replace('&gt;', '>').replace('&quot;', '"').replace('&apos;', "'")


def rows(path):
    t = load(path)
    out = []
    for m in re.finditer(r'^(\s*)<LEDGER NAME="([^"]*)"[^>]*>\r?\n(.*?)^\1</LEDGER>', t, re.S | re.M):
        indent = m.group(1) + ' '
        body = m.group(3)
        children = {}
        order = []
        cur = None
        for line in body.splitlines():
            if line.startswith(indent + '<') and not line.startswith(indent + ' ') and not line.startswith(indent + '</'):
                tag = re.match(r'<([A-Za-z0-9_.]+)', line[len(indent):]).group(1)
                k = tag
                n = 1
                while k in children:
                    n += 1
                    k = '%s#%d' % (tag, n)
                children[k] = line.strip()
                order.append(k)
                cur = k
            elif cur:
                children[cur] += '\n' + line.strip()
        def val(tag):
            v = children.get(tag, '')
            mm = re.match(r'<[^>]*>(.*?)</', v, re.S)
            return unesc(mm.group(1).strip()) if mm else ''
        out.append({'name': unesc(m.group(2)), 'guid': val('GUID'), 'parent': val('PARENT'), 'alterid': val('ALTERID'), 'children': children, 'order': order})
    return out


def part_p(p02, q1):
    rp, rq = rows(p02), rows(q1)
    gp = {r['guid']: r for r in rp}
    gq = {r['guid']: r for r in rq}
    res = {'p02_rows': len(rp), 'q1_rows': len(rq)}
    res['q1_guids_missing_from_p02'] = [g for g in gq if g not in gp]
    res['p02_rows_not_in_q1'] = [{'guid': r['guid'], 'name': r['name'], 'parent': r['parent'], 'alterid': r['alterid']} for r in rp if r['guid'] not in gq]
    res['p02_rows_parent_outside_four'] = [{'guid': r['guid'], 'name': r['name'], 'parent': r['parent']} for r in rp if r['parent'].lower() not in FOUR]
    res['p02_parents_as_returned'] = sorted({r['parent'] for r in rp})
    comp = []
    q1_only = set()
    for g, a in gq.items():
        if g not in gp:
            continue
        b = gp[g]
        only_q = [k for k in a['children'] if k not in b['children']]
        only_p = [k for k in b['children'] if k not in a['children']]
        diff = [k for k in a['children'] if k in b['children'] and a['children'][k] != b['children'][k]]
        q1_only.update(re.sub(r'#\d+$', '', k) for k in only_q)
        comp.append({'guid': g, 'name': b['name'], 'parent': b['parent'], 'alterid_q1': a['alterid'], 'alterid_now': b['alterid'],
                     'elements_only_in_q1': only_q, 'elements_only_now': only_p, 'elements_differing': diff,
                     'differing_values': {k: {'q1': a['children'][k], 'now': b['children'][k]} for k in diff}})
    res['common_guid_comparison'] = comp
    res['element_tags_only_in_q1_any_row'] = sorted(q1_only)
    co = {r['guid'].rsplit('-', 1)[0] for r in rp if r['guid']}
    res['company_guid_prefixes_in_p02'] = sorted(co)
    return res


def part_d(d02, d03):
    r2, r3 = rows(d02), rows(d03)
    exp = {r['guid'] for r in r2 if r['parent'].lower() in FOUR}
    got = {r['guid'] for r in r3}
    by2 = {r['guid']: r for r in r2}
    by3 = {r['guid']: r for r in r3}
    res = {'d02_rows': len(r2), 'd03_rows': len(r3), 'd02_rows_under_four_parents_casefolded': len(exp)}
    res['missing_from_d03'] = [{'guid': g, 'name': by2[g]['name'], 'parent': by2[g]['parent']} for g in sorted(exp - got)]
    res['extra_in_d03'] = [{'guid': g, 'name': by3[g]['name'], 'parent': by3[g]['parent']} for g in sorted(got - exp)]
    pc = {}
    for r in r2:
        pc[r['parent']] = pc.get(r['parent'], 0) + 1
    res['d02_parent_counts'] = dict(sorted(pc.items(), key=lambda kv: -kv[1]))
    pc3 = {}
    for r in r3:
        pc3[r['parent']] = pc3.get(r['parent'], 0) + 1
    res['d03_parent_counts'] = pc3
    same = sum(1 for g in got & exp if by2[g]['children'] == by3[g]['children'])
    res['d03_rows_identical_to_d02_row'] = same
    res['d02_tags'] = sorted({re.sub(r'#\d+$', '', k) for r in r2 for k in r['children']})
    res['d03_tags'] = sorted({re.sub(r'#\d+$', '', k) for r in r3 for k in r['children']})
    return res


if __name__ == '__main__':
    mode = sys.argv[1]
    out = part_p(sys.argv[2], sys.argv[3]) if mode == 'P' else part_d(sys.argv[2], sys.argv[3])
    json.dump(out, open(sys.argv[4], 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
    print(json.dumps({k: (v if not isinstance(v, list) or len(v) < 12 else '%d items' % len(v)) for k, v in out.items() if k not in ('common_guid_comparison',)}, ensure_ascii=False, indent=1)[:4000])
