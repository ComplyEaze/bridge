# Lab helper (#2027 R1, not product code): from the relay's saved bytes and the tool answers, by an XML parser: each window request's FETCH; step 1 against the shipped build and step 2 against step 1; each window answer's bytes, sha256, rows, and per voucher each of the six as a direct child (count, TYPE, text); the answer with the six cut against step 1's; the native read-back and the 30 Oct candidates; each tool answer's voucher_gst. Writes evidence/check.txt.
import glob, hashlib, json, os, re, subprocess
import xml.etree.ElementTree as ET
L = '<lab folder>/'; RL = L + 'relay/'
out = open(L + 'evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
SIX = ('PLACEOFSUPPLY', 'STATENAME', 'GSTREGISTRATIONTYPE', 'GSTNATUREOFRETURN', 'VATPARTYTRANSRETURNNUMBER', 'VATPARTYTRANSRETURNDATE')
WORDS = ',' + ','.join(SIX)
def dec(b): return b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff))
def tree(b):
    s = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', dec(b)); s = re.sub(r'&#([0-9]);', '', s); return ET.fromstring(s)
def vouchers(b): return [v for v in tree(b).iter('VOUCHER') if v.attrib]
def six(v): return {t: [(e.attrib.get('TYPE'), e.text) for e in v.findall(t)] for t in SIX}
def key(v): return (v.attrib.get('VCHTYPE'), v.findtext('DATE'))
reqs = []
for f in sorted(glob.glob(RL + 'req-*.bin'), key=lambda x: (int(re.search(r'req-(\d+)-', x).group(1)), x)):
    b = open(f, 'rb').read()
    if b and 'Bridge Agent Vouchers' in dec(b): reqs.append((os.path.basename(f)[4:-4], b))
by = {}
for n, b in reqs: by.setdefault(hashlib.sha256(b).hexdigest(), []).append(n)
p('== window requests: sha256, bytes, sends, FETCH')
for h, ns in by.items():
    b = open(RL + f'req-{ns[0]}.bin', 'rb').read(); s = dec(b)
    p(h[:16], len(b), ns, '|', re.findall(r'<FETCH>([^<]*)', s)[0], '| filter', re.findall(r'<SYSTEM[^>]*>([^<]*)', s))
def first(pred):
    for n, b in reqs:
        if pred(n, dec(b)): return n, b
plain = [x for x in reqs if x[0].startswith('1-') and 'PLACEOFSUPPLY' not in dec(x[1]) and 'IRN' not in dec(x[1])]
ship = [x for x in reqs if x[0].startswith('2-')]
gst = [x for x in reqs if x[0].startswith('1-') and 'PLACEOFSUPPLY' in dec(x[1]) and 'IRN' not in dec(x[1]) and '20250607' not in dec(x[1])]
einv = [x for x in reqs if x[0].startswith('1-') and 'IRN' in dec(x[1]) and '20250607' not in dec(x[1])]
s5 = [x for x in reqs if '20250607' in dec(x[1])]
p('step 1 request byte-identical to the 0.7.0 release build:', plain[0][1] == ship[0][1], plain[0][0], ship[0][0])
p('step 2 request minus the six words equals step 1:', dec(gst[0][1]).replace(WORDS, '') == dec(plain[0][1]), '| words added', len(dec(gst[0][1])) - len(dec(plain[0][1])), 'characters')
p('step 3 request minus the six words equals the e-invoice request shape:', 'IRN,IRNACKNO,IRNACKDATE' + WORDS in dec(einv[0][1]))
def ans(n): return open(RL + f'ans-{n}.bin', 'rb').read()
p('== window answers: bytes, sha256, VOUCHER rows')
for lab, xs in (('step 1', plain), ('step 2 and 4', gst), ('step 3', einv), ('step 5', s5), ('0.7.0 release', ship)):
    for n, _ in xs: a = ans(n); p(lab, n, len(a), hashlib.sha256(a).hexdigest()[:16], 'rows', len(vouchers(a)))
p('== per voucher, step 2 window answer: each of the six as a direct child: count, TYPE, text')
a2 = ans(gst[0][0])
for v in vouchers(a2): p(key(v), json.dumps(six(v), ensure_ascii=False))
native = {key(v): six(v) for v in vouchers(open(L + 'reads/n2.xml', 'rb').read())}
p('== against the native read-back of the seeded vouchers (texts):')
for v in vouchers(a2):
    w = six(v); nat = native[key(v)]
    p(key(v), 'texts equal', {t: [x[1] for x in w[t]] == [x[1] for x in nat[t]] for t in SIX}, '| native TYPEs', {t: [x[0] for x in nat[t]] for t in SIX if nat[t]})
def strip6(b):
    t = tree(b)
    for c in t.iter('CMPINFO'): c.clear()
    for v in t.iter('VOUCHER'):
        for e in [e for e in v if e.tag in SIX]: v.remove(e)
    return ET.tostring(t)
p('step 2 answer with the six cut equals step 1 answer (parsed, CMPINFO cleared):', strip6(a2) == strip6(ans(plain[0][0])))
p('step 2 and step 4 answers byte-equal (every pair):', len({ans(n) for n, _ in gst}) == 1, [n for n, _ in gst])
a3 = ans(einv[0][0])
p('step 3 voucher six equal step 2:', [six(v) for v in vouchers(a3)] == [six(v) for v in vouchers(a2)], '| IRN fields per voucher', [[(t, len(v.findall(t))) for t in ('IRN', 'IRNACKNO', 'IRNACKDATE')] for v in vouchers(a3)][:1])
p('== against the 30 Oct candidates (lab/2027-voucher-gst-fields-fetch-answers at 67039e2b)')
old = {}
for d, n in (('20250603', '005'), ('20250604', '006'), ('20250605', '007'), ('20250606', '008')):
    b = subprocess.run(['git', '-C', 'D:/dev/bridge', 'show', f'67039e2b:lab/2027-voucher-gst-fields-fetch-answers/raw/{n}-candidate-{d}.xml'], capture_output=True).stdout
    for v in vouchers(b): old[key(v)] = six(v)
for v in vouchers(a2):
    w = six(v); o = old.get(key(v))
    p(key(v), 'six equal to 30 Oct, TYPE and text:', w == o, '' if w == o else json.dumps({t: (w[t], o and o[t]) for t in SIX if o is None or w[t] != o[t]}, ensure_ascii=False))
p('== tool answers: voucher_gst per item')
for f in sorted(glob.glob(L + 'run/answers/*-s?.json')) + sorted(glob.glob(L + 'ship/answers/*-s1-shipped.json')):
    d = json.load(open(f, encoding='utf-8')); r = d['response']['result']; x = r['structuredContent']['result']
    p(os.path.basename(f), 'isError', r.get('isError'), 'seconds', d.get('round_trip_seconds'), 'state', x.get('state'), 'total', x.get('total'), 'bytes', os.path.getsize(f))
    for i in x.get('items', []):
        g = i.get('voucher_gst')
        p('   ', i.get('voucher_type'), i.get('date'), json.dumps({k: (v.get('state'), v.get('value')) for k, v in g.items()}, ensure_ascii=False) if g else 'no voucher_gst', '| e_invoice' if 'e_invoice' in i else '')
for n in ('m0', 'm1', 'm3', 'm-s1', 'm-s4', 'm5', 'm6', 'm7'):
    s = open(L + f'reads/{n}.xml', 'rb').read().decode('utf-16-le', 'replace'); p('marks', n, re.findall(r'<(ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', s))
