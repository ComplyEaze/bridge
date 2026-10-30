# Lab helper (#2027 step 2, not product code): step 8, reading the raw answers by structure. For each day 3 to 7 June:
# the six words as direct children of ENVELOPE/BODY/DATA/COLLECTION/VOUCHER in the candidate (count, form, raw and
# trimmed text, against the typed values); the cut (each whole element with the CRLF and indent before it) and the size
# check; then candidate-cut against the control in three layers: (a) bytes, (b) bytes with ENVELOPE/BODY/DESC/CMPINFO cut
# from both, (c) parsed trees ignoring only the six. The two 4 June controls compared in the same three layers. The six
# looked for in every control. Writes evidence/analysis.txt and evidence/analysis.json.
import json, re, xml.etree.ElementTree as ET
W = '<LAB_DIR>'; R = W + '/raw'
SIX = ['PLACEOFSUPPLY', 'STATENAME', 'GSTREGISTRATIONTYPE', 'GSTNATUREOFRETURN', 'VATPARTYTRANSRETURNNUMBER', 'VATPARTYTRANSRETURNDATE']
DAYS = ['20250603', '20250604', '20250605', '20250606', '20250607']
FILES = {('control', '20250603'): '000', ('control', '20250604'): '001', ('control', '20250605'): '002', ('control', '20250606'): '003',
         ('control', '20250607'): '004', ('candidate', '20250603'): '005', ('candidate', '20250604'): '006', ('candidate', '20250605'): '007',
         ('candidate', '20250606'): '008', ('candidate', '20250607'): '009'}
EXPECT = {
    '20250603': {'name': 'SV2', 'GSTREGISTRATIONTYPE': 'Regular', 'PLACEOFSUPPLY': 'Maharashtra', 'STATENAME': 'Maharashtra', 'GSTNATUREOFRETURN': 'empty or absent', 'VATPARTYTRANSRETURNNUMBER': 'empty or absent', 'VATPARTYTRANSRETURNDATE': 'empty or absent'},
    '20250604': {'name': 'CN2', 'GSTREGISTRATIONTYPE': 'Regular', 'PLACEOFSUPPLY': 'Maharashtra', 'STATENAME': 'Maharashtra', 'GSTNATUREOFRETURN': '01-Sales Return', 'VATPARTYTRANSRETURNNUMBER': 'SYN-NOTE-1', 'VATPARTYTRANSRETURNDATE': '20250513'},
    '20250605': {'name': 'DN2', 'GSTREGISTRATIONTYPE': 'Regular', 'PLACEOFSUPPLY': 'Maharashtra', 'STATENAME': 'Maharashtra', 'GSTNATUREOFRETURN': '04-Correction in Invoice', 'VATPARTYTRANSRETURNNUMBER': 'SYN-NOTE-2', 'VATPARTYTRANSRETURNDATE': '20250515'},
    '20250606': {'name': 'PV2', 'GSTREGISTRATIONTYPE': 'Regular', 'PLACEOFSUPPLY': 'Maharashtra', 'STATENAME': 'Maharashtra', 'GSTNATUREOFRETURN': 'empty or absent', 'VATPARTYTRANSRETURNNUMBER': 'empty or absent', 'VATPARTYTRANSRETURNDATE': 'empty or absent'},
    '20250607': {'name': 'SV3', 'GSTREGISTRATIONTYPE': 'Regular', 'PLACEOFSUPPLY': 'Karnataka', 'STATENAME': "the party's state (Maharashtra)", 'GSTNATUREOFRETURN': 'empty or absent', 'VATPARTYTRANSRETURNNUMBER': 'empty or absent', 'VATPARTYTRANSRETURNDATE': 'empty or absent'},
}
out = []; res = {}
def p(s): out.append(s); print(s)
def load(kind, day, label=None):
    fn = f'{R}/{FILES[(kind, day)]}-{label or kind + "-" + day}.xml'
    b = open(fn, 'rb').read(); return b, b.decode('utf-16-le')
def tree(t): return ET.fromstring(t.replace('&#4;', ''))
CHILD = lambda n: re.compile(r'\r\n {5}<' + n + r'(?:\s[^>]*)?(?:/>|>[^<]*</' + n + r'>)')
VIND = re.compile(r'\n( *)<VOUCHER ')
CMP = re.compile(r'\r\n {3}<CMPINFO>.*?</CMPINFO>', re.S)
def cut_six(t):
    total = 0
    for n in SIX:
        for m in CHILD(n).finditer(t): total += len(m.group(0))
        t = CHILD(n).sub('', t)
    return t, total
def cut_cmp(t): return CMP.sub('', t)
def first_diff_text(a, b):
    if a == b: return 'equal'
    i = next((k for k in range(min(len(a), len(b))) if a[k] != b[k]), min(len(a), len(b)))
    line = a.count('\r\n', 0, i) + 1
    return f'differs at character {i} (line {line}): control {a[max(0, i - 40):i + 40]!r} / other {b[max(0, i - 40):i + 40]!r}'
def walk(a, b, path, skip_six_at_voucher):
    if a.tag != b.tag: return f'{path}: tag {a.tag} vs {b.tag}'
    if a.attrib != b.attrib: return f'{path}/{a.tag}: attributes {a.attrib} vs {b.attrib}'
    if (a.text or '').strip() != (b.text or '').strip(): return f'{path}/{a.tag}: text {a.text!r} vs {b.text!r}'
    ca, cb = list(a), list(b)
    if skip_six_at_voucher and a.tag == 'VOUCHER' and path.endswith('ENVELOPE/BODY/DATA/COLLECTION'):
        ca = [c for c in ca if c.tag not in SIX]; cb = [c for c in cb if c.tag not in SIX]
    if len(ca) != len(cb): return f'{path}/{a.tag}: {len(ca)} children vs {len(cb)}'
    for x, y in zip(ca, cb):
        d = walk(x, y, f'{path}/{a.tag}', skip_six_at_voucher)
        if d: return d
    return None
def layers(ctl_b, ctl_t, oth_t, oth_is_candidate):
    r = {}
    oth_cut, n = cut_six(oth_t) if oth_is_candidate else (oth_t, 0)
    r['chars_cut'] = n
    r['size'] = {'control_bytes': len(ctl_b), 'other_bytes': len(oth_t.encode('utf-16-le')), 'expected_other_bytes': len(ctl_b) + 2 * n}
    r['size']['equal'] = r['size']['other_bytes'] == r['size']['expected_other_bytes']
    if not r['size']['equal']:
        c2, o2 = cut_cmp(ctl_t), cut_cmp(oth_t)
        r['size_with_cmpinfo_cut'] = len(o2.encode('utf-16-le')) == len(c2.encode('utf-16-le')) + 2 * n
    r['a_bytes'] = 'equal' if oth_cut.encode('utf-16-le') == ctl_b else first_diff_text(ctl_t, oth_cut)
    r['b_bytes_cmpinfo_cut'] = 'equal' if cut_cmp(oth_cut) == cut_cmp(ctl_t) else first_diff_text(cut_cmp(ctl_t), cut_cmp(oth_cut))
    d = walk(tree(ctl_t), tree(oth_t), '', oth_is_candidate)
    r['c_structure'] = 'equal' if d is None else 'first difference ' + d
    return r
p('2027 step 2, step 8: reading the answers by structure (analyse.py).')
for kind in ('control', 'candidate'):
    for d in DAYS:
        b, t = load(kind, d)
        vind = len(re.search(VIND, t).group(1)); kids = bool(CHILD('DATE').search(t))
        p(f'format {kind} {d}: {len(b)} B; BOM {b[:2] == bytes([255, 254])}; line ends CRLF {t.count(chr(13) + chr(10))} of {t.count(chr(10))}; '
          f'indent of VOUCHER {vind} spaces, of its children 5: {kids}')
for d in DAYS:
    e = EXPECT[d]; cb, ct = load('control', d); kb, kt = load('candidate', d)
    p(f'\n== {d} {e["name"]} ==')
    root = tree(kt); vs = root.findall('./BODY/DATA/COLLECTION/VOUCHER'); p(f'candidate VOUCHER rows {len(vs)}; control VOUCHER rows {len(tree(ct).findall("./BODY/DATA/COLLECTION/VOUCHER"))}')
    day = {'voucher': e['name'], 'six': {}}
    for n in SIX:
        els = [c for v in vs for c in v if c.tag == n]
        raws = [m.group(0) for m in CHILD(n).finditer(kt)]
        forms = ['<X/>' if r.endswith('/>') else ('with text' if re.search(r'>[^<]+<', r) else 'empty (open and close)') for r in raws]
        texts = [c.text for c in els]
        trimmed = [(x or '').strip(' \t\r\n') for x in texts]
        typed = e[n]
        if typed == 'empty or absent': match = all(x == '' for x in trimmed)
        elif typed.startswith("the party's"): match = trimmed == ['Maharashtra']
        else: match = trimmed == [typed]
        lead = [x for x in texts if x and x != x.strip(' \t\r\n')]
        in_ctl = len([c for v in tree(ct).findall('./BODY/DATA/COLLECTION/VOUCHER') for c in v if c.tag == n])
        day['six'][n] = {'count': len(els), 'regex_count': len(raws), 'forms': forms, 'attrs': [c.attrib for c in els], 'raw': texts, 'trimmed': trimmed,
                         'typed': typed, 'matches_typed': match, 'leading_or_trailing_space': bool(lead), 'in_control_as_child': in_ctl}
        p(f'  {n}: count {len(els)} (text scan {len(raws)}); form {forms}; attrs {[c.attrib for c in els]}; raw {texts!r}; trimmed {trimmed!r}; '
          f'typed/expected {typed!r}: {"MATCH" if match else "DIFFERS"}; space at an end: {bool(lead)}; in the control as a voucher child: {in_ctl}')
    lay = layers(cb, ct, kt, True); day['layers'] = lay
    p(f'  cut: {lay["chars_cut"]} characters; size check control {lay["size"]["control_bytes"]} + 2 x {lay["chars_cut"]} = {lay["size"]["expected_other_bytes"]} vs candidate {lay["size"]["other_bytes"]}: '
      f'{"equal" if lay["size"]["equal"] else "NOT equal (with CMPINFO cut: " + str(lay.get("size_with_cmpinfo_cut")) + ")"}')
    p(f'  (a) bytes: {lay["a_bytes"]}'); p(f'  (b) bytes, CMPINFO cut: {lay["b_bytes_cmpinfo_cut"]}'); p(f'  (c) structure ignoring the six: {lay["c_structure"]}')
    res[d] = day
cb, ct = load('control', '20250604'); rb = open(f'{R}/010-control-repeat-20250604.xml', 'rb').read(); rt = rb.decode('utf-16-le')
lay = layers(cb, ct, rt, False); res['repeat-20250604'] = lay
p(f'\n== the two 4 June controls ==\n  sha256 equal: {cb == rb}; (a) bytes: {lay["a_bytes"]}; (b) CMPINFO cut: {lay["b_bytes_cmpinfo_cut"]}; (c) structure: {lay["c_structure"]}')
open(W + '/evidence/analysis.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
json.dump(res, open(W + '/evidence/analysis.json', 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
