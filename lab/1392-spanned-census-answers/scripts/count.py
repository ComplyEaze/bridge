# Lab helper (#2164 spanned census, not product code): steps 5 and 6. Decodes each census answer as UTF-16LE, drops only
# the characters XML forbids, parses it, counts VOUCHER nodes that carry attributes or children, and lists GUID, ALTERID,
# DATE, VOUCHERTYPENAME, VOUCHERNUMBER. Then compares each answer's sha256 with earlier-answers.sha256 and, where they
# differ, gives the first differing byte offset and the 2 Nov answer's voucher objects. Writes evidence\count.txt.
import hashlib, re, xml.etree.ElementTree as ET
R = '<lab folder>'; DAYS = ['12', '13', '14', '19', '20', '21', '26', '27', '28', '29']
out = open(R + '/evidence/count.txt', 'w', encoding='utf-8', newline='\n')
def p(s=''): print(s); out.write(s + '\n')
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124-9]|2[0-9]|3[01]);')
def objects(b):
    t = (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le')
    root = ET.fromstring(BAD.sub('', t))
    vs = [v for v in root.iter('VOUCHER') if v.attrib or len(v)]
    return [(v.findtext('GUID'), v.findtext('ALTERID'), v.findtext('DATE'), v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER')) for v in vs]
earlier = {l.split()[1]: l.split()[0] for l in open(R + '/sheet/earlier-answers.sha256', encoding='utf-8')}
for n in ('m1', 'm2'):
    b = open(f'{R}/evidence/{n}.response', 'rb').read(); p(f'{n}: {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
for d in DAYS:
    b = open(f'{R}/evidence/d{d}-c2.response', 'rb').read(); h = hashlib.sha256(b).hexdigest(); obj = objects(b)
    p(f'd{d}-c2: {len(b)} B sha256 {h}; voucher objects {len(obj)}')
    for o in obj: p(f'    GUID {o[0]} ALTERID {o[1].strip() if o[1] else o[1]} DATE {o[2]} TYPE {o[3]} NUMBER {o[4]}')
    e = open(f'{R}/earlier/d{d}-c.response', 'rb').read()
    if h == earlier[f'd{d}-c']: p(f'    equals 2 Nov (d{d}-c)')
    else:
        off = next((i for i, (x, y) in enumerate(zip(b, e)) if x != y), min(len(b), len(e)))
        eo = objects(e)
        p(f'    differs from 2 Nov (d{d}-c, {len(e)} B): first differing byte offset {off}; 2 Nov voucher objects {len(eo)}')
        for o in eo: p(f'      2 Nov: GUID {o[0]} ALTERID {o[1].strip() if o[1] else o[1]} DATE {o[2]} TYPE {o[3]} NUMBER {o[4]}')
